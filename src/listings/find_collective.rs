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
// File: find_collective.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/listings/find_collective.rs
// Author: Dr. Christoph Pospiech

use crate::chapters::ChapterMap;
use crate::cmdline::CliArgs;
use crate::config::{Config, Templates};
use crate::listings::find_collective_chapter;
use anyhow::Result;
use std::path::PathBuf;

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build a minimal `ChapterMap` from a slice of `(chapter_number, page, title)` tuples.
fn make_chapter_map(entries: &[(u32, usize, &str)]) -> ChapterMap {
    entries
        .iter()
        .map(|&(ch, page, title)| (ch, (page, title.to_string())))
        .collect()
}

/// Build a `Config` whose `chapters` list is set to `chapters`.
/// All other fields are filled with neutral/dummy values.
fn make_config(chapters: Vec<u32>) -> Config {
    Config {
        mpi_pdf: PathBuf::from("dummy.pdf"),
        output_file: PathBuf::from("dummy_output.c"),
        templates: Templates {
            header: String::new(),
            ch_init: String::new(),
            ch_body: String::new(),
            bar_init: String::new(),
            bar_body: String::new(),
            footer: String::new(),
        },
        chapters,
        excludes: vec![],
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

/// When `config.chapters` is empty the function should return `Ok(None)`.
#[test]
fn test_empty_chapters_returns_none() -> Result<()> {
    let chapter_map = make_chapter_map(&[(1, 1, "Introduction"), (2, 10, "Datatypes")]);
    let config = make_config(vec![]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, None);
    Ok(())
}

/// When none of the configured chapters has "collective" in its title the
/// function should return `Ok(None)`.
#[test]
fn test_no_collective_title_returns_none() -> Result<()> {
    let chapter_map = make_chapter_map(&[
        (1, 1, "Introduction"),
        (2, 5, "Point-to-Point Communication"),
        (3, 20, "Datatypes"),
    ]);
    let config = make_config(vec![1, 2, 3]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, None);
    Ok(())
}

/// When exactly one chapter title contains "collective" (lower-case) the
/// function should return `Ok(Some(chapter_number))`.
#[test]
fn test_single_collective_chapter_returns_some() -> Result<()> {
    let chapter_map = make_chapter_map(&[
        (1, 1, "Introduction"),
        (2, 5, "Point-to-Point Communication"),
        (3, 20, "collective operations"),
    ]);
    let config = make_config(vec![1, 2, 3]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, Some(3));
    Ok(())
}

/// The match must be case-insensitive: "Collective Communication" should be
/// found when the stored title uses title-case.
#[test]
fn test_collective_title_case_insensitive_title_case() -> Result<()> {
    let chapter_map =
        make_chapter_map(&[(1, 1, "Introduction"), (4, 30, "Collective Communication")]);
    let config = make_config(vec![1, 4]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, Some(4));
    Ok(())
}

/// The match must be case-insensitive: an all-uppercase title "COLLECTIVE OPS"
/// should be detected.
#[test]
fn test_collective_title_case_insensitive_uppercase() -> Result<()> {
    let chapter_map = make_chapter_map(&[(1, 1, "Introduction"), (5, 40, "COLLECTIVE OPS")]);
    let config = make_config(vec![1, 5]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, Some(5));
    Ok(())
}

/// The word "collective" embedded within a longer word (e.g. "non-collective")
/// still triggers the match because the check uses `contains`.
#[test]
fn test_collective_embedded_in_longer_word() -> Result<()> {
    let chapter_map = make_chapter_map(&[(1, 1, "Non-Collective Routines")]);
    let config = make_config(vec![1]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    assert_eq!(result, Some(1));
    Ok(())
}

/// With `verbose = true` the function must produce the same result as with
/// `verbose = false` (the flag must not alter return values).
#[test]
fn test_verbose_flag_does_not_affect_result() -> Result<()> {
    let chapter_map =
        make_chapter_map(&[(1, 1, "Introduction"), (3, 20, "Collective Communication")]);
    let config = make_config(vec![1, 3]);

    let quiet = find_collective_chapter(&chapter_map, &quiet_args(), &config)?;
    let loud = find_collective_chapter(&chapter_map, &verbose_args(), &config)?;

    assert_eq!(quiet, Some(3));
    assert_eq!(loud, Some(3));
    Ok(())
}

/// When two chapters both contain "collective" in their title the function
/// must return an error describing both chapters.
#[test]
fn test_two_collective_chapters_returns_error() -> Result<()> {
    let chapter_map = make_chapter_map(&[
        (1, 1, "Introduction"),
        (3, 20, "Collective Communication"),
        (5, 40, "Advanced Collective Operations"),
    ]);
    let config = make_config(vec![1, 3, 5]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config);
    assert!(
        result.is_err(),
        "expected an error for two collective chapters"
    );
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("Multiple collective chapters found"),
        "unexpected error message: {msg}"
    );
    Ok(())
}

/// The error message for multiple collective chapters must name both chapter
/// numbers so the caller can diagnose the conflict.
#[test]
fn test_two_collective_chapters_error_names_both_chapters() -> Result<()> {
    let chapter_map = make_chapter_map(&[(2, 5, "collective routines"), (7, 60, "Collective I/O")]);
    let config = make_config(vec![2, 7]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("chapter 2"), "expected 'chapter 2' in: {msg}");
    assert!(msg.contains("chapter 7"), "expected 'chapter 7' in: {msg}");
    Ok(())
}

/// Requesting a chapter that is not present in the `ChapterMap` must propagate
/// an error (coming from `get_chapter_title`).
#[test]
fn test_chapter_missing_from_map_returns_error() -> Result<()> {
    let chapter_map = make_chapter_map(&[(1, 1, "Introduction")]);
    // chapter 99 is in the config but not in the map
    let config = make_config(vec![1, 99]);
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config);
    assert!(result.is_err(), "expected an error for a missing chapter");
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("99"),
        "expected the missing chapter number in: {msg}"
    );
    Ok(())
}

/// A config whose chapter list contains the same chapter number twice should
/// return an error because the function iterates `config.chapters` linearly,
/// so seeing the same "collective" chapter twice triggers the
/// "multiple collective chapters" error.
#[test]
fn test_duplicate_chapter_entry_in_config_returns_error() -> Result<()> {
    // The function iterates `config.chapters` linearly; seeing the same
    // collective chapter twice triggers the "multiple collective chapters" bail.
    let chapter_map = make_chapter_map(&[(3, 20, "Collective Communication")]);
    let config = make_config(vec![3, 3]); // chapter 3 listed twice
    let result = find_collective_chapter(&chapter_map, &quiet_args(), &config);
    assert!(
        result.is_err(),
        "duplicate collective chapter entries should produce an error"
    );
    Ok(())
}
