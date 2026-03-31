// Copyright 2026 mpi_wrap_gen C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// File: chapters.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/chapters.rs
// Author: Dr. Christoph Pospiech

use anyhow::{Context, Result};
use lopdf::{Document, Object, ObjectId};
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub type ChapterMap = HashMap<u32, (usize, String)>;

const CHAPTER_REGEX: &str = r"^(\d+)\s+(.+)";

/// Looks up a chapter in the `ChapterMap` and returns its 1-based page number
/// as a `u32`.
pub fn get_chapter_page(map: &ChapterMap, chapter: u32) -> Result<u32> {
    map.get(&chapter)
        .ok_or_else(|| anyhow::anyhow!("Chapter {} not found in PDF outlines", chapter))?
        .0
        .try_into()
        .context(format!("Chapter {} page number out of range", chapter))
}

/// Finds the 1-based page indices where each chapter starts by walking the
/// PDF outline (bookmark) tree. Outline entries whose title matches
/// "<N> <title>" (e.g. "3 Point-to-Point Communication") are resolved to
/// their target page, which is then mapped to a 1-based page number.
///
/// Args:
///     doc: A reference to the lopdf PDF document.
///
/// Returns:
///     A ChapterMap where the keys are chapter numbers and the values are tuples
///     of the 1-based page index and the chapter title string.
pub fn find_chapter_pages(doc: &Document) -> Result<ChapterMap> {
    let re = Regex::new(CHAPTER_REGEX)?;

    // Build a mapping from page object ID to 1-based page number
    let page_id_to_num: HashMap<ObjectId, usize> = doc
        .page_iter()
        .enumerate()
        .map(|(i, id)| (id, i + 1))
        .collect();

    // Build a mapping from named destination strings to page arrays (if present)
    let named_dests = build_named_dests(doc).unwrap_or_default();

    let mut chapters = HashMap::new();

    let catalog = doc.catalog().context("Failed to get PDF catalog")?;
    let outlines_ref = catalog
        .get(b"Outlines")
        .context("No /Outlines in catalog")?;
    let outlines_dict = doc
        .dereference(outlines_ref)?
        .1
        .as_dict()
        .context("Failed to dereference /Outlines")?
        .clone();

    // Get the first top-level outline entry
    let first_ref = outlines_dict
        .get(b"First")
        .context("No /First in /Outlines")?;

    let mut current_obj = Some(first_ref.clone());

    while let Some(ref obj) = current_obj {
        // Resolve to actual object if it's a reference
        let (_obj_id, resolved) = doc.dereference(obj)?;
        let dict = resolved.as_dict().context("Outline entry is not a dict")?;

        // Extract the title — may be inline or an indirect reference
        let title = extract_title(doc, dict).unwrap_or_default();

        if let Some(cap) = re.captures(&title) {
            let chapter_num: u32 = cap
                .get(1)
                .with_context(|| {
                    format!(
                        "Missing capture group 1 (chapter number) in outline title {:?}; expected format \"<N> <title>\" matching regex {}",
                        title, CHAPTER_REGEX
                    )
                })?
                .as_str()
                .parse()?;
            let chapter_title = cap
                .get(2)
                .with_context(|| {
                    format!(
                        "Missing capture group 2 (chapter title) in outline title {:?}; expected format \"<N> <title>\" matching regex {}",
                        title, CHAPTER_REGEX
                    )
                })?
                .as_str()
                .to_string();

            // Resolve destination to a page number
            if let Some(page_num) = resolve_dest_to_page(doc, dict, &page_id_to_num, &named_dests) {
                chapters.insert(chapter_num, (page_num, chapter_title));
            }
        }

        // Move to the next sibling
        current_obj = dict.get(b"Next").ok().cloned();
    }

    if chapters.is_empty() {
        anyhow::bail!("No chapters found in PDF outlines");
    }

    Ok(chapters)
}

