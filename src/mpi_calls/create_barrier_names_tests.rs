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
// File: create_barrier_names_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/create_barrier_names_tests.rs
// Author: Dr. Christoph Pospiech

use super::*;
use lopdf::Document;
use std::path::PathBuf;

fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("mpi_call_xmp.pdf")
}

/// Helper that loads the test PDF and returns the full page range as (1, num_pages + 1).
fn full_page_range(doc: &Document) -> (u32, u32) {
    let num_pages = doc.page_iter().count() as u32;
    (1, num_pages + 1)
}

#[test]
fn test_create_barrier_names_count_matches_extract() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let (start, end) = full_page_range(&doc);

    let original = extract_mpi_calls(&doc, start, end)?;
    let barrier = create_barrier_names(original.clone());

    assert_eq!(
        original.len(),
        barrier.len(),
        "create_barrier_names should return the same number of entries as extract_mpi_calls"
    );
    Ok(())
}

#[test]
fn test_create_barrier_names_all_names_have_barrier_suffix() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let (start, end) = full_page_range(&doc);

    let original = extract_mpi_calls(&doc, start, end)?;
    let barrier = create_barrier_names(original);

    assert!(
        !barrier.is_empty(),
        "Expected at least one barrier name to be found"
    );
    for (name, _) in &barrier {
        assert!(
            name.ends_with("_barrier"),
            "Expected name '{}' to end with '_barrier'",
            name
        );
    }
    Ok(())
}

#[test]
fn test_create_barrier_names_prefix_matches_original_call_name() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let (start, end) = full_page_range(&doc);

    let original = extract_mpi_calls(&doc, start, end)?;
    let barrier = create_barrier_names(original.clone());

    for ((orig_name, _), (barrier_name, _)) in original.iter().zip(barrier.iter()) {
        let expected = format!("{}_barrier", orig_name);
        assert_eq!(
            *barrier_name, expected,
            "Expected barrier name '{}', got '{}'",
            expected, barrier_name
        );
    }
    Ok(())
}

#[test]
fn test_create_barrier_names_preserves_parameters() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let (start, end) = full_page_range(&doc);

    let original = extract_mpi_calls(&doc, start, end)?;
    let barrier = create_barrier_names(original.clone());

    for ((_, orig_params), (barrier_name, barrier_params)) in original.iter().zip(barrier.iter()) {
        assert_eq!(
            orig_params, barrier_params,
            "Parameters for '{}' should be unchanged, expected {:?}, got {:?}",
            barrier_name, orig_params, barrier_params
        );
    }
    Ok(())
}

#[test]
fn test_create_barrier_names_finds_send_barrier() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let (start, end) = full_page_range(&doc);

    let original = extract_mpi_calls(&doc, start, end)?;
    let barrier = create_barrier_names(original);

    let send_barrier: Vec<_> = barrier
        .iter()
        .filter(|(name, _)| name == "Send_barrier")
        .collect();
    assert!(
        !send_barrier.is_empty(),
        "Expected to find 'Send_barrier', found: {:?}",
        barrier.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &send_barrier[0];
    assert!(
        params.iter().any(|p| p.contains("buf")),
        "Send_barrier should have a 'buf' parameter, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("comm")),
        "Send_barrier should have a 'comm' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_create_barrier_names_empty_input() {
    let barrier = create_barrier_names(vec![]);
    assert!(
        barrier.is_empty(),
        "Empty input should return no barrier names"
    );
}

#[test]
fn test_create_barrier_names_single_entry() {
    let input = vec![("Send".to_string(), vec!["int count".to_string()])];
    let barrier = create_barrier_names(input);
    assert_eq!(barrier.len(), 1);
    assert_eq!(barrier[0].0, "Send_barrier");
    assert_eq!(barrier[0].1, vec!["int count".to_string()]);
}
