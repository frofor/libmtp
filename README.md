# libmtp

A memory safe API for interacting with libmtp library in Rust.

[![crates.io](https://img.shields.io/crates/v/libmtp)](https://crates.io/crates/libmtp)

## Install

To install the latest version of `libmtp` from [crates.io](https://crates.io/crates/libmtp), run:

```sh
$ cargo add libmtp
```

## Get Started

To get started, create a new program that prints all objects in the root folder of your storage:

```rust
use libmtp::dev::discover;

fn main() -> libmtp::Result<()> {
	for device in discover()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			for object in &storage {
				println!("{object:?}");
			}
		}
	}
	Ok(())
}
```

For more examples, see [examples](https://codeberg.org/frofor/libmtp/src/branch/stable/examples).

## Changelog

For a release history, see [CHANGELOG.md](https://codeberg.org/frofor/libmtp/src/branch/stable/doc/CHANGELOG.md).

## Contributing

For a contibuting guide, see [CONTRIBUTING.md](https://codeberg.org/frofor/libmtp/src/branch/stable/doc/CONTRIBUTING.md).

## License

This crate is distributed under the terms of MIT License.

See [LICENSE](https://codeberg.org/frofor/libmtp/src/branch/stable/LICENSE) for details.
