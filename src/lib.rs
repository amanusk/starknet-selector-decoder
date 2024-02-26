include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn get_selector(keyword: &str) -> Option<&str> {
    SELECTORS.get(keyword).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectors() {
        let result =
            get_selector("0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e");
        match result {
            Some(selector) => assert_eq!(selector, "transfer"),
            None => assert!(false),
        }
    }
}
