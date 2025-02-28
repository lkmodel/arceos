use numeric_enum_macro::numeric_enum;

numeric_enum! {
#[repr(usize)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum FsSyscallId {
    // `fs`
    GETCWD = 17,
    EVENTFD = 19,
    EPOLL_CREATE = 20,
    EPOLL_CTL = 21,
    EPOLL_WAIT = 22,
    DUP = 23,
    DUP3 = 24,
    FCNTL64 = 25,
    IOCTL = 29,
    MKDIRAT = 34,
    SYMLINKAT = 36,
    UNLINKAT = 35,
    LINKAT = 37,
    RENAMEAT = 38,
    UNMOUNT = 39,
    MOUNT = 40,
    STATFS = 43,
    FTRUNCATE64 = 46,
    FACCESSAT = 48,
    CHDIR = 49,
    FCHMOD = 52,
    FCHMODAT = 53,
    FCHOWNAT = 54,
    FCHOWN = 55,
    OPENAT = 56,
    CLOSE = 57,
    PIPE2 = 59,
    GETDENTS64 = 61,
    LSEEK = 62,
    READ = 63,
    WRITE = 64,
    READV = 65,
    WRITEV = 66,
    PPOLL = 73,
    FSTATAT = 79,
    PREAD64 = 67,
    PWRITE64 = 68,
    SENDFILE64 = 71,
    PSELECT6 = 72,
    PREADLINKAT = 78,
    FSTAT = 80,
    SYNC = 81,
    FSYNC = 82,
    UTIMENSAT = 88,
    RENAMEAT2 = 276,
    COPYFILERANGE = 285,
}
}
