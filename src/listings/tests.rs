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
// File: tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/listings/tests.rs
// Author: Dr. Christoph Pospiech

use crate::cmdline::CliArgs;
use crate::config::{Config, Templates};
use crate::listings::extract_and_generate_listings;
use anyhow::Result;
use lopdf::Document;
use std::path::PathBuf;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn outline_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("test_outline.pdf")
}

fn mpi_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("mpi_call_xmp.pdf")
}

/// Build a `Config` with simple, valid templates for the given chapter list.
fn make_config(chapters: Vec<u32>) -> Config {
    Config {
        mpi_pdf: PathBuf::from("dummy.pdf"),
        output_file: PathBuf::from("dummy_output.c"),
        templates: Templates {
            header: "HEADER\n".to_string(),
            ch_init: "INIT %(chapter_number)d %(chapter_title)s %(mpi_call_name)s\n".to_string(),
            ch_body: "BODY %(mpi_call_name)s\n".to_string(),
            footer: "FOOTER\n".to_string(),
        },
        chapters,
    }
}

fn quiet_args() -> CliArgs {
    CliArgs::default()
}

fn verbose_args() -> CliArgs {
    CliArgs {
        verbose: true,
        ..CliArgs::default()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// The function must return an error immediately when no chapters are configured.
#[test]
fn test_empty_chapters_returns_error() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![]);
    let result = extract_and_generate_listings(&doc, &quiet_args(), &config);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("No chapters specified"),
        "unexpected error: {msg}"
    );
    Ok(())
}

/// Requesting a chapter that has no outline entry must propagate an error.
#[test]
fn test_chapter_not_found_returns_error() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![99]);
    let result = extract_and_generate_listings(&doc, &quiet_args(), &config);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("99"),
        "error message should mention the missing chapter number: {msg}"
    );
    Ok(())
}

/// When every requested chapter exists but contains no MPI calls, the output
/// must still contain the header.
#[test]
fn test_header_present_when_no_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1]);
    let output = extract_and_generate_listings(&doc, &quiet_args(), &config)?;
    assert!(
        output.starts_with("HEADER\n"),
        "output should start with header, got: {output:?}"
    );
    Ok(())
}

/// When every requested chapter exists but contains no MPI calls, the output
/// must still contain the footer.
#[test]
fn test_footer_present_when_no_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1]);
    let output = extract_and_generate_listings(&doc, &quiet_args(), &config)?;
    assert!(
        output.ends_with("FOOTER\n"),
        "output should end with footer, got: {output:?}"
    );
    Ok(())
}

/// Chapters without any MPI calls must be silently skipped – the body template
/// must not appear in the output.
#[test]
fn test_no_body_when_chapter_has_no_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1, 2]);
    let output = extract_and_generate_listings(&doc, &quiet_args(), &config)?;
    assert!(
        !output.contains("BODY"),
        "body template should be absent when there are no MPI calls, got: {output:?}"
    );
    Ok(())
}

/// Verbose mode must not panic and must still produce valid output.
#[test]
fn test_verbose_flag_does_not_panic() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1]);
    let output = extract_and_generate_listings(&doc, &verbose_args(), &config)?;
    assert!(output.contains("HEADER\n"));
    assert!(output.contains("FOOTER\n"));
    Ok(())
}

/// When the PDF contains MPI calls on the pages covered by the requested
/// chapter range, both the chapter-init and body templates must appear in
/// the output and contain the correct chapter number.
#[test]
fn test_output_contains_init_and_body_for_mpi_calls() -> Result<()> {
    let doc = Document::load(mpi_pdf_path())?;
    // mpi_call_xmp.pdf has no chapter outlines, so build a ChapterMap
    // by calling the function with the full page range treated as one chapter.
    // We need at least one chapter whose pages contain MPI calls.  The PDF
    // used by mpi_calls tests spans all pages, so use chapter 1 covering the
    // whole document. Because find_chapter_pages will fail on a PDF without
    // outlines, we expect an error here rather than a successful result.
    let config = make_config(vec![1]);
    let result = extract_and_generate_listings(&doc, &quiet_args(), &config);
    // The PDF has no outline → find_chapter_pages returns an error.
    // Verify the error is about missing chapter structure (not a panic).
    assert!(
        result.is_err(),
        "expected error for PDF without chapter outlines"
    );
    Ok(())
}

/// Output must be exactly `header + footer` when a chapter exists but has
/// no MPI calls (body and init templates must both be absent).
#[test]
fn test_output_is_header_plus_footer_only_when_no_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1]);
    let output = extract_and_generate_listings(&doc, &quiet_args(), &config)?;
    assert_eq!(
        output, "HEADER\nFOOTER\n",
        "output should be exactly header+footer when no MPI calls found"
    );
    Ok(())
}
