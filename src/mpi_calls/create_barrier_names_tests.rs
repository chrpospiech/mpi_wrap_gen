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
use crate::chapters::find_chapter_pages;
use lopdf::Document;
use std::path::PathBuf;

// The test PDF `test_outline.pdf` has the following structure:
//   Chapter 1 – Introduction               (page 1) – no MPI calls
//   Chapter 2 – Terms and Conventions      (page 2) – no MPI calls
//   Chapter 3 – Point-to-Point Comm.       (page 3) – MPI_Send, MPI_Isend, MPI_Recv
//   Chapter 4 – Datatypes                  (page 4) – no MPI calls
//   Chapter 5 – Collective Communication   (page 5) – MPI_Barrier, MPI_Bcast,
//                                                      MPI_Scatter, MPI_Scatterv
fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("test_outline.pdf")
}

#[test]
fn test_create_barrier_names_count_matches_chapter() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let original = mpi_calls_for_chapter(&doc, &chapters, 3)?;
    let barrier = create_barrier_names(&doc, &chapters, 3)?;

    assert_eq!(
        original.len(),
        barrier.len(),
        "create_barrier_names should return the same number of entries as mpi_calls_for_chapter"
    );
    Ok(())
}

#[test]
fn test_create_barrier_names_all_names_have_barrier_suffix() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let barrier = create_barrier_names(&doc, &chapters, 3)?;

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
    let chapters = find_chapter_pages(&doc)?;

    let original = mpi_calls_for_chapter(&doc, &chapters, 3)?;
    let barrier = create_barrier_names(&doc, &chapters, 3)?;

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
    let chapters = find_chapter_pages(&doc)?;

    let original = mpi_calls_for_chapter(&doc, &chapters, 3)?;
    let barrier = create_barrier_names(&doc, &chapters, 3)?;

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
    let chapters = find_chapter_pages(&doc)?;

    let barrier = create_barrier_names(&doc, &chapters, 3)?;

    let send_barrier: Vec<_> = barrier
        .iter()
        .filter(|(name, _)| name == "Send_barrier")
        .collect();
    assert!(
        !send_barrier.is_empty(),
        "Expected to find 'Send_barrier' in chapter 3, found: {:?}",
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
fn test_create_barrier_names_empty_chapter_returns_empty() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    // Chapter 1 (Introduction) has no MPI call signatures.
    let barrier = create_barrier_names(&doc, &chapters, 1)?;

    assert!(
        barrier.is_empty(),
        "Chapter with no MPI calls should return no barrier names"
    );
    Ok(())
}

#[test]
fn test_create_barrier_names_invalid_chapter_returns_error() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to find chapter pages");

    let result = create_barrier_names(&doc, &chapters, 99);

    assert!(
        result.is_err(),
        "Expected an error for non-existent chapter 99"
    );
}

#[test]
fn test_create_barrier_names_collective_chapter() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let barrier = create_barrier_names(&doc, &chapters, 5)?;

    assert_eq!(
        barrier.len(),
        4,
        "Chapter 5 should produce 4 barrier names, found: {:?}",
        barrier.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    let names: Vec<&str> = barrier.iter().map(|(n, _)| n.as_str()).collect();
    for expected in &[
        "Barrier_barrier",
        "Bcast_barrier",
        "Scatter_barrier",
        "Scatterv_barrier",
    ] {
        assert!(
            names.contains(expected),
            "Expected '{}' in chapter 5 barrier names, got: {:?}",
            expected,
            names
        );
    }
    Ok(())
}
