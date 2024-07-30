# setvar-mt-example

`set_var` will be unsafe in the Rust 2024 edition.

This example contains two implementations of reading env vars, one that modifies the program environment and one that does not.

## Usage

Typical use:

```rs
#[tokio::main]
async fn main() {
	// the actual program environment is not modified
	let env_vars = EnvVars::load();

	// retrieves from our env var store
	env_vars.get("FOO");
}
```

If you need to actually modify the program's environment, e.g., to allow access from non-Rust code:

```rs
fn main() {
    unsafe {
        load_unsafe();
    }

	tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
			std::env::var("FOO");
            load_py().await;  // access the env var from Python
        });
}
```

If you need to modify environment variables before Tokio runtime startup:
```rs
fn main() {
    unsafe {
        std::env::set_var("TOKIO_WORKER_THREADS", "1");
        std::env::set_var("RUST_LOG", "info");
    }

	tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
			// do things
        });
}
```