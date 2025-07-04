use crate::types::error::*;
use crate::types::re::*;

pub fn get_regex(pattern: &str) -> Result<RegEx, Error> {
    RegEx::from_pattern(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_regex() {
        let re = RegEx::from_pattern("aabbaa|b").unwrap();

        assert_eq!(re.search("aabbab"), Some(0));
        assert_eq!(re.search("ac"), None);
    }

    #[test]
    fn exact_match_only() {
        let re = RegEx::from_pattern("abc").unwrap();

        assert_eq!(re.search("abc"), Some(0));
        assert_eq!(re.search("xabcx"), Some(1));
        assert_eq!(re.search("ab"), None);
        assert_eq!(re.search("xyz"), None);
    }

    #[test]
    fn class() {
        let re = RegEx::from_pattern("a[ab]").unwrap();

        assert_eq!(re.search("aa"), Some(0));
        assert_eq!(re.search("ab"), Some(0));
    }

    #[test]
    fn class_2() {
        let re = RegEx::from_pattern("a[bc]a").unwrap();
        println!("{:#?}", re);
        assert_eq!(re.search("aba"), Some(0));
        assert_eq!(re.search("ab"), None);
        assert_eq!(re.search("ba"), None);
    }

    #[test]
    fn empty() {
        let re = RegEx::from_pattern("a[bc]").unwrap();

        assert_eq!(re.search(""), None);
    }
}
