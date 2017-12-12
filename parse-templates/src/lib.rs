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

extern crate walkdir;

use std::collections::HashMap;
use std::io;

pub fn parse_templates(src_dirs: Vec<String>) -> io::Result<()> {

    let files = src_dirs.iter().flat_map(|dir | walkdir::WalkDir::new(&dir));

    Ok(())
}

fn parse_template_source(templates: &mut HashMap<String, String>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
