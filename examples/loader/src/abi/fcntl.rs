// #[abi(open)]
// #[unsafe(no_mangle)]
// extern "C" fn abi_open(filename: *const c_char, flags: c_int, mut mode: VaList) -> c_int {
//     if filename.is_null() {
//         return -1;
//     }
// }