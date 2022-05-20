use std::cell::RefCell;
use std::str::FromStr;

use cucumber::gherkin::Step;
use cucumber::{given, then, when};
use json_dotpath::DotPaths;
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use kuchiki::{Attributes, ElementData, NodeDataRef, NodeRef};
use regex::Regex;
use serde_json::Value;

use crate::BrowserTester;
use crate::TestWorld;

// GIVENS

#[given(regex = "^I have an? (?:\"|')(.*)(?:\"|') file$")]
fn new_empty_file(world: &mut TestWorld, filename: String) {
    world.write_file(&filename, "");
}

#[given(regex = "^I have an? (?:\"|')(.*)(?:\"|') file with the content:$")]
fn new_file(world: &mut TestWorld, step: &Step, filename: String) {
    match &step.docstring {
        Some(contents) => {
            world.write_file(&filename, contents);
        }
        None => panic!("`{}` step expected a docstring", step.value),
    }
}

#[given(regex = "^I have an? (?:\"|')(.*)(?:\"|') file with the body:$")]
fn new_templated_file(world: &mut TestWorld, step: &Step, filename: String) {
    match &step.docstring {
        Some(contents) => {
            world.write_file(&filename, &template_file(contents));
        }
        None => panic!("`{}` step expected a docstring", step.value),
    }
}

// BINARY WHENS

#[when(regex = "^I run my program$")]
fn run_rosey(world: &mut TestWorld) {
    world.run_command(None);
}

#[when(regex = "^I run my program with the flags:$")]
fn run_rosey_with_options(world: &mut TestWorld, step: &Step) {
    match &step.table {
        Some(table) => {
            world.run_command(Some(table));
        }
        None => panic!("`{}` step expected a docstring", step.value),
    }
}

// THENS

#[then(regex = "^(DEBUG )?I should see (?:\"|')(.*)(?:\"|') in stdout$")]
fn stdout_does_contain(world: &mut TestWorld, debug: StepDebug, expected: String) {
    match &world.last_command_output {
        Some(command) => {
            debug.log(&command.stdout);
            debug.log(&command.stderr);
            assert!(command.stdout.contains(&expected));
        }
        None => panic!("No stdout to check"),
    }
}

#[then(regex = "^(DEBUG )?I should see (?:\"|')(.*)(?:\"|') in (?:\"|')(.*)(?:\"|')$")]
fn file_does_contain(world: &mut TestWorld, debug: StepDebug, expected: String, filename: String) {
    world.assert_file_exists(&filename);
    let contents = world.read_file(&filename);
    debug.log(&contents);
    assert!(contents.contains(&expected));
}

#[then(regex = "^(DEBUG )?I should not see (?:\"|')(.*)(?:\"|') in (?:\"|')(.*)(?:\"|')$")]
fn file_does_not_contain(
    world: &mut TestWorld,
    debug: StepDebug,
    expected: String,
    filename: String,
) {
    world.assert_file_exists(&filename);
    let contents = world.read_file(&filename);
    debug.log(&contents);
    assert!(!contents.contains(&expected));
}

#[then(regex = "^I should see the file (?:\"|')(.*)(?:\"|')$")]
fn file_does_exist(world: &mut TestWorld, filename: String) {
    world.assert_file_exists(&filename);
}

#[then(regex = "^I should not see the file (?:\"|')(.*)(?:\"|')$")]
fn file_does_not_exist(world: &mut TestWorld, filename: String) {
    world.assert_file_doesnt_exist(&filename);
}

#[then(
    regex = "^(DEBUG )?I should (not )?see a selector (?:\"|')(.*)(?:\"|') in (?:\"|')(\\S*)(?:\"|')$"
)]
fn selector_exists(
    world: &mut TestWorld,
    debug: StepDebug,
    negation: Not,
    selector: String,
    filename: String,
) {
    world.assert_file_exists(&filename);
    let contents = world.read_file(&filename);
    debug.log(&contents);
    let parsed_file = parse_html_file(&contents);
    if negation.0 {
        assert!(select_nodes(&parsed_file, &selector).next().is_none());
    } else {
        assert!(select_nodes(&parsed_file, &selector).next().is_some());
    }
}

