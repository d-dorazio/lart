use std::process::Command;

use crate::Plugin;

pub struct Vpype {
    plugins: Vec<String>,
}

impl Vpype {
    pub fn new(args: &[&str]) -> Self {
        Self {
            plugins: args.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Plugin for Vpype {
    fn execute(&self, svg: &str) {
        Command::new("vpype")
            .arg("-v")
            .args(&["read", svg])
            .args(&self.plugins)
            .args(&["write", svg])
            .status()
            .unwrap();
    }
}
