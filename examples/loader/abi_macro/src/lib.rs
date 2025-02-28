use abi_macro_core::*;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn abi(args:TokenStream,fn_tokens:TokenStream)->TokenStream{
    abi_function_register(args.into(), fn_tokens.into()).into()
}