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
// File: mpi_calls_for_chapter_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/mpi_calls_for_chapter_tests.rs
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

// ── chapter with MPI calls ─────────────────────────────────────────────────

#[test]
fn test_mpi_calls_for_chapter_point_to_point_count() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let calls = mpi_calls_for_chapter(&doc, &chapters, 3)?;

    assert_eq!(
        calls.len(),
        3,
        "Chapter 3 should contain exactly 3 MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_chapter_point_to_point_names() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let calls = mpi_calls_for_chapter(&doc, &chapters, 3)?;
    let names: Vec<&str> = calls.iter().map(|(n, _)| n.as_str()).collect();

    assert!(
        names.contains(&"Send"),
        "Expected 'Send' in chapter 3, got: {:?}",
        names
    );
    assert!(
        names.contains(&"Isend"),
        "Expected 'Isend' in chapter 3, got: {:?}",
        names
    );
    assert!(
        names.contains(&"Recv"),
        "Expected 'Recv' in chapter 3, got: {:?}",
        names
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_chapter_collective_count() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let calls = mpi_calls_for_chapter(&doc, &chapters, 5)?;

    assert_eq!(
        calls.len(),
        4,
        "Chapter 5 should contain exactly 4 MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_chapter_collective_names() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let calls = mpi_calls_for_chapter(&doc, &chapters, 5)?;
    let names: Vec<&str> = calls.iter().map(|(n, _)| n.as_str()).collect();

    for expected in &["Barrier", "Bcast", "Scatter", "Scatterv"] {
        assert!(
            names.contains(expected),
            "Expected '{}' in chapter 5, got: {:?}",
            expected,
            names
        );
    }
    Ok(())
}

// ── chapter without MPI calls ──────────────────────────────────────────────

#[test]
fn test_mpi_calls_for_chapter_empty_chapter_returns_empty() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    // Chapter 1 (Introduction) has no MPI call signatures.
    let calls = mpi_calls_for_chapter(&doc, &chapters, 1)?;

    assert!(
        calls.is_empty(),
        "Chapter 1 should have no MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_chapter_datatypes_returns_empty() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    // Chapter 4 (Datatypes) has no MPI call signatures in the test PDF.
    let calls = mpi_calls_for_chapter(&doc, &chapters, 4)?;

    assert!(
        calls.is_empty(),
        "Chapter 4 should have no MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

// ── last chapter uses end-of-document boundary ────────────────────────────

#[test]
fn test_mpi_calls_for_chapter_last_chapter_uses_end_of_doc() -> Result<()> {
    // Chapter 5 is the last chapter in the PDF, so mpi_calls_for_chapter must
    // fall back to using (num_pages + 1) as the exclusive end page.
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    let calls = mpi_calls_for_chapter(&doc, &chapters, 5)?;

    assert!(
        !calls.is_empty(),
        "Chapter 5 (last chapter) should return MPI calls even without a following chapter"
    );
    Ok(())
}

// ── invalid chapter ────────────────────────────────────────────────────────

#[test]
fn test_mpi_calls_for_chapter_invalid_chapter_returns_error() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to find chapter pages");

    let result = mpi_calls_for_chapter(&doc, &chapters, 99);

    assert!(
        result.is_err(),
        "Expected an error for non-existent chapter 99"
    );
}

#[test]
fn test_mpi_calls_for_chapter_invalid_chapter_error_message() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to find chapter pages");

    let err = mpi_calls_for_chapter(&doc, &chapters, 42).unwrap_err();

    assert!(
        err.to_string().contains("42"),
        "Error message should mention chapter number 42, got: {}",
        err
    );
}

// ── parameters are non-empty for returned calls ────────────────────────────

#[test]
fn test_mpi_calls_for_chapter_all_calls_have_parameters() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    for ch in [3u32, 5u32] {
        let calls = mpi_calls_for_chapter(&doc, &chapters, ch)?;
        for (name, params) in &calls {
            assert!(
                !params.is_empty(),
                "MPI call '{}' in chapter {} should have at least one parameter",
                name,
                ch
            );
        }
    }
    Ok(())
}
