//! Redirect deps
use crate::{registry::Registry, result::Result};
use etc::{Etc, Read, Write};
use std::path::PathBuf;

const PATH_ATTR_PATT: &str = "path = \"";
const VERSION_ATTR_PATT: &str = "version = \"";
const TAG_ATTR_PATT: &str = "tag = \"";
const INLINE_DEP_ANCHOR: &str = " = ";
const INLINE_DEP_END_PATT: &str = "\n";
const BLOCK_DEP_ANCHOR: [&str; 2] = [".", "]"];
const BLOCK_DEP_END_PATT: &str = "\n\n\n";
const PACKAGE_DEP_ANCHOR: &str = "package = \"";

/// Override attr with new pattern
fn attr(mut src: String, attr: &str, dst: &str, end: Option<&str>) -> String {
    let end_patt = if let Some(patt) = end { patt } else { "\"" };
    if src.contains(attr) {
        let begin = src.as_str().find(attr).unwrap_or(0);
        let first_single_qoute = begin + src.as_str()[begin..].find('\'').unwrap_or(0) + 1;
        let first_double_qoute = begin + src.as_str()[begin..].find('"').unwrap_or(0) + 1;
        let first_qoute =
            if first_single_qoute < first_double_qoute && first_single_qoute > begin + 1 {
                first_single_qoute
            } else {
                first_double_qoute
            };
        let second_qoute =
            first_qoute + src.as_str()[first_qoute..].find(end_patt).unwrap_or(0) + 1;
        src.replace_range(begin..second_qoute, dst);
    }

    src
}

// Add attr to a depenedency
fn add_attr(mut src: String, attr: &str, value: &str) -> String {
    let mut dep = false;
    let begin = src.as_str().find('{').unwrap_or_else(|| {
        dep = true;
        src.as_str().find(']').unwrap_or(0) + 1
    });

    // Check if add failed
    if begin < 2 {
        return src;
    }

    src.insert_str(
        begin + 1,
        &format!(
            "{}{} = \"{}\"{}",
            if dep { "" } else { " " },
            attr,
            value,
            if dep { "\n " } else { "," }
        ),
    );

    src
}

// Remove attr to a dependency
fn remove_attr(mut src: String, attr: &str) -> String {
    let begin = src.as_str().find(&attr).unwrap_or(0);
    if begin == 0 {
        return src;
    }

    let mut end = &src[begin..].find('\"').unwrap_or(0) + begin + 1;
    end += &src[end..].find('\"').unwrap_or(0) + 1;
    end += &src[end..]
        .find(|c: char| !c.is_whitespace() && c != ',')
        .unwrap_or(0);

    if &src[begin..begin + 1] == "{" {
        src.replace_range(begin..end, "");
    } else {
        src.replace_range(begin..end, "");
    }

    src
}

/// Check if contains dependency
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
        let pos = ms.find(&*anchor).unwrap_or(0);
        let begin = ms[..pos].rfind('\n').unwrap_or(0) + 1;
        *anchor = ms[begin..pos].to_string();
        return true;
    }

    // check block dep syntax
    ms.contains(&*anchor)
}

/// Redirect the dependencies from relative paths to git resgistry
pub fn redirect(mani: &PathBuf, registry: &Registry) -> Result<()> {
    let tag = registry.cur_tag()?;
    let mut has_tag = false;
    if !tag.is_empty() {
        has_tag = true;
    }

    // Batch the target manifests
    let target = Etc::from(mani);
    let bytes = target.read()?;
    let mut ms = String::from_utf8_lossy(&bytes).to_string();
    for (k, v, e) in registry.config.metadata.tuple() {
        ms = attr(ms, k, &v, e);
    }

    // Replace Dependencies
    for dep in registry.source()? {
        let mut anchor = format!("{}{}", dep.0, INLINE_DEP_ANCHOR);
        let mut end_patt = INLINE_DEP_END_PATT.to_string();
        if !contains_dep(&ms, &dep.0, &mut anchor, &mut end_patt) {
            continue;
        }

        let begin = ms.as_str().find(&anchor).unwrap_or(0);
        let end = begin + ms.as_str()[begin..].find(&end_patt).unwrap_or(0);
        let mut patt = ms.as_str()[begin..end].to_string();

        // Replace `path` to `git`
        patt = attr(
            patt,
            PATH_ATTR_PATT,
            &format!("git = \"{}\"", registry.config.node.registry),
            None,
        );

        // Pair `version`
        patt = attr(
            patt,
            VERSION_ATTR_PATT,
            &format!("version = \"{}\"", dep.1),
            None,
        );

        // Pair `tag`
        if has_tag {
            if patt.contains(TAG_ATTR_PATT) {
                patt = attr(patt, TAG_ATTR_PATT, &format!("tag = \"{}\"", tag), None);
            } else {
                patt = add_attr(patt, "tag", &tag);
            }
        } else {
            patt = remove_attr(patt, TAG_ATTR_PATT);
        }

        ms.replace_range(begin..end, &patt);
    }

    target.write(ms.as_bytes())?;
    Ok(())
}
