use core::ptr;
use std::time::{Duration, Instant};

use libc::{free, malloc, memcpy, size_t};

use crate::lib::common::bits::{ZSTD_NbCommonBytes, ZSTD_highbit32};
use crate::lib::common::error_private::{ERR_getErrorName, ERR_isError, Error};
use crate::lib::common::huf::{HUF_CElt, HUF_WORKSPACE_SIZE};
use crate::lib::common::mem::{MEM_read16, MEM_read64, MEM_readLE32, MEM_readST, MEM_writeLE32};
use crate::lib::common::xxhash::ZSTD_XXH64;
use crate::lib::common::zstd_internal::{
    repStartValue, LLFSELog, MLFSELog, MaxLL, MaxML, OffFSELog, ZSTD_REP_NUM,
};
use crate::lib::compress::fse_compress::{FSE_normalizeCount, FSE_writeNCount};
use crate::lib::compress::huf_compress::{HUF_buildCTable_wksp, HUF_writeCTable_wksp};
use crate::lib::compress::zstd_compress::{
    SeqDef, ZSTD_CCtx, ZSTD_CDict, ZSTD_compressBegin_usingCDict_deprecated,
    ZSTD_compressBlock_deprecated, ZSTD_compressedBlockState_t, ZSTD_createCCtx,
    ZSTD_createCDict_advanced, ZSTD_freeCCtx, ZSTD_freeCDict, ZSTD_getParams, ZSTD_getSeqStore,
    ZSTD_loadCEntropy, ZSTD_reset_compressedBlockState, ZSTD_seqToCodes,
};
use crate::lib::dictBuilder::divsufsort::divsufsort;
use crate::lib::dictBuilder::fastcover::ZDICT_optimizeTrainFromBuffer_fastCover;
#[expect(deprecated)]
use crate::lib::zdict::experimental::{
    ZDICT_fastCover_params_t, ZDICT_legacy_params_t, ZDICT_CONTENTSIZE_MIN, ZDICT_DICTSIZE_MIN,
};
use crate::lib::zdict::ZDICT_params_t;
use crate::lib::zstd::*;

#[derive(Copy, Clone)]
#[repr(C)]
struct EStats_ress_t {
    dict: *mut ZSTD_CDict,
    zc: *mut ZSTD_CCtx,
    workPlace: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct offsetCount_t {
    offset: u32,
    count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct dictItem {
    pos: u32,
    length: u32,
    savings: u32,
}
const MINRATIO: core::ffi::c_int = 4;
const ZDICT_MAX_SAMPLES_SIZE: core::ffi::c_uint = (2000) << 20;
#[expect(deprecated)]
const ZDICT_MIN_SAMPLES_SIZE: core::ffi::c_int = ZDICT_CONTENTSIZE_MIN * MINRATIO;

const NOISELENGTH: core::ffi::c_int = 32;
static g_selectivity_default: u32 = 9;
unsafe fn ZDICT_printHex(ptr: *const core::ffi::c_void, length: size_t) {
    let b = ptr as *const u8;
    let mut u: size_t = 0;
    u = 0;
    while u < length {
        let mut c = *b.add(u);
        if (c as core::ffi::c_int) < 32 || c as core::ffi::c_int > 126 {
            c = b'.';
        }
        eprint!("{}", char::from(c));
        u = u.wrapping_add(1);
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_isError))]
pub extern "C" fn ZDICT_isError(errorCode: size_t) -> core::ffi::c_uint {
    ERR_isError(errorCode) as _
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_getErrorName))]
pub extern "C" fn ZDICT_getErrorName(errorCode: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(errorCode)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_getDictID))]
pub unsafe extern "C" fn ZDICT_getDictID(
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
) -> core::ffi::c_uint {
    if dictSize < 8 {
        return 0;
    }
    if MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return 0;
    }
    MEM_readLE32((dictBuffer as *const core::ffi::c_char).add(4) as *const core::ffi::c_void)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_getDictHeaderSize))]
pub unsafe extern "C" fn ZDICT_getDictHeaderSize(
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut headerSize: size_t = 0;
    if dictSize <= 8 || MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return Error::dictionary_corrupted.to_error_code();
    }
    let bs = malloc(::core::mem::size_of::<ZSTD_compressedBlockState_t>())
        as *mut ZSTD_compressedBlockState_t;
    let wksp = malloc(HUF_WORKSPACE_SIZE as size_t) as *mut u32;
    if bs.is_null() || wksp.is_null() {
        headerSize = Error::memory_allocation.to_error_code();
    } else {
        ZSTD_reset_compressedBlockState(bs);
        headerSize = ZSTD_loadCEntropy(bs, wksp as *mut core::ffi::c_void, dictBuffer, dictSize);
    }
    free(bs as *mut core::ffi::c_void);
    free(wksp as *mut core::ffi::c_void);
    headerSize
}

unsafe fn ZDICT_count(
    mut pIn: *const core::ffi::c_void,
    mut pMatch: *const core::ffi::c_void,
) -> size_t {
    let pStart = pIn as *const core::ffi::c_char;
    loop {
        let diff = MEM_readST(pMatch) ^ MEM_readST(pIn);
        if diff == 0 {
            pIn = (pIn as *const core::ffi::c_char).add(::core::mem::size_of::<size_t>())
                as *const core::ffi::c_void;
            pMatch = (pMatch as *const core::ffi::c_char).add(::core::mem::size_of::<size_t>())
                as *const core::ffi::c_void;
        } else {
            pIn = (pIn as *const core::ffi::c_char).offset(ZSTD_NbCommonBytes(diff) as isize)
                as *const core::ffi::c_void;
            return (pIn as *const core::ffi::c_char).offset_from(pStart) as core::ffi::c_long
                as size_t;
        }
    }
}

