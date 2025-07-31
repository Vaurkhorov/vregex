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

    #[test]
    fn any_char() {
        let re = RegEx::from_pattern("a.b").unwrap();

        assert_eq!(re.search("axb"), Some(0));
        assert_eq!(re.search("ayb"), Some(0));
        assert_eq!(re.search("a.b"), Some(0));
        assert_eq!(re.search("aaxb"), Some(1));
        assert_eq!(re.search("ab"), None);
    }

    #[test]
    fn any_char_2() {
        let re = RegEx::from_pattern(".").unwrap();

        assert_eq!(re.search("a"), Some(0));
        assert_eq!(re.search(" "), Some(0));
        assert_eq!(re.search("aa"), Some(0));
        assert_eq!(re.search(""), None);
    }

    #[test]
    fn digit() {
        let re = RegEx::from_pattern(r"\d").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("11"), Some(0));
        assert_eq!(re.search("a"), None);
        assert_eq!(re.search("123abc7123abc"), Some(0));
        assert_eq!(re.search("++123abc7123abc++"), Some(2));
    }

    #[test]
    fn digit_2() {
        let re = RegEx::from_pattern(r"123abc\d123abc").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("123abc7123abc"), Some(0));
        assert_eq!(re.search("++123abc7123abc++"), Some(2));
        assert_eq!(re.search("++123abc+123abc++"), None);
    }

    #[test]
    fn digit_exclusive() {
        let re = RegEx::from_pattern(r"\D").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("11"), None);
        assert_eq!(re.search("a"), Some(0));
        assert_eq!(re.search("123abc7123abc"), Some(3));
        assert_eq!(re.search("++123abc7123abc++"), Some(0));
    }

    #[test]
    fn digit_exclusive_2() {
        let re = RegEx::from_pattern(r"123abc\D123abc").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("123abc7123abc"), None);
        assert_eq!(re.search("123abcX123abc"), Some(0));
        assert_eq!(re.search("++123abcX123abc++"), Some(2));
        assert_eq!(re.search("++123abc7123abc++"), None);
    }

    #[test]
    fn digit_alternate() {
        let re = RegEx::from_pattern(r"a|\d").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("a"), Some(0));
        assert_eq!(re.search("1"), Some(0));
        assert_eq!(re.search("x73ax"), Some(1));

        let re = RegEx::from_pattern(r"\d|a").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("a"), Some(0));
        assert_eq!(re.search("1"), Some(0));
        assert_eq!(re.search("x73ax"), Some(1));
    }

    #[test]
    fn misc_classes() {
        let re = RegEx::from_pattern(r"+\s+\l+\u+\S+\L+\U+").unwrap();

        assert_eq!(re.search(""), None);
        assert_eq!(re.search("+ + + + + + +"), None);
        assert_eq!(re.search("+ +v+V+x+V+v+"), Some(0));
    }
}