#[then(
    regex = "^(DEBUG )?I should (not )?see a selector (?:\"|')(.*)(?:\"|') in (?:\"|')(.*)(?:\"|') with the attributes:$"
)]
fn selector_attributes(
    world: &mut TestWorld,
    step: &Step,
    debug: StepDebug,
    negation: Not,
    selector: String,
    filename: String,
) {
    world.assert_file_exists(&filename);
    let contents = world.read_file(&filename);
    debug.log(&contents);
    let parsed_file = parse_html_file(&contents);
    let mut last_looked_at: Option<NodeDataRef<ElementData>> = None;

    'nodes: for node in select_nodes(&parsed_file, &selector) {
        last_looked_at = Some(node.clone());
        let atts = node_attributes(&node);
        let attributes = atts.borrow_mut();
        let rows = &step
            .table
            .as_ref()
            .expect("This step requires a table")
            .rows;
        for row in rows {
            let attribute_key = normalize_table_cell(&row[0]);
            let value = match attribute_key.as_ref() {
                "innerText" => node.text_contents(),
                _ => {
                    let value = attributes.get(attribute_key);
                    match value {
                        Some(value) => value.to_string(),
                        None => continue 'nodes,
                    }
                }
            };
            if value != normalize_table_cell(&row[1]) {
                continue 'nodes;
            }
        }
        for attribute in attributes.map.keys() {
            let attribute_expected = rows
                .iter()
                .map(|row| &row[0])
                .any(|x| x == &attribute.local.to_string());
            if !attribute_expected {
                continue 'nodes;
            }
        }
        if negation.0 {
            panic!("A node that exactly matched all provided attributes was found.")
        }
        return;
    }
    if !negation.0 {
        match last_looked_at {
            Some(last_node) => panic!(
                "No nodes found that exactly match all provided attributes. Last node had attributes {:#?}", last_node.attributes
            ),
            None => panic!("No nodes found with that selector!"),
        }
    }
}

#[then(regex = "^(DEBUG )?I should see (?:\"|')(\\S+\\.json)(?:\"|') containing the values:$")]
fn json_contains_values(world: &mut TestWorld, debug: StepDebug, step: &Step, filename: String) {
    world.assert_file_exists(&filename);
    let contents = world.read_file(&filename);
    debug.log(&contents);
    let parsed_json = parse_json_file(&contents);
    let int_re = Regex::new(r"^int:(\d+)$").unwrap();

    for row in &step
        .table
        .as_ref()
        .expect("This step requires a table")
        .rows
    {
        let expected_value = normalize_table_cell(&row[1]);
        if let Some(expected_int) = int_re.captures(&expected_value) {
            let value: i64 = parsed_json
                .dot_get(&row[0])
                .unwrap_or_else(|_| panic!("JSON path {} lookup failed", &row[0]))
                .unwrap_or_else(|| {
                    panic!(
                        "JSON path {} yielded none\nLooked at the structure {:#?}",
                        &row[0], parsed_json
                    )
                });
            let expected_int: i64 = expected_int
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .expect("expected_int wasn't an int");
            assert_eq!(value, expected_int);
        } else {
            let value: String = parsed_json
                .dot_get(&row[0])
                .unwrap_or_else(|_| panic!("JSON path {} lookup failed", &row[0]))
                .unwrap_or_else(|| {
                    panic!(
                        "JSON path {} yielded none\nLooked at the structure {:#?}",
                        &row[0], parsed_json
                    )
                });
            assert_eq!(value, expected_value);
        }
    }
}

// HELPERS

fn parse_json_file(json: &str) -> Value {
    serde_json::from_str(json).expect("File contained invalid JSON")
}

fn parse_html_file(html: &str) -> NodeRef {
    kuchiki::parse_html().one(html)
}

fn select_nodes(parsed_file: &NodeRef, selector: &str) -> Select<Elements<Descendants>> {
    parsed_file
        .select(selector)
        .expect("Valid selector was given")
}

fn node_attributes(node: &NodeDataRef<ElementData>) -> RefCell<Attributes> {
    node.as_node()
        .as_element()
        .expect("Given selector was an element")
        .attributes
        .clone()
}

fn normalize_table_cell(table_value: &str) -> String {
    table_value.replace("\\PIPE", "|").replace("\\n", "\n")
}

fn template_file(body_contents: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        body_contents
    )
}

// Helpers

struct StepDebug(bool);

impl FromStr for StepDebug {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEBUG " => Ok(StepDebug(true)),
            _ => Ok(StepDebug(false)),
        }
    }
}

impl StepDebug {
    fn log(&self, contents: &str) {
        if self.0 {
            println!("\n\nDEBUG:\n---\n{:?}\n---\n\n", contents);
        }
    }
}

struct Not(bool);

impl FromStr for Not {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "not " => Ok(Not(true)),
            _ => Ok(Not(false)),
        }
    }
}
