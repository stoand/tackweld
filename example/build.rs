extern crate tackweld_parse_templates;

use std::env;
use tackweld_parse_templates::parse_templates;

fn main() {
    parse_templates(vec!["src/**/*.html".into()]).unwrap();
}
