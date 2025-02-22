use alloc::string::String;
use axerrno::{AxError, AxResult};
use axio::{Read, Seek, SeekFrom, Write};
use axlog::debug;
use core::any::Any;

/// 文件系统信息
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Kstat {
    /// 设备
    pub st_dev: u64,
    /// `inode`编号
    pub st_ino: u64,
    /// 文件类型
    pub st_mode: u32,
    /// 硬链接数
    pub st_nlink: u32,
    /// 用户id
    pub st_uid: u32,
    /// 用户组id
    pub st_gid: u32,
    /// 设备号
    pub st_rdev: u64,
    /// `padding`
    pub _pad0: u64,
    /// 文件大小
    pub st_size: u64,
    /// 块大小
    pub st_blksize: u32,
    /// `padding`
    pub _pad1: u32,
    /// 块个数
    pub st_blocks: u64,
    /// 最后一次访问时间(秒)
    pub st_atime_sec: isize,
    /// 最后一次访问时间(纳秒)
    pub st_atime_nsec: isize,
    /// 最后一次修改时间(秒)
    pub st_mtime_sec: isize,
    /// 最后一次修改时间(纳秒)
    pub st_mtime_nsec: isize,
    /// 最后一次改变状态时间(秒)
    pub st_ctime_sec: isize,
    /// 最后一次改变状态时间(纳秒)
    pub st_ctime_nsec: isize,
}

use bitflags::*;

bitflags! {
    /// 指定文件打开时的权限
    #[derive(Clone, Copy, Default, Debug)]
    pub struct OpenFlags: u32 {
        /// 只读
        const RDONLY = 0;
        /// 只能写入
        const WRONLY = 1 << 0;
        /// 读写
        const RDWR = 1 << 1;
        /// 如文件不存在，可创建它
        const CREATE = 1 << 6;
        /// 确认一定是创建文件。如文件已存在，返回`EEXIST`。
        const EXCLUSIVE = 1 << 7;
        /// 使打开的文件不会成为该进程的控制终端。目前没有终端设置，不处理
        const NOCTTY = 1 << 8;
        /// 同上，在不同的库中可能会用到这个或者上一个
        const EXCL = 1 << 9;
        /// 非阻塞读写?(虽然不知道为什么但`date.lua`也要)
        /// 在 socket 中使用得较多
        const NON_BLOCK = 1 << 11;
        /// 要求把`CR-LF`都换成`LF`
        const TEXT = 1 << 14;
        /// 和上面不同，要求输入输出都不进行这个翻译
        const BINARY = 1 << 15;
        /// 对这个文件的输出需符合`IO`同步一致性。可以理解为随时`fsync`
        const DSYNC = 1 << 16;
        /// 如果是符号链接，不跟随符号链接去寻找文件，而是针对连接本身
        const NOFOLLOW = 1 << 17;
        /// 在`exec`时需关闭
        const CLOEXEC = 1 << 19;
        /// 是否是目录
        const DIR = 1 << 21;
        /// 是否是不命名临时常规文件
        const TMP_FILE = 1 << 22;
    }
}

impl OpenFlags {
    /// 获得文件的读/写权限
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
    /// 获取读权限
    pub fn readable(&self) -> bool {
        !self.contains(Self::WRONLY)
    }
    /// 获取写权限
    pub fn writable(&self) -> bool {
        self.contains(Self::WRONLY) || self.contains(Self::RDWR)
    }

    /// 获取创建权限
    pub fn creatable(&self) -> bool {
        self.contains(Self::CREATE)
    }
    /// 获取创建新文件权限
    /// 与上面的区别是，如果文件已存在，返回`EEXIST`
    pub fn new_creatable(&self) -> bool {
        self.contains(Self::EXCLUSIVE)
    }

    /// 获取是否是目录
    pub fn is_dir(&self) -> bool {
        self.contains(Self::DIR)
    }

    /// 获取是否需要在`exec()`时关闭
    pub fn is_close_on_exec(&self) -> bool {
        self.contains(Self::CLOEXEC)
    }
}

/// 文件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileIOType {
    /// 文件
    FileDesc,
    /// 目录
    DirDesc,
    /// 标准输入
    Stdin,
    /// 标准输出
    Stdout,
    /// 标准错误
    Stderr,
    /// 管道
    Pipe,
    /// 链接
    Link,
    /// Socket
    Socket,
    /// 其他
    Other,
}

/// File I/O trait. 文件I/O操作，用于设置文件描述符，值得注意的是，这里的read/write/seek都是不可变引用
///
/// 因为文件描述符读取的时候，是用到内部File成员的读取函数，自身应当为不可变，从而可以被Arc指针调用
pub trait FileIO: AsAny + Send + Sync {
    /// 读取操作
    fn read(&self, _buf: &mut [u8]) -> AxResult<usize> {
        Err(AxError::Unsupported) // 如果没有实现, 则返回Unsupported
    }

    /// 写入操作
    fn write(&self, _buf: &[u8]) -> AxResult<usize> {
        Err(AxError::Unsupported) // 如果没有实现, 则返回Unsupported
    }

    /// 刷新操作
    fn flush(&self) -> AxResult<()> {
        Err(AxError::Unsupported) // 如果没有实现, 则返回Unsupported
    }

    /// 移动指针操作
    fn seek(&self, _pos: SeekFrom) -> AxResult<u64> {
        Err(AxError::Unsupported) // 如果没有实现, 则返回Unsupported
    }

    /// Whether the file is readable
    fn readable(&self) -> bool;

    /// Whether the file is writable
    fn writable(&self) -> bool;

    /// Whether the file is executable
    fn executable(&self) -> bool;

    /// 获取类型
    fn get_type(&self) -> FileIOType;

