use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

pub fn abi_function_register(args: TokenStream2, fn_token: TokenStream2) -> TokenStream2 {
    let input_fn = syn::parse2::<syn::ItemFn>(fn_token).unwrap();

    let name = args.to_string();

    let fn_ident = &input_fn.sig.ident;
    let up_ident = Ident::new(
        &input_fn.sig.ident.to_string().to_ascii_uppercase(),
        Span::call_site(),
    );
    // let new_ident=Ident::new(&format!("_hidden_{}", fn_ident), fn_ident.span());
    // input_fn.sig.ident = new_ident.clone();

    quote! (
        #[::linkme::distributed_slice(ABI_TABLE)]
        #[linkme(crate = ::linkme)]
        static #up_ident : AbiEntry = AbiEntry {
            name: #name,
            addr: #fn_ident as *const (),
        };

        #input_fn
    )
}
