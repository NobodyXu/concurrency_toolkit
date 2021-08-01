extern crate proc_macro_test;
extern crate concurrency_toolkit;

#[proc_macro_test::test]
fn f() {
    println!("Hello, world!");
}
