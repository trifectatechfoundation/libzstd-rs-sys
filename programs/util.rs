use std::ffi::CStr;
use std::io;

#[cfg(target_os = "linux")]
use libc::__errno_location;
#[cfg(target_vendor = "apple")]
use libc::__error as __errno_location;
use libc::{
    calloc, chmod, chown, closedir, dirent, exit, fchmod, fchown, fclose, feof, ferror, fgets,
    fileno, fopen, fprintf, fread, free, getchar, isatty, malloc, memcpy, mkdir, mode_t, opendir,
    readdir, realloc, size_t, strchr, strcmp, strdup, strlen, strncmp, strrchr, strstr, strtol,
    sysconf, timespec, DIR, FILE, _SC_NPROCESSORS_ONLN,
};

extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn stat(__file: *const core::ffi::c_char, __buf: *mut stat) -> core::ffi::c_int;
    fn fstat(__fd: core::ffi::c_int, __buf: *mut stat) -> core::ffi::c_int;
    fn lstat(__file: *const core::ffi::c_char, __buf: *mut stat) -> core::ffi::c_int;
    fn utimensat(
        __fd: core::ffi::c_int,
        __path: *const core::ffi::c_char,
        __times: *const timespec,
        __flags: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
type __dev_t = core::ffi::c_ulong;
type __uid_t = core::ffi::c_uint;
type __gid_t = core::ffi::c_uint;
type __ino_t = core::ffi::c_ulong;
type __mode_t = core::ffi::c_uint;
type __nlink_t = core::ffi::c_ulong;
type __off_t = core::ffi::c_long;
type __off64_t = core::ffi::c_long;
type __time_t = core::ffi::c_long;
type __blksize_t = core::ffi::c_long;
type __blkcnt_t = core::ffi::c_long;
type __syscall_slong_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
type __compar_fn_t = Option<
    unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> core::ffi::c_int,
>;
pub type stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_HumanReadableSize_t {
    pub value: core::ffi::c_double,
    pub precision: core::ffi::c_int,
    pub suffix: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union charunion {
    pub chr: *mut core::ffi::c_char,
    pub cchr: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const core::ffi::c_char,
    pub buf: *mut core::ffi::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
pub const EOF: core::ffi::c_int = -(1);
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const core::ffi::c_char) -> core::ffi::c_int {
    strtol(__nptr, core::ptr::null_mut(), 10) as core::ffi::c_int
}
pub const PATH_SEP: core::ffi::c_int = '/' as i32;
pub const UTIL_FILESIZE_UNKNOWN: core::ffi::c_int = -(1);
pub const AT_FDCWD: core::ffi::c_int = -(100);
pub const __S_IFMT: core::ffi::c_int = 0o170000 as core::ffi::c_int;
pub const UTIME_NOW: core::ffi::c_long = ((1) << 30) - 1;
pub const ENOMEM: core::ffi::c_int = 12;
pub const EEXIST: core::ffi::c_int = 17;
static mut g_traceDepth: core::ffi::c_int = 0;
pub static mut g_traceFileStat: core::ffi::c_int = 0;
unsafe fn UTIL_realloc(ptr: *mut core::ffi::c_void, size: size_t) -> *mut core::ffi::c_void {
    let newptr = realloc(ptr, size);
    if !newptr.is_null() {
        return newptr;
    }
    free(ptr);
    core::ptr::null_mut()
}
pub static mut g_utilDisplayLevel: core::ffi::c_int = 0;
pub unsafe fn UTIL_requireUserConfirmation(
    prompt: *const core::ffi::c_char,
    abortMsg: *const core::ffi::c_char,
    acceptableLetters: *const core::ffi::c_char,
    hasStdinInput: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut ch: core::ffi::c_int = 0;
    let mut result: core::ffi::c_int = 0;
    if hasStdinInput != 0 {
        fprintf(
            stderr,
            b"stdin is an input - not proceeding.\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 1;
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        prompt,
    );
    ch = getchar();
    result = 0;
    if (strchr(acceptableLetters, ch)).is_null() {
        fprintf(
            stderr,
            b"%s \n\0" as *const u8 as *const core::ffi::c_char,
            abortMsg,
        );
        result = 1;
    }
    while ch != EOF && ch != '\n' as i32 {
        ch = getchar();
    }
    result
}
pub const LIST_SIZE_INCREASE: size_t = 8 as size_t * ((1) << 10) as size_t;
pub const MAX_FILE_OF_FILE_NAMES_SIZE: core::ffi::c_int = ((1) << 20) * 50;
pub unsafe fn UTIL_traceFileStat() {
    g_traceFileStat = 1;
}
pub unsafe fn UTIL_fstat(
    fd: core::ffi::c_int,
    filename: *const core::ffi::c_char,
    statbuf: *mut stat_t,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_stat(%d, %s)\0" as *const u8 as *const core::ffi::c_char,
            fd,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if fd >= 0 {
        ret = core::ffi::c_int::from(fstat(fd, statbuf) == 0);
    } else {
        ret = core::ffi::c_int::from(stat(filename, statbuf) == 0);
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_stat(
    filename: *const core::ffi::c_char,
    statbuf: *mut stat_t,
) -> core::ffi::c_int {
    UTIL_fstat(-(1), filename, statbuf)
}
pub unsafe fn UTIL_isFdRegularFile(fd: core::ffi::c_int) -> core::ffi::c_int {
    let mut statbuf = core::mem::zeroed::<stat>();
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isFdRegularFile(%d)\0" as *const u8 as *const core::ffi::c_char,
            fd,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    ret = core::ffi::c_int::from(
        fd >= 0
            && UTIL_fstat(
                fd,
                b"\0" as *const u8 as *const core::ffi::c_char,
                &mut statbuf,
            ) != 0
            && UTIL_isRegularFileStat(&statbuf) != 0,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_isRegularFile(infilename: *const core::ffi::c_char) -> core::ffi::c_int {
    let mut statbuf = core::mem::zeroed::<stat>();
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isRegularFile(%s)\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    ret = core::ffi::c_int::from(
        UTIL_stat(infilename, &mut statbuf) != 0 && UTIL_isRegularFileStat(&statbuf) != 0,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_isRegularFileStat(statbuf: *const stat_t) -> core::ffi::c_int {
    core::ffi::c_int::from(
        core::ffi::c_int::from(
            (*statbuf).st_mode & __S_IFMT as __mode_t == 0o100000 as core::ffi::c_int as __mode_t,
        ) != 0,
    )
}
pub unsafe fn UTIL_chmod(
    filename: *const core::ffi::c_char,
    statbuf: *const stat_t,
    permissions: mode_t,
) -> core::ffi::c_int {
    UTIL_fchmod(-(1), filename, statbuf, permissions)
}
pub unsafe fn UTIL_fchmod(
    fd: core::ffi::c_int,
    filename: *const core::ffi::c_char,
    mut statbuf: *const stat_t,
    permissions: mode_t,
) -> core::ffi::c_int {
    let mut localStatBuf = core::mem::zeroed::<stat>();
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_chmod(%s, %#4o)\0" as *const u8 as *const core::ffi::c_char,
            filename,
            permissions as core::ffi::c_uint,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if statbuf.is_null() {
        if UTIL_fstat(fd, filename, &mut localStatBuf) == 0 {
            if g_traceFileStat != 0 {
                g_traceDepth -= 1;
                fprintf(
                    stderr,
                    b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                    g_traceDepth,
                    b"\0" as *const u8 as *const core::ffi::c_char,
                    0,
                );
            }
            return 0;
        }
        statbuf = &mut localStatBuf;
    }
    if UTIL_isRegularFileStat(statbuf) == 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                0,
            );
        }
        return 0;
    }
    if fd >= 0 {
        let mut ret: core::ffi::c_int = 0;
        if g_traceFileStat != 0 {
            fprintf(
                stderr,
                b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
            );
            fprintf(stderr, b"fchmod\0" as *const u8 as *const core::ffi::c_char);
            fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
            g_traceDepth += 1;
        }
        ret = fchmod(fd, permissions);
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                ret,
            );
        }
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                ret,
            );
        }
        ret
    } else {
        let mut ret_0: core::ffi::c_int = 0;
        if g_traceFileStat != 0 {
            fprintf(
                stderr,
                b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
            );
            fprintf(stderr, b"chmod\0" as *const u8 as *const core::ffi::c_char);
            fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
            g_traceDepth += 1;
        }
        ret_0 = chmod(filename, permissions);
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                ret_0,
            );
        }
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                ret_0,
            );
        }
        ret_0
    }
}
pub unsafe fn UTIL_utime(
    filename: *const core::ffi::c_char,
    statbuf: *const stat_t,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_utime(%s)\0" as *const u8 as *const core::ffi::c_char,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    let mut timebuf: [timespec; 2] = [{
        timespec {
            tv_sec: 0,
            tv_nsec: UTIME_NOW,
        }
    }; 2];
    *timebuf.as_mut_ptr().offset(1) = (*statbuf).st_mtim;
    ret = utimensat(
        AT_FDCWD,
        filename,
        timebuf.as_mut_ptr() as *const timespec,
        0,
    );
    *__errno_location() = 0;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_setFileStat(
    filename: *const core::ffi::c_char,
    statbuf: *const stat_t,
) -> core::ffi::c_int {
    UTIL_setFDStat(-(1), filename, statbuf)
}
pub unsafe fn UTIL_setFDStat(
    fd: core::ffi::c_int,
    filename: *const core::ffi::c_char,
    statbuf: *const stat_t,
) -> core::ffi::c_int {
    let mut res = 0;
    let mut curStatBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_setFileStat(%d, %s)\0" as *const u8 as *const core::ffi::c_char,
            fd,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if UTIL_fstat(fd, filename, &mut curStatBuf) == 0 || UTIL_isRegularFileStat(&curStatBuf) == 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                -(1),
            );
        }
        return -(1);
    }
    if fd >= 0 {
        res += fchown(fd, -(1 as core::ffi::c_int) as __uid_t, (*statbuf).st_gid);
    } else {
        res += chown(
            filename,
            -(1 as core::ffi::c_int) as __uid_t,
            (*statbuf).st_gid,
        );
    }
    res += UTIL_fchmod(
        fd,
        filename,
        &curStatBuf,
        ((*statbuf).st_mode & 0o777) as mode_t,
    );
    if fd >= 0 {
        res += fchown(fd, (*statbuf).st_uid, -(1 as core::ffi::c_int) as __gid_t);
    } else {
        res += chown(
            filename,
            (*statbuf).st_uid,
            -(1 as core::ffi::c_int) as __gid_t,
        );
    }
    *__errno_location() = 0;
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            -res,
        );
    }
    -res
}
pub unsafe fn UTIL_isDirectory(infilename: *const core::ffi::c_char) -> core::ffi::c_int {
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isDirectory(%s)\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    ret = core::ffi::c_int::from(
        UTIL_stat(infilename, &mut statbuf) != 0 && UTIL_isDirectoryStat(&statbuf) != 0,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_isDirectoryStat(statbuf: *const stat_t) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isDirectoryStat()\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    ret = core::ffi::c_int::from(
        core::ffi::c_int::from(
            (*statbuf).st_mode & __S_IFMT as __mode_t == 0o40000 as core::ffi::c_int as __mode_t,
        ) != 0,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe extern "C" fn UTIL_compareStr(
    p1: *const core::ffi::c_void,
    p2: *const core::ffi::c_void,
) -> core::ffi::c_int {
    strcmp(
        *(p1 as *const *mut core::ffi::c_char),
        *(p2 as *const *mut core::ffi::c_char),
    )
}
pub unsafe fn UTIL_isSameFile(
    fName1: *const core::ffi::c_char,
    fName2: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    assert!(!fName1.is_null());
    assert!(!fName2.is_null());
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isSameFile(%s, %s)\0" as *const u8 as *const core::ffi::c_char,
            fName1,
            fName2,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    let mut file1Stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut file2Stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    ret = core::ffi::c_int::from(
        UTIL_stat(fName1, &mut file1Stat) != 0
            && UTIL_stat(fName2, &mut file2Stat) != 0
            && UTIL_isSameFileStat(fName1, fName2, &file1Stat, &file2Stat) != 0,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_isSameFileStat(
    fName1: *const core::ffi::c_char,
    fName2: *const core::ffi::c_char,
    file1Stat: *const stat_t,
    file2Stat: *const stat_t,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    assert!(!fName1.is_null());
    assert!(!fName2.is_null());
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isSameFileStat(%s, %s)\0" as *const u8 as *const core::ffi::c_char,
            fName1,
            fName2,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    ret = core::ffi::c_int::from(
        (*file1Stat).st_dev == (*file2Stat).st_dev && (*file1Stat).st_ino == (*file2Stat).st_ino,
    );
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_isFIFO(infilename: *const core::ffi::c_char) -> core::ffi::c_int {
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isFIFO(%s)\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(infilename, &mut statbuf) != 0 && UTIL_isFIFOStat(&statbuf) != 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        return 1;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            0,
        );
    }
    0
}
pub unsafe fn UTIL_isFIFOStat(statbuf: *const stat_t) -> core::ffi::c_int {
    if (*statbuf).st_mode & __S_IFMT as __mode_t == 0o10000 as core::ffi::c_int as __mode_t {
        return 1;
    }
    0
}
pub unsafe fn UTIL_isFileDescriptorPipe(filename: *const core::ffi::c_char) -> core::ffi::c_int {
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isFileDescriptorPipe(%s)\0" as *const u8 as *const core::ffi::c_char,
            filename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if core::ffi::c_int::from(*filename.offset(0)) == '/' as i32
        && strncmp(
            filename,
            b"/dev/fd/\0" as *const u8 as *const core::ffi::c_char,
            8,
        ) == 0
    {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        return 1;
    }
    if core::ffi::c_int::from(*filename.offset(0)) == '/' as i32
        && strncmp(
            filename,
            b"/proc/self/fd/\0" as *const u8 as *const core::ffi::c_char,
            14,
        ) == 0
    {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        return 1;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            0,
        );
    }
    0
}
pub unsafe fn UTIL_isBlockDevStat(statbuf: *const stat_t) -> core::ffi::c_int {
    if (*statbuf).st_mode & __S_IFMT as __mode_t == 0o60000 as core::ffi::c_int as __mode_t {
        return 1;
    }
    0
}
pub unsafe fn UTIL_isLink(infilename: *const core::ffi::c_char) -> core::ffi::c_int {
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isLink(%s)\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let r = lstat(infilename, &mut statbuf);
    if r == 0 && statbuf.st_mode & __S_IFMT as __mode_t == 0o120000 as core::ffi::c_int as __mode_t
    {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        return 1;
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            0,
        );
    }
    0
}
static mut g_fakeStdinIsConsole: core::ffi::c_int = 0;
static mut g_fakeStderrIsConsole: core::ffi::c_int = 0;
static mut g_fakeStdoutIsConsole: core::ffi::c_int = 0;
pub unsafe fn UTIL_isConsole(file: *mut FILE) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_isConsole(%d)\0" as *const u8 as *const core::ffi::c_char,
            fileno(file),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if file == stdin && g_fakeStdinIsConsole != 0 {
        ret = 1;
    } else if file == stderr && g_fakeStderrIsConsole != 0 {
        ret = 1;
    } else if file == stdout && g_fakeStdoutIsConsole != 0 {
        ret = 1;
    } else {
        ret = isatty(fileno(file));
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}
pub unsafe fn UTIL_fakeStdinIsConsole() {
    g_fakeStdinIsConsole = 1;
}
pub unsafe fn UTIL_fakeStdoutIsConsole() {
    g_fakeStdoutIsConsole = 1;
}
pub unsafe fn UTIL_fakeStderrIsConsole() {
    g_fakeStderrIsConsole = 1;
}
pub unsafe fn UTIL_getFileSize(infilename: *const core::ffi::c_char) -> u64 {
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_getFileSize(%s)\0" as *const u8 as *const core::ffi::c_char,
            infilename,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    if UTIL_stat(infilename, &mut statbuf) == 0 {
        if g_traceFileStat != 0 {
            g_traceDepth -= 1;
            fprintf(
                stderr,
                b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                g_traceDepth,
                b"\0" as *const u8 as *const core::ffi::c_char,
                -(1),
            );
        }
        return UTIL_FILESIZE_UNKNOWN as u64;
    }
    let size = UTIL_getFileSizeStat(&statbuf);
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            size as core::ffi::c_int,
        );
    }
    size
}
pub unsafe fn UTIL_getFileSizeStat(statbuf: *const stat_t) -> u64 {
    if UTIL_isRegularFileStat(statbuf) == 0 {
        return UTIL_FILESIZE_UNKNOWN as u64;
    }
    if (*statbuf).st_mode & __S_IFMT as __mode_t != 0o100000 as core::ffi::c_int as __mode_t {
        return UTIL_FILESIZE_UNKNOWN as u64;
    }
    (*statbuf).st_size as u64
}
pub unsafe fn UTIL_makeHumanReadableSize(size: u64) -> UTIL_HumanReadableSize_t {
    let mut hrs = UTIL_HumanReadableSize_t {
        value: 0.,
        precision: 0,
        suffix: core::ptr::null::<core::ffi::c_char>(),
    };
    if g_utilDisplayLevel > 3 {
        if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 53 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 20) as core::ffi::c_double;
            hrs.suffix = b" MiB\0" as *const u8 as *const core::ffi::c_char;
            hrs.precision = 2;
        } else {
            hrs.value = size as core::ffi::c_double;
            hrs.suffix = b" B\0" as *const u8 as *const core::ffi::c_char;
            hrs.precision = 0;
        }
    } else {
        if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 60 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 60) as core::ffi::c_double;
            hrs.suffix = b" EiB\0" as *const u8 as *const core::ffi::c_char;
        } else if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 50 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 50) as core::ffi::c_double;
            hrs.suffix = b" PiB\0" as *const u8 as *const core::ffi::c_char;
        } else if size as core::ffi::c_ulonglong
            >= (1 as core::ffi::c_ulonglong) << 40 as core::ffi::c_int
        {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 40) as core::ffi::c_double;
            hrs.suffix = b" TiB\0" as *const u8 as *const core::ffi::c_char;
        } else if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 30 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 30) as core::ffi::c_double;
            hrs.suffix = b" GiB\0" as *const u8 as *const core::ffi::c_char;
        } else if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 20 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 20) as core::ffi::c_double;
            hrs.suffix = b" MiB\0" as *const u8 as *const core::ffi::c_char;
        } else if size as core::ffi::c_ulonglong >= (1 as core::ffi::c_ulonglong) << 10 {
            hrs.value = size as core::ffi::c_double
                / ((1 as core::ffi::c_ulonglong) << 10) as core::ffi::c_double;
            hrs.suffix = b" KiB\0" as *const u8 as *const core::ffi::c_char;
        } else {
            hrs.value = size as core::ffi::c_double;
            hrs.suffix = b" B\0" as *const u8 as *const core::ffi::c_char;
        }
        if hrs.value >= 100.0 || hrs.value as u64 == size {
            hrs.precision = 0;
        } else if hrs.value >= 10.0 {
            hrs.precision = 1;
        } else if hrs.value > 1.0 {
            hrs.precision = 2;
        } else {
            hrs.precision = 3;
        }
    }
    hrs
}
pub unsafe fn UTIL_getTotalFileSize(
    fileNamesTable: *const *const core::ffi::c_char,
    nbFiles: core::ffi::c_uint,
) -> u64 {
    let mut total = 0u64;
    let mut n: core::ffi::c_uint = 0;
    if g_traceFileStat != 0 {
        fprintf(
            stderr,
            b"Trace:FileStat: %*s> \0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"UTIL_getTotalFileSize(%u)\0" as *const u8 as *const core::ffi::c_char,
            nbFiles,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        g_traceDepth += 1;
    }
    n = 0;
    while n < nbFiles {
        let size = UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if size == UTIL_FILESIZE_UNKNOWN as u64 {
            if g_traceFileStat != 0 {
                g_traceDepth -= 1;
                fprintf(
                    stderr,
                    b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
                    g_traceDepth,
                    b"\0" as *const u8 as *const core::ffi::c_char,
                    -(1),
                );
            }
            return UTIL_FILESIZE_UNKNOWN as u64;
        }
        total = total.wrapping_add(size);
        n = n.wrapping_add(1);
    }
    if g_traceFileStat != 0 {
        g_traceDepth -= 1;
        fprintf(
            stderr,
            b"Trace:FileStat: %*s< %d\n\0" as *const u8 as *const core::ffi::c_char,
            g_traceDepth,
            b"\0" as *const u8 as *const core::ffi::c_char,
            total as core::ffi::c_int,
        );
    }
    total
}
unsafe fn UTIL_readFileContent(
    inFile: *mut FILE,
    totalReadPtr: *mut size_t,
) -> *mut core::ffi::c_char {
    let mut bufSize = (64 * ((1) << 10)) as size_t;
    let mut totalRead = 0;
    let mut bytesRead = 0;
    let mut buf = malloc(bufSize) as *mut core::ffi::c_char;
    if buf.is_null() {
        return core::ptr::null_mut();
    }
    loop {
        bytesRead = fread(
            buf.add(totalRead) as *mut core::ffi::c_void,
            1,
            bufSize.wrapping_sub(totalRead).wrapping_sub(1),
            inFile,
        );
        if bytesRead <= 0 {
            break;
        }
        totalRead = totalRead.wrapping_add(bytesRead);
        if bufSize.wrapping_sub(totalRead) < ((1) << 10) as size_t {
            if bufSize >= MAX_FILE_OF_FILE_NAMES_SIZE as size_t {
                free(buf as *mut core::ffi::c_void);
                return core::ptr::null_mut();
            }
            let mut newBufSize = bufSize * 2;
            if newBufSize > MAX_FILE_OF_FILE_NAMES_SIZE as size_t {
                newBufSize = MAX_FILE_OF_FILE_NAMES_SIZE as size_t;
            }
            let newBuf =
                realloc(buf as *mut core::ffi::c_void, newBufSize) as *mut core::ffi::c_char;
            if newBuf.is_null() {
                free(buf as *mut core::ffi::c_void);
                return core::ptr::null_mut();
            }
            buf = newBuf;
            bufSize = newBufSize;
        }
    }
    *buf.add(totalRead) = '\0' as i32 as core::ffi::c_char;
    *totalReadPtr = totalRead;
    buf
}
unsafe fn UTIL_processLines(buffer: *mut core::ffi::c_char, bufferSize: size_t) -> size_t {
    let mut lineCount = 0 as size_t;
    let mut i = 0;
    while i < bufferSize {
        if core::ffi::c_int::from(*buffer.add(i)) == '\n' as i32 {
            *buffer.add(i) = '\0' as i32 as core::ffi::c_char;
            lineCount = lineCount.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if bufferSize > 0
        && (i == 0 || core::ffi::c_int::from(*buffer.add(i.wrapping_sub(1))) != '\0' as i32)
    {
        lineCount = lineCount.wrapping_add(1);
    }
    lineCount
}
unsafe fn UTIL_createLinePointers(
    buffer: *mut core::ffi::c_char,
    numLines: size_t,
    bufferSize: size_t,
) -> *mut *const core::ffi::c_char {
    let mut lineIndex = 0;
    let mut pos = 0;
    let bufferPtrs =
        malloc(numLines.wrapping_mul(::core::mem::size_of::<*mut *const core::ffi::c_char>()));
    let linePointers = bufferPtrs as *mut *const core::ffi::c_char;
    if bufferPtrs.is_null() {
        return core::ptr::null_mut();
    }
    while lineIndex < numLines && pos < bufferSize {
        let mut len = 0;
        let fresh0 = lineIndex;
        lineIndex = lineIndex.wrapping_add(1);
        let fresh1 = &mut (*linePointers.add(fresh0));
        *fresh1 = buffer.add(pos);
        while pos.wrapping_add(len) < bufferSize
            && core::ffi::c_int::from(*buffer.add(pos.wrapping_add(len))) != '\0' as i32
        {
            len = len.wrapping_add(1);
        }
        pos = pos.wrapping_add(len);
        if pos < bufferSize {
            pos = pos.wrapping_add(1);
        }
    }
    if lineIndex != numLines {
        free(bufferPtrs);
        return core::ptr::null_mut();
    }
    linePointers
}
pub unsafe fn UTIL_createFileNamesTable_fromFileList(
    fileList: *const core::ffi::c_char,
) -> *mut FileNamesTable {
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut buffer = core::ptr::null_mut();
    let mut numLines = 0;
    let mut bufferSize = 0;
    if UTIL_stat(fileList, &mut statbuf) == 0 {
        return core::ptr::null_mut();
    }
    if UTIL_isRegularFileStat(&statbuf) == 0
        && UTIL_isFIFOStat(&statbuf) == 0
        && UTIL_isFileDescriptorPipe(fileList) == 0
    {
        return core::ptr::null_mut();
    }
    let inFile = fopen(fileList, b"rb\0" as *const u8 as *const core::ffi::c_char);
    if inFile.is_null() {
        return core::ptr::null_mut();
    }
    buffer = UTIL_readFileContent(inFile, &mut bufferSize);
    fclose(inFile);
    if buffer.is_null() {
        return core::ptr::null_mut();
    }
    numLines = UTIL_processLines(buffer, bufferSize);
    if numLines == 0 {
        free(buffer as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    let linePointers = UTIL_createLinePointers(buffer, numLines, bufferSize);
    if linePointers.is_null() {
        free(buffer as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    UTIL_assembleFileNamesTable(linePointers, numLines, buffer)
}
unsafe fn UTIL_assembleFileNamesTable2(
    filenames: *mut *const core::ffi::c_char,
    tableSize: size_t,
    tableCapacity: size_t,
    buf: *mut core::ffi::c_char,
) -> *mut FileNamesTable {
    let table = malloc(::core::mem::size_of::<FileNamesTable>()) as *mut FileNamesTable;
    if table.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                803,
                b"table != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    (*table).fileNames = filenames;
    (*table).buf = buf;
    (*table).tableSize = tableSize;
    (*table).tableCapacity = tableCapacity;
    table
}
pub unsafe fn UTIL_assembleFileNamesTable(
    filenames: *mut *const core::ffi::c_char,
    tableSize: size_t,
    buf: *mut core::ffi::c_char,
) -> *mut FileNamesTable {
    UTIL_assembleFileNamesTable2(filenames, tableSize, tableSize, buf)
}
pub unsafe fn UTIL_freeFileNamesTable(table: *mut FileNamesTable) {
    if table.is_null() {
        return;
    }
    free((*table).fileNames as *mut core::ffi::c_void);
    free((*table).buf as *mut core::ffi::c_void);
    free(table as *mut core::ffi::c_void);
}
pub unsafe fn UTIL_allocateFileNamesTable(tableSize: size_t) -> *mut FileNamesTable {
    let fnTable = malloc(tableSize.wrapping_mul(::core::mem::size_of::<*const core::ffi::c_char>()))
        as *mut *const core::ffi::c_char;
    let mut fnt = core::ptr::null_mut::<FileNamesTable>();
    if fnTable.is_null() {
        return core::ptr::null_mut();
    }
    fnt = UTIL_assembleFileNamesTable(fnTable, tableSize, core::ptr::null_mut());
    (*fnt).tableSize = 0;
    fnt
}
pub unsafe fn UTIL_searchFileNamesTable(
    table: *mut FileNamesTable,
    name: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut i: size_t = 0;
    i = 0;
    while i < (*table).tableSize {
        if strcmp(*((*table).fileNames).add(i), name) == 0 {
            return i as core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    -(1)
}
pub unsafe fn UTIL_refFilename(fnt: *mut FileNamesTable, filename: *const core::ffi::c_char) {
    assert!((*fnt).tableSize < (*fnt).tableCapacity);
    let fresh2 = &mut (*((*fnt).fileNames).add((*fnt).tableSize));
    *fresh2 = filename;
    (*fnt).tableSize = ((*fnt).tableSize).wrapping_add(1);
}
unsafe fn getTotalTableSize(table: *mut FileNamesTable) -> size_t {
    let mut fnb: size_t = 0;
    let mut totalSize = 0 as size_t;
    fnb = 0;
    while fnb < (*table).tableSize && !(*((*table).fileNames).add(fnb)).is_null() {
        totalSize =
            totalSize.wrapping_add((strlen(*((*table).fileNames).add(fnb))).wrapping_add(1));
        fnb = fnb.wrapping_add(1);
    }
    totalSize
}
pub unsafe fn UTIL_mergeFileNamesTable(
    table1: *mut FileNamesTable,
    table2: *mut FileNamesTable,
) -> *mut FileNamesTable {
    let mut newTableIdx = 0 as core::ffi::c_uint;
    let mut pos = 0;
    let mut newTotalTableSize: size_t = 0;
    let mut buf = core::ptr::null_mut::<core::ffi::c_char>();
    let newTable = UTIL_assembleFileNamesTable(core::ptr::null_mut(), 0, core::ptr::null_mut());
    if newTable.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                870,
                b"newTable != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    newTotalTableSize = (getTotalTableSize(table1)).wrapping_add(getTotalTableSize(table2));
    buf = calloc(
        newTotalTableSize,
        ::core::mem::size_of::<core::ffi::c_char>(),
    ) as *mut core::ffi::c_char;
    if buf.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                875,
                b"buf != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    (*newTable).buf = buf;
    (*newTable).tableSize = ((*table1).tableSize).wrapping_add((*table2).tableSize);
    (*newTable).fileNames = calloc(
        (*newTable).tableSize,
        ::core::mem::size_of::<*const core::ffi::c_char>(),
    ) as *mut *const core::ffi::c_char;
    if ((*newTable).fileNames).is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                880,
                b"newTable->fileNames != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    let mut idx1: core::ffi::c_uint = 0;
    idx1 = 0;
    while (idx1 as size_t) < (*table1).tableSize
        && !(*((*table1).fileNames).offset(idx1 as isize)).is_null()
        && pos < newTotalTableSize
    {
        let curLen = strlen(*((*table1).fileNames).offset(idx1 as isize));
        memcpy(
            buf.add(pos) as *mut core::ffi::c_void,
            *((*table1).fileNames).offset(idx1 as isize) as *const core::ffi::c_void,
            curLen,
        );
        assert!(newTableIdx as size_t <= (*newTable).tableSize);
        let fresh3 = &mut (*((*newTable).fileNames).offset(newTableIdx as isize));
        *fresh3 = buf.add(pos);
        pos = pos.wrapping_add(curLen.wrapping_add(1));
        idx1 = idx1.wrapping_add(1);
        newTableIdx = newTableIdx.wrapping_add(1);
    }
    let mut idx2: core::ffi::c_uint = 0;
    idx2 = 0;
    while (idx2 as size_t) < (*table2).tableSize
        && !(*((*table2).fileNames).offset(idx2 as isize)).is_null()
        && pos < newTotalTableSize
    {
        let curLen_0 = strlen(*((*table2).fileNames).offset(idx2 as isize));
        memcpy(
            buf.add(pos) as *mut core::ffi::c_void,
            *((*table2).fileNames).offset(idx2 as isize) as *const core::ffi::c_void,
            curLen_0,
        );
        assert!((newTableIdx as size_t) < (*newTable).tableSize);
        let fresh4 = &mut (*((*newTable).fileNames).offset(newTableIdx as isize));
        *fresh4 = buf.add(pos);
        pos = pos.wrapping_add(curLen_0.wrapping_add(1));
        idx2 = idx2.wrapping_add(1);
        newTableIdx = newTableIdx.wrapping_add(1);
    }
    assert!(pos <= newTotalTableSize);
    (*newTable).tableSize = newTableIdx as size_t;
    UTIL_freeFileNamesTable(table1);
    UTIL_freeFileNamesTable(table2);
    newTable
}
unsafe fn UTIL_prepareFileList(
    dirName: *const core::ffi::c_char,
    bufStart: *mut *mut core::ffi::c_char,
    pos: *mut size_t,
    bufEnd: *mut *mut core::ffi::c_char,
    followLinks: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut dir = core::ptr::null_mut::<DIR>();
    let mut entry = core::ptr::null_mut::<dirent>();
    let mut dirLength: size_t = 0;
    let mut nbFiles = 0;
    dir = opendir(dirName);
    if dir.is_null() {
        if g_utilDisplayLevel >= 1 {
            eprintln!(
                "Cannot open directory '{}': {}",
                CStr::from_ptr(dirName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        return 0;
    }
    dirLength = strlen(dirName);
    *__errno_location() = 0;
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        let mut path = core::ptr::null_mut::<core::ffi::c_char>();
        let mut fnameLength: size_t = 0;
        let mut pathLength: size_t = 0;
        if strcmp(
            ((*entry).d_name).as_mut_ptr(),
            b"..\0" as *const u8 as *const core::ffi::c_char,
        ) == 0
            || strcmp(
                ((*entry).d_name).as_mut_ptr(),
                b".\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
        {
            continue;
        }
        fnameLength = strlen(((*entry).d_name).as_mut_ptr());
        path =
            malloc(dirLength.wrapping_add(fnameLength).wrapping_add(2)) as *mut core::ffi::c_char;
        if path.is_null() {
            closedir(dir);
            return 0;
        }
        memcpy(
            path as *mut core::ffi::c_void,
            dirName as *const core::ffi::c_void,
            dirLength,
        );
        *path.add(dirLength) = '/' as i32 as core::ffi::c_char;
        memcpy(
            path.add(dirLength).offset(1) as *mut core::ffi::c_void,
            ((*entry).d_name).as_mut_ptr() as *const core::ffi::c_void,
            fnameLength,
        );
        pathLength = dirLength.wrapping_add(1).wrapping_add(fnameLength);
        *path.add(pathLength) = 0;
        if followLinks == 0 && UTIL_isLink(path) != 0 {
            if g_utilDisplayLevel >= 2 {
                fprintf(
                    stderr,
                    b"Warning : %s is a symbolic link, ignoring\n\0" as *const u8
                        as *const core::ffi::c_char,
                    path,
                );
            }
            free(path as *mut core::ffi::c_void);
        } else {
            if UTIL_isDirectory(path) != 0 {
                nbFiles += UTIL_prepareFileList(path, bufStart, pos, bufEnd, followLinks);
                if (*bufStart).is_null() {
                    free(path as *mut core::ffi::c_void);
                    closedir(dir);
                    return 0;
                }
            } else {
                if (*bufStart).add(*pos).add(pathLength) >= *bufEnd {
                    let newListSize = (*bufEnd).offset_from(*bufStart) as core::ffi::c_long
                        + LIST_SIZE_INCREASE as core::ffi::c_long;
                    assert!(newListSize >= 0);
                    *bufStart =
                        UTIL_realloc(*bufStart as *mut core::ffi::c_void, newListSize as size_t)
                            as *mut core::ffi::c_char;
                    if !(*bufStart).is_null() {
                        *bufEnd = (*bufStart).offset(newListSize as isize);
                    } else {
                        free(path as *mut core::ffi::c_void);
                        closedir(dir);
                        return 0;
                    }
                }
                if (*bufStart).add(*pos).add(pathLength) < *bufEnd {
                    memcpy(
                        (*bufStart).add(*pos) as *mut core::ffi::c_void,
                        path as *const core::ffi::c_void,
                        pathLength.wrapping_add(1),
                    );
                    *pos = (*pos).wrapping_add(pathLength.wrapping_add(1));
                    nbFiles += 1;
                }
            }
            free(path as *mut core::ffi::c_void);
            *__errno_location() = 0;
        }
    }
    if io::Error::last_os_error().raw_os_error().unwrap() != 0 {
        if g_utilDisplayLevel >= 1 {
            eprintln!(
                "readdir({}) error: {}",
                CStr::from_ptr(dirName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        free(*bufStart as *mut core::ffi::c_void);
        *bufStart = core::ptr::null_mut();
    }
    closedir(dir);
    nbFiles
}
pub unsafe fn UTIL_isCompressedFile(
    inputName: *const core::ffi::c_char,
    extensionList: &[&CStr],
) -> core::ffi::c_int {
    let ext = UTIL_getFileExtension(inputName);
    for &candidate_ext in extensionList {
        let isCompressedExtension = strcmp(ext, candidate_ext.as_ptr());
        if isCompressedExtension == 0 {
            return 1;
        }
    }
    0
}
pub unsafe fn UTIL_getFileExtension(
    infilename: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    let extension: *const core::ffi::c_char = strrchr(infilename, '.' as i32);
    if extension.is_null() || extension == infilename {
        return b"\0" as *const u8 as *const core::ffi::c_char;
    }
    extension
}
unsafe fn pathnameHas2Dots(pathname: *const core::ffi::c_char) -> core::ffi::c_int {
    let mut needle = pathname;
    loop {
        needle = strstr(needle, b"..\0" as *const u8 as *const core::ffi::c_char);
        if needle.is_null() {
            return 0;
        }
        if (needle == pathname || core::ffi::c_int::from(*needle.offset(-1_isize)) == PATH_SEP)
            && (core::ffi::c_int::from(*needle.offset(2)) == '\0' as i32
                || core::ffi::c_int::from(*needle.offset(2)) == PATH_SEP)
        {
            return 1;
        }
        needle = needle.offset(1);
    }
}
unsafe fn isFileNameValidForMirroredOutput(filename: *const core::ffi::c_char) -> core::ffi::c_int {
    core::ffi::c_int::from(pathnameHas2Dots(filename) == 0)
}
pub const DIR_DEFAULT_MODE: core::ffi::c_int = 0o755 as core::ffi::c_int;
unsafe fn getDirMode(dirName: *const core::ffi::c_char) -> mode_t {
    let mut st = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(dirName, &mut st) == 0 {
        eprintln!(
            "zstd: failed to get DIR stats {}: {}",
            CStr::from_ptr(dirName).to_string_lossy(),
            io::Error::last_os_error(),
        );
        return DIR_DEFAULT_MODE as mode_t;
    }
    if UTIL_isDirectoryStat(&st) == 0 {
        fprintf(
            stderr,
            b"zstd: expected directory: %s\n\0" as *const u8 as *const core::ffi::c_char,
            dirName,
        );
        return DIR_DEFAULT_MODE as mode_t;
    }
    st.st_mode as mode_t
}
unsafe fn makeDir(dir: *const core::ffi::c_char, mode: mode_t) -> core::ffi::c_int {
    let ret = mkdir(dir, mode);
    if ret != 0 {
        if io::Error::last_os_error().kind() == io::ErrorKind::AlreadyExists {
            return 0;
        }
        eprintln!(
            "zstd: failed to create DIR {}: {}",
            CStr::from_ptr(dir).to_string_lossy(),
            io::Error::last_os_error(),
        );
    }
    ret
}
unsafe fn convertPathnameToDirName(pathname: *mut core::ffi::c_char) {
    let mut len = 0;
    let mut pos = core::ptr::null_mut();
    assert!(!pathname.is_null());
    len = strlen(pathname);
    assert!(len > 0);
    while core::ffi::c_int::from(*pathname.add(len)) == PATH_SEP {
        *pathname.add(len) = 0;
        len = len.wrapping_sub(1);
    }
    if len == 0 {
        return;
    }
    pos = strrchr(pathname, PATH_SEP);
    if pos.is_null() {
        *pathname.offset(0) = '.' as i32 as core::ffi::c_char;
        *pathname.offset(1) = '\0' as i32 as core::ffi::c_char;
    } else {
        *pos = '\0' as i32 as core::ffi::c_char;
    };
}
unsafe fn trimLeadingRootChar(pathname: *const core::ffi::c_char) -> *const core::ffi::c_char {
    assert!(!pathname.is_null());
    if core::ffi::c_int::from(*pathname.offset(0)) == PATH_SEP {
        return pathname.offset(1);
    }
    pathname
}
unsafe fn trimLeadingCurrentDirConst(
    pathname: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    assert!(!pathname.is_null());
    if core::ffi::c_int::from(*pathname.offset(0)) == '.' as i32
        && core::ffi::c_int::from(*pathname.offset(1)) == PATH_SEP
    {
        return pathname.offset(2);
    }
    pathname
}
unsafe fn trimLeadingCurrentDir(pathname: *mut core::ffi::c_char) -> *mut core::ffi::c_char {
    let mut ptr = charunion {
        chr: core::ptr::null_mut::<core::ffi::c_char>(),
    };
    ptr.cchr = trimLeadingCurrentDirConst(pathname);
    ptr.chr
}
unsafe fn trimPath(pathname: *const core::ffi::c_char) -> *const core::ffi::c_char {
    trimLeadingRootChar(trimLeadingCurrentDirConst(pathname))
}
unsafe fn mallocAndJoin2Dir(
    dir1: *const core::ffi::c_char,
    dir2: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    assert!(!dir1.is_null() && !dir2.is_null());
    let dir1Size = strlen(dir1);
    let dir2Size = strlen(dir2);
    let mut outDirBuffer = core::ptr::null_mut::<core::ffi::c_char>();
    let mut buffer = core::ptr::null_mut::<core::ffi::c_char>();
    outDirBuffer =
        malloc(dir1Size.wrapping_add(dir2Size).wrapping_add(2)) as *mut core::ffi::c_char;
    if outDirBuffer.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                1216,
                b"outDirBuffer != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    memcpy(
        outDirBuffer as *mut core::ffi::c_void,
        dir1 as *const core::ffi::c_void,
        dir1Size,
    );
    *outDirBuffer.add(dir1Size) = 0;
    buffer = outDirBuffer.add(dir1Size);
    if dir1Size > 0 && core::ffi::c_int::from(*buffer.offset(-(1))) != PATH_SEP {
        *buffer = PATH_SEP as core::ffi::c_char;
        buffer = buffer.offset(1);
    }
    memcpy(
        buffer as *mut core::ffi::c_void,
        dir2 as *const core::ffi::c_void,
        dir2Size,
    );
    *buffer.add(dir2Size) = 0;
    outDirBuffer
}
pub unsafe fn UTIL_createMirroredDestDirName(
    srcFileName: *const core::ffi::c_char,
    outDirRootName: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let mut pathname = core::ptr::null_mut();
    if isFileNameValidForMirroredOutput(srcFileName) == 0 {
        return core::ptr::null_mut();
    }
    pathname = mallocAndJoin2Dir(outDirRootName, trimPath(srcFileName));
    convertPathnameToDirName(pathname);
    pathname
}
unsafe fn mirrorSrcDir(
    srcDirName: *mut core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut srcMode: mode_t = 0;
    let mut status = 0;
    let newDir = mallocAndJoin2Dir(outDirName, trimPath(srcDirName));
    if newDir.is_null() {
        return -ENOMEM;
    }
    srcMode = getDirMode(srcDirName);
    status = makeDir(newDir, srcMode);
    free(newDir as *mut core::ffi::c_void);
    status
}
unsafe fn mirrorSrcDirRecursive(
    srcDirName: *mut core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut status = 0;
    let mut pp = trimLeadingCurrentDir(srcDirName);
    let mut sp = core::ptr::null_mut();
    loop {
        sp = strchr(pp, PATH_SEP);
        if sp.is_null() {
            break;
        }
        if sp != pp {
            *sp = '\0' as i32 as core::ffi::c_char;
            status = mirrorSrcDir(srcDirName, outDirName);
            if status != 0 {
                return status;
            }
            *sp = PATH_SEP as core::ffi::c_char;
        }
        pp = sp.offset(1);
    }
    status = mirrorSrcDir(srcDirName, outDirName);
    status
}
unsafe fn makeMirroredDestDirsWithSameSrcDirMode(
    srcDirNames: *mut *mut core::ffi::c_char,
    nbFile: core::ffi::c_uint,
    outDirName: *const core::ffi::c_char,
) {
    let mut i = 0;
    i = 0;
    while i < nbFile {
        mirrorSrcDirRecursive(*srcDirNames.offset(i as isize), outDirName);
        i = i.wrapping_add(1);
    }
}
unsafe fn firstIsParentOrSameDirOfSecond(
    firstDir: *const core::ffi::c_char,
    secondDir: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let firstDirLen = strlen(firstDir);
    let secondDirLen = strlen(secondDir);
    core::ffi::c_int::from(
        firstDirLen <= secondDirLen
            && (core::ffi::c_int::from(*secondDir.add(firstDirLen)) == PATH_SEP
                || core::ffi::c_int::from(*secondDir.add(firstDirLen)) == '\0' as i32)
            && 0 == strncmp(firstDir, secondDir, firstDirLen),
    )
}
unsafe extern "C" fn compareDir(
    pathname1: *const core::ffi::c_void,
    pathname2: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let s1 = trimPath(*(pathname1 as *const *mut core::ffi::c_char));
    let s2 = trimPath(*(pathname2 as *const *mut core::ffi::c_char));
    strcmp(s1, s2)
}
unsafe fn makeUniqueMirroredDestDirs(
    srcDirNames: *mut *mut core::ffi::c_char,
    nbFile: core::ffi::c_uint,
    outDirName: *const core::ffi::c_char,
) {
    let mut i = 0;
    let mut uniqueDirNr = 0 as core::ffi::c_uint;
    let mut uniqueDirNames = core::ptr::null_mut();
    if nbFile == 0 {
        return;
    }
    uniqueDirNames =
        malloc((nbFile as size_t).wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_char>()))
            as *mut *mut core::ffi::c_char;
    if uniqueDirNames.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                1317,
                b"uniqueDirNames != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    qsort(
        srcDirNames as *mut core::ffi::c_void,
        nbFile as size_t,
        ::core::mem::size_of::<*mut core::ffi::c_char>(),
        Some(
            compareDir
                as unsafe extern "C" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    uniqueDirNr = 1;
    let fresh5 = &mut (*uniqueDirNames.offset(uniqueDirNr.wrapping_sub(1) as isize));
    *fresh5 = *srcDirNames.offset(0);
    i = 1;
    while i < nbFile {
        let prevDirName = *srcDirNames.offset(i.wrapping_sub(1) as isize);
        let currDirName = *srcDirNames.offset(i as isize);
        if firstIsParentOrSameDirOfSecond(trimPath(prevDirName), trimPath(currDirName)) == 0 {
            uniqueDirNr = uniqueDirNr.wrapping_add(1);
        }
        let fresh6 = &mut (*uniqueDirNames.offset(uniqueDirNr.wrapping_sub(1) as isize));
        *fresh6 = currDirName;
        i = i.wrapping_add(1);
    }
    makeMirroredDestDirsWithSameSrcDirMode(uniqueDirNames, uniqueDirNr, outDirName);
    free(uniqueDirNames as *mut core::ffi::c_void);
}
unsafe fn makeMirroredDestDirs(
    srcFileNames: *mut *mut core::ffi::c_char,
    nbFile: core::ffi::c_uint,
    outDirName: *const core::ffi::c_char,
) {
    let mut i = 0;
    i = 0;
    while i < nbFile {
        convertPathnameToDirName(*srcFileNames.offset(i as isize));
        i = i.wrapping_add(1);
    }
    makeUniqueMirroredDestDirs(srcFileNames, nbFile, outDirName);
}
pub unsafe fn UTIL_mirrorSourceFilesDirectories(
    inFileNames: *mut *const core::ffi::c_char,
    nbFile: core::ffi::c_uint,
    outDirName: *const core::ffi::c_char,
) {
    let mut i = 0;
    let mut validFilenamesNr = 0 as core::ffi::c_uint;
    let srcFileNames =
        malloc((nbFile as size_t).wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_char>()))
            as *mut *mut core::ffi::c_char;
    if srcFileNames.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                1359,
                b"srcFileNames != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    i = 0;
    while i < nbFile {
        if isFileNameValidForMirroredOutput(*inFileNames.offset(i as isize)) != 0 {
            let fname = strdup(*inFileNames.offset(i as isize));
            if fname.is_null() {
                if g_utilDisplayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                        b"util.c\0" as *const u8 as *const core::ffi::c_char,
                        1365,
                        b"fname != NULL\0" as *const u8 as *const core::ffi::c_char,
                    );
                }
                exit(1);
            }
            let fresh7 = validFilenamesNr;
            validFilenamesNr = validFilenamesNr.wrapping_add(1);
            let fresh8 = &mut (*srcFileNames.offset(fresh7 as isize));
            *fresh8 = fname;
        }
        i = i.wrapping_add(1);
    }
    if validFilenamesNr > 0 {
        makeDir(outDirName, DIR_DEFAULT_MODE as mode_t);
        makeMirroredDestDirs(srcFileNames, validFilenamesNr, outDirName);
    }
    i = 0;
    while i < validFilenamesNr {
        free(*srcFileNames.offset(i as isize) as *mut core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(srcFileNames as *mut core::ffi::c_void);
}
pub unsafe fn UTIL_createExpandedFNT(
    inputNames: *const *const core::ffi::c_char,
    nbIfns: size_t,
    followLinks: core::ffi::c_int,
) -> *mut FileNamesTable {
    let mut nbFiles: core::ffi::c_uint = 0;
    let mut buf = malloc(LIST_SIZE_INCREASE) as *mut core::ffi::c_char;
    let mut bufend = buf.add(LIST_SIZE_INCREASE);
    if buf.is_null() {
        return core::ptr::null_mut();
    }
    let mut ifnNb: size_t = 0;
    let mut pos: size_t = 0;
    ifnNb = 0;
    pos = 0;
    nbFiles = 0;
    while ifnNb < nbIfns {
        if UTIL_isDirectory(*inputNames.add(ifnNb)) == 0 {
            let len = strlen(*inputNames.add(ifnNb));
            if buf.add(pos).add(len) >= bufend {
                let newListSize = bufend.offset_from(buf) as core::ffi::c_long
                    + LIST_SIZE_INCREASE as core::ffi::c_long;
                assert!(newListSize >= 0);
                buf = UTIL_realloc(buf as *mut core::ffi::c_void, newListSize as size_t)
                    as *mut core::ffi::c_char;
                if buf.is_null() {
                    return core::ptr::null_mut();
                }
                bufend = buf.offset(newListSize as isize);
            }
            if buf.add(pos).add(len) < bufend {
                memcpy(
                    buf.add(pos) as *mut core::ffi::c_void,
                    *inputNames.add(ifnNb) as *const core::ffi::c_void,
                    len.wrapping_add(1),
                );
                pos = pos.wrapping_add(len.wrapping_add(1));
                nbFiles = nbFiles.wrapping_add(1);
            }
        } else {
            nbFiles = nbFiles.wrapping_add(UTIL_prepareFileList(
                *inputNames.add(ifnNb),
                &mut buf,
                &mut pos,
                &mut bufend,
                followLinks,
            ) as core::ffi::c_uint);
            if buf.is_null() {
                return core::ptr::null_mut();
            }
        }
        ifnNb = ifnNb.wrapping_add(1);
    }
    let mut ifnNb_0: size_t = 0;
    let mut pos_0: size_t = 0;
    let fntCapacity = nbFiles.wrapping_add(1) as size_t;
    let fileNamesTable =
        malloc(fntCapacity.wrapping_mul(::core::mem::size_of::<*const core::ffi::c_char>()))
            as *mut *const core::ffi::c_char;
    if fileNamesTable.is_null() {
        free(buf as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    ifnNb_0 = 0;
    pos_0 = 0;
    while ifnNb_0 < nbFiles as size_t {
        let fresh9 = &mut (*fileNamesTable.add(ifnNb_0));
        *fresh9 = buf.add(pos_0);
        if buf.add(pos_0) > bufend {
            free(buf as *mut core::ffi::c_void);
            free(fileNamesTable as *mut core::ffi::c_void);
            return core::ptr::null_mut();
        }
        pos_0 = pos_0.wrapping_add((strlen(*fileNamesTable.add(ifnNb_0))).wrapping_add(1)) as size_t
            as size_t;
        ifnNb_0 = ifnNb_0.wrapping_add(1);
    }
    UTIL_assembleFileNamesTable2(fileNamesTable, nbFiles as size_t, fntCapacity, buf)
}
pub unsafe fn UTIL_expandFNT(fnt: *mut *mut FileNamesTable, followLinks: core::ffi::c_int) {
    let newFNT = UTIL_createExpandedFNT((**fnt).fileNames, (**fnt).tableSize, followLinks);
    if newFNT.is_null() {
        if g_utilDisplayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s, %i : %s\0" as *const u8 as *const core::ffi::c_char,
                b"util.c\0" as *const u8 as *const core::ffi::c_char,
                1430,
                b"newFNT != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    UTIL_freeFileNamesTable(*fnt);
    *fnt = newFNT;
}
pub unsafe fn UTIL_createFNT_fromROTable(
    filenames: *mut *const core::ffi::c_char,
    nbFilenames: size_t,
) -> *mut FileNamesTable {
    let sizeof_FNTable =
        nbFilenames.wrapping_mul(::core::mem::size_of::<*const core::ffi::c_char>());
    let newFNTable = malloc(sizeof_FNTable) as *mut *const core::ffi::c_char;
    if newFNTable.is_null() {
        return core::ptr::null_mut();
    }
    memcpy(
        newFNTable as *mut core::ffi::c_void,
        filenames as *const core::ffi::c_void,
        sizeof_FNTable,
    );
    UTIL_assembleFileNamesTable(newFNTable, nbFilenames, core::ptr::null_mut())
}

/// parse /proc/cpuinfo
/// siblings / cpu cores should give hyperthreading ratio
/// otherwise fall back on sysconf
/// FIXME handle logic for OSes other than Linux
pub unsafe fn UTIL_countCores(logical: bool) -> core::ffi::c_int {
    static mut numCores: core::ffi::c_int = 0;

    if numCores != 0 {
        return numCores;
    }

    numCores = sysconf(_SC_NPROCESSORS_ONLN) as core::ffi::c_int;
    if numCores == -1 {
        // value not queryable, fall back on 1
        numCores = 1;
        return numCores;
    }

    // try to determine if there's hyperthreading
    let cpuinfo = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
    let mut buff: [core::ffi::c_char; BUF_SIZE] = [0; BUF_SIZE];

    let mut siblings = 0;
    let mut cpu_cores = 0;
    let mut ratio = 1;

    if cpuinfo.is_null() {
        // fall back on the sysconf value
        return numCores;
    }

    'failed: {
        // assume the cpu cores/siblings values will be constant across all
        // present processors
        while feof(cpuinfo) == 0 {
            if !fgets(buff.as_mut_ptr(), BUF_SIZE as _, cpuinfo).is_null() {
                if strncmp(buff.as_mut_ptr(), c"siblings".as_ptr(), 8) == 0 {
                    let sep = strchr(buff.as_mut_ptr(), core::ffi::c_int::from(b':'));
                    if sep.is_null() || *sep == 0 {
                        // formatting was broken?
                        break 'failed;
                    }

                    siblings = atoi(sep.offset(1));
                }
                if strncmp(buff.as_mut_ptr(), c"cpu cores".as_ptr(), 9) == 0 {
                    let sep = strchr(buff.as_mut_ptr(), core::ffi::c_int::from(b':'));
                    if sep.is_null() || *sep == 0 {
                        // formatting was broken?
                        break 'failed;
                    }

                    cpu_cores = atoi(sep.offset(1));
                }
            } else if ferror(cpuinfo) != 0 {
                // fall back on the sysconf value
                break 'failed;
            }
        }
        if siblings != 0 && cpu_cores != 0 && siblings > cpu_cores {
            ratio = siblings / cpu_cores;
        }

        if ratio != 0 && numCores > ratio && !logical {
            numCores /= ratio;
        }
    }

    fclose(cpuinfo);
    numCores
}
pub const BUF_SIZE: usize = 80;
pub unsafe fn UTIL_countPhysicalCores() -> core::ffi::c_int {
    UTIL_countCores(false)
}
pub unsafe fn UTIL_countLogicalCores() -> core::ffi::c_int {
    UTIL_countCores(true)
}
