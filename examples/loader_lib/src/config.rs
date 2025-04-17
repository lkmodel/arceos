/// Stack size of each task.
pub const TASK_STACK_SIZE: usize = 0x40000;
/// `bin`的开始位置
pub const PLASH_START: usize = 0xffff_ffc0_2200_0000;
// `PLASH` 大小, `32M`
pub const PLASH_SIZE: usize = 32 * 1024 * 1024;

/// 最大的APP_SIZE，用于检查
pub const MAX_APP_SIZE: usize = 0x20_0000;
/// APP 的加载地址
pub const APP_START: usize = 0xffff_ffc0_8700_0000;
/// 最大的LIB_SIZE，用于检查
pub const MAX_LIB_SIZE: usize = 0x10_0000;
/// LIB 的加载地址
pub const LIB_START: usize = 0xffff_ffc0_8010_0000;
/// 用于存储全局信息
pub const GLOBAL_SOTRE: usize = 0xffff_ffc0_801f_fff0;
/// 支持目录最长长度
pub const FILE_NAME_LENGTH: usize = 255usize;
// 0x802c9000
/// 可用内存区域(Unikernel)
pub const PHYS_MEMORY_BASE: usize = 0xffff_ffc0_8000_0000; // 0x8000_0000;
pub const PHYS_MEMORY_SIZE: usize = 0x800_0000; // 0x800_0000;

/// 最大堆体积
pub const MAX_HEAP_SIZE: usize = 0x20000;

// 魔数 `_header_`
pub const HEADER_MAGIC: u64 = 0x5F7265646165685F;
// 魔数 `_script_`
pub const SCRIPT_MAGIC: u64 = 0x5F7470697263735F;
