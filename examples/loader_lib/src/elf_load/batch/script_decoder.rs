use core::{
    alloc::Layout,
    ptr::{null_mut, write_volatile},
    slice::from_raw_parts,
};

use alloc::{alloc::alloc_zeroed, ffi::CString, vec::Vec};
use axlog::info;
use axstd::string::{String, ToString};

use crate::elf_load::decoder::Decoder;

const MAGIC: u64 = 0x5F7470697263735F; // 魔数 `_script_`
const HEADER_MARKER: u8 = 0xFF; // 每行开头的验证字节

/// 解码器，用于从二进制数据中解析出指令行列表
pub struct ScriptDecoder<'a> {
    decoder: Decoder<'a>,
}

impl<'a> ScriptDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ScriptDecoder {
            decoder: Decoder::new(data),
        }
    }

    /// 从头部解析魔术和行数
    fn parse_header(&mut self) -> Result<(u64, u32), String> {
        let magic = self.decoder.read_u64()?;
        if magic != MAGIC {
            return Err("Invalid magic number!".to_string());
        }
        let num_lines = self.decoder.read_u32()?;
        Ok((magic, num_lines))
    }

    /// 解析每个指令行，返回 C 风格的 argv 数组
    fn parse_command_line(&mut self) -> Result<Vec<CString>, String> {
        if self.decoder.read_u8()? != HEADER_MARKER {
            return Err("Invalid line header marker!".to_string());
        }
        // FIX: 在这里我们假设是每一行都有，但是如果没有呢？
        // 如果只有一个参数呢？
        let argc = self.decoder.read_u32()?;
        let mut argv = Vec::new();

        for _ in 0..argc {
            let arg = self.decoder.read_c_string()?;
            argv.push(arg);
        }
        Ok(argv)
    }
}

/// 参数设置器：将解码得到的 `argv` 列表组织为 `argc` 和 `argv` C 格式布局
pub struct ParameterSetup;

impl ParameterSetup {
    /// 为一个指令行设置 C 兼容的 `argc` 和 `argv` 格式，确保在内存中连续分配
    ///
    /// # Safety
    /// - 返回一个指向 `argc` 的指针，并确保 `argc` 和 `argv` 在内存中是连续分配的。
    pub unsafe fn setup_args_contiguous(argv: &Vec<CString>) -> *mut u64 {
        // 计算所需的内存大小
        let argc = argv.len();
        let total_size = (1 + argc + 1) * core::mem::size_of::<u64>(); // argc + argv指针 + NULL终止符
        let memory = unsafe {
            alloc_zeroed(Layout::from_size_align(total_size, core::mem::align_of::<u64>()).unwrap())
                as *mut u64
        };

        // 设置 argc
        unsafe { write_volatile(memory, argc as u64) };

        // 设置 argv
        let argv_base = unsafe { memory.add(1) as *mut *mut u8 };
        for (i, arg) in argv.iter().map(|c| c.as_ptr() as *mut u8).enumerate() {
            unsafe { write_volatile(argv_base.add(i), arg) };
        }
        // 设置 NULL 终止符
        unsafe { write_volatile(argv_base.add(argc), null_mut()) };

        memory
    }
}

/// 通过输入的地址，解码执行脚本
///
/// # 参数
/// * `script_slice` - 执行脚本切片
///
/// # 返回值
/// 0: `Vec<CString>` 的直接命令名字，方便查找
/// 1: 在 `mem` 中，存放 `argc` 地址的 `Vec<*mut u64>`，每个地址存放一组连续的 `argc` 和 `argv` 数据
pub fn decode_script(script_slice: &[u8]) -> (u32, Vec<(CString, *mut u64)>) {
    // 解码二进制脚本文件
    let mut decoder = ScriptDecoder::new(script_slice);
    let mut line_num = 0;
    let mut result = Vec::new();

    if let Ok((_magic, num_lines)) = decoder.parse_header() {
        info!("Magic verified, number of command lines: {}", num_lines);

        // 解码每一行指令行
        for i in 0..num_lines {
            if let Ok(argv) = decoder.parse_command_line() {
                info!("Line {} parsed with args: {:?}", i, argv);

                // 调用连续内存分配函数
                unsafe {
                    // NOTE: 在这里仅仅借用所有权，而不是进行所有权转移
                    let argc_ptr = ParameterSetup::setup_args_contiguous(&argv);
                    result.push((argv[0].clone(), argc_ptr));
                }
            }
        }

        line_num = num_lines;
    }

    line_num.eq(&0).then(|| panic!("Line num is zero"));
    result.is_empty().then(|| panic!("Script line is empty"));

    (line_num, result)
}

/// 解码之后的脚本数据
pub struct ScriptDecoded {
    /// 用于指定脚本行的行数
    pub line_num: u32,
    /// 0: `Vec<CString>` 的直接命令名字，方便查找
    /// 1: 在 `mem` 中，存放 `argc` 地址的 `Vec<*mut u64>`，每个地址存放一组连续的 `argc` 和 `argv` 数据
    pub lines_meta: Vec<(CString, *mut u64)>,
}

/// 解码执行脚本
///
/// # 参数
/// * `script_start` - 执行脚本在 `PLASH` 中的地址
/// * `script_size` - 执行脚本在 `PLASH` 中的大小
///
/// # 返回
/// 返回解码后的脚本数据
pub fn script_decoded(script_start: usize, script_size: usize) -> ScriptDecoded {
    let script_slice = unsafe { from_raw_parts(script_start as *const u8, script_size) };
    let (line_num, lines_meta) = decode_script(script_slice);

    ScriptDecoded {
        line_num,
        lines_meta,
    }
}
