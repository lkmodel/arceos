use abi_macro_core::abi_function_register;
use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn abi_macro_basic() {
    let input = quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn abi_printf(fat: *const c_char, mut args: ...) -> c_int {
            info!("[ABI:Print] Print a formatted string!");
            // 空指针检查
            if fat.is_null() {
                error!("[ABI:Print] Print NULL!");
                return -1;
            }

            let fat = ((fat as usize)) as *const c_char;

            info!("fat: {:p}", fat);

            let mut s = String::new();
            let bytes_written = unsafe { format(fat, args.as_va_list(), output::fmt_write(&mut s)) };
            print!("{}", s);
            bytes_written as c_int
        }
    };
    let result = abi_function_register(quote!(printf), input).to_string();
    let result = prettyplease::unparse(&syn::parse_str(&result).unwrap());

    insta::assert_snapshot!(result);
}
