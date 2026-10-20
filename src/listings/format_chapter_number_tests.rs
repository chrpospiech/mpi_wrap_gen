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
// File: format_chapter_number_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/listings/format_chapter_number_tests.rs
// Author: Dr. Christoph Pospiech

use super::format_chapter_number;
use anyhow::Result;
use std::path::{Path, PathBuf};

// ── No placeholder ────────────────────────────────────────────────────────────

/// A path with no format placeholder is returned unchanged.
#[test]
fn no_placeholder_returns_path_unchanged() -> Result<()> {
    let template = Path::new("/output/listings.c");
    let result = format_chapter_number(template, 5, 0).unwrap();
    assert_eq!(result, PathBuf::from("/output/listings.c"));
    Ok(())
}

// ── %(chapter_number)s substitution ──────────────────────────────────────────

/// `%(chapter_number)s` in the filename is replaced with the chapter number.
#[test]
fn string_format_substitutes_chapter_number() -> Result<()> {
    let template = Path::new("/output/chapter_%(chapter_number)s.c");
    let result = format_chapter_number(template, 3, 0).unwrap();
    assert_eq!(result, PathBuf::from("/output/chapter_3.c"));
    Ok(())
}

/// `%(chapter_number)d` in the filename is replaced with the chapter number.
#[test]
fn integer_format_substitutes_chapter_number() -> Result<()> {
    let template = Path::new("/output/chapter_%(chapter_number)d.c");
    let result = format_chapter_number(template, 7, 0).unwrap();
    assert_eq!(result, PathBuf::from("/output/chapter_7.c"));
    Ok(())
}

// ── Placeholder in directory component ───────────────────────────────────────

/// A placeholder embedded in a directory component is substituted correctly.
#[test]
fn placeholder_in_directory_component() -> Result<()> {
    let template = Path::new("/chapters/ch_%(chapter_number)s/listings.c");
    let result = format_chapter_number(template, 12, 0).unwrap();
    assert_eq!(result, PathBuf::from("/chapters/ch_12/listings.c"));
    Ok(())
}

// ── Edge-case chapter numbers ─────────────────────────────────────────────────

/// Chapter number 0 is substituted correctly.
#[test]
fn chapter_number_zero() -> Result<()> {
    let template = Path::new("out_%(chapter_number)s.c");
    let result = format_chapter_number(template, 0, 0).unwrap();
    assert_eq!(result, PathBuf::from("out_0.c"));
    Ok(())
}

/// Large chapter numbers are substituted correctly.
#[test]
fn large_chapter_number() -> Result<()> {
    let template = Path::new("out_%(chapter_number)s.c");
    let result = format_chapter_number(template, 100, 0).unwrap();
    assert_eq!(result, PathBuf::from("out_100.c"));
    Ok(())
}

// ── Multiple occurrences ──────────────────────────────────────────────────────

/// When the placeholder appears more than once it is replaced at every position.
#[test]
fn multiple_occurrences_are_all_replaced() -> Result<()> {
    let template = Path::new("ch%(chapter_number)s/ch%(chapter_number)s.c");
    let result = format_chapter_number(template, 4, 0).unwrap();
    assert_eq!(result, PathBuf::from("ch4/ch4.c"));
    Ok(())
}

// ── Result type ───────────────────────────────────────────────────────────────

/// The function returns a `PathBuf`, not a bare string.
#[test]
fn result_is_pathbuf() -> Result<()> {
    let template = Path::new("listing_%(chapter_number)s.c");
    let result: PathBuf = format_chapter_number(template, 2, 0).unwrap();
    assert_eq!(result, PathBuf::from("listing_2.c"));
    Ok(())
}

// ── Typo / unknown placeholder ────────────────────────────────────────────────

/// A misspelled placeholder name is treated as a missing argument by dynfmt2,
/// so the function returns an error rather than silently leaving the raw
/// `%(chabter_nummer)s` text in the path.
#[test]
fn typo_in_placeholder_name_returns_error() -> Result<()> {
    let template = Path::new("typo_%(chabter_nummer)s_in_template");
    let err = format_chapter_number(template, 5, 0).unwrap_err();
    assert!(
        err.to_string().contains("chabter_nummer"),
        "error message should name the unknown key, got: {err}"
    );
    Ok(())
}

// ── %(u)s underscores substitution ───────────────────────────────────────────

/// `%(u)s` in the filename is replaced with the requested number of underscores.
#[test]
fn underscores_placeholder_is_substituted() -> Result<()> {
    let template = Path::new("/output/chapter_%(chapter_number)s%(u)s.c");
    let result = format_chapter_number(template, 3, 2).unwrap();
    assert_eq!(result, PathBuf::from("/output/chapter_3__.c"));
    Ok(())
}

/// Zero underscores substitutes an empty string for `%(u)s`.
#[test]
fn zero_underscores_substitutes_empty_string() -> Result<()> {
    let template = Path::new("/output/chapter_%(chapter_number)s%(u)s.c");
    let result = format_chapter_number(template, 3, 0).unwrap();
    assert_eq!(result, PathBuf::from("/output/chapter_3.c"));
    Ok(())
}
