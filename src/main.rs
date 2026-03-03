// Copyright 2026 MPI_wrapper C. Pospiech
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
// File: main.rs
// Description: ~/tools/Rust/mpi_wrapper/src/main.rs
// Author: Dr. Christoph Pospiech

use anyhow::Result;
use pdf_oxide::object::{Object, ObjectRef};
use pdf_oxide::Destination;
use pdf_oxide::PdfDocument;
use std::collections::HashMap;

pub(crate) mod cmdline;

fn main() -> Result<()> {
    let args = cmdline::parse_args()?;
    cmdline::echo_args(&args)?;

    let mut doc = PdfDocument::open(&args.input_file)?;
    if args.dry_run || args.verbose {
        println!("PDF opened successfully: {:?}", &doc);
    }

    let text = doc.extract_text(55)?;
    println!("{text}");

    if let Some(outline) = doc.get_outline()? {
        let page_ref_to_index = build_page_ref_to_index_map(&mut doc)?;
        println!("PDF Outline:");
        print_outline_items(&mut doc, &outline, &page_ref_to_index, 0)?;
    } else {
        println!("No outline found in the PDF.");
    }

    println!("Hello, world!");
    Ok(())
}

fn build_page_ref_to_index_map(doc: &mut PdfDocument) -> Result<HashMap<ObjectRef, usize>> {
    let mut page_ref_to_index = HashMap::new();
    let catalog = doc.catalog()?;
    let pages_ref = catalog
        .as_dict()
        .and_then(|dict| dict.get("Pages"))
        .and_then(Object::as_reference)
        .ok_or_else(|| anyhow::anyhow!("Catalog is missing /Pages reference"))?;

    collect_page_refs(doc, pages_ref, &mut page_ref_to_index)?;
    Ok(page_ref_to_index)
}

fn collect_page_refs(
    doc: &mut PdfDocument,
    node_ref: ObjectRef,
    page_ref_to_index: &mut HashMap<ObjectRef, usize>,
) -> Result<()> {
    let node_obj = doc.load_object(node_ref)?;
    let Some(node_dict) = node_obj.as_dict() else {
        return Ok(());
    };

    match node_dict.get("Type").and_then(Object::as_name) {
        Some("Page") => {
            let index = page_ref_to_index.len();
            page_ref_to_index.insert(node_ref, index);
        }
        Some("Pages") => {
            if let Some(Object::Array(kids)) = node_dict.get("Kids") {
                for kid in kids {
                    if let Some(kid_ref) = kid.as_reference() {
                        collect_page_refs(doc, kid_ref, page_ref_to_index)?;
                    }
                }
            }
        }
        _ => {
            if let Some(Object::Array(kids)) = node_dict.get("Kids") {
                for kid in kids {
                    if let Some(kid_ref) = kid.as_reference() {
                        collect_page_refs(doc, kid_ref, page_ref_to_index)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_outline_items(
    doc: &mut PdfDocument,
    items: &[pdf_oxide::OutlineItem],
    page_ref_to_index: &HashMap<ObjectRef, usize>,
    depth: usize,
) -> Result<()> {
    let chapters_only: bool = true;
    for item in items {
        let should_print = match &item.dest {
            Some(Destination::Named(name)) => !chapters_only || is_chapter_name(name),
            _ => !chapters_only,
        };

        let indent = "  ".repeat(depth);
        let dest_text = match &item.dest {
            Some(Destination::PageIndex(page_index)) => format!("page_index={page_index}"),
            Some(Destination::Named(name)) => {
                match resolve_named_destination_page_index(doc, page_ref_to_index, name)? {
                    Some(page_index) => format!("named={name}, page_index={page_index}"),
                    None => format!("named={name}, page_index=<unresolved>"),
                }
            }
            None => "dest=<none>".to_string(),
        };

        if should_print {
            println!("{indent}- {} ({dest_text})", item.title);
        }

        if !item.children.is_empty() {
            print_outline_items(doc, &item.children, page_ref_to_index, depth + 1)?;
        }
    }

    Ok(())
}

fn is_chapter_name(name: &str) -> bool {
    name.to_ascii_lowercase().starts_with("chapter.")
}

fn resolve_named_destination_page_index(
    doc: &mut PdfDocument,
    page_ref_to_index: &HashMap<ObjectRef, usize>,
    name: &str,
) -> Result<Option<usize>> {
    let Some(dest_obj) = find_named_destination_object(doc, name)? else {
        return Ok(None);
    };

    let Some(page_ref) = destination_object_to_page_ref(doc, &dest_obj)? else {
        return Ok(None);
    };

    Ok(page_ref_to_index.get(&page_ref).copied())
}

fn find_named_destination_object(doc: &mut PdfDocument, name: &str) -> Result<Option<Object>> {
    let catalog = doc.catalog()?;
    let Some(catalog_dict) = catalog.as_dict() else {
        return Ok(None);
    };

    if let Some(dests_obj) = catalog_dict.get("Dests") {
        let dests = dereference_object(doc, dests_obj)?;
        if let Some(dests_dict) = dests.as_dict() {
            if let Some(dest_obj) = dests_dict.get(name) {
                return Ok(Some(dest_obj.clone()));
            }
        }
    }

    if let Some(names_obj) = catalog_dict.get("Names") {
        let names = dereference_object(doc, names_obj)?;
        if let Some(names_dict) = names.as_dict() {
            if let Some(dests_obj) = names_dict.get("Dests") {
                return find_in_name_tree(doc, dests_obj, name);
            }
        }
    }

    Ok(None)
}

fn find_in_name_tree(
    doc: &mut PdfDocument,
    tree_obj: &Object,
    target: &str,
) -> Result<Option<Object>> {
    let tree = dereference_object(doc, tree_obj)?;
    let Some(tree_dict) = tree.as_dict() else {
        return Ok(None);
    };

    if let Some(Object::Array(entries)) = tree_dict.get("Names") {
        for pair in entries.chunks_exact(2) {
            let Some(key) = object_to_name_key(&pair[0]) else {
                continue;
            };
            if key == target {
                return Ok(Some(pair[1].clone()));
            }
        }
    }

    if let Some(Object::Array(kids)) = tree_dict.get("Kids") {
        for kid in kids {
            if let Some(found) = find_in_name_tree(doc, kid, target)? {
                return Ok(Some(found));
            }
        }
    }

    Ok(None)
}

fn destination_object_to_page_ref(
    doc: &mut PdfDocument,
    obj: &Object,
) -> Result<Option<ObjectRef>> {
    match obj {
        Object::Reference(obj_ref) => {
            let resolved = doc.load_object(*obj_ref)?;
            destination_object_to_page_ref(doc, &resolved)
        }
        Object::Array(arr) => Ok(arr.first().and_then(Object::as_reference)),
        Object::Dictionary(dict) => {
            if let Some(d_obj) = dict.get("D") {
                destination_object_to_page_ref(doc, d_obj)
            } else if let Some(dest_obj) = dict.get("Dest") {
                destination_object_to_page_ref(doc, dest_obj)
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

fn dereference_object(doc: &mut PdfDocument, obj: &Object) -> Result<Object> {
    match obj {
        Object::Reference(obj_ref) => Ok(doc.load_object(*obj_ref)?),
        _ => Ok(obj.clone()),
    }
}

fn object_to_name_key(obj: &Object) -> Option<String> {
    match obj {
        Object::String(bytes) => Some(String::from_utf8_lossy(bytes).to_string()),
        Object::Name(name) => Some(name.clone()),
        _ => None,
    }
}
