use convert_case::{Case, Casing};

pub fn get_discrete_words<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .replace(|c| c == '.' || c == ',' || c == '/' || c == ':', " ")
        .to_case(Case::Lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphenated_words() {
        let input = "these-words-are-hyphenated";
        assert_eq!(get_discrete_words(input), "these words are hyphenated");
    }

    #[test]
    fn underscored_words() {
        let input = "__array_structures";
        assert_eq!(get_discrete_words(input), "array structures");
    }

    #[test]
    fn camel_words() {
        let input = "WKWebVIEWComponent";
        assert_eq!(get_discrete_words(input), "wk web view component");
    }

    #[test]
    fn dotted_words() {
        let input = "page.Find";
        assert_eq!(get_discrete_words(input), "page find");
    }

    #[test]
    fn misc_punctuation() {
        let input = "cloud/cannon,page.find";
        assert_eq!(get_discrete_words(input), "cloud cannon page find");
    }
}
