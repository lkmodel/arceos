use alloc::{ffi::CString, vec::Vec};
use axlog::debug;
use axstd::string::{String, ToString};
use core::slice::from_raw_parts;

use crate::{config::HEADER_MAGIC, elf_load::decoder::Decoder};

/// 解码器，用于从二进制数据中解析出指令行列表
#[derive(Debug)]
pub struct HeadDecoder<'a> {
    decoder: Decoder<'a>,
}

impl<'a> HeadDecoder<'a> {
    fn new(data: &'a [u8]) -> Self {
        HeadDecoder {
            decoder: Decoder::new(data),
        }
    }

    /// 从头部解析魔术和行数
    fn parse_header(&mut self) -> Result<(u64, u32, u64), String> {
        let magic = self.decoder.read_u64()?;
        if magic != HEADER_MAGIC {
            return Err("Invalid magic number!".to_string());
        }
        let app_lines = self.decoder.read_u32()?;
        let header_size = self.decoder.read_u64()?;
        Ok((magic, app_lines, header_size))
    }

    /// 解析每个应用数据行
    fn parse_app_line(&mut self) -> Result<(u64, CString, u64), String> {
        let app_size = self.decoder.read_u64()?;
        let app_name = self.decoder.read_c_string()?;
        let app_offset = self.decoder.read_u64()?;

        Ok((app_size, app_name, app_offset))
    }

    /// 解析lib库数据行
    fn parse_lib_line(&mut self) -> Result<(u64, u64), String> {
        let lib_size = self.decoder.read_u64()?;
        let lib_offset = self.decoder.read_u64()?;

        Ok((lib_size, lib_offset))
    }

    /// 解析 script 数据行
    fn parse_script_line(&mut self) -> Result<(u64, u64), String> {
        let script_size = self.decoder.read_u64()?;
        let script_offset = self.decoder.read_u64()?;

        Ok((script_size, script_offset))
    }
}

/// 解码之后的头文件
#[derive(Debug)]
pub struct HeadDecoded {
    ///  应用数量
    #[allow(unused)]
    pub app_num: u32,
    /// 头结构的大小
    #[allow(unused)]
    pub head_size: u64,
    /// 存放多个应用的（应用大小，`C` 风格字符串，应用在 `PLASH` 中的偏移）
    pub apps: Vec<(u64, CString, u64)>,
    /// (`Lib` 库大小, 库在 `PLASH` 中的偏移)
    pub lib: (u64, u64),
    /// (脚本大小, 脚本在 `PLASH` 中的偏移)
    pub script: (u64, u64),
}

/// 头结构解码
/// # 参数
/// * `plash_start` - `PLASH` 的起始地址
/// * `plash_size` - `PLASH` 的大小
///
/// # 返回值
/// 解码后的头文件
pub fn head_decoded(plash_start: usize, plash_size: usize) -> HeadDecoded {
    debug!("Decode head...");
    let plash = unsafe { from_raw_parts(plash_start as *const u8, plash_size) };
    let mut decode = HeadDecoder::new(plash);

    let (_, app_num, head_size) = decode.parse_header().expect("Failed to parse header");
    let mut apps = Vec::new();
    for _ in 0..app_num {
        let app = decode.parse_app_line().expect("Failed to parse app line");
        apps.push(app);
    }
    let lib = decode.parse_lib_line().expect("Failed to parse lib line");
    let script = decode
        .parse_script_line()
        .expect("Failed to parse script line");

    debug!("Decode head done");
    HeadDecoded {
        app_num,
        head_size,
        apps,
        lib,
        script,
    }
}
