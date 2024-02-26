#[derive(Clone)]
pub enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse_keyword("loop");
        if let Some(Keyword::Loop) = result {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
