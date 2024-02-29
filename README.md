# Starknet Selector Decoder

## Description
Simple tool to get the string value of a starknet selector hash.
From readable string to hash is easy, just compute the hash, but if you only have the hash and want the string, it needs to be looked up.

The map is comrised of all selectors declared on Starknet mainnet and gets updated.

## Example
 ```rust
use starknet_selector_decoder::get_selector;

let result =
    get_selector("0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e");
match result {
    Some(selector) => assert_eq!(selector, "transfer"),
    None => assert!(false),
}
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
