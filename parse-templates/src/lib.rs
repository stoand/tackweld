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
use std::path::Path;
use globset::{Glob, GlobSet, GlobSetBuilder};

error_chain! {
   foreign_links {
       Io(io::Error);
       Globset(globset::Error);
       WalkDir(walkdir::Error);
   }

   errors {
       ParseTemplate(template_path: String) {
           description("Unable to parse tackweld template source.")
           display("Invalid template format at path: \"{:?}\"", template_path)
       }
   }
}

pub fn parse_templates(src_dirs: Vec<String>) -> Result<()> {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new());
    let glob_matcher = build_globset(&base_dir, src_dirs)?;

    let mut templates = HashMap::new();

    let template_files = walkdir::WalkDir::new(&base_dir)
        .into_iter()
        .filter_entry(|entry| glob_matcher.matches(entry.path()).len() > 0);

    for template_file in template_files {
        println!("{}", template_file?.path().to_string_lossy());
        parse_template_source("1", &mut templates)?;
    }
    // let files = src_dirs.iter().flat_map(|dir| );

    Ok(())
}

fn build_globset(base_dir: &str, glob_strings: Vec<String>) -> Result<GlobSet> {
    let mut globset_builder = GlobSetBuilder::new();

    for glob_string in glob_strings.iter() {
        let path = Path::new(base_dir).join(glob_string);
        globset_builder.add(Glob::new(&path.to_string_lossy())?);
    }

    Ok(globset_builder.build()?)
}

fn parse_template_source(source: &str, templates: &mut HashMap<String, String>) -> Result<()> {
    // let a = ErrorKind::ParseTemplate("asdf".into());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::parse_templates(vec!["src/**/*.html".into()]).unwrap();
    }
}
