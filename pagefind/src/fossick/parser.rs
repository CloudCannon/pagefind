use lazy_static::lazy_static;
use lol_html::{element, text, HtmlRewriter, Settings};
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
    static ref NEWLINES: Regex = Regex::new("(\n|\r\n)+").unwrap();
    static ref TRIM_NEWLINES: Regex = Regex::new("^[\n\r\\s]+|[\n\r\\s]+$").unwrap();
    static ref EXTRANEOUS_SPACES: Regex = Regex::new("\\s{2,}").unwrap();
    static ref SENTENCE_CHARS: Regex = Regex::new("[\\w'\"\\)\\$\\*]").unwrap();
}
lazy_static! {
    static ref SENTENCE_SELECTORS: Vec<&'static str> =
        vec!("p", "td", "div", "ul", "li", "article", "section");
    static ref REMOVE_SELECTORS: Vec<&'static str> =
        vec!("script", "noscript", "label", "form", "svg", "footer", "header", "nav", "iframe");
}

struct EmptySink;
impl lol_html::OutputSink for EmptySink {
    fn handle_chunk(&mut self, _: &[u8]) {}
}

pub struct DomParser<'a> {
    rewriter: HtmlRewriter<'a, EmptySink>,
    data: Rc<RefCell<DomParserData>>,
}

// TODO: Store digest as a tree so that we can drop nodes correctly
//       i.e. when we reach the end of a <form>, we can drop everything within.
#[derive(Default, Debug)]
struct DomParserData {
    current_node: Rc<RefCell<DomParsingNode>>,
    title: Option<String>,
}

#[derive(Default, Debug)]
struct DomParsingNode {
    current_value: String,
    parent: Option<Rc<RefCell<DomParsingNode>>>,
    ignore: bool,
}

pub struct DomParserResult {
    pub digest: String,
    pub title: String,
}

// From https://github.com/rust-lang/rfcs/issues/2407#issuecomment-385291238
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

impl<'a> DomParser<'a> {
    pub fn new() -> Self {
        let data = Rc::new(RefCell::new(DomParserData::default()));
        let empty = EmptySink {};

        let rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    enclose! { (data) element!("body *", move |el| {
                        let data = Rc::clone(&data);


                        let mut node = DomParsingNode{
                            parent: Some(Rc::clone(&data.borrow().current_node)),
                            ..DomParsingNode::default()
                        };
                        if el.has_attribute("data-pagefind-ignore") || REMOVE_SELECTORS.contains(&el.tag_name().as_str())  {
                            node.ignore = true;
                        }
                        let node = Rc::new(RefCell::new(node));
                        {
                            let mut data = data.borrow_mut();
                            data.current_node = Rc::clone(&node);
                        }

                        let tail_data = Rc::clone(&data);
                        let tail_node = Rc::clone(&node);

                        let can_have_content = el.on_end_tag(move |end| {
                            let mut data = data.borrow_mut();
                            let mut node = node.borrow_mut();

                            if let Some(parent) = &node.parent {
                                data.current_node = Rc::clone(parent);
                            }

                            if node.ignore {
                                return Ok(());
                            }

                            let tag_name = end.name();
                            if SENTENCE_SELECTORS.contains(&tag_name.as_str()) {
                                let mut padded = " ".to_owned();
                                padded.push_str(&node.current_value);
                                node.current_value = padded;

                                if node.current_value.chars()
                                    .last()
                                    .filter(|c| SENTENCE_CHARS.is_match(&c.to_string()))
                                    .is_some() {
                                        node.current_value.push_str(". ");
                                }
                            }

                            let mut parent = data.current_node.borrow_mut();
                            parent.current_value.push_str(&node.current_value);
                            Ok(())
                        });

                        if can_have_content.is_err() {
                            let mut data = tail_data.borrow_mut();
                            let node = tail_node.borrow();
                            if let Some(parent) = &node.parent {
                                data.current_node = Rc::clone(parent);
                            }
                        }
                        Ok(())
                    })},
                    // Slap any text we encounter inside the body into the current node's current value
                    enclose! { (data) text!("body", move |el| {
                        let data = data.borrow_mut();
                        let mut node = data.current_node.borrow_mut();
                        node.current_value.push_str(el.as_str());
                        Ok(())
                    })},
                    // Track the first h1 on the page as the title to return in search
                    // TODO: This doesn't handle a chunk boundary,
                    // we can instead handle this by marking the node as a title and handling it in end_node
                    enclose! { (data) text!("h1", move |el| {
                        let mut data = data.borrow_mut();
                        let text = normalize_content(el.as_str());
                        if data.title.is_none() && !text.is_empty() {
                            data.title = Some(text);
                        }
                        Ok(())
                    })},
                ],
                ..Settings::default()
            },
            empty,
        );

        Self { rewriter, data }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), lol_html::errors::RewritingError> {
        self.rewriter.write(data)
    }

    pub fn wrap(self) -> DomParserResult {
        drop(self.rewriter); // Clears the extra Rcs on and within data
        let data = Rc::try_unwrap(self.data).unwrap().into_inner();
        let node = data.current_node.borrow();
        DomParserResult {
            digest: normalize_content(&node.current_value),
            title: data.title.unwrap_or_default(),
        }
    }
}

fn normalize_content(content: &str) -> String {
    let content = TRIM_NEWLINES.replace_all(content, "");
    let content = NEWLINES.replace_all(&content, " ");
    let content = EXTRANEOUS_SPACES.replace_all(&content, " ");

    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_content() {
        let input = "\nHello  Wor\n ld? \n \n";
        let output = normalize_content(input);

        assert_eq!(&output, "Hello Wor ld?");
    }

    fn test_parse(input: Vec<&'static str>) -> DomParserResult {
        let mut rewriter = DomParser::new();
        let _ = rewriter.write(b"<body>");
        for line in input {
            let _ = rewriter.write(line.as_bytes());
        }
        let _ = rewriter.write(b"</body>");
        rewriter.wrap()
    }

    #[test]
    fn block_tag_formatting() {
        let data = test_parse(vec![
            "<p>Sentences should have periods</p>",
            "<p>Unless one exists.</p>",
            "<div>Or it ends with punctuation:</div>",
            "<article>Except for 'quotes'</article>",
        ]);

        assert_eq!(
            data.digest,
            "Sentences should have periods. Unless one exists. Or it ends with punctuation: Except for 'quotes'."
        )
    }

    #[test]
    fn inline_tag_formatting() {
        let data = test_parse(vec![
            "<p>Inline tags like <span>span</span>",
            " and <b>bol",
            "d</b> shouldn't have periods</p>",
            "<p>And should n<i>o</i>t add any space.</p>",
        ]);

        assert_eq!(
            data.digest,
            "Inline tags like span and bold shouldn't have periods. And should not add any space."
        )
    }

    #[test]
    fn ignored_elements() {
        let data = test_parse(vec![
            "<p>Elements like:</p>",
            "<form>Should <b>not</b> be indexed</form>",
            "<p>forms</p>",
            "<div> As well as <div data-pagefind-ignore=''>",
            "Manually ignored <p>Elements</p></div>",
            "*crickets*</div>",
        ]);

        assert_eq!(data.digest, "Elements like: forms. As well as *crickets*.")
    }
}
