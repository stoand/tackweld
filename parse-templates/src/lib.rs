//! Convert templates with many component definitions into files containing
//! a single component definition.
//! 
//! Outputs are saved to the build directory where the `tw!()` macro will read them.
//! 
//! # Example
//! 
//! ```
//! // Basic usage
//! parse_templates(vec!["src/**/*.html"]).unwrap();
//! ```
//! 
//! Contents of `src/home.html`:
//! 
//! ```html
//! ::root
//! <div>Items: {items}</div>
//! 
//! ::item
//! <div>value: {val}</div>
//! ```
//! 
//! Contents written to `<OUT_DIR>/tw_tpl_root.html`:
//! 
//! 
//! ```html
//! <div>Items: {items}</div>
//! ```
//! Contents written to `<OUT_DIR>/tw_tpl_item.html`:
//! 
//! ```html
//! <div>value: {val}</div>
//! ```

use std::collections::HashMap;
use std::io;

pub fn parse_templates(src_dirs: Vec<String>) -> io::Result<()> {
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