/// Extract the title string from an outline entry dictionary.
/// Handles both inline strings and indirect references.
fn extract_title(doc: &Document, dict: &lopdf::Dictionary) -> Option<String> {
    let title_obj = dict.get(b"Title").ok()?;
    match title_obj {
        Object::String(bytes, _) => Some(String::from_utf8_lossy(bytes).to_string()),
        Object::Reference(r) => {
            let resolved = doc.get_object(*r).ok()?;
            match resolved {
                Object::String(bytes, _) => Some(String::from_utf8_lossy(bytes).to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Build a lookup table from named destination strings to their destination
/// arrays by inspecting /Catalog -> /Names -> /Dests.
/// Handles /Names arrays (leaf nodes), /Kids trees (intermediate nodes),
/// and /D entries within destination dictionaries.
fn build_named_dests(doc: &Document) -> Result<HashMap<String, Vec<Object>>> {
    let mut map = HashMap::new();

    let catalog = doc.catalog().context("No catalog")?;
    let names_ref = catalog.get(b"Names").context("No /Names")?;
    let names_dict = doc
        .dereference(names_ref)?
        .1
        .as_dict()
        .context("Failed to dereference /Names")?
        .clone();
    let dests_ref = names_dict.get(b"Dests").context("No /Dests")?;
    let dests_obj = doc.dereference(dests_ref)?.1;
    let dests_dict = dests_obj
        .as_dict()
        .context("Failed to dereference /Dests")?
        .clone();

    // Recursively collect named destinations from the name tree
    collect_name_tree_entries(doc, &dests_dict, &mut map)?;

    Ok(map)
}

/// Recursively collect entries from a PDF name tree node.
/// Name trees can have /Names (leaf) and/or /Kids (intermediate) entries.
fn collect_name_tree_entries(
    doc: &Document,
    node: &lopdf::Dictionary,
    map: &mut HashMap<String, Vec<Object>>,
) -> Result<()> {
    // Process /Names array if present (leaf node)
    if let Ok(names_obj) = node.get(b"Names") {
        let names_array = names_obj.as_array().context("/Names is not an array")?;
        for pair in names_array.chunks(2) {
            if pair.len() != 2 {
                continue;
            }
            let name = match &pair[0] {
                Object::String(bytes, _) => String::from_utf8_lossy(bytes).to_string(),
                _ => continue,
            };
            if let Some(dest_arr) = resolve_dest_value(doc, &pair[1]) {
                map.insert(name, dest_arr);
            }
        }
    }

    // Process /Kids array if present (intermediate node)
    if let Ok(kids_obj) = node.get(b"Kids") {
        let kids_array = kids_obj.as_array().context("/Kids is not an array")?;
        for kid_ref in kids_array {
            let kid_obj = match kid_ref {
                Object::Reference(r) => doc.get_object(*r)?,
                other => other,
            };
            let kid_dict = kid_obj.as_dict().context("Kid is not a dict")?;
            collect_name_tree_entries(doc, kid_dict, map)?;
        }
    }

    Ok(())
}

/// Resolve a destination value from the name tree.
/// The value can be:
/// - A direct array: [page_ref /Fit ...]
/// - A reference to an array
/// - A dictionary with a /D key containing the array
/// - A reference to such a dictionary
fn resolve_dest_value(doc: &Document, value: &Object) -> Option<Vec<Object>> {
    // Dereference if needed
    let resolved = match value {
        Object::Reference(r) => doc.get_object(*r).ok()?.clone(),
        other => other.clone(),
    };

    match &resolved {
        Object::Array(arr) => Some(arr.clone()),
        Object::Dictionary(dict) => {
            // Some PDFs wrap the dest array in a dict with /D key
            if let Ok(d) = dict.get(b"D") {
                match d {
                    Object::Array(arr) => Some(arr.clone()),
                    Object::Reference(r) => {
                        let obj = doc.get_object(*r).ok()?;
                        obj.as_array().ok().cloned()
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Resolve an outline entry's destination to a 1-based page number.
/// Handles /Dest (direct array or named string) and /A (GoTo action) entries.
fn resolve_dest_to_page(
    doc: &Document,
    dict: &lopdf::Dictionary,
    page_id_to_num: &HashMap<ObjectId, usize>,
    named_dests: &HashMap<String, Vec<Object>>,
) -> Option<usize> {
    // Try /Dest first
    if let Ok(dest) = dict.get(b"Dest") {
        return page_ref_from_dest(doc, dest, page_id_to_num, named_dests);
    }

    // Try /A (action) with /S /GoTo and /D
    if let Ok(action_obj) = dict.get(b"A") {
        let (_, action_resolved) = doc.dereference(action_obj).ok()?;
        let action_dict = action_resolved.as_dict().ok()?;

        if let Ok(dest) = action_dict.get(b"D") {
            return page_ref_from_dest(doc, dest, page_id_to_num, named_dests);
        }
    }

    None
}

/// Extract a page reference from a destination object.
/// Handles direct arrays, references to arrays, and named string destinations.
fn page_ref_from_dest(
    doc: &Document,
    dest: &Object,
    page_id_to_num: &HashMap<ObjectId, usize>,
    named_dests: &HashMap<String, Vec<Object>>,
) -> Option<usize> {
    // First dereference if it's a reference
    let resolved = match dest {
        Object::Reference(r) => doc.get_object(*r).ok()?.clone(),
        other => other.clone(),
    };

    let arr = match &resolved {
        Object::Array(arr) => arr.clone(),
        Object::String(name_bytes, _) => {
            let name = String::from_utf8_lossy(name_bytes).to_string();
            named_dests.get(&name)?.clone()
        }
        _ => return None,
    };

    // The first element of the dest array is a page reference
    match arr.first()? {
        Object::Reference(page_ref) => page_id_to_num.get(page_ref).copied(),
        _ => None,
    }
}
