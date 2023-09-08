use convert_case::{Case, Casing};
use emojis;
use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref EMOJI: Regex = Regex::new("\\p{Emoji}").unwrap();
}

pub fn get_discrete_words<S: AsRef<str>>(s: S) -> (String, Option<Vec<String>>) {
    let mut extras = None;

    let words = s
        .as_ref()
        .replace(|c: char| c.is_ascii_punctuation(), " ")
        .to_case(Case::Lower);

    if EMOJI.is_match(s.as_ref()) {
        extras = Some(
            s.as_ref()
                .graphemes(true)
                .into_iter()
                .filter_map(|x| {
                    if emojis::get(x).is_some() {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );
    }

    (words, extras)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphenated_words() {
        let input = "these-words-are-hyphenated";
        assert_eq!(
            get_discrete_words(input),
            ("these words are hyphenated".into(), None)
        );
    }

    #[test]
    fn underscored_words() {
        let input = "__array_structures";
        assert_eq!(get_discrete_words(input), ("array structures".into(), None));
    }

    #[test]
    fn camel_words() {
        let input = "WKWebVIEWComponent";
        assert_eq!(
            get_discrete_words(input),
            ("wk web view component".into(), None)
        );
    }

    #[test]
    fn dotted_words() {
        let input = "page.Find";
        assert_eq!(get_discrete_words(input), ("page find".into(), None));
    }

    #[test]
    fn misc_punctuation() {
        let input = "cloud/cannon,page.find";
        assert_eq!(
            get_discrete_words(input),
            ("cloud cannon page find".into(), None)
        );
    }

    #[test]
    fn french() {
        let input = "l'alphabet";
        assert_eq!(get_discrete_words(input), ("l alphabet".into(), None));
    }

    #[test]
    fn html() {
        let input = "<FormComponent data-pagefind-meta='[key:(value)]'>";
        assert_eq!(
            get_discrete_words(input),
            ("form component data pagefind meta key value".into(), None)
        );
    }

    #[test]
    fn emoji() {
        let input = "cloud🌦️cannon";
        assert_eq!(
            get_discrete_words(input),
            ("cloud🌦️cannon".into(), Some(vec!["🌦️".into()]))
        );

        let input = "👋👨‍👩‍👧‍👦🌾";
        assert_eq!(
            get_discrete_words(input),
            (
                "👋👨‍👩‍👧‍👦🌾".into(),
                Some(vec!["👋".into(), "👨‍👩‍👧‍👦".into(), "🌾".into()])
            )
        );
    }
}
