use core::ptr;
use std::ffi::CStr;
use std::io;

use libc::{exit, fclose, fflush, fopen, fprintf, fread, free, fwrite, malloc, size_t, FILE};
use libzstd_rs_sys::lib::zdict::experimental::{
    ZDICT_cover_params_t, ZDICT_fastCover_params_t, ZDICT_legacy_params_t,
    ZDICT_optimizeTrainFromBuffer_cover, ZDICT_optimizeTrainFromBuffer_fastCover,
    ZDICT_trainFromBuffer_cover, ZDICT_trainFromBuffer_fastCover, ZDICT_trainFromBuffer_legacy,
};
use libzstd_rs_sys::lib::zdict::{ZDICT_getErrorName, ZDICT_isError};
use libzstd_rs_sys::lib::zstd::*;

use crate::timefn::{PTime, UTIL_clockSpanMicro, UTIL_getTime, UTIL_time_t};
use crate::util::UTIL_getFileSize;

extern "C" {
    static mut stderr: *mut FILE;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileStats {
    pub totalSizeToLoad: i64,
    pub nbSamples: core::ffi::c_int,
    pub oneSampleTooLarge: core::ffi::c_int,
}
pub const UTIL_FILESIZE_UNKNOWN: core::ffi::c_int = -(1);
pub const SEC_TO_MICRO: core::ffi::c_int = 1000000;
pub const SAMPLESIZE_MAX: core::ffi::c_int = 128 * ((1) << 10);
pub const MEMMULT: core::ffi::c_int = 11;
pub const COVER_MEMMULT: core::ffi::c_int = 9;
pub const FASTCOVER_MEMMULT: core::ffi::c_int = 1;
static g_maxMemory: usize = if ::core::mem::size_of::<size_t>() == 4 {
    2usize * (1 << 30) - 64 * (1 << 20)
} else {
    (512usize * (1 << 20)) << ::core::mem::size_of::<size_t>()
};
pub const NOISELENGTH: core::ffi::c_int = 32;
static g_refreshRate: u64 = SEC_TO_MICRO as PTime / 6;
static mut g_displayClock: UTIL_time_t = UTIL_time_t { t: 0 };
pub const DEBUG: core::ffi::c_int = 0;
unsafe fn DiB_getFileSize(fileName: *const core::ffi::c_char) -> i64 {
    let fileSize = UTIL_getFileSize(fileName);
    if fileSize == UTIL_FILESIZE_UNKNOWN as u64 {
        -1_i64
    } else {
        fileSize as i64
    }
}
unsafe fn DiB_loadFiles(
    buffer: *mut core::ffi::c_void,
    bufferSizePtr: *mut size_t,
    sampleSizes: *mut size_t,
    sstSize: core::ffi::c_int,
    fileNamesTable: *mut *const core::ffi::c_char,
    nbFiles: core::ffi::c_int,
    targetChunkSize: size_t,
    displayLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let buff = buffer as *mut core::ffi::c_char;
    let mut totalDataLoaded = 0 as size_t;
    let mut nbSamplesLoaded = 0 as core::ffi::c_int;
    let mut fileIndex = 0 as core::ffi::c_int;
    let mut f = core::ptr::null_mut();
    while nbSamplesLoaded < sstSize && fileIndex < nbFiles {
        let mut fileDataLoaded: size_t = 0;
        let fileSize = DiB_getFileSize(*fileNamesTable.offset(fileIndex as isize));
        if fileSize <= 0 {
            fileIndex += 1;
        } else {
            f = fopen(
                *fileNamesTable.offset(fileIndex as isize),
                b"rb\0" as *const u8 as *const core::ffi::c_char,
            );
            if f.is_null() {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                    10,
                );
                eprintln!(
                    "zstd: dictBuilder: {} {}",
                    CStr::from_ptr(*fileNamesTable.offset(fileIndex as isize)).to_string_lossy(),
                    io::Error::last_os_error()
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
                exit(10);
            }
            if displayLevel >= 2
                && (UTIL_clockSpanMicro(g_displayClock) > g_refreshRate || displayLevel >= 4)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"Loading %s...       \r\0" as *const u8 as *const core::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                );
                if displayLevel >= 4 {
                    fflush(stderr);
                }
            }
            fileDataLoaded = if targetChunkSize > 0 {
                (if fileSize < targetChunkSize as i64 {
                    fileSize
                } else {
                    targetChunkSize as i64
                }) as size_t
            } else {
                (if fileSize < i64::from(128 * ((1) << 10)) {
                    fileSize
                } else {
                    i64::from(128 * ((1) << 10))
                }) as size_t
            };
            if totalDataLoaded.wrapping_add(fileDataLoaded) > *bufferSizePtr {
                break;
            }
            if fread(
                buff.add(totalDataLoaded) as *mut core::ffi::c_void,
                1,
                fileDataLoaded,
                f,
            ) != fileDataLoaded
            {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
                fprintf(
                    stderr,
                    b"Pb reading %s\0" as *const u8 as *const core::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
                exit(11);
            }
            let fresh0 = nbSamplesLoaded;
            nbSamplesLoaded += 1;
            *sampleSizes.offset(fresh0 as isize) = fileDataLoaded;
            totalDataLoaded = totalDataLoaded.wrapping_add(fileDataLoaded);
            if targetChunkSize > 0 {
                while (fileDataLoaded as i64) < fileSize && nbSamplesLoaded < sstSize {
                    let chunkSize =
                        if (fileSize as size_t).wrapping_sub(fileDataLoaded) < targetChunkSize {
                            (fileSize as size_t).wrapping_sub(fileDataLoaded)
                        } else {
                            targetChunkSize
                        };
                    if totalDataLoaded.wrapping_add(chunkSize) > *bufferSizePtr {
                        break;
                    }
                    if fread(
                        buff.add(totalDataLoaded) as *mut core::ffi::c_void,
                        1,
                        chunkSize,
                        f,
                    ) != chunkSize
                    {
                        fprintf(
                            stderr,
                            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                            11,
                        );
                        fprintf(
                            stderr,
                            b"Pb reading %s\0" as *const u8 as *const core::ffi::c_char,
                            *fileNamesTable.offset(fileIndex as isize),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
                        exit(11);
                    }
                    let fresh1 = nbSamplesLoaded;
                    nbSamplesLoaded += 1;
                    *sampleSizes.offset(fresh1 as isize) = chunkSize;
                    totalDataLoaded = totalDataLoaded.wrapping_add(chunkSize);
                    fileDataLoaded = fileDataLoaded.wrapping_add(chunkSize);
                }
            }
            fileIndex += 1;
            fclose(f);
            f = core::ptr::null_mut();
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    if displayLevel >= 2 {
        println!("\r{:79 }\r", "");
    }
    if displayLevel >= 4 {
        fprintf(
            stderr,
            b"Loaded %d KB total training data, %d nb samples \n\0" as *const u8
                as *const core::ffi::c_char,
            (totalDataLoaded / ((1) << 10) as size_t) as core::ffi::c_int,
            nbSamplesLoaded,
        );
    }
    *bufferSizePtr = totalDataLoaded;
    nbSamplesLoaded
}
unsafe fn DiB_rand(src: *mut u32) -> u32 {
    static prime1: u32 = 2654435761;
    static prime2: u32 = 2246822519;
    let mut rand32 = *src;
    rand32 *= prime1;
    rand32 ^= prime2;
    rand32 = rand32.rotate_left(13);
    *src = rand32;
    rand32 >> 5
}
unsafe fn DiB_shuffle(fileNamesTable: *mut *const core::ffi::c_char, nbFiles: core::ffi::c_uint) {
    let mut seed = 0xfd2fb528 as core::ffi::c_uint;
    let mut i: core::ffi::c_uint = 0;
    if nbFiles == 0 {
        return;
    }
    i = nbFiles.wrapping_sub(1);
    while i > 0 {
        let j = (DiB_rand(&mut seed)).wrapping_rem(i.wrapping_add(1));
        let tmp = *fileNamesTable.offset(j as isize);
        let fresh2 = &mut (*fileNamesTable.offset(j as isize));
        *fresh2 = *fileNamesTable.offset(i as isize);
        let fresh3 = &mut (*fileNamesTable.offset(i as isize));
        *fresh3 = tmp;
        i = i.wrapping_sub(1);
    }
}
unsafe fn DiB_findMaxMem(mut requiredMem: core::ffi::c_ulonglong) -> size_t {
    let step = (8 * ((1) << 20)) as size_t;
    let mut testmem = core::ptr::null_mut::<core::ffi::c_void>();
    requiredMem = (requiredMem >> 23).wrapping_add(1) << 23;
    requiredMem = requiredMem.wrapping_add(step as core::ffi::c_ulonglong);
    if requiredMem > g_maxMemory as core::ffi::c_ulonglong {
        requiredMem = g_maxMemory as core::ffi::c_ulonglong;
    }
    while testmem.is_null() {
        testmem = malloc(requiredMem as size_t);
        requiredMem = requiredMem.wrapping_sub(step as core::ffi::c_ulonglong);
    }
    free(testmem);
    requiredMem as size_t
}
unsafe fn DiB_fillNoise(buffer: *mut core::ffi::c_void, length: size_t) {
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
unsafe fn DiB_saveDict(
    dictFileName: *const core::ffi::c_char,
    buff: *const core::ffi::c_void,
    buffSize: size_t,
) {
    let f = fopen(
        dictFileName,
        b"wb\0" as *const u8 as *const core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
            3,
        );
        fprintf(
            stderr,
            b"cannot open %s \0" as *const u8 as *const core::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        exit(3);
    }
    let n = fwrite(buff, 1, buffSize, f);
    if n != buffSize {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
            4,
        );
        fprintf(
            stderr,
            b"%s : write error\0" as *const u8 as *const core::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        exit(4);
    }
    let n_0 = fclose(f) as size_t;
    if n_0 != 0 {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
            5,
        );
        fprintf(
            stderr,
            b"%s : flush error\0" as *const u8 as *const core::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        exit(5);
    }
}
unsafe fn DiB_fileStats(
    fileNamesTable: *mut *const core::ffi::c_char,
    nbFiles: core::ffi::c_int,
    chunkSize: size_t,
    displayLevel: core::ffi::c_int,
) -> fileStats {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut n: core::ffi::c_int = 0;
    ptr::write_bytes(
        &mut fs as *mut fileStats as *mut u8,
        0,
        ::core::mem::size_of::<fileStats>(),
    );
    n = 0;
    while n < nbFiles {
        let fileSize = DiB_getFileSize(*fileNamesTable.offset(n as isize));
        if fileSize == 0 {
            if displayLevel >= 3 {
                fprintf(
                    stderr,
                    b"Sample file '%s' has zero size, skipping...\n\0" as *const u8
                        as *const core::ffi::c_char,
                    *fileNamesTable.offset(n as isize),
                );
            }
        } else if chunkSize > 0 {
            fs.nbSamples += ((fileSize as size_t).wrapping_add(chunkSize).wrapping_sub(1)
                / chunkSize) as core::ffi::c_int;
            fs.totalSizeToLoad += fileSize;
        } else {
            if fileSize > i64::from(SAMPLESIZE_MAX) {
                fs.oneSampleTooLarge |=
                    core::ffi::c_int::from(fileSize > i64::from(2 * SAMPLESIZE_MAX));
                if displayLevel >= 3 {
                    fprintf(
                        stderr,
                        b"Sample file '%s' is too large, limiting to %d KB\n\0" as *const u8
                            as *const core::ffi::c_char,
                        *fileNamesTable.offset(n as isize),
                        128 * ((1) << 10) / ((1) << 10),
                    );
                }
            }
            fs.nbSamples += 1;
            fs.totalSizeToLoad += if fileSize < i64::from(128 * ((1) << 10)) {
                fileSize
            } else {
                i64::from(128 * ((1) << 10))
            };
        }
        n += 1;
    }
    if displayLevel >= 4 {
        fprintf(
            stderr,
            b"Found training data %d files, %d KB, %d samples\n\0" as *const u8
                as *const core::ffi::c_char,
            nbFiles,
            (fs.totalSizeToLoad / i64::from((1) << 10)) as core::ffi::c_int,
            fs.nbSamples,
        );
    }
    fs
}
pub unsafe fn DiB_trainFromFiles(
    dictFileName: *const core::ffi::c_char,
    maxDictSize: size_t,
    fileNamesTable: *mut *const core::ffi::c_char,
    nbFiles: core::ffi::c_int,
    chunkSize: size_t,
    params: *mut ZDICT_legacy_params_t,
    coverParams: *mut ZDICT_cover_params_t,
    fastCoverParams: *mut ZDICT_fastCover_params_t,
    optimize: core::ffi::c_int,
    memLimit: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut sampleSizes = core::ptr::null_mut::<size_t>();
    let mut nbSamplesLoaded: core::ffi::c_int = 0;
    let mut loadedSize: size_t = 0;
    let mut srcBuffer = core::ptr::null_mut::<core::ffi::c_void>();
    let dictBuffer = malloc(maxDictSize);
    let mut result = 0;
    let displayLevel = (if !params.is_null() {
        (*params).zParams.notificationLevel
    } else if !coverParams.is_null() {
        (*coverParams).zParams.notificationLevel
    } else if !fastCoverParams.is_null() {
        (*fastCoverParams).zParams.notificationLevel
    } else {
        0
    }) as core::ffi::c_int;
    if displayLevel >= 3 {
        fprintf(
            stderr,
            b"Shuffling input files\n\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    DiB_shuffle(fileNamesTable, nbFiles as core::ffi::c_uint);
    fs = DiB_fileStats(fileNamesTable, nbFiles, chunkSize, displayLevel);
    let memMult = if !params.is_null() {
        MEMMULT
    } else if !coverParams.is_null() {
        COVER_MEMMULT
    } else {
        FASTCOVER_MEMMULT
    };
    let maxMem =
        DiB_findMaxMem((fs.totalSizeToLoad * i64::from(memMult)) as core::ffi::c_ulonglong)
            / memMult as size_t;
    loadedSize = (if (if (maxMem as i64) < fs.totalSizeToLoad {
        maxMem as i64
    } else {
        fs.totalSizeToLoad
    }) < i64::from(
        (2 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30),
    ) {
        if (maxMem as i64) < fs.totalSizeToLoad {
            maxMem as i64
        } else {
            fs.totalSizeToLoad
        }
    } else {
        i64::from((2 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30))
    }) as size_t;
    if memLimit != 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : setting manual memory limit for dictionary training data at %u MB \n\0"
                    as *const u8 as *const core::ffi::c_char,
                memLimit
                    .wrapping_div(
                        (((1) << 20))
                            as core::ffi::c_uint,
                    ),
            );
        }
        loadedSize = if loadedSize < memLimit as size_t {
            loadedSize
        } else {
            memLimit as size_t
        };
    }
    srcBuffer = malloc(loadedSize.wrapping_add(NOISELENGTH as size_t));
    sampleSizes = malloc((fs.nbSamples as size_t).wrapping_mul(::core::mem::size_of::<size_t>()))
        as *mut size_t;
    if fs.nbSamples != 0 && sampleSizes.is_null() || srcBuffer.is_null() || dictBuffer.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
            12,
        );
        fprintf(
            stderr,
            b"not enough memory for DiB_trainFiles\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        exit(12);
    }
    if fs.oneSampleTooLarge != 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : some sample(s) are very large \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Note that dictionary is only useful for small samples. \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  As a consequence, only the first %u bytes of each sample are loaded \n\0"
                    as *const u8 as *const core::ffi::c_char,
                128 * ((1) << 10),
            );
        }
    }
    if fs.nbSamples < 5 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : nb of samples too low for proper processing !\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Please provide _one file per sample_.\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Alternatively, split file(s) into fixed-size samples, with --split=#\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
            14,
        );
        fprintf(
            stderr,
            b"nb of samples too low\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        exit(14);
    }
    if fs.totalSizeToLoad < maxDictSize as i64 * 8 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : data size of samples too small for target dictionary size \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Samples should be about 100x larger than target dictionary size \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    if (loadedSize as i64) < fs.totalSizeToLoad && displayLevel >= 1 {
        fprintf(
            stderr,
            b"Training samples set too large (%u MB); training on %u MB only...\n\0" as *const u8
                as *const core::ffi::c_char,
            (fs.totalSizeToLoad / i64::from((1) << 20)) as core::ffi::c_uint,
            (loadedSize / ((1) << 20) as size_t) as core::ffi::c_uint,
        );
    }
    nbSamplesLoaded = DiB_loadFiles(
        srcBuffer,
        &mut loadedSize,
        sampleSizes,
        fs.nbSamples,
        fileNamesTable,
        nbFiles,
        chunkSize,
        displayLevel,
    );
    let mut dictSize = ZSTD_error_GENERIC as core::ffi::c_int as size_t;
    if !params.is_null() {
        DiB_fillNoise(
            (srcBuffer as *mut core::ffi::c_char).add(loadedSize) as *mut core::ffi::c_void,
            NOISELENGTH as size_t,
        );
        dictSize = ZDICT_trainFromBuffer_legacy(
            dictBuffer,
            maxDictSize,
            srcBuffer,
            sampleSizes,
            nbSamplesLoaded as core::ffi::c_uint,
            *params,
        );
    } else if !coverParams.is_null() {
        if optimize != 0 {
            dictSize = ZDICT_optimizeTrainFromBuffer_cover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as core::ffi::c_uint,
                coverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let splitPercentage = ((*coverParams).splitPoint * 100.0) as core::ffi::c_uint;
                if displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nsteps=%u\nsplit=%u\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*coverParams).k,
                        (*coverParams).d,
                        (*coverParams).steps,
                        splitPercentage,
                    );
                }
            }
        } else {
            dictSize = ZDICT_trainFromBuffer_cover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as core::ffi::c_uint,
                *coverParams,
            );
        }
    } else if !fastCoverParams.is_null() {
        if optimize != 0 {
            dictSize = ZDICT_optimizeTrainFromBuffer_fastCover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as core::ffi::c_uint,
                fastCoverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let splitPercentage_0 =
                    ((*fastCoverParams).splitPoint * 100.0) as core::ffi::c_uint;
                if displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*fastCoverParams).k,
                        (*fastCoverParams).d,
                        (*fastCoverParams).f,
                        (*fastCoverParams).steps,
                        splitPercentage_0,
                        (*fastCoverParams).accel,
                    );
                }
            }
        } else {
            dictSize = ZDICT_trainFromBuffer_fastCover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as core::ffi::c_uint,
                *fastCoverParams,
            );
        }
    }
    if ZDICT_isError(dictSize) != 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictionary training failed : %s \n\0" as *const u8 as *const core::ffi::c_char,
                ZDICT_getErrorName(dictSize),
            );
        }
        result = 1;
    } else {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"Save dictionary of size %u into file %s \n\0" as *const u8
                    as *const core::ffi::c_char,
                dictSize as core::ffi::c_uint,
                dictFileName,
            );
        }
        DiB_saveDict(dictFileName, dictBuffer, dictSize);
    }
    free(srcBuffer);
    free(sampleSizes as *mut core::ffi::c_void);
    free(dictBuffer);
    result
}
