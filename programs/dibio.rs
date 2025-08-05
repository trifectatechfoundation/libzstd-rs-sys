use std::ptr;

use libc::{__errno_location, exit, fclose, fflush, fopen, fprintf, strerror, FILE};
use libzstd_rs::lib::dictBuilder::cover::{
    ZDICT_cover_params_t, ZDICT_optimizeTrainFromBuffer_cover, ZDICT_trainFromBuffer_cover,
};
use libzstd_rs::lib::dictBuilder::fastcover::{
    ZDICT_fastCover_params_t, ZDICT_optimizeTrainFromBuffer_fastCover,
    ZDICT_trainFromBuffer_fastCover,
};
use libzstd_rs::lib::dictBuilder::zdict::{
    ZDICT_getErrorName, ZDICT_isError, ZDICT_legacy_params_t, ZDICT_trainFromBuffer_legacy,
};
use libzstd_rs::lib::zstd::*;

use crate::timefn::{PTime, UTIL_clockSpanMicro, UTIL_getTime, UTIL_time_t};
use crate::util::UTIL_getFileSize;

extern "C" {
    static mut stderr: *mut FILE;
    fn fread(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn fwrite(
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
}
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileStats {
    pub totalSizeToLoad: i64,
    pub nbSamples: std::ffi::c_int,
    pub oneSampleTooLarge: std::ffi::c_int,
}
pub const UTIL_FILESIZE_UNKNOWN: std::ffi::c_int = -(1);
pub const SEC_TO_MICRO: std::ffi::c_int = 1000000;
pub const NULL: std::ffi::c_int = 0;
pub const SAMPLESIZE_MAX: std::ffi::c_int = 128 * ((1) << 10);
pub const MEMMULT: std::ffi::c_int = 11;
pub const COVER_MEMMULT: std::ffi::c_int = 9;
pub const FASTCOVER_MEMMULT: std::ffi::c_int = 1;
static mut g_maxMemory: size_t = 0;
pub const NOISELENGTH: std::ffi::c_int = 32;
static mut g_refreshRate: u64 = 0;
static mut g_displayClock: UTIL_time_t = UTIL_time_t { t: 0 } ;
pub const DEBUG: std::ffi::c_int = 0;
unsafe extern "C" fn DiB_getFileSize(mut fileName: *const std::ffi::c_char) -> i64 {
    let fileSize = UTIL_getFileSize(fileName);
    if fileSize == UTIL_FILESIZE_UNKNOWN as u64 {
        -(1) as i64
    } else {
        fileSize as i64
    }
}
unsafe extern "C" fn DiB_loadFiles(
    mut buffer: *mut std::ffi::c_void,
    mut bufferSizePtr: *mut size_t,
    mut sampleSizes: *mut size_t,
    mut sstSize: std::ffi::c_int,
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut targetChunkSize: size_t,
    mut displayLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let buff = buffer as *mut std::ffi::c_char;
    let mut totalDataLoaded = 0 as std::ffi::c_int as size_t;
    let mut nbSamplesLoaded = 0 as std::ffi::c_int;
    let mut fileIndex = 0 as std::ffi::c_int;
    let mut f = NULL as *mut FILE;
    while nbSamplesLoaded < sstSize && fileIndex < nbFiles {
        let mut fileDataLoaded: size_t = 0;
        let fileSize = DiB_getFileSize(*fileNamesTable.offset(fileIndex as isize));
        if fileSize <= 0 {
            fileIndex += 1;
        } else {
            f = fopen(
                *fileNamesTable.offset(fileIndex as isize),
                b"rb\0" as *const u8 as *const std::ffi::c_char,
            );
            if f.is_null() {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                    10,
                );
                fprintf(
                    stderr,
                    b"zstd: dictBuilder: %s %s \0" as *const u8 as *const std::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                    strerror(*__errno_location()),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
                exit(10);
            }
            if displayLevel >= 2
                && (UTIL_clockSpanMicro(g_displayClock) > g_refreshRate || displayLevel >= 4)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"Loading %s...       \r\0" as *const u8 as *const std::ffi::c_char,
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
                (if fileSize < (128 * ((1) << 10)) as i64 {
                    fileSize
                } else {
                    (128 * ((1) << 10)) as i64
                }) as size_t
            };
            if totalDataLoaded.wrapping_add(fileDataLoaded) > *bufferSizePtr {
                break;
            }
            if fread(
                buff.offset(totalDataLoaded as isize) as *mut std::ffi::c_void,
                1,
                fileDataLoaded,
                f,
            ) != fileDataLoaded
            {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11,
                );
                fprintf(
                    stderr,
                    b"Pb reading %s\0" as *const u8 as *const std::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
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
                        buff.offset(totalDataLoaded as isize) as *mut std::ffi::c_void,
                        1,
                        chunkSize,
                        f,
                    ) != chunkSize
                    {
                        fprintf(
                            stderr,
                            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                            11,
                        );
                        fprintf(
                            stderr,
                            b"Pb reading %s\0" as *const u8 as *const std::ffi::c_char,
                            *fileNamesTable.offset(fileIndex as isize),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
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
            f = NULL as *mut FILE;
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if displayLevel >= 4 {
        fprintf(
            stderr,
            b"Loaded %d KB total training data, %d nb samples \n\0" as *const u8
                as *const std::ffi::c_char,
            (totalDataLoaded / (1 * ((1) << 10)) as size_t) as std::ffi::c_int,
            nbSamplesLoaded,
        );
    }
    *bufferSizePtr = totalDataLoaded;
    nbSamplesLoaded
}
unsafe extern "C" fn DiB_rand(mut src: *mut u32) -> u32 {
    static mut prime1: u32 = 2654435761;
    static mut prime2: u32 = 2246822519;
    let mut rand32 = *src;
    rand32 *= prime1;
    rand32 ^= prime2;
    rand32 = rand32 << 13 | rand32 >> (32 - 13);
    *src = rand32;
    rand32 >> 5
}
unsafe extern "C" fn DiB_shuffle(
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_uint,
) {
    let mut seed = 0xfd2fb528 as std::ffi::c_uint;
    let mut i: std::ffi::c_uint = 0;
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
unsafe extern "C" fn DiB_findMaxMem(mut requiredMem: std::ffi::c_ulonglong) -> size_t {
    let step = (8 * ((1) << 20)) as size_t;
    let mut testmem = NULL as *mut std::ffi::c_void;
    requiredMem = (requiredMem >> 23).wrapping_add(1) << 23;
    requiredMem = requiredMem.wrapping_add(step as std::ffi::c_ulonglong);
    if requiredMem > g_maxMemory as std::ffi::c_ulonglong {
        requiredMem = g_maxMemory as std::ffi::c_ulonglong;
    }
    while testmem.is_null() {
        testmem = malloc(requiredMem as size_t);
        requiredMem = requiredMem.wrapping_sub(step as std::ffi::c_ulonglong);
    }
    free(testmem);
    requiredMem as size_t
}
unsafe extern "C" fn DiB_fillNoise(mut buffer: *mut std::ffi::c_void, mut length: size_t) {
    let prime1 = 2654435761 as std::ffi::c_uint;
    let prime2 = 2246822519 as std::ffi::c_uint;
    let mut acc = prime1;
    let mut p = 0;
    p = 0;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut std::ffi::c_uchar).offset(p as isize) = (acc >> 21) as std::ffi::c_uchar;
        p = p.wrapping_add(1);
    }
}
unsafe extern "C" fn DiB_saveDict(
    mut dictFileName: *const std::ffi::c_char,
    mut buff: *const std::ffi::c_void,
    mut buffSize: size_t,
) {
    let f = fopen(
        dictFileName,
        b"wb\0" as *const u8 as *const std::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            3,
        );
        fprintf(
            stderr,
            b"cannot open %s \0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(3);
    }
    let n = fwrite(buff, 1, buffSize, f);
    if n != buffSize {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            4,
        );
        fprintf(
            stderr,
            b"%s : write error\0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(4);
    }
    let n_0 = fclose(f) as size_t;
    if n_0 != 0 {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            5,
        );
        fprintf(
            stderr,
            b"%s : flush error\0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(5);
    }
}
unsafe extern "C" fn DiB_fileStats(
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut chunkSize: size_t,
    mut displayLevel: std::ffi::c_int,
) -> fileStats {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut n: std::ffi::c_int = 0;
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
                        as *const std::ffi::c_char,
                    *fileNamesTable.offset(n as isize),
                );
            }
        } else if chunkSize > 0 {
            fs.nbSamples += ((fileSize as size_t).wrapping_add(chunkSize).wrapping_sub(1)
                / chunkSize) as std::ffi::c_int;
            fs.totalSizeToLoad += fileSize;
        } else {
            if fileSize > SAMPLESIZE_MAX as i64 {
                fs.oneSampleTooLarge |= (fileSize > (2 * SAMPLESIZE_MAX) as i64) as std::ffi::c_int;
                if displayLevel >= 3 {
                    fprintf(
                        stderr,
                        b"Sample file '%s' is too large, limiting to %d KB\n\0" as *const u8
                            as *const std::ffi::c_char,
                        *fileNamesTable.offset(n as isize),
                        128 * ((1) << 10) / (1 * ((1) << 10)),
                    );
                }
            }
            fs.nbSamples += 1;
            fs.totalSizeToLoad += if fileSize < (128 * ((1) << 10)) as i64 {
                fileSize
            } else {
                (128 * ((1) << 10)) as i64
            };
        }
        n += 1;
    }
    if displayLevel >= 4 {
        fprintf(
            stderr,
            b"Found training data %d files, %d KB, %d samples\n\0" as *const u8
                as *const std::ffi::c_char,
            nbFiles,
            (fs.totalSizeToLoad / (1 * ((1) << 10)) as i64) as std::ffi::c_int,
            fs.nbSamples,
        );
    }
    fs
}
#[no_mangle]
pub unsafe extern "C" fn DiB_trainFromFiles(
    mut dictFileName: *const std::ffi::c_char,
    mut maxDictSize: size_t,
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut chunkSize: size_t,
    mut params: *mut ZDICT_legacy_params_t,
    mut coverParams: *mut ZDICT_cover_params_t,
    mut fastCoverParams: *mut ZDICT_fastCover_params_t,
    mut optimize: std::ffi::c_int,
    mut memLimit: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut sampleSizes = std::ptr::null_mut::<size_t>();
    let mut nbSamplesLoaded: std::ffi::c_int = 0;
    let mut loadedSize: size_t = 0;
    let mut srcBuffer = std::ptr::null_mut::<std::ffi::c_void>();
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
    }) as std::ffi::c_int;
    if displayLevel >= 3 {
        fprintf(
            stderr,
            b"Shuffling input files\n\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    DiB_shuffle(fileNamesTable, nbFiles as std::ffi::c_uint);
    fs = DiB_fileStats(fileNamesTable, nbFiles, chunkSize, displayLevel);
    let memMult = if !params.is_null() {
        MEMMULT
    } else if !coverParams.is_null() {
        COVER_MEMMULT
    } else {
        FASTCOVER_MEMMULT
    };
    let maxMem = DiB_findMaxMem((fs.totalSizeToLoad * memMult as i64) as std::ffi::c_ulonglong)
        / memMult as size_t;
    loadedSize = (if (if (maxMem as i64) < fs.totalSizeToLoad {
        maxMem as i64
    } else {
        fs.totalSizeToLoad
    }) < (2 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
        as i64
    {
        if (maxMem as i64) < fs.totalSizeToLoad {
            maxMem as i64
        } else {
            fs.totalSizeToLoad
        }
    } else {
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int) as i64
    }) as size_t;
    if memLimit != 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : setting manual memory limit for dictionary training data at %u MB \n\0"
                    as *const u8 as *const std::ffi::c_char,
                memLimit
                    .wrapping_div(
                        (1
                            * ((1) << 20))
                            as std::ffi::c_uint,
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
    sampleSizes = malloc(
        (fs.nbSamples as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as std::ffi::c_ulong),
    ) as *mut size_t;
    if fs.nbSamples != 0 && sampleSizes.is_null() || srcBuffer.is_null() || dictBuffer.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            12,
        );
        fprintf(
            stderr,
            b"not enough memory for DiB_trainFiles\0" as *const u8 as *const std::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(12);
    }
    if fs.oneSampleTooLarge != 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : some sample(s) are very large \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Note that dictionary is only useful for small samples. \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  As a consequence, only the first %u bytes of each sample are loaded \n\0"
                    as *const u8 as *const std::ffi::c_char,
                128 * ((1) << 10),
            );
        }
    }
    if fs.nbSamples < 5 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : nb of samples too low for proper processing !\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Please provide _one file per sample_.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Alternatively, split file(s) into fixed-size samples, with --split=#\n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            14,
        );
        fprintf(
            stderr,
            b"nb of samples too low\0" as *const u8 as *const std::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(14);
    }
    if fs.totalSizeToLoad < maxDictSize as i64 * 8 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Warning : data size of samples too small for target dictionary size \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"!  Samples should be about 100x larger than target dictionary size \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    if (loadedSize as i64) < fs.totalSizeToLoad && displayLevel >= 1 {
        fprintf(
            stderr,
            b"Training samples set too large (%u MB); training on %u MB only...\n\0" as *const u8
                as *const std::ffi::c_char,
            (fs.totalSizeToLoad / (1 * ((1) << 20)) as i64) as std::ffi::c_uint,
            (loadedSize / (1 * ((1) << 20)) as size_t) as std::ffi::c_uint,
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
    let mut dictSize = ZSTD_error_GENERIC as std::ffi::c_int as size_t;
    if !params.is_null() {
        DiB_fillNoise(
            (srcBuffer as *mut std::ffi::c_char).offset(loadedSize as isize)
                as *mut std::ffi::c_void,
            NOISELENGTH as size_t,
        );
        dictSize = ZDICT_trainFromBuffer_legacy(
            dictBuffer,
            maxDictSize,
            srcBuffer,
            sampleSizes,
            nbSamplesLoaded as std::ffi::c_uint,
            *params,
        );
    } else if !coverParams.is_null() {
        if optimize != 0 {
            dictSize = ZDICT_optimizeTrainFromBuffer_cover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as std::ffi::c_uint,
                coverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let mut splitPercentage = ((*coverParams).splitPoint
                    * 100 as std::ffi::c_int as std::ffi::c_double)
                    as std::ffi::c_uint;
                if displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nsteps=%u\nsplit=%u\n\0" as *const u8
                            as *const std::ffi::c_char,
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
                nbSamplesLoaded as std::ffi::c_uint,
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
                nbSamplesLoaded as std::ffi::c_uint,
                fastCoverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let mut splitPercentage_0 = ((*fastCoverParams).splitPoint
                    * 100 as std::ffi::c_int as std::ffi::c_double)
                    as std::ffi::c_uint;
                if displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\n\0" as *const u8
                            as *const std::ffi::c_char,
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
                nbSamplesLoaded as std::ffi::c_uint,
                *fastCoverParams,
            );
        }
    }
    if ZDICT_isError(dictSize) != 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictionary training failed : %s \n\0" as *const u8 as *const std::ffi::c_char,
                ZDICT_getErrorName(dictSize),
            );
        }
        result = 1;
    } else {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"Save dictionary of size %u into file %s \n\0" as *const u8
                    as *const std::ffi::c_char,
                dictSize as std::ffi::c_uint,
                dictFileName,
            );
        }
        DiB_saveDict(dictFileName, dictBuffer, dictSize);
    }
    free(srcBuffer);
    free(sampleSizes as *mut std::ffi::c_void);
    free(dictBuffer);
    result
}
unsafe extern "C" fn run_static_initializers() {
    g_maxMemory = if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 4 {
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
            .wrapping_sub(
                (64 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                    as std::ffi::c_uint,
            ) as size_t
    } else {
        ((512 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int)) as size_t)
            << ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
    };
    g_refreshRate = SEC_TO_MICRO as PTime / 6;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: unsafe extern "C" fn() = run_static_initializers;
