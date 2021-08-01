#[cfg(not( any(feature = "default", feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At least one feature flag needs to be enabled");

#[cfg(all( feature = "default", any(feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At most one feature flag can to be enabled");

#[cfg(all(feature = "async_tokio", feature = "permutation_testing"))]
compile_error!("feature async cannot be used with feature permutation_testing");

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(item as ItemFn);

    let vis = input.vis;
    let sig = input.sig;
    let block = &*input.block;

    let expanded = {
        #[cfg(feature = "default")] {
            quote! {
                #[test]
                # vis # sig # block
            }
        }
        #[cfg(feature = "async_tokio")] {
            quote! {
                #[concurrency_toolkit::tokio::test]
                # vis async # sig # block
            }
        }
        #[cfg(feature = "permutation_testing")] {
            quote! {
                #[test]
                # vis # sig {
                    concurrency_toolkit::loom::model(
                        || # block
                    );
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
