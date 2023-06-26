# CowBytes

This is a crate to solve the issue that `serde_bytes` borrows the deserializer when deserializing `Cow<u8>`: <https://github.com/serde-rs/bytes/issues/40>.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
