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
use std::io;
use globset::{Glob, GlobSet, GlobSetBuilder};

error_chain! {
   links {
       Io(io::Error, io::ErrorKind);
       Globset(globset::Error, globset::ErrorKind);
   }
}

pub fn parse_templates(base_dir: &str, src_dirs: Vec<String>, out_dir: &str) -> Result<()> {
    let glob_matcher = build_globset(src_dirs)?;

    // let a = walkdir::WalkDir::new(&base_dir).into_iter();
    // let files = src_dirs.iter().flat_map(|dir| );

    Ok(())
}

fn build_globset(glob_strings: Vec<String>) -> Result<GlobSet> {
    let mut globset_builder = GlobSetBuilder::new();

    for glob_string in glob_strings.iter() {
        globset_builder.add(Glob::new(glob_string)?);
    }

    Ok(globset_builder.build()?)
}

fn parse_template_source(templates: &mut HashMap<String, String>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
