use std::{
    env, fs,
    sync::{Arc, RwLock},
};

use tokio::process::Command;

/// Stores the value for the RUST_LOG environment variables
#[derive(Debug, Default)]
struct EnvVars(Arc<RwLock<Vec<(String, String)>>>);

unsafe fn load_unsafe() {
    let f = fs::read_to_string(".env").unwrap();

    for lines in f.lines() {
        let (k, v) = lines.split_once('=').unwrap();
        unsafe {
            env::set_var(k, v);
        }
    }
}

impl EnvVars {
    /// Loads environment variables from a env file without modifying the program environment.
    fn load() -> Self {
        let f = fs::read_to_string(".env").unwrap();
        let vars = EnvVars::default();

        println!("Loaded {} variables from .env", f.lines().count());
        for line in f.lines() {
            let (k, v) = line.split_once('=').unwrap();
            println!("{k}: {v}");
            vars.insert(k.to_string(), v.to_string());
        }

        vars
    }

    fn insert(&self, k: String, v: String) {
        self.0.write().unwrap().push((k, v));
    }
}

fn main() {
    // set_var is marked unsafe in Rust 2024 edition (Oct 2024 release date)
    // it is not safe to use in multi-threaded apps
    // https://github.com/rust-lang/rust/issues/27970#issuecomment-264624978
    // we are calling it here before starting the multi-threaded runtime
    unsafe {
        env::set_var("TOKIO_WORKER_THREADS", "1");
        env::set_var("RUST_LOG", "info");

        //load_unsafe();
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

            // if load_py is required, we should use `load_unsafe` before starting the MT runtime
            // load_py().await;


            // for most cases, the safe version is sufficient
            let _ = EnvVars::load();
        });
}

const SCRIPT: &str = r#"
import os

print('\nHello, Python world')
print(f'RUST_LOG: {os.environ.get("RUST_LOG")}')
print(f'FOO: {os.environ.get("FOO")}')
"#;

/// Reads `RUST_LOG` from the program environment and prints it
/// 
/// This should be used with `load_unsafe`
async fn get_py() {
    let output = Command::new("python3")
        .arg("-c")
        .arg(SCRIPT)
        .output()
        .await
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
