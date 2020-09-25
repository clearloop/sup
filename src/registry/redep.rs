//! Redirect deps
use crate::{registry::Registry, result::Result};
use etc::{Etc, Read, Write};
use std::path::PathBuf;

const PATH_ATTR_PATT: &str = "path = \"";
const INLINE_DEP_ANCHOR: &str = " = ";
const INLINE_DEP_END_PATT: &str = "\n";
const BLOCK_DEP_ANCHOR: [&str; 2] = [".", "]"];
const BLOCK_DEP_END_PATT: &str = "\n\n\n";
const PACKAGE_DEP_ANCHOR: &str = "package = \"";

/// Override attr with new pattern
fn attr(mut src: String, attr: &str, dst: &str) -> String {
    if src.contains(attr) {
        let start = src.as_str().find(attr).unwrap_or(0);
        let first_qoute = start + src.as_str()[start..].find('"').unwrap_or(0) + 1;
        let second_qoute = first_qoute + src.as_str()[first_qoute..].find('"').unwrap_or(0) + 1;
        src.replace_range(start..second_qoute, dst);
    }

    src
}

fn contains_dep(ms: &str, dep: &str, anchor: &mut String, end_patt: &mut String) -> bool {
    // check custom dep syntax
    if !ms.contains(&*anchor) {
        *anchor = format!("{}{}\"", PACKAGE_DEP_ANCHOR, &dep);
    } else {
        return true;
    }

    // check package dep syntax
    if !ms.contains(&*anchor) {
        *anchor = BLOCK_DEP_ANCHOR.join(&dep);
        *end_patt = BLOCK_DEP_END_PATT.to_string();
    } else {
        return true;
    }

    // check block dep syntax
    ms.contains(&*anchor)
}

/// Redirect the dependencies from relative paths to git resgistry
pub fn redirect(mani: &PathBuf, registry: &Registry) -> Result<()> {
    let target = Etc::from(mani);
    let bytes = target.read()?;
    let mut ms = String::from_utf8_lossy(&bytes).to_string();
    for dep in registry.source()? {
        let mut anchor = format!("{}{}", dep, INLINE_DEP_ANCHOR);
        let mut end_patt = INLINE_DEP_END_PATT.to_string();
        if !contains_dep(&ms, &dep, &mut anchor, &mut end_patt) {
            continue;
        }

        let start = ms.as_str().find(&anchor).unwrap_or(0);
        let end = start + ms.as_str()[start..].find(&end_patt).unwrap_or(0);
        let mut patt = ms.as_str()[start..end].to_string();
        patt = attr(
            patt,
            PATH_ATTR_PATT,
            &format!(
                "git = \"{}\"",
                "https://github.com/paritytech/substrate.git"
            ),
        );
        ms.replace_range(start..end, &patt);
    }

    target.write(ms.as_bytes())?;
    Ok(())
}
