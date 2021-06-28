# asynv (async-env)

`async-env` was already taken on crates so I decided to come up with another troupe. Anyways an asynchronous wrapper around Rust's [`std::env`](https://doc.rust-lang.org/std/env/).

## Usage

```rs
extern crate asynv;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
	asynv::set("SOME_VARIABLE", "SOME_VALUE").await;

	let some_var = asynv::get("SOME_VARIABLE").await;

	println!("from environment: {}", some_var); // from environment: SOME_VALUE
	
	Ok(())
}
```

## Contribute

If you'll like to see anything, or you have a problem with anything please shoot an [PR]() or [Issue]()