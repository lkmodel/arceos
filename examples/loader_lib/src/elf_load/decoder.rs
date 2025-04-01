use alloc::ffi::CString;
use axstd::string::{String, ToString};

/// 通用解码器模板
#[derive(Debug)]
pub struct Decoder<'a> {
    data: &'a [u8],  // 输入的二进制脚本数据
    position: usize, // 当前读取偏移
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Decoder { data, position: 0 }
    }

    /// 读取一个 `u64`
    pub fn read_u64(&mut self) -> Result<u64, String> {
        self.read_bytes(8)
            .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
    }

    /// 读取一个 `u32`
    pub fn read_u32(&mut self) -> Result<u32, String> {
        self.read_bytes(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    }

    /// 读取一个 `u8`
    pub fn read_u8(&mut self) -> Result<u8, String> {
        self.read_bytes(1).map(|b| b[0])
    }

    /// 从当前偏移读取指定长度的字节，并更新偏移
    pub fn read_bytes(&mut self, length: usize) -> Result<&[u8], String> {
        if self.position + length > self.data.len() {
            return Err("Unexpected end of data!".to_string());
        }
        let bytes = &self.data[self.position..self.position + length];
        self.position += length;
        Ok(bytes)
    }

    /// 读取 C 风格的字符串（以 `\0` 结尾）
    pub fn read_c_string(&mut self) -> Result<CString, String> {
        let start = self.position;
        while self.position < self.data.len() && self.data[self.position] != 0 {
            self.position += 1;
        }
        if self.position >= self.data.len() {
            return Err("Unterminated C string!".to_string());
        }
        let cstr = CString::new(&self.data[start..self.position])
            .map_err(|_| "CString error".to_string())?;
        self.position += 1; // 跳过 `\0`
        Ok(cstr)
    }
}
