use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(item as ItemFn);

    let vis = input.vis;
    let sig = input.sig;
    let mut block = input.block.to_token_stream();

    #[cfg(feature = "permutation_testing")]
    {
        block = quote! {
            {
                concurrency_toolkit::loom::model(
                    || # block
                );
            }
        };
    }

    let expanded = quote! {
        #[concurrency_toolkit::maybe_async::test(
            feature = "is_sync",
            async(feature = "async_tokio", concurrency_toolkit::tokio::test),
        )]
        # vis async # sig # block
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