unsafe fn ZDICT_initDictItem(d: *mut dictItem) {
    (*d).pos = 1;
    (*d).length = 0;
    (*d).savings = -(1 as core::ffi::c_int) as u32;
}
const LLIMIT: core::ffi::c_int = 64;
const MINMATCHLENGTH: core::ffi::c_int = 7;
unsafe fn ZDICT_analyzePos(
    doneMarks: *mut u8,
    suffix: *const core::ffi::c_uint,
    mut start: u32,
    buffer: *const core::ffi::c_void,
    minRatio: u32,
    notificationLevel: u32,
) -> dictItem {
    let mut lengthList: [u32; 64] = [0; 64];
    let mut cumulLength: [u32; 64] = [0; 64];
    let mut savings: [u32; 64] = [0; 64];
    let b = buffer as *const u8;
    let mut maxLength = LLIMIT as size_t;
    let mut pos = *suffix.offset(start as isize) as size_t;
    let mut end = start;
    let mut solution = dictItem {
        pos: 0,
        length: 0,
        savings: 0,
    };
    ptr::write_bytes(
        &mut solution as *mut dictItem as *mut u8,
        0,
        ::core::mem::size_of::<dictItem>(),
    );
    *doneMarks.add(pos) = 1;
    if MEM_read16(b.add(pos) as *const core::ffi::c_void) as core::ffi::c_int
        == MEM_read16(b.add(pos).add(2) as *const core::ffi::c_void) as core::ffi::c_int
        || MEM_read16(b.add(pos).add(1) as *const core::ffi::c_void) as core::ffi::c_int
            == MEM_read16(b.add(pos).add(3) as *const core::ffi::c_void) as core::ffi::c_int
        || MEM_read16(b.add(pos).add(2) as *const core::ffi::c_void) as core::ffi::c_int
            == MEM_read16(b.add(pos).add(4) as *const core::ffi::c_void) as core::ffi::c_int
    {
        let pattern16 = MEM_read16(b.add(pos).add(4) as *const core::ffi::c_void);
        let mut u: u32 = 0;
        let mut patternEnd = 6u32;
        while MEM_read16(b.add(pos).offset(patternEnd as isize) as *const core::ffi::c_void)
            as core::ffi::c_int
            == pattern16 as core::ffi::c_int
        {
            patternEnd = patternEnd.wrapping_add(2);
        }
        if *b.add(pos.wrapping_add(patternEnd as size_t)) as core::ffi::c_int
            == *b.add(pos.wrapping_add(patternEnd as size_t).wrapping_sub(1)) as core::ffi::c_int
        {
            patternEnd = patternEnd.wrapping_add(1);
        }
        u = 1;
        while u < patternEnd {
            *doneMarks.add(pos.wrapping_add(u as size_t)) = 1;
            u = u.wrapping_add(1);
        }
        return solution;
    }
    let mut length: size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length = ZDICT_count(
            b.add(pos) as *const core::ffi::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const core::ffi::c_void,
        );
        if length < MINMATCHLENGTH as size_t {
            break;
        }
    }
    let mut length_0: size_t = 0;
    loop {
        length_0 = ZDICT_count(
            b.add(pos) as *const core::ffi::c_void,
            b.offset(*suffix.offset(start as isize).sub(1) as isize) as *const core::ffi::c_void,
        );
        if length_0 >= MINMATCHLENGTH as size_t {
            start = start.wrapping_sub(1);
        }
        if length_0 < MINMATCHLENGTH as size_t {
            break;
        }
    }
    if end.wrapping_sub(start) < minRatio {
        let mut idx: u32 = 0;
        idx = start;
        while idx < end {
            *doneMarks.offset(*suffix.offset(idx as isize) as isize) = 1;
            idx = idx.wrapping_add(1);
        }
        return solution;
    }
    let mut i: core::ffi::c_int = 0;
    let mut mml: u32 = 0;
    let mut refinedStart = start;
    let mut refinedEnd = end;
    if notificationLevel >= 4 {
        eprintln!();
    }
    if notificationLevel >= 4 {
        eprint!(
            "found {:>3} matches of length >= {} at pos {:>7}  ",
            end.wrapping_sub(start),
            7,
            pos as core::ffi::c_uint,
        );
    }
    if notificationLevel >= 4 {
        eprintln!();
    }
    mml = MINMATCHLENGTH as u32;
    loop {
        let mut currentChar = 0;
        let mut currentCount = 0u32;
        let mut currentID = refinedStart;
        let mut id: u32 = 0;
        let mut selectedCount = 0;
        let mut selectedID = currentID;
        id = refinedStart;
        while id < refinedEnd {
            if *b.offset((*suffix.offset(id as isize)).wrapping_add(mml) as isize)
                as core::ffi::c_int
                != currentChar as core::ffi::c_int
            {
                if currentCount > selectedCount {
                    selectedCount = currentCount;
                    selectedID = currentID;
                }
                currentID = id;
                currentChar = *b.offset((*suffix.offset(id as isize)).wrapping_add(mml) as isize);
                currentCount = 0;
            }
            currentCount = currentCount.wrapping_add(1);
            id = id.wrapping_add(1);
        }
        if currentCount > selectedCount {
            selectedCount = currentCount;
            selectedID = currentID;
        }
        if selectedCount < minRatio {
            break;
        }
        refinedStart = selectedID;
        refinedEnd = refinedStart.wrapping_add(selectedCount);
        mml = mml.wrapping_add(1);
    }
    start = refinedStart;
    pos = *suffix.offset(refinedStart as isize) as size_t;
    end = start;
    ptr::write_bytes(
        lengthList.as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    let mut length_1: size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length_1 = ZDICT_count(
            b.add(pos) as *const core::ffi::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const core::ffi::c_void,
        );
        if length_1 >= LLIMIT as size_t {
            length_1 = (LLIMIT - 1) as size_t;
        }
        let fresh0 = &mut (*lengthList.as_mut_ptr().add(length_1));
        *fresh0 = (*fresh0).wrapping_add(1);
        if length_1 < MINMATCHLENGTH as size_t {
            break;
        }
    }
    let mut length_2 = MINMATCHLENGTH as size_t;
    while (length_2 >= MINMATCHLENGTH as size_t) as core::ffi::c_int
        & (start > 0) as core::ffi::c_int
        != 0
    {
        length_2 = ZDICT_count(
            b.add(pos) as *const core::ffi::c_void,
            b.offset(*suffix.offset(start.wrapping_sub(1) as isize) as isize)
                as *const core::ffi::c_void,
        );
        if length_2 >= LLIMIT as size_t {
            length_2 = (LLIMIT - 1) as size_t;
        }
        let fresh1 = &mut (*lengthList.as_mut_ptr().add(length_2));
        *fresh1 = (*fresh1).wrapping_add(1);
        if length_2 >= MINMATCHLENGTH as size_t {
            start = start.wrapping_sub(1);
        }
    }
    ptr::write_bytes(
        cumulLength.as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    *cumulLength.as_mut_ptr().add(maxLength.wrapping_sub(1)) =
        *lengthList.as_mut_ptr().add(maxLength.wrapping_sub(1));
    i = maxLength.wrapping_sub(2) as core::ffi::c_int;
    while i >= 0 {
        *cumulLength.as_mut_ptr().offset(i as isize) =
            (*cumulLength.as_mut_ptr().offset((i + 1) as isize))
                .wrapping_add(*lengthList.as_mut_ptr().offset(i as isize));
        i -= 1;
    }
    let mut u_0: core::ffi::c_uint = 0;
    u_0 = (LLIMIT - 1) as core::ffi::c_uint;
    while u_0 >= MINMATCHLENGTH as core::ffi::c_uint {
        if *cumulLength.as_mut_ptr().offset(u_0 as isize) >= minRatio {
            break;
        }
        u_0 = u_0.wrapping_sub(1);
    }
    maxLength = u_0 as size_t;
    let mut l = maxLength as u32;
    let c = *b.add(pos.wrapping_add(maxLength).wrapping_sub(1));
    while *b.add(pos.wrapping_add(l as size_t).wrapping_sub(2)) as core::ffi::c_int
        == c as core::ffi::c_int
    {
        l = l.wrapping_sub(1);
    }
    maxLength = l as size_t;
    if maxLength < MINMATCHLENGTH as size_t {
        return solution;
    }
    *savings.as_mut_ptr().add(5) = 0;
    let mut u_1: core::ffi::c_uint = 0;
    u_1 = MINMATCHLENGTH as core::ffi::c_uint;
    while u_1 as size_t <= maxLength {
        *savings.as_mut_ptr().offset(u_1 as isize) =
            (*savings.as_mut_ptr().offset(u_1.wrapping_sub(1) as isize)).wrapping_add(
                (*lengthList.as_mut_ptr().offset(u_1 as isize)).wrapping_mul(u_1.wrapping_sub(3)),
            );
        u_1 = u_1.wrapping_add(1);
    }
    if notificationLevel >= 4 {
        eprintln!(
            "Selected dict at position {}, of length {} : saves {} (ratio: {:.2})  ",
            pos,
            maxLength,
            *savings.as_mut_ptr().add(maxLength),
            *savings.as_mut_ptr().add(maxLength) as core::ffi::c_double
                / maxLength as core::ffi::c_double,
        );
    }
    solution.pos = pos as u32;
    solution.length = maxLength as u32;
    solution.savings = *savings.as_mut_ptr().add(maxLength);
    let mut id_0: u32 = 0;
    id_0 = start;
    while id_0 < end {
        let mut p: u32 = 0;
        let mut pEnd: u32 = 0;
        let mut length_3: u32 = 0;
        let testedPos = *suffix.offset(id_0 as isize);
        if testedPos as size_t == pos {
            length_3 = solution.length;
        } else {
            length_3 = ZDICT_count(
                b.add(pos) as *const core::ffi::c_void,
                b.offset(testedPos as isize) as *const core::ffi::c_void,
            ) as u32;
            if length_3 > solution.length {
                length_3 = solution.length;
            }
        }
        pEnd = testedPos.wrapping_add(length_3);
        p = testedPos;
        while p < pEnd {
            *doneMarks.offset(p as isize) = 1;
            p = p.wrapping_add(1);
        }
        id_0 = id_0.wrapping_add(1);
    }
    solution
}
unsafe fn isIncluded(
    in_0: *const core::ffi::c_void,
    container: *const core::ffi::c_void,
    length: size_t,
) -> core::ffi::c_int {
    let ip = in_0 as *const core::ffi::c_char;
    let into = container as *const core::ffi::c_char;
    let mut u: size_t = 0;
    u = 0;
    while u < length {
        if *ip.add(u) as core::ffi::c_int != *into.add(u) as core::ffi::c_int {
            break;
        }
        u = u.wrapping_add(1);
    }
    (u == length) as core::ffi::c_int
}
unsafe fn ZDICT_tryMerge(
    table: *mut dictItem,
    mut elt: dictItem,
    eltNbToSkip: u32,
    buffer: *const core::ffi::c_void,
) -> u32 {
    let tableSize = (*table).pos;
    let eltEnd = (elt.pos).wrapping_add(elt.length);
    let buf = buffer as *const core::ffi::c_char;
    let mut u: u32 = 0;
    u = 1;
    while u < tableSize {
        if (u != eltNbToSkip)
            && (*table.offset(u as isize)).pos > elt.pos
            && (*table.offset(u as isize)).pos <= eltEnd
        {
            let addedLength = ((*table.offset(u as isize)).pos).wrapping_sub(elt.pos);
            let fresh2 = &mut (*table.offset(u as isize)).length;
            *fresh2 = (*fresh2).wrapping_add(addedLength);
            (*table.offset(u as isize)).pos = elt.pos;
            let fresh3 = &mut (*table.offset(u as isize)).savings;
            *fresh3 = (*fresh3).wrapping_add(elt.savings * addedLength / elt.length);
            let fresh4 = &mut (*table.offset(u as isize)).savings;
            *fresh4 = (*fresh4).wrapping_add(elt.length / 8);
            elt = *table.offset(u as isize);
            while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                u = u.wrapping_sub(1);
            }
            *table.offset(u as isize) = elt;
            return u;
        }
        u = u.wrapping_add(1);
    }
    u = 1;
    while u < tableSize {
        if u != eltNbToSkip {
            if ((*table.offset(u as isize)).pos).wrapping_add((*table.offset(u as isize)).length)
                >= elt.pos
                && (*table.offset(u as isize)).pos < elt.pos
            {
                let addedLength_0 = eltEnd as core::ffi::c_int
                    - ((*table.offset(u as isize)).pos)
                        .wrapping_add((*table.offset(u as isize)).length)
                        as core::ffi::c_int;
                let fresh5 = &mut (*table.offset(u as isize)).savings;
                *fresh5 = (*fresh5).wrapping_add(elt.length / 8);
                if addedLength_0 > 0 {
                    let fresh6 = &mut (*table.offset(u as isize)).length;
                    *fresh6 = (*fresh6 as core::ffi::c_uint)
                        .wrapping_add(addedLength_0 as core::ffi::c_uint);
                    let fresh7 = &mut (*table.offset(u as isize)).savings;
                    *fresh7 = (*fresh7 as core::ffi::c_uint).wrapping_add(
                        (elt.savings)
                            .wrapping_mul(addedLength_0 as core::ffi::c_uint)
                            .wrapping_div(elt.length),
                    );
                }
                elt = *table.offset(u as isize);
                while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                    *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                    u = u.wrapping_sub(1);
                }
                *table.offset(u as isize) = elt;
                return u;
            }
            if MEM_read64(
                buf.offset((*table.offset(u as isize)).pos as isize) as *const core::ffi::c_void
            ) == MEM_read64(buf.offset(elt.pos as isize).add(1) as *const core::ffi::c_void)
                && isIncluded(
                    buf.offset((*table.offset(u as isize)).pos as isize)
                        as *const core::ffi::c_void,
                    buf.offset(elt.pos as isize).add(1) as *const core::ffi::c_void,
                    (*table.offset(u as isize)).length as size_t,
                ) != 0
            {
                let addedLength_1 =
                    (if (elt.length).wrapping_sub((*table.offset(u as isize)).length) > 1 {
                        (elt.length).wrapping_sub((*table.offset(u as isize)).length)
                    } else {
                        1
                    }) as size_t;
                (*table.offset(u as isize)).pos = elt.pos;
                let fresh8 = &mut (*table.offset(u as isize)).savings;
                *fresh8 = (*fresh8).wrapping_add(
                    (elt.savings as size_t * addedLength_1 / elt.length as size_t) as u32,
                );
                (*table.offset(u as isize)).length =
                    if elt.length < ((*table.offset(u as isize)).length).wrapping_add(1) {
                        elt.length
                    } else {
                        ((*table.offset(u as isize)).length).wrapping_add(1)
                    };
                return u;
            }
        }
        u = u.wrapping_add(1);
    }
    0
}
unsafe fn ZDICT_removeDictItem(table: *mut dictItem, id: u32) {
    let max = (*table).pos;
    let mut u: u32 = 0;
    if id == 0 {
        return;
    }
    u = id;
    while u < max.wrapping_sub(1) {
        *table.offset(u as isize) = *table.offset(u.wrapping_add(1) as isize);
        u = u.wrapping_add(1);
    }
    (*table).pos = ((*table).pos).wrapping_sub(1);
}
unsafe fn ZDICT_insertDictItem(
    table: *mut dictItem,
    maxSize: u32,
    elt: dictItem,
    buffer: *const core::ffi::c_void,
) {
    let mut mergeId = ZDICT_tryMerge(table, elt, 0, buffer);
    if mergeId != 0 {
        let mut newMerge = 1;
        while newMerge != 0 {
            newMerge = ZDICT_tryMerge(table, *table.offset(mergeId as isize), mergeId, buffer);
            if newMerge != 0 {
                ZDICT_removeDictItem(table, mergeId);
            }
            mergeId = newMerge;
        }
        return;
    }
    let mut current: u32 = 0;
    let mut nextElt = (*table).pos;
    if nextElt >= maxSize {
        nextElt = maxSize.wrapping_sub(1);
    }
    current = nextElt.wrapping_sub(1);
    while (*table.offset(current as isize)).savings < elt.savings {
        *table.offset(current.wrapping_add(1) as isize) = *table.offset(current as isize);
        current = current.wrapping_sub(1);
    }
    *table.offset(current.wrapping_add(1) as isize) = elt;
    (*table).pos = nextElt.wrapping_add(1);
}
unsafe fn ZDICT_dictSize(dictList: *const dictItem) -> u32 {
    let mut u: u32 = 0;
    let mut dictSize = 0u32;
    u = 1;
    while u < (*dictList).pos {
        dictSize = dictSize.wrapping_add((*dictList.offset(u as isize)).length);
        u = u.wrapping_add(1);
    }
    dictSize
}
unsafe fn ZDICT_trainBuffer_legacy(
    dictList: *mut dictItem,
    dictListSize: u32,
    buffer: *const core::ffi::c_void,
    mut bufferSize: size_t,
    fileSizes: *const size_t,
    mut nbFiles: core::ffi::c_uint,
    mut minRatio: core::ffi::c_uint,
    notificationLevel: u32,
) -> size_t {
    let suffix0 = malloc(
        bufferSize
            .wrapping_add(2)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
    ) as *mut core::ffi::c_uint;
    let suffix = suffix0.add(1);
    let reverseSuffix = malloc(bufferSize.wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    let doneMarks = malloc(
        bufferSize
            .wrapping_add(16)
            .wrapping_mul(::core::mem::size_of::<u8>()),
    ) as *mut u8;
    let filePos =
        malloc((nbFiles as size_t).wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    let mut result = 0;
    let mut displayClock = Instant::now();
    let refresh_rate = Duration::from_millis(300);
    if notificationLevel >= 2 {
        eprintln!("\r{:70 }\r", "");
    }
    if suffix0.is_null() || reverseSuffix.is_null() || doneMarks.is_null() || filePos.is_null() {
        result = Error::memory_allocation.to_error_code();
    } else {
        if minRatio < MINRATIO as core::ffi::c_uint {
            minRatio = MINRATIO as core::ffi::c_uint;
        }
        core::ptr::write_bytes(doneMarks, 0, bufferSize.wrapping_add(16));
        if bufferSize > ZDICT_MAX_SAMPLES_SIZE as size_t && notificationLevel >= 3 {
            eprintln!(
                "sample set too large : reduced to {} MB ...",
                (2000) << 20 >> 20,
            );
        }
        while bufferSize > ZDICT_MAX_SAMPLES_SIZE as size_t {
            nbFiles = nbFiles.wrapping_sub(1);
            bufferSize = bufferSize.wrapping_sub(*fileSizes.offset(nbFiles as isize));
        }
        if notificationLevel >= 2 {
            eprintln!(
                "sorting {} files of total size {} MB ...",
                nbFiles,
                bufferSize >> 20,
            );
        }
        let divSuftSortResult = divsufsort(
            core::slice::from_raw_parts(buffer as *const u8, bufferSize),
            core::slice::from_raw_parts_mut(suffix as *mut i32, bufferSize),
            false,
        );
        if divSuftSortResult != 0 {
            result = Error::GENERIC.to_error_code();
        } else {
            *suffix.add(bufferSize) = bufferSize as core::ffi::c_uint;
            *suffix0 = bufferSize as core::ffi::c_uint;
            let mut pos: size_t = 0;
            pos = 0;
            while pos < bufferSize {
                *reverseSuffix.offset(*suffix.add(pos) as isize) = pos as u32;
                pos = pos.wrapping_add(1);
            }
            *filePos = 0;
            pos = 1;
            while pos < nbFiles as size_t {
                *filePos.add(pos) = (*filePos.add(pos.wrapping_sub(1)) as size_t)
                    .wrapping_add(*fileSizes.add(pos.wrapping_sub(1)))
                    as u32;
                pos = pos.wrapping_add(1);
            }
            if notificationLevel >= 2 {
                eprintln!("finding patterns ...");
            }
            if notificationLevel >= 3 {
                eprintln!("minimum ratio : {} ", minRatio);
            }
            let mut cursor: u32 = 0;
            cursor = 0;
            while (cursor as size_t) < bufferSize {
                let mut solution = dictItem {
                    pos: 0,
                    length: 0,
                    savings: 0,
                };
                if *doneMarks.offset(cursor as isize) != 0 {
                    cursor = cursor.wrapping_add(1);
                } else {
                    solution = ZDICT_analyzePos(
                        doneMarks,
                        suffix,
                        *reverseSuffix.offset(cursor as isize),
                        buffer,
                        minRatio,
                        notificationLevel,
                    );
                    if solution.length == 0 {
                        cursor = cursor.wrapping_add(1);
                    } else {
                        ZDICT_insertDictItem(dictList, dictListSize, solution, buffer);
                        cursor = cursor.wrapping_add(solution.length);
                        if notificationLevel >= 2 {
                            if displayClock.elapsed() > refresh_rate {
                                displayClock = Instant::now();
                                eprint!(
                                    "\r{:4.2} % \r",
                                    cursor as core::ffi::c_double
                                        / bufferSize as core::ffi::c_double
                                        * 100.0f64,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    free(suffix0 as *mut core::ffi::c_void);
    free(reverseSuffix as *mut core::ffi::c_void);
    free(doneMarks as *mut core::ffi::c_void);
    free(filePos as *mut core::ffi::c_void);
    result
}
unsafe fn ZDICT_fillNoise(buffer: *mut core::ffi::c_void, length: size_t) {
    let prime1 = 2654435761 as core::ffi::c_uint;
    let prime2 = 2246822519 as core::ffi::c_uint;
    let mut acc = prime1;
    let mut p = 0;
    p = 0;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut core::ffi::c_uchar).add(p) = (acc >> 21) as core::ffi::c_uchar;
        p = p.wrapping_add(1);
    }
}
const MAXREPOFFSET: core::ffi::c_int = 1024;
unsafe fn ZDICT_countEStats(
    esr: EStats_ress_t,
    params: *const ZSTD_parameters,
    countLit: *mut core::ffi::c_uint,
    offsetcodeCount: *mut core::ffi::c_uint,
    matchlengthCount: *mut core::ffi::c_uint,
    litlengthCount: *mut core::ffi::c_uint,
    repOffsets: *mut u32,
    src: *const core::ffi::c_void,
    mut srcSize: size_t,
    notificationLevel: u32,
) {
    let blockSizeMax = (if ((1) << 17) < (1) << (*params).cParams.windowLog {
        (1) << 17
    } else {
        (1) << (*params).cParams.windowLog
    }) as size_t;
    let mut cSize: size_t = 0;
    if srcSize > blockSizeMax {
        srcSize = blockSizeMax;
    }
    let errorCode = ZSTD_compressBegin_usingCDict_deprecated(esr.zc, esr.dict);
    if ERR_isError(errorCode) {
        if notificationLevel >= 1 {
            eprintln!("warning : ZSTD_compressBegin_usingCDict failed");
        }
        return;
    }
    cSize = ZSTD_compressBlock_deprecated(
        esr.zc,
        esr.workPlace,
        ZSTD_BLOCKSIZE_MAX as size_t,
        src,
        srcSize,
    );
    if ERR_isError(cSize) {
        if notificationLevel >= 3 {
            eprintln!("warning : could not compress sample size {} ", srcSize);
        }
        return;
    }
    if cSize != 0 {
        let seqStorePtr = ZSTD_getSeqStore(esr.zc);
        let mut bytePtr = core::ptr::null::<u8>();
        bytePtr = (*seqStorePtr).litStart;
        while bytePtr < (*seqStorePtr).lit as *const u8 {
            let fresh9 = &mut (*countLit.offset(*bytePtr as isize));
            *fresh9 = (*fresh9).wrapping_add(1);
            bytePtr = bytePtr.add(1);
        }
        let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
            as core::ffi::c_long as u32;
        ZSTD_seqToCodes(seqStorePtr);
        let codePtr: *const u8 = (*seqStorePtr).ofCode;
        let mut u: u32 = 0;
        u = 0;
        while u < nbSeq {
            let fresh10 = &mut (*offsetcodeCount.offset(*codePtr.offset(u as isize) as isize));
            *fresh10 = (*fresh10).wrapping_add(1);
            u = u.wrapping_add(1);
        }
        let codePtr_0: *const u8 = (*seqStorePtr).mlCode;
        let mut u_0: u32 = 0;
        u_0 = 0;
        while u_0 < nbSeq {
            let fresh11 = &mut (*matchlengthCount.offset(*codePtr_0.offset(u_0 as isize) as isize));
            *fresh11 = (*fresh11).wrapping_add(1);
            u_0 = u_0.wrapping_add(1);
        }
        let codePtr_1: *const u8 = (*seqStorePtr).llCode;
        let mut u_1: u32 = 0;
        u_1 = 0;
        while u_1 < nbSeq {
            let fresh12 = &mut (*litlengthCount.offset(*codePtr_1.offset(u_1 as isize) as isize));
            *fresh12 = (*fresh12).wrapping_add(1);
            u_1 = u_1.wrapping_add(1);
        }
        if nbSeq >= 2 {
            let seq: *const SeqDef = (*seqStorePtr).sequencesStart;
            let mut offset1 = ((*seq).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
            let mut offset2 = ((*seq.add(1)).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
            if offset1 >= MAXREPOFFSET as u32 {
                offset1 = 0;
            }
            if offset2 >= MAXREPOFFSET as u32 {
                offset2 = 0;
            }
            let fresh13 = &mut (*repOffsets.offset(offset1 as isize));
            *fresh13 = (*fresh13).wrapping_add(3);
            let fresh14 = &mut (*repOffsets.offset(offset2 as isize));
            *fresh14 = (*fresh14).wrapping_add(1);
        }
    }
}
unsafe fn ZDICT_totalSampleSize(fileSizes: *const size_t, nbFiles: core::ffi::c_uint) -> size_t {
    let mut total = 0 as size_t;
    let mut u: core::ffi::c_uint = 0;
    u = 0;
    while u < nbFiles {
        total = total.wrapping_add(*fileSizes.offset(u as isize));
        u = u.wrapping_add(1);
    }
    total
}
unsafe fn ZDICT_insertSortCount(table: *mut offsetCount_t, val: u32, count: u32) {
    let mut u: u32 = 0;
    (*table.offset(ZSTD_REP_NUM as isize)).offset = val;
    (*table.offset(ZSTD_REP_NUM as isize)).count = count;
    u = ZSTD_REP_NUM as u32;
    while u > 0 {
        let mut tmp = offsetCount_t {
            offset: 0,
            count: 0,
        };
        if (*table.offset(u.wrapping_sub(1) as isize)).count >= (*table.offset(u as isize)).count {
            break;
        }
        tmp = *table.offset(u.wrapping_sub(1) as isize);
        *table.offset(u.wrapping_sub(1) as isize) = *table.offset(u as isize);
        *table.offset(u as isize) = tmp;
        u = u.wrapping_sub(1);
    }
}
unsafe fn ZDICT_flatLit(countLit: *mut core::ffi::c_uint) {
    let mut u: core::ffi::c_int = 0;
    u = 1;
    while u < 256 {
        *countLit.offset(u as isize) = 2;
        u += 1;
    }
    *countLit = 4;
    *countLit.add(253) = 1;
    *countLit.add(254) = 1;
}
const OFFCODE_MAX: core::ffi::c_int = 30;
unsafe fn ZDICT_analyzeEntropy(
    dstBuffer: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut compressionLevel: core::ffi::c_int,
    srcBuffer: *const core::ffi::c_void,
    fileSizes: *const size_t,
    nbFiles: core::ffi::c_uint,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    notificationLevel: core::ffi::c_uint,
) -> size_t {
    let mut countLit: [core::ffi::c_uint; 256] = [0; 256];
    let mut hufTable: [HUF_CElt; 257] = [0; 257];
    let mut offcodeCount: [core::ffi::c_uint; 31] = [0; 31];
    let mut offcodeNCount: [core::ffi::c_short; 31] = [0; 31];
    let offcodeMax =
        ZSTD_highbit32(dictBufferSize.wrapping_add((128 * ((1) << 10)) as size_t) as u32);
    let mut matchLengthCount: [core::ffi::c_uint; 53] = [0; 53];
    let mut matchLengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut litLengthCount: [core::ffi::c_uint; 36] = [0; 36];
    let mut litLengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut bestRepOffset: [offsetCount_t; 4] = [offsetCount_t {
        offset: 0,
        count: 0,
    }; 4];
    let mut esr = EStats_ress_t {
        dict: core::ptr::null_mut(),
        zc: core::ptr::null_mut(),
        workPlace: core::ptr::null_mut(),
    };
    let mut params = ZSTD_parameters::default();
    let mut u: u32 = 0;
    let mut huffLog = 11;
    let mut Offlog = OffFSELog as u32;
    let mut mlLog = MLFSELog as u32;
    let mut llLog = LLFSELog as u32;
    let mut total: u32 = 0;
    let mut pos = 0 as size_t;
    let mut errorCode: size_t = 0;
    let mut eSize = 0;
    let totalSrcSize = ZDICT_totalSampleSize(fileSizes, nbFiles);
    let averageSampleSize = totalSrcSize
        / nbFiles.wrapping_add((nbFiles == 0) as core::ffi::c_int as core::ffi::c_uint) as size_t;
    let mut dstPtr = dstBuffer as *mut u8;
    let mut wksp: [u32; 1216] = [0; 1216];
    if offcodeMax > OFFCODE_MAX as u32 {
        eSize = Error::dictionaryCreation_failed.to_error_code();
    } else {
        u = 0;
        while u < 256 {
            *countLit.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= offcodeMax {
            *offcodeCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= MaxML as u32 {
            *matchLengthCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= MaxLL as u32 {
            *litLengthCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }

        let mut repOffset: [u32; 1024] = [0; 1024];
        repOffset[1] = 1;
        repOffset[4] = 1;
        repOffset[8] = 1;

        ptr::write_bytes(
            bestRepOffset.as_mut_ptr() as *mut u8,
            0,
            ::core::mem::size_of::<[offsetCount_t; 4]>(),
        );

        if compressionLevel == 0 {
            compressionLevel = ZSTD_CLEVEL_DEFAULT;
        }
        params = ZSTD_getParams(
            compressionLevel,
            averageSampleSize as core::ffi::c_ulonglong,
            dictBufferSize,
        );
        esr.dict = ZSTD_createCDict_advanced(
            dictBuffer,
            dictBufferSize,
            ZSTD_dlm_byRef,
            ZSTD_dct_rawContent,
            params.cParams,
            ZSTD_customMem::default(),
        );
        esr.zc = ZSTD_createCCtx();
        esr.workPlace = malloc(ZSTD_BLOCKSIZE_MAX as size_t);
        if (esr.dict).is_null() || (esr.zc).is_null() || (esr.workPlace).is_null() {
            eSize = Error::memory_allocation.to_error_code();
            if notificationLevel >= 1 {
                eprintln!("Not enough memory");
            }
        } else {
            u = 0;
            while u < nbFiles {
                ZDICT_countEStats(
                    esr,
                    &params,
                    countLit.as_mut_ptr(),
                    offcodeCount.as_mut_ptr(),
                    matchLengthCount.as_mut_ptr(),
                    litLengthCount.as_mut_ptr(),
                    repOffset.as_mut_ptr(),
                    (srcBuffer as *const core::ffi::c_char).add(pos) as *const core::ffi::c_void,
                    *fileSizes.offset(u as isize),
                    notificationLevel,
                );
                pos = pos.wrapping_add(*fileSizes.offset(u as isize));
                u = u.wrapping_add(1);
            }
            if notificationLevel >= 4 {
                if notificationLevel >= 4 {
                    eprintln!("Offset Code Frequencies :");
                }
                u = 0;
                while u <= offcodeMax {
                    if notificationLevel >= 4 {
                        eprintln!(
                            "{:>2} :{:>7} ",
                            u,
                            *offcodeCount.as_mut_ptr().offset(u as isize),
                        );
                    }
                    u = u.wrapping_add(1);
                }
            }
            let mut maxNbBits = HUF_buildCTable_wksp(
                hufTable.as_mut_ptr(),
                countLit.as_mut_ptr(),
                255,
                huffLog,
                wksp.as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<[u32; 1216]>(),
            );
            if ERR_isError(maxNbBits) {
                eSize = maxNbBits;
                if notificationLevel >= 1 {
                    eprintln!(" HUF_buildCTable error");
                }
            } else {
                if maxNbBits == 8 {
                    if notificationLevel >= 2 {
                        eprintln!(
                            "warning : pathological dataset : literals are not compressible : samples are noisy or too regular "
                        );
                    }
                    ZDICT_flatLit(countLit.as_mut_ptr());
                    maxNbBits = HUF_buildCTable_wksp(
                        hufTable.as_mut_ptr(),
                        countLit.as_mut_ptr(),
                        255,
                        huffLog,
                        wksp.as_mut_ptr() as *mut core::ffi::c_void,
                        ::core::mem::size_of::<[u32; 1216]>(),
                    );
                }
                huffLog = maxNbBits as u32;
                let mut offset: u32 = 0;
                offset = 1;
                while offset < MAXREPOFFSET as u32 {
                    ZDICT_insertSortCount(
                        bestRepOffset.as_mut_ptr(),
                        offset,
                        *repOffset.as_mut_ptr().offset(offset as isize),
                    );
                    offset = offset.wrapping_add(1);
                }
                total = 0;
                u = 0;
                while u <= offcodeMax {
                    total = (total as core::ffi::c_uint)
                        .wrapping_add(*offcodeCount.as_mut_ptr().offset(u as isize));
                    u = u.wrapping_add(1);
                }
                errorCode = FSE_normalizeCount(
                    offcodeNCount.as_mut_ptr(),
                    Offlog,
                    offcodeCount.as_mut_ptr(),
                    total as size_t,
                    offcodeMax,
                    1,
                );
                if ERR_isError(errorCode) {
                    eSize = errorCode;
                    if notificationLevel >= 1 {
                        eprintln!("FSE_normalizeCount error with offcodeCount");
                    }
                } else {
                    Offlog = errorCode as u32;
                    total = 0;
                    u = 0;
                    while u <= MaxML as u32 {
                        total = (total as core::ffi::c_uint)
                            .wrapping_add(*matchLengthCount.as_mut_ptr().offset(u as isize));
                        u = u.wrapping_add(1);
                    }
                    errorCode = FSE_normalizeCount(
                        matchLengthNCount.as_mut_ptr(),
                        mlLog,
                        matchLengthCount.as_mut_ptr(),
                        total as size_t,
                        MaxML as core::ffi::c_uint,
                        1,
                    );
                    if ERR_isError(errorCode) {
                        eSize = errorCode;
                        if notificationLevel >= 1 {
                            eprintln!("FSE_normalizeCount error with matchLengthCount");
                        }
                    } else {
                        mlLog = errorCode as u32;
                        total = 0;
                        u = 0;
                        while u <= MaxLL as u32 {
                            total = (total as core::ffi::c_uint)
                                .wrapping_add(*litLengthCount.as_mut_ptr().offset(u as isize));
                            u = u.wrapping_add(1);
                        }
                        errorCode = FSE_normalizeCount(
                            litLengthNCount.as_mut_ptr(),
                            llLog,
                            litLengthCount.as_mut_ptr(),
                            total as size_t,
                            MaxLL as core::ffi::c_uint,
                            1,
                        );
                        if ERR_isError(errorCode) {
                            eSize = errorCode;
                            if notificationLevel >= 1 {
                                eprintln!("FSE_normalizeCount error with litLengthCount");
                            }
                        } else {
                            llLog = errorCode as u32;
                            let hhSize = HUF_writeCTable_wksp(
                                dstPtr as *mut core::ffi::c_void,
                                maxDstSize,
                                hufTable.as_mut_ptr(),
                                255,
                                huffLog,
                                wksp.as_mut_ptr() as *mut core::ffi::c_void,
                                ::core::mem::size_of::<[u32; 1216]>(),
                            );
                            if ERR_isError(hhSize) {
                                eSize = hhSize;
                                if notificationLevel >= 1 {
                                    eprintln!("HUF_writeCTable error");
                                }
                            } else {
                                dstPtr = dstPtr.add(hhSize);
                                maxDstSize = maxDstSize.wrapping_sub(hhSize);
                                eSize = eSize.wrapping_add(hhSize);
                                let ohSize = FSE_writeNCount(
                                    dstPtr as *mut core::ffi::c_void,
                                    maxDstSize,
                                    offcodeNCount.as_mut_ptr(),
                                    OFFCODE_MAX as core::ffi::c_uint,
                                    Offlog,
                                );
                                if ERR_isError(ohSize) {
                                    eSize = ohSize;
                                    if notificationLevel >= 1 {
                                        eprintln!("FSE_writeNCount error with offcodeNCount");
                                    }
                                } else {
                                    dstPtr = dstPtr.add(ohSize);
                                    maxDstSize = maxDstSize.wrapping_sub(ohSize);
                                    eSize = eSize.wrapping_add(ohSize);
                                    let mhSize = FSE_writeNCount(
                                        dstPtr as *mut core::ffi::c_void,
                                        maxDstSize,
                                        matchLengthNCount.as_mut_ptr(),
                                        MaxML as core::ffi::c_uint,
                                        mlLog,
                                    );
                                    if ERR_isError(mhSize) {
                                        eSize = mhSize;
                                        if notificationLevel >= 1 {
                                            eprintln!(
                                                "FSE_writeNCount error with matchLengthNCount "
                                            );
                                        }
                                    } else {
                                        dstPtr = dstPtr.add(mhSize);
                                        maxDstSize = maxDstSize.wrapping_sub(mhSize);
                                        eSize = eSize.wrapping_add(mhSize);
                                        let lhSize = FSE_writeNCount(
                                            dstPtr as *mut core::ffi::c_void,
                                            maxDstSize,
                                            litLengthNCount.as_mut_ptr(),
                                            MaxLL as core::ffi::c_uint,
                                            llLog,
                                        );
                                        if ERR_isError(lhSize) {
                                            eSize = lhSize;
                                            if notificationLevel >= 1 {
                                                eprintln!(
                                                    "FSE_writeNCount error with litlengthNCount "
                                                );
                                            }
                                        } else {
                                            dstPtr = dstPtr.add(lhSize);
                                            maxDstSize = maxDstSize.wrapping_sub(lhSize);
                                            eSize = eSize.wrapping_add(lhSize);
                                            if maxDstSize < 12 {
                                                eSize = -(ZSTD_error_dstSize_tooSmall
                                                    as core::ffi::c_int)
                                                    as size_t;
                                                if notificationLevel >= 1 {
                                                    eprintln!(
                                                        "not enough space to write RepOffsets "
                                                    );
                                                }
                                            } else {
                                                MEM_writeLE32(
                                                    dstPtr as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr(),
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.add(4) as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr().add(1),
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.add(8) as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr().add(2),
                                                );
                                                eSize = eSize.wrapping_add(12);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ZSTD_freeCDict(esr.dict);
    ZSTD_freeCCtx(esr.zc);
    free(esr.workPlace);
    eSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_finalizeDictionary))]
pub unsafe extern "C" fn ZDICT_finalizeDictionary(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    customDictContent: *const core::ffi::c_void,
    mut dictContentSize: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    params: ZDICT_params_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut header: [u8; 256] = [0; 256];
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let minContentSize = *repStartValue.iter().max().unwrap() as size_t;
    let mut paddingSize: size_t = 0;
    if dictBufferCapacity < dictContentSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }
    header[..4].copy_from_slice(&ZSTD_MAGIC_DICTIONARY.to_le_bytes());
    let randomID = ZSTD_XXH64(customDictContent, dictContentSize, 0);
    let compliantID = (randomID % ((1 as core::ffi::c_uint) << 31).wrapping_sub(32768) as u64)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    header[4..][..4].copy_from_slice(&dictID.to_le_bytes());
    hSize = 8;
    if notificationLevel >= 2 {
        eprintln!("\r{:70 }\r", "");
    }
    if notificationLevel >= 2 {
        eprintln!("statistics ...");
    }
    let eSize = ZDICT_analyzeEntropy(
        header[hSize..].as_mut_ptr() as *mut core::ffi::c_void,
        (HBUFFSIZE as size_t).wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        customDictContent,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = hSize.wrapping_add(eSize);
    if hSize.wrapping_add(dictContentSize) > dictBufferCapacity {
        dictContentSize = dictBufferCapacity.wrapping_sub(hSize);
    }
    if dictContentSize < minContentSize {
        if hSize.wrapping_add(minContentSize) > dictBufferCapacity {
            return Error::dstSize_tooSmall.to_error_code();
        }
        paddingSize = minContentSize.wrapping_sub(dictContentSize);
    } else {
        paddingSize = 0;
    }
    let dictSize = hSize
        .wrapping_add(paddingSize)
        .wrapping_add(dictContentSize);
    let outDictHeader = dictBuffer as *mut u8;
    let outDictPadding = outDictHeader.add(hSize);
    let outDictContent = outDictPadding.add(paddingSize);
    core::ptr::copy(
        customDictContent.cast::<u8>(),
        outDictContent,
        dictContentSize,
    );
    core::ptr::copy_nonoverlapping(header.as_mut_ptr(), outDictHeader, hSize);
    core::ptr::write_bytes(outDictPadding, 0, paddingSize);
    dictSize
}

const HBUFFSIZE: core::ffi::c_int = 256;
unsafe fn ZDICT_addEntropyTablesFromBuffer_advanced(
    dictBuffer: *mut core::ffi::c_void,
    dictContentSize: size_t,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    params: ZDICT_params_t,
) -> size_t {
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let mut hSize = 8;
    if notificationLevel >= 2 {
        eprintln!("\r{:70 }\r", "");
    }
    if notificationLevel >= 2 {
        eprintln!("statistics ...");
    }
    let eSize = ZDICT_analyzeEntropy(
        (dictBuffer as *mut core::ffi::c_char).add(hSize) as *mut core::ffi::c_void,
        dictBufferCapacity.wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        (dictBuffer as *mut core::ffi::c_char)
            .add(dictBufferCapacity)
            .offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = hSize.wrapping_add(eSize);
    MEM_writeLE32(dictBuffer, ZSTD_MAGIC_DICTIONARY);
    let randomID = ZSTD_XXH64(
        (dictBuffer as *mut core::ffi::c_char)
            .add(dictBufferCapacity)
            .offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
        dictContentSize,
        0,
    );
    let compliantID = (randomID % ((1 as core::ffi::c_uint) << 31).wrapping_sub(32768) as u64)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    MEM_writeLE32(
        (dictBuffer as *mut core::ffi::c_char).add(4) as *mut core::ffi::c_void,
        dictID,
    );
    if hSize.wrapping_add(dictContentSize) < dictBufferCapacity {
        core::ptr::copy(
            (dictBuffer as *mut core::ffi::c_char)
                .add(dictBufferCapacity)
                .sub(dictContentSize),
            (dictBuffer as *mut core::ffi::c_char).add(hSize),
            dictContentSize,
        )
    }
    if dictBufferCapacity < hSize.wrapping_add(dictContentSize) {
        dictBufferCapacity
    } else {
        hSize.wrapping_add(dictContentSize)
    }
}
unsafe fn ZDICT_trainFromBuffer_unsafe_legacy(
    dictBuffer: *mut core::ffi::c_void,
    maxDictSize: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    params: ZDICT_legacy_params_t,
) -> size_t {
    let dictListSize =
        if (if 10000 > nbSamples { 10000 } else { nbSamples }) > (maxDictSize / 16) as u32 {
            if 10000 > nbSamples {
                10000
            } else {
                nbSamples
            }
        } else {
            (maxDictSize / 16) as u32
        };
    let dictList = malloc((dictListSize as size_t).wrapping_mul(::core::mem::size_of::<dictItem>()))
        as *mut dictItem;
    let selectivity = if params.selectivityLevel == 0 {
        g_selectivity_default
    } else {
        params.selectivityLevel
    };
    let minRep = if selectivity > 30 {
        MINRATIO as core::ffi::c_uint
    } else {
        nbSamples >> selectivity
    };
    let targetDictSize = maxDictSize;
    let samplesBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    let mut dictSize = 0;
    let notificationLevel = params.zParams.notificationLevel;
    if dictList.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    if maxDictSize < ZDICT_DICTSIZE_MIN as size_t {
        free(dictList as *mut core::ffi::c_void);
        return Error::dstSize_tooSmall.to_error_code();
    }
    if samplesBuffSize < ZDICT_MIN_SAMPLES_SIZE as size_t {
        free(dictList as *mut core::ffi::c_void);
        return Error::dictionaryCreation_failed.to_error_code();
    }
    ZDICT_initDictItem(dictList);
    ZDICT_trainBuffer_legacy(
        dictList,
        dictListSize,
        samplesBuffer,
        samplesBuffSize,
        samplesSizes,
        nbSamples,
        minRep,
        notificationLevel,
    );
    if params.zParams.notificationLevel >= 3 {
        let nb = if (25) < (*dictList).pos {
            25
        } else {
            (*dictList).pos
        };
        let dictContentSize = ZDICT_dictSize(dictList);
        let mut u: core::ffi::c_uint = 0;
        if notificationLevel >= 3 {
            eprintln!(
                "\n {} segments found, of total size {} ",
                ((*dictList).pos).wrapping_sub(1),
                dictContentSize,
            );
        }
        if notificationLevel >= 3 {
            eprintln!("list {} best segments ", nb.wrapping_sub(1));
        }
        u = 1;
        while u < nb {
            let pos = (*dictList.offset(u as isize)).pos;
            let length = (*dictList.offset(u as isize)).length;
            let printedLength = if (40) < length { 40 } else { length };
            if pos as size_t > samplesBuffSize
                || pos.wrapping_add(length) as size_t > samplesBuffSize
            {
                free(dictList as *mut core::ffi::c_void);
                return Error::GENERIC.to_error_code();
            }
            if notificationLevel >= 3 {
                eprint!(
                    "{:3}:{:3} bytes at pos {:8}, savings {:7} bytes |",
                    u,
                    length,
                    pos,
                    (*dictList.offset(u as isize)).savings,
                );
            }
            ZDICT_printHex(
                (samplesBuffer as *const core::ffi::c_char).offset(pos as isize)
                    as *const core::ffi::c_void,
                printedLength as size_t,
            );
            if notificationLevel >= 3 {
                eprintln!("|");
            }
            u = u.wrapping_add(1);
        }
    }
    let mut dictContentSize_0 = ZDICT_dictSize(dictList);
    #[expect(deprecated)]
    if dictContentSize_0 < ZDICT_CONTENTSIZE_MIN as core::ffi::c_uint {
        free(dictList as *mut core::ffi::c_void);
        return Error::dictionaryCreation_failed.to_error_code();
    }
    if (dictContentSize_0 as size_t) < targetDictSize / 4 {
        if notificationLevel >= 2 {
            eprintln!(
                "!  warning : selected content significantly smaller than requested ({} < {}) ",
                dictContentSize_0, maxDictSize,
            );
        }
        if samplesBuffSize < 10 * targetDictSize && notificationLevel >= 2 {
            eprintln!(
                "!  consider increasing the number of samples (total size : {} MB)",
                samplesBuffSize >> 20,
            );
        }
        if minRep > MINRATIO as core::ffi::c_uint {
            if notificationLevel >= 2 {
                eprintln!(
                    "!  consider increasing selectivity to produce larger dictionary (-s{}) ",
                    selectivity.wrapping_add(1),
                );
            }
            if notificationLevel >= 2 {
                eprintln!(
                    "!  note : larger dictionaries are not necessarily better, test its efficiency on samples "
                );
            }
        }
    }
    if dictContentSize_0 as size_t > targetDictSize * 3
        && nbSamples > (2 * MINRATIO) as core::ffi::c_uint
        && selectivity > 1
    {
        let mut proposedSelectivity = selectivity.wrapping_sub(1);
        while nbSamples >> proposedSelectivity <= MINRATIO as core::ffi::c_uint {
            proposedSelectivity = proposedSelectivity.wrapping_sub(1);
        }
        if notificationLevel >= 2 {
            eprintln!(
                "!  note : calculated dictionary significantly larger than requested ({} > {}) ",
                dictContentSize_0, maxDictSize,
            );
        }
        if notificationLevel >= 2 {
            eprintln!(
                "!  consider increasing dictionary size, or produce denser dictionary (-s{}) ",
                proposedSelectivity,
            );
        }
        if notificationLevel >= 2 {
            eprintln!("!  always test dictionary efficiency on real samples");
        }
    }
    let max = (*dictList).pos;
    let mut currentSize = 0u32;
    let mut n: u32 = 0;
    n = 1;
    while n < max {
        currentSize = currentSize.wrapping_add((*dictList.offset(n as isize)).length);
        if currentSize as size_t > targetDictSize {
            currentSize = currentSize.wrapping_sub((*dictList.offset(n as isize)).length);
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    (*dictList).pos = n;
    dictContentSize_0 = currentSize;
    let mut u_0: u32 = 0;
    let mut ptr = (dictBuffer as *mut u8).add(maxDictSize);
    u_0 = 1;
    while u_0 < (*dictList).pos {
        let l = (*dictList.offset(u_0 as isize)).length;
        ptr = ptr.offset(-(l as isize));
        if ptr < dictBuffer as *mut u8 {
            free(dictList as *mut core::ffi::c_void);
            return Error::GENERIC.to_error_code();
        }
        memcpy(
            ptr as *mut core::ffi::c_void,
            (samplesBuffer as *const core::ffi::c_char)
                .offset((*dictList.offset(u_0 as isize)).pos as isize)
                as *const core::ffi::c_void,
            l as size_t,
        );
        u_0 = u_0.wrapping_add(1);
    }
    dictSize = ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize_0 as size_t,
        maxDictSize,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params.zParams,
    );
    free(dictList as *mut core::ffi::c_void);
    dictSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_trainFromBuffer_legacy))]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_legacy(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    params: ZDICT_legacy_params_t,
) -> size_t {
    let mut result: size_t = 0;
    let mut newBuff = core::ptr::null_mut::<core::ffi::c_void>();
    let sBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    if sBuffSize < ZDICT_MIN_SAMPLES_SIZE as size_t {
        return 0;
    }
    newBuff = malloc(sBuffSize.wrapping_add(NOISELENGTH as size_t));
    if newBuff.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    memcpy(newBuff, samplesBuffer, sBuffSize);
    ZDICT_fillNoise(
        (newBuff as *mut core::ffi::c_char).add(sBuffSize) as *mut core::ffi::c_void,
        NOISELENGTH as size_t,
    );
    result = ZDICT_trainFromBuffer_unsafe_legacy(
        dictBuffer,
        dictBufferCapacity,
        newBuff,
        samplesSizes,
        nbSamples,
        params,
    );
    free(newBuff);
    result
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_trainFromBuffer))]
pub unsafe extern "C" fn ZDICT_trainFromBuffer(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
) -> size_t {
    let mut params = ZDICT_fastCover_params_t {
        d: 8,
        steps: 4,
        ..Default::default()
    };
    params.zParams.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        &mut params,
    )
}

#[deprecated = "use ZDICT_finalizeDictionary() instead"]
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_addEntropyTablesFromBuffer))]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer(
    dictBuffer: *mut core::ffi::c_void,
    dictContentSize: size_t,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
) -> size_t {
    let params = ZDICT_params_t::default();
    ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params,
    )
}