    /// 获取路径
    fn get_path(&self) -> String {
        debug!("Function get_path not implemented");
        String::from("Function get_path not implemented")
    }
    /// 获取文件信息
    fn get_stat(&self) -> AxResult<Kstat> {
        Err(AxError::Unsupported) // 如果没有实现get_stat, 则返回Unsupported
    }

    /// 截断文件到指定长度
    fn truncate(&self, _len: usize) -> AxResult<()> {
        debug!("Function truncate not implemented");
        Err(AxError::Unsupported)
    }

    /// Debug
    fn print_content(&self) {
        debug!("Function print_content not implemented");
    }

    /// 设置文件状态
    fn set_status(&self, _flags: OpenFlags) -> bool {
        false
    }

    /// 获取文件状态
    fn get_status(&self) -> OpenFlags {
        OpenFlags::empty()
    }

    /// 设置`close_on_exec`位
    /// 设置成功返回`false`
    fn set_close_on_exec(&self, _is_set: bool) -> bool {
        false
    }

    /// 处于“意外情况”。在`(p)select`和`(p)poll`中会使用到
    ///
    /// 当前基本默认为`false`
    fn in_exceptional_conditions(&self) -> bool {
        false
    }

    /// 是否已经终止，对`pipe`来说相当于另一端已经关闭
    ///
    /// 对于其他文件类型来说，是在被`close`的时候终止，但这个时候已经没有对应的`filedesc`了，所以自然不会调用这个函数
    fn is_hang_up(&self) -> bool {
        false
    }

    /// 已准备好读。对于`pipe`来说，这意味着读端的`buffer`内有值
    fn ready_to_read(&self) -> bool {
        false
    }
    /// 已准备好写。对于`pipe`来说，这意味着写端的`buffer`未满
    fn ready_to_write(&self) -> bool {
        false
    }

    /// To control the file descriptor
    fn ioctl(&self, _request: usize, _arg1: usize) -> AxResult<isize> {
        Err(AxError::Unsupported)
    }
}

/// 用于给虚存空间进行懒分配
pub trait FileExt: Read + Write + Seek + AsAny + Send + Sync {
    /// Whether the file is readable
    fn readable(&self) -> bool;

    /// Whether the file is writable
    fn writable(&self) -> bool;

    /// Whether the file is executable
    fn executable(&self) -> bool;

    /// Read from position without changing cursor.
    fn read_from_seek(&mut self, pos: SeekFrom, mut buf: &mut [u8]) -> AxResult<usize> {
        // Get old position
        let old_pos = self
            .seek(SeekFrom::Current(0))
            .expect("Error get current pos in file");

        // Seek to read position
        let _ = self.seek(pos).unwrap();

        // Read
        let buf_len = buf.len();

        while !buf.is_empty() {
            match self.read(buf) {
                // Read to `EOF`
                Ok(0) => return Ok(buf_len - buf.len()),
                // Read n bytes
                Ok(n) => buf = &mut buf[n..],
                // Error
                Err(e) => return Err(e),
            }
        }
        let read_len = Ok(buf_len);
        // Seek back to old_pos
        let new_pos = self.seek(SeekFrom::Start(old_pos)).unwrap();

        assert_eq!(old_pos, new_pos);

        read_len
    }

    /// Write to position without changing cursor.
    fn write_to_seek(&mut self, pos: SeekFrom, buf: &[u8]) -> AxResult<usize> {
        // Get old position
        let old_pos = self
            .seek(SeekFrom::Current(0))
            .expect("Error get current pos in file");
        // Seek to write position
        let _ = self.seek(pos).unwrap();

        let write_len = self.write(buf);

        // Seek back to old_pos
        let _ = self.seek(SeekFrom::Start(old_pos)).unwrap();

        write_len
    }
}

/// `FileExt` 需要满足 `AsAny` 的要求，即可以转化为 `Any` 类型，从而能够进行向下类型转换。
pub trait AsAny {
    /// 把当前对象转化为 `Any` 类型，供后续`downcast`使用
    fn as_any(&self) -> &dyn Any;
    /// 供`downcast_mut`使用
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

bitflags! {
    /// 指定文件打开时的权限
    #[derive(Clone, Copy)]
    pub struct AccessMode: u16 {
        /// 用户读权限
        const S_IRUSR = 1 << 8;
        /// 用户写权限
        const S_IWUSR = 1 << 7;
        /// 用户执行权限
        const S_IXUSR = 1 << 6;
        /// 用户组读权限
        const S_IRGRP = 1 << 5;
        /// 用户组写权限
        const S_IWGRP = 1 << 4;
        /// 用户组执行权限
        const S_IXGRP = 1 << 3;
        /// 其他用户读权限
        const S_IROTH = 1 << 2;
        /// 其他用户写权限
        const S_IWOTH = 1 << 1;
        /// 其他用户执行权限
        const S_IXOTH = 1 << 0;
    }
}

impl From<usize> for OpenFlags {
    fn from(val: usize) -> Self {
        Self::from_bits_truncate(val as u32)
    }
}

/// IOCTL系统调用支持
#[allow(missing_docs)]
pub const TCGETS: usize = 0x5401;
#[allow(missing_docs)]
pub const TIOCGPGRP: usize = 0x540F;
#[allow(missing_docs)]
pub const TIOCSPGRP: usize = 0x5410;
#[allow(missing_docs)]
pub const TIOCGWINSZ: usize = 0x5413;
#[allow(missing_docs)]
pub const FIONBIO: usize = 0x5421;
#[allow(missing_docs)]
pub const FIOCLEX: usize = 0x5451;
#[repr(C)]
#[derive(Clone, Copy, Default)]
/// the size of the console window
pub struct ConsoleWinSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}
