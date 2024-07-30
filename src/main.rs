use std::env;

fn main() {
    // set_var is marked unsafe in Rust 2024 edition (Oct 2024 release date)
    // it is not safe to use in multi-threaded apps
    // https://github.com/rust-lang/rust/issues/27970#issuecomment-264624978
    // we are calling it here before starting the multi-threaded runtime
    unsafe {
        env::set_var("RUST_LOG", "info");
    }

    // this is equivalent to the tokio::main attribute macro, used in the majority of multi-threaded applications in Rust
    // https://docs.rs/tokio/latest/tokio/attr.main.html
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello, multi-threaded world");
            println!("RUST_LOG: {}", env::var("RUST_LOG").unwrap());
        });
}
