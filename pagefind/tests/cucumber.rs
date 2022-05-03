use cucumber::gherkin::Table;
use portpicker::pick_unused_port;
use std::convert::Infallible;
use std::io::{Read, Write};
use std::process::Command;
use std::str::from_utf8;
use std::{fs, path::PathBuf};
use tempfile::tempdir;
use wax::Glob;

use async_trait::async_trait;
use browser::BrowserTester;
use cucumber::{World, WorldInit};

mod browser;
mod steps;

#[derive(Debug)]
struct CommandOutput {
    stdout: String,
    stderr: String,
}

#[derive(Debug, Default, WorldInit)]
struct TestWorld {
    tmp_dir: Option<tempfile::TempDir>,
    last_command_output: Option<CommandOutput>,
    browser: Option<BrowserTester>,
    assigned_server_port: Option<u16>,
}

impl TestWorld {
    fn ensure_port(&mut self) -> u16 {
        if self.assigned_server_port.is_none() {
            self.assigned_server_port = pick_unused_port();
        }
        self.assigned_server_port.expect("No port was available")
    }
    async fn ensure_browser(&mut self) -> &mut BrowserTester {
        if self.browser.is_none() {
            self.browser = Some(BrowserTester::new().await);
        }
        self.browser.as_mut().unwrap()
    }

    fn tmp_dir(&mut self) -> PathBuf {
        if self.tmp_dir.is_none() {
            self.tmp_dir = Some(tempdir().expect("testing on a system with a temp dir"));
        }
        self.tmp_dir
            .as_ref()
            .expect("just created")
            .path()
            .to_path_buf()
    }

    fn tmp_file_path(&mut self, filename: &str) -> PathBuf {
        let tmp_dir = self.tmp_dir();
        tmp_dir.join(PathBuf::from(filename))
    }

    fn write_file(&mut self, filename: &str, contents: &str) {
        let file_path = self.tmp_file_path(filename);
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        let mut file = std::fs::File::create(&file_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn read_file(&mut self, filename: &str) -> String {
        let file_path = self.tmp_file_path(filename);
        let mut file = std::fs::File::open(&file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn get_file_tree(&mut self) -> String {
        let glob = Glob::new("**/*").unwrap();
        let base_dir = self.tmp_file_path(".");
        let entries: Vec<String> = glob
            .walk(&base_dir, usize::MAX)
            .flatten()
            .map(|entry| {
                let file = entry.path().strip_prefix(&base_dir).unwrap();
                let indentation = "  ".repeat(file.components().count() - 1);
                format!(
                    "| {}{}",
                    indentation,
                    file.file_name().unwrap().to_str().unwrap()
                )
            })
            .collect();
        entries.join("\n")
    }

    fn assert_file_exists(&mut self, filename: &str) {
        if !self.check_file_exists(filename) {
            panic!(
                "\"{}\" does not exist in the tree:\n-----\n{}\n-----\n",
                filename,
                self.get_file_tree()
            );
        }
    }

    fn assert_file_doesnt_exist(&mut self, filename: &str) {
        if self.check_file_exists(filename) {
            panic!(
                "\"{}\" should not exist but does in the tree:\n-----\n{}\n-----\n",
                filename,
                self.get_file_tree()
            );
        }
    }

    fn check_file_exists(&mut self, filename: &str) -> bool {
        self.tmp_file_path(filename).exists()
    }

    fn run_command(&mut self, options: Option<&Table>) {
        let binary = std::env::var("TEST_BINARY").unwrap_or_else(|_| {
            panic!("No binary supplied — please provide a TEST_BINARY environment variable");
        });

        let cli = build_command(&binary, None, options);
        let output = Command::new("sh")
            .arg("-c")
            .current_dir(self.tmp_dir())
            .arg(&cli.replace(std::path::MAIN_SEPARATOR, "/"))
            .output()
            .unwrap_or_else(|_| panic!("failed to run {}", binary));
        self.last_command_output = Some(CommandOutput {
            stdout: from_utf8(&output.stdout).unwrap_or("failed utf8").into(),
            stderr: from_utf8(&output.stderr).unwrap_or("failed utf8").into(),
        });
    }
}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl World for TestWorld {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
    }
}

// This runs before everything else, so you can setup things here
#[tokio::main]
async fn main() {
    TestWorld::run("features").await;
}

struct BinaryCommand(String);

impl BinaryCommand {
    fn add_flag(&mut self, flag: &str) {
        self.0 = format!("{} {}", self.0, flag);
    }

    fn consume(self) -> String {
        self.0
    }
}

fn build_command(binary: &str, subcommand: Option<&str>, options: Option<&Table>) -> String {
    let cwd = std::env::current_dir().unwrap();
    let binary_path = cwd.join(PathBuf::from(binary));
    let binary_path = binary_path.to_str().unwrap();

    let mut command = match subcommand {
        Some(subcommand) => BinaryCommand(format!("{} {}", binary_path, subcommand)),
        None => BinaryCommand(binary_path.into()),
    };

    if let Some(options) = options {
        for row in &options.rows {
            command.add_flag(&row[0]);
        }
    }

    command.consume()
}
