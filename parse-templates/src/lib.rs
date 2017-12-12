//! Convert templates with many tpl definitions into files containing
//! a single tpl definition.
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
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate walkdir;

use std::collections::HashMap;
use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, StripPrefixError};
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ComponentDefinition {
    contents: String,
    defined_in_templates: Vec<String>,
}

pub const TEMPLATE_PREFIX : &str = "tw_tpl_";

error_chain! {
   foreign_links {
       Io(io::Error);
       Globset(globset::Error);
       WalkDir(walkdir::Error);
       StripPrefix(StripPrefixError);
   }

   errors {
       TemplateMissingStartDef(template_path: String) {
           description("All tackweld templates must start with a definition like:
            \"::component_name1234\"")
           display("Tackweld template \"{}\" missing component\
            definition (like for example \"::component_name1234\") at line 1", template_path)
       }

       ComponentRedefinition(declarations: String) {
           description("Tackweld components ids were defined multiple times")
           display("There are conflicting component declarations:\n{}", declarations)
       }
   }
}

pub fn parse_templates(src_dirs: Vec<String>, allow_redefinition: bool) -> Result<()> {
    let base_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("OUT_DIR env var missing. This function should be run from build.rs");

    let glob_matcher = build_globset(src_dirs)?;

    let mut components = HashMap::new();

    let search_files = walkdir::WalkDir::new(&base_dir)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in search_files {
        let relative_path = entry.path().strip_prefix(&base_dir)?;

        if glob_matcher.matches(&relative_path).len() > 0 {
            let mut contents = String::new();
            File::open(entry.path())?.read_to_string(&mut contents)?;

            parse_template_source(&relative_path.to_string_lossy(), &contents, &mut components)?;
        }
    }

    if allow_redefinition {
        write_components(components)
    } else {
        // Generate a meaningful error message listing component naming conflicts
        let template_redefinitions = components.iter().fold(String::new(), |acc, (id, def)| {
            if def.defined_in_templates.len() > 1 {
                acc + "\n" + id + ":\n" + &def.defined_in_templates.join("\n")
            } else {
                acc
            }
        });

        if !template_redefinitions.is_empty() {
            Err(ErrorKind::ComponentRedefinition(template_redefinitions).into())
        } else {
            write_components(components)
        }
    }
}

fn write_components(components: HashMap<String, ComponentDefinition>) -> Result<()> {
    let out_dir = env::var("OUT_DIR")
        .expect("OUT_DIR env var missing. This function should be run from build.rs");

    for (id, def) in components.into_iter() {
        let file_name = TEMPLATE_PREFIX.to_string() + &id;
        let output_path = Path::new(&out_dir).join(&file_name);

        let mut file = File::create(output_path)?;
        file.write(def.contents.as_bytes())?;
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

fn parse_template_source(
    template_path: &str,
    source: &str,
    components: &mut HashMap<String, ComponentDefinition>,
) -> Result<()> {
    lazy_static! {
        static ref COMPONENTS_DEF_RE: Regex = Regex::new(r"^\s*::([\w_]+)\s*$").unwrap();
    };

    let mut current_component_id: Option<String> = None;

    for line in source.trim_left().lines() {
        let component_id_match = COMPONENTS_DEF_RE.captures(line).and_then(|c| c.get(1));

        if let Some(component_id) = component_id_match {
            let id = component_id.as_str().to_string();
            current_component_id = Some(id.clone());

            // Add a new component definition if it doesn't exist
            // or add our template file to the list of files defining the component
            // We will need this list to print an error if a component is defined multiple times
            let mut def = components.entry(id).or_insert(ComponentDefinition {
                contents: String::new(),
                defined_in_templates: Vec::new(),
            });

            def.contents = String::new();
            def.defined_in_templates.push(template_path.to_string());
        } else {
            if let Some(def) = current_component_id
                .as_ref()
                .and_then(|id| components.get_mut(id))
            {
                def.contents += line;
            } else {
                return Err(ErrorKind::TemplateMissingStartDef(template_path.into()).into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut components_expected = HashMap::new();

        let src1 = "::root\n<div>Items: {items}</div>\n::item\n<div>value: {val}</div>";
        // "root" is defined twice!
        let src2 = "::root\n<span>asdf</span>";

        parse_template_source("src/one.html", src1, &mut components_expected).unwrap();
        parse_template_source("src/two.html", src2, &mut components_expected).unwrap();

        let mut components_actual = HashMap::new();

        let root_def = ComponentDefinition {
            contents: "<span>asdf</span>".to_string(),
            defined_in_templates: vec!["src/one.html".to_string(), "src/two.html".to_string()],
        };
        components_actual.insert("root".to_string(), root_def);

        let item_def = ComponentDefinition {
            contents: "<div>value: {val}</div>".to_string(),
            defined_in_templates: vec!["src/one.html".to_string()],
        };
        components_actual.insert("item".to_string(), item_def);

        assert_eq!(components_expected, components_actual);
    }
}
