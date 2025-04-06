// use axstd::string::String;
//
// use crate::elf_load::decoder::Decoder;
//
// /// 解码器，用于从二进制数据中解析出指令行列表
// pub struct HeadDecoder<'a> {
//     decoder: Decoder<'a>,
// }
// const MAGIC_NUMBER: u64 = 0x5F7265646165685F; // 魔数 `_header_`
//
// impl<'a> HeadDecoder<'a> {
//     pub fn new(data: &'a [u8]) -> Self {
//         HeadDecoder {
//             decoder: Decoder::new(data),
//         }
//     }
//
//     pub fn get_app_size(&mut self) -> Result<usize, String> {
//         let app_elf_size = self.decoder.read_u64()? as usize;
//
//         Ok(app_elf_size)
//     }
// }
