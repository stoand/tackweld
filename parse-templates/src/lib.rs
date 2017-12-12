//! Convert templates with many component definitions into files containing
//! a single component definition.
//!
//! Outputs are saved to the build directory where the `tw!()` macro will read them.
//!
//! # Example
//!
//! ## `build.rs`
//! ```
//! extern crate tackweld_parse_templates;
//!
//! use std::env;
//! use tackweld_parse_templates::parse_templates;
//!
//! fn main() {
//!     parse_templates(vec!["src/**/*.html".into()]).unwrap();
//! }
//! ```
//!
//! ## `src/home.html`
//!
//! ```html
//! ::root
//! <div>Items: {items}</div>
//!
//! ::item
//! <div>value: {val}</div>
//! ```
//!
//! ## `<OUT_DIR>/tw_tpl_root.html` (generated)
//!
//!
//! ```html
//! <div>Items: {items}</div>
//! ```
//! ## `<OUT_DIR>/tw_tpl_item.html` (generated)
//!
//! ```html
//! <div>value: {val}</div>
//! ```

#[macro_use]
extern crate error_chain;
extern crate globset;
extern crate walkdir;

use std::collections::HashMap;
use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, StripPrefixError};
use globset::{Glob, GlobSet, GlobSetBuilder};

error_chain! {
   foreign_links {
       Io(io::Error);
       Globset(globset::Error);
       WalkDir(walkdir::Error);
       StripPrefix(StripPrefixError);
   }

   errors {
       ParseTemplate(template_path: String) {
           description("Unable to parse tackweld template source.")
           display("Invalid template format at path: \"{:?}\"", template_path)
       }
   }
}

pub fn parse_templates(src_dirs: Vec<String>) -> Result<()> {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap(); //_or(String::new());
    let glob_matcher = build_globset(src_dirs)?;

    let mut templates = HashMap::new();

    let search_files = walkdir::WalkDir::new(&base_dir)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in search_files {
        let relative_path = entry.path().strip_prefix(&base_dir)?;

        if glob_matcher.matches(&relative_path).len() > 0 {
            let mut contents = String::new();
            File::open(entry.path())?.read_to_string(&mut contents)?;

            parse_template_source(&contents, &mut templates)?;
        }
    }

    Ok(())
}

fn build_globset(glob_strings: Vec<String>) -> Result<GlobSet> {
    let mut globset_builder = GlobSetBuilder::new();

    for glob_string in glob_strings.iter() {
        globset_builder.add(Glob::new(glob_string)?);
    }

    Ok(globset_builder.build()?)
}

fn parse_template_source(source: &str, templates: &mut HashMap<String, String>) -> Result<()> {
    // let a = ErrorKind::ParseTemplate("asdf".into());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut templates = HashMap::new();

        let src = "
        ::root
        <div>Items: {items}</div>

        ::item
        <div>value: {val}</div>
        ";

        parse_template_source(src, &mut templates).unwrap();

        let expected_keys = vec!["root", "item"];

        let expected_values = vec!["<div>Items: {items}</div>\n", "<div>value: {val}</div>\n\n"];

        assert_eq!(templates.keys().collect::<Vec<_>>(), expected_keys);
        assert_eq!(templates.values().collect::<Vec<_>>(), expected_values);
    }
}
