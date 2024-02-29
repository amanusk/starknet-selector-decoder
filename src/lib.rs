include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Returns the selector as a string if it is part of the known selector list,
/// or None if it is not
/// # Example
/// ```rust
///use starknet_selector_decoder::get_selector;
///
///let result =
///    get_selector("0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e");
///match result {
///    Some(selector) => assert_eq!(selector, "transfer"),
///    None => assert!(false),
///}
///```
pub fn get_selector(keyword: &str) -> Option<&str> {
    SELECTORS.get(keyword).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let result =
            get_selector("0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e");
        match result {
            Some(selector) => assert_eq!(selector, "transfer"),
            None => assert!(false),
        }
    }

    #[test]
    fn test_approve() {
        let result =
            get_selector("0x0219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c");
        if let Some(selector) = result {
            println!("{}", selector);
        }
        match result {
            Some(selector) => assert_eq!(selector, "approve"),
            None => assert!(false),
        }
    }

    #[test]
    fn test_selector_not_present() {
        let result =
            get_selector("0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12f");
        match result {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
