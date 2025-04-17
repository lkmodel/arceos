use core::{
    alloc::Layout,
    ptr::{null_mut, write_volatile},
    slice::from_raw_parts,
};

use alloc::{
    alloc::{alloc_zeroed, dealloc},
    ffi::CString,
    vec::Vec,
};
use axlog::{debug, info};
use axstd::string::{String, ToString};

use crate::{config::SCRIPT_MAGIC, elf_load::decoder::Decoder};

/// 解码器，用于从二进制数据中解析出指令行列表
#[derive(Debug)]
pub struct ScriptDecoder<'a> {
    decoder: Decoder<'a>,
}

impl<'a> ScriptDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ScriptDecoder {
            decoder: Decoder::new(data),
        }
    }

    /// 从头部解析魔术
    fn parse_header(&mut self) -> Result<u64, String> {
        let magic = self.decoder.read_u64()?;
        if magic != SCRIPT_MAGIC {
            return Err("Invalid magic number!".to_string());
        }
        Ok(magic)
    }

    /// 解析每个指令行，返回 C 风格的 argv 数组
    fn parse_command_line(&mut self) -> Result<Vec<CString>, String> {
        let argc = self.decoder.read_u32()?;
        // `argv` 在栈上创建，但是 `Vec` 在堆上
        // 此时 argv 的所有权属于 parse_command_line 这个函数。
        let mut argv = Vec::new();

        for _ in 0..argc {
            let arg = self.decoder.read_c_string()?; // 返回 CString，所有权在 arg 上
            argv.push(arg); // arg 移动到 argv，arg 失效
        }
        Ok(argv) // argv 移动到 Result，argv 失效
        // Result<Vec<CString>, String> -> 返回给调用者
        // 最终调用者获取 Result<Vec<CString>>，因此也拥有了 Vec<CString> 的所有权，进而拥有所有 CString 的所有权。
    }
}

#[derive(Debug, Clone)]
/// 参数设置器：将解码得到的 `argv` 列表组织为 `argc` 和 `argv` C 格式布局
pub struct ArgvStorage {
    memory: *mut u64,   // 指向连续内存
    args: Vec<CString>, // 保存原始的 CString，确保生命周期
}

impl ArgvStorage {
    /// 为一个指令行设置 C 兼容的 `argc` 和 `argv` 格式，确保在内存中连续分配
    ///
    /// # Safety
    /// - 返回一个指向 `argc` 的指针，并确保 `argc` 和 `argv` 在内存中是连续分配的。
    pub unsafe fn new(argv: Vec<CString>) -> Self {
        info!("Argv {:?}", argv);
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

        Self { memory, args: argv }
    }

    pub fn argc_ptr(&self) -> *mut u64 {
        self.memory
    }
}

impl Drop for ArgvStorage {
    fn drop(&mut self) {
        unsafe { dealloc(self.memory as *mut u8, Layout::new::<u64>()) };
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
fn decode_script(script_slice: &[u8]) -> ArgvStorage {
    debug!("decode by script_slice");
    // 解码二进制脚本文件
    debug!("script_slice_addr {:?}", script_slice.as_ptr());
    let mut decoder = ScriptDecoder::new(script_slice);

    let _ = decoder
        .parse_header()
        .expect("Failed to check script magic");
    let argv = decoder
        .parse_command_line()
        .expect("Failed to load command line");
    info!("Parsed with args: {:?}", argv);
    let storage = unsafe { ArgvStorage::new(argv) };
    storage
}

/// 解码执行脚本
///
/// # 参数
/// * `script_start` - 执行脚本在 `PLASH` 中的地址
/// * `script_size` - 执行脚本在 `PLASH` 中的大小
///
/// # 返回
/// 返回解码后的脚本数据
pub fn script_decoded(script_start: usize, script_size: usize) -> ArgvStorage {
    debug!("Decode script...");
    info!(
        "script_start: 0x{:x}, script_size: 0x{:x}",
        script_start, script_size
    );
    // NOTE: 这里应该加上PLASH的偏移，否则就是在低地址区取值了
    let script_slice = unsafe { from_raw_parts(script_start as *const u8, script_size) };
    let line_meta = decode_script(script_slice);

    debug!("Decode script done");
    line_meta
}
