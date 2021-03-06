use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

/// Automatically start async runtime or call `loom::model` if required:
///
/// ```ignore
/// #[concurrency_toolkit::test]
/// fn test() {
///     // ...
/// }
/// ```
///
/// However, unlike `maybe_async::maybe_async`, this proc macro requires the function
/// to not be declared as `async` due to implementation detail
/// (`syn` doesn't provides an easy way to parse `async function), but it still can
/// remove `async`-related keywords just like `maybe_async::maybe_async`.
///
/// **NOTE that this macro requires `concurrency_toolkit` to be imported**
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(item as ItemFn);

    let vis = input.vis;
    let sig = input.sig;
    let block = &input.block;

    #[cfg(feature = "permutation_testing")]
    let block = quote! {
        {
            concurrency_toolkit::loom::model(
                || # block
            );
        }
    };

    #[cfg(not(feature = "async_tokio"))]
    let expanded = quote! {
        #[test]
        #[concurrency_toolkit::maybe_async::must_be_sync]
        # vis # sig # block
    };

    #[cfg(feature = "async_tokio")]
    let expanded = quote! {
        #[concurrency_toolkit::tokio::test]
        #[concurrency_toolkit::maybe_async::must_be_async]
        # vis async # sig # block
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
