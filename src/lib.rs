use zed_extension_api::{self as zed, Result};

struct GitAutomatorExtension {
    cached_binary_path: Option<String>,
}

pub struct GitAddCommit {
    pub name: String,
    pub description: String,
    pub tooltip_text: String,
    pub requires_argument: bool,
}

pub struct GitPush {
    pub name: String,
    pub description: String,
    pub tooltip_text: String,
    pub requires_argument: bool,
}

// Idea is to press a hotkey, such as CMD + SHIFT + Z
// Then have
fn check_for_git() {
    println!("Check if git is installed");
}

fn git_add() {
    println!("add file");
}

fn git_commit() {
    println!("commit message");
}

fn git_push() {
    println!("push changes to repo");
}

fn cancel() {
    println!("if");
}

zed::register_extension!(GitAutomatorExtension);
