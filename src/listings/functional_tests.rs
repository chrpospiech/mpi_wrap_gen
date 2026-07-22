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
// File: functional_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/listings/functional_tests.rs
// Author: Dr. Christoph Pospiech

use crate::cmdline::CliArgs;
use crate::config::{Config, Templates};
use crate::listings::extract_and_generate_listings;
use anyhow::Result;
use lopdf::Document;
use std::path::PathBuf;

// ── Helpers ───────────────────────────────────────────────────────────────────

/// test_outline.pdf structure (pages 1–5):
///   Chapter 1 – "Introduction"                 (page 1) – no MPI calls
///   Chapter 2 – "Terms and Conventions"        (page 2) – no MPI calls
///   Chapter 3 – "Point-to-Point Communication" (page 3) – Send, Isend, Recv  (3 calls)
///   Chapter 4 – "Datatypes"                    (page 4) – no MPI calls
///   Chapter 5 – "Collective Communication"     (page 5) – Barrier, Bcast, Scatter, Scatterv (4 calls)
fn outline_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("test_outline.pdf")
}

/// Build a `Config` with simple, recognisable templates for the given chapter list.
fn make_config(chapters: Vec<u32>) -> Config {
    Config {
        mpi_pdf: PathBuf::from("dummy.pdf"),
        output_file: PathBuf::from("dummy_output.c"),
        templates: Templates {
            header: "HEADER\n".to_string(),
            ch_init: "INIT %(chapter_number)d %(chapter_title)s %(mpi_call_name)s\n".to_string(),
            ch_decl: String::new(),
            ch_load_imb: String::new(),
            ch_msize: String::new(),
            ch_body: "BODY %(mpi_call_name)s\n".to_string(),
            bar_init: "BAR_INIT %(chapter_number)d %(chapter_title)s %(mpi_call_name)s\n"
                .to_string(),
            bar_body: "BAR_BODY %(mpi_call_name)s\n".to_string(),
            footer: "FOOTER\n".to_string(),
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

fn single_output(outputs: Vec<(PathBuf, String)>) -> String {
    outputs
        .into_iter()
        .next()
        .expect("expected at least one output entry")
        .1
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// When a chapter with MPI calls is requested the output must still begin with
/// the header and end with the footer.
#[test]
fn test_chapter_with_mpi_calls_has_header_and_footer() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.starts_with("HEADER\n"),
        "output should start with header, got: {output:?}"
    );
    assert!(
        output.ends_with("FOOTER\n"),
        "output should end with footer, got: {output:?}"
    );
    Ok(())
}

/// Chapter 3 contains Send, Isend, and Recv – all three names must appear in
/// the generated output.
#[test]
fn test_chapter_3_output_contains_all_mpi_call_names() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("Send"),
        "output should contain MPI_Send, got: {output:?}"
    );
    assert!(
        output.contains("Isend"),
        "output should contain MPI_Isend, got: {output:?}"
    );
    assert!(
        output.contains("Recv"),
        "output should contain MPI_Recv, got: {output:?}"
    );
    Ok(())
}

/// The ch_init template must be instantiated exactly once for chapter 3,
/// using the first MPI call found in that chapter (Send).
#[test]
fn test_ch_init_appears_exactly_once_for_chapter_3() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let init_count = output.lines().filter(|l| l.starts_with("INIT")).count();
    assert_eq!(
        init_count, 1,
        "ch_init should appear exactly once for chapter 3, got: {output:?}"
    );
    Ok(())
}

/// The ch_init line for chapter 3 must contain the chapter number, title, and
/// the name of the first MPI call (Send).
#[test]
fn test_ch_init_content_for_chapter_3() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("INIT 3 Point-to-Point Communication Send\n"),
        "ch_init line should contain chapter number, title, and first call name, got: {output:?}"
    );
    Ok(())
}

/// Chapter 3 has exactly 3 MPI calls, so the ch_body template must be
/// instantiated 3 times.
#[test]
fn test_ch_body_count_matches_chapter_3_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        body_count, 3,
        "chapter 3 should produce 3 BODY entries (Send, Isend, Recv), got: {output:?}"
    );
    Ok(())
}

/// The chapter title "Point-to-Point Communication" must be substituted into
/// the ch_init output for chapter 3.
#[test]
fn test_chapter_title_substituted_in_output() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("Point-to-Point Communication"),
        "output should contain the chapter title, got: {output:?}"
    );
    Ok(())
}

/// The chapter number must be substituted into the ch_init output for chapter 3.
#[test]
fn test_chapter_number_substituted_in_output() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("INIT 3 "),
        "ch_init line should contain the chapter number 3, got: {output:?}"
    );
    Ok(())
}

/// Chapter 5 (Collective Communication) has exactly 4 MPI calls, so the
/// ch_body template must be instantiated 4 times.
#[test]
fn test_ch_body_count_matches_chapter_5_mpi_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        body_count, 4,
        "chapter 5 should produce 4 BODY entries (Barrier, Bcast, Scatter, Scatterv), got: {output:?}"
    );
    Ok(())
}

/// Chapter 5 must produce output containing the names of all four collective
/// communication functions.
#[test]
fn test_chapter_5_contains_collective_call_names() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("Barrier"),
        "output should contain MPI_Barrier, got: {output:?}"
    );
    assert!(
        output.contains("Bcast"),
        "output should contain MPI_Bcast, got: {output:?}"
    );
    assert!(
        output.contains("Scatter"),
        "output should contain MPI_Scatter, got: {output:?}"
    );
    assert!(
        output.contains("Scatterv"),
        "output should contain MPI_Scatterv, got: {output:?}"
    );
    Ok(())
}

/// When chapters 3 and 5 are both requested the output must contain exactly
/// 2 INIT lines and 7 BODY lines (3 + 4).
#[test]
fn test_multiple_chapters_combined_call_counts() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3, 5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let init_count = output.lines().filter(|l| l.starts_with("INIT")).count();
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        init_count, 2,
        "should have 2 INIT entries for 2 chapters with MPI calls, got: {output:?}"
    );
    assert_eq!(
        body_count, 7,
        "should have 7 BODY entries total (3 from ch3 + 4 from ch5), got: {output:?}"
    );
    Ok(())
}

/// When chapters 3 and 5 are both requested the chapter 3 content must appear
/// before chapter 5 content in the output.
#[test]
fn test_multiple_chapters_appear_in_requested_order() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3, 5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let pos_ch3 = output
        .find("Point-to-Point Communication")
        .expect("chapter 3 title should appear in output");
    let pos_ch5 = output
        .find("Collective Communication")
        .expect("chapter 5 title should appear in output");
    assert!(
        pos_ch3 < pos_ch5,
        "chapter 3 content should precede chapter 5 content in the output"
    );
    Ok(())
}

/// When a chapter without MPI calls (chapter 1) is listed alongside a chapter
/// that has calls (chapter 3), only chapter 3 must contribute INIT and BODY
/// entries.
#[test]
fn test_chapter_without_mpi_calls_skipped_in_mixed_list() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![1, 3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let init_count = output.lines().filter(|l| l.starts_with("INIT")).count();
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        init_count, 1,
        "only chapter 3 should contribute an INIT entry, got: {output:?}"
    );
    assert_eq!(
        body_count, 3,
        "only chapter 3's 3 calls should produce BODY entries, got: {output:?}"
    );
    Ok(())
}

/// The output sections must appear in the correct order:
/// HEADER → ch_init → ch_body × n → FOOTER.
#[test]
fn test_output_section_order() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let header_pos = output.find("HEADER").expect("HEADER must be present");
    let init_pos = output.find("INIT").expect("INIT must be present");
    let body_pos = output.find("BODY").expect("BODY must be present");
    let footer_pos = output.find("FOOTER").expect("FOOTER must be present");
    assert!(header_pos < init_pos, "HEADER must come before INIT");
    assert!(init_pos < body_pos, "INIT must come before first BODY");
    assert!(
        body_pos < footer_pos,
        "BODY entries must come before FOOTER"
    );
    Ok(())
}

/// Running with the verbose flag must produce identical output to quiet mode –
/// verbose output goes to stdout only and must not alter the returned string.
#[test]
fn test_verbose_mode_produces_identical_output_to_quiet_mode() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let quiet_output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let verbose_output = single_output(extract_and_generate_listings(
        &doc,
        &verbose_args(),
        &config,
    )?);
    assert_eq!(
        quiet_output, verbose_output,
        "verbose mode should not change the returned output string"
    );
    Ok(())
}

// ── Barrier / Collective-chapter tests ────────────────────────────────────────

/// Chapter 5 ("Collective Communication") must cause a BAR_INIT entry to appear
/// in the output because its title matches the "collective" keyword.
#[test]
fn test_collective_chapter_produces_bar_init() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("BAR_INIT"),
        "output for collective chapter 5 should contain BAR_INIT, got: {output:?}"
    );
    Ok(())
}

/// Chapter 5 must cause BAR_BODY entries to appear in the output.
#[test]
fn test_collective_chapter_produces_bar_body() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("BAR_BODY"),
        "output for collective chapter 5 should contain BAR_BODY entries, got: {output:?}"
    );
    Ok(())
}

/// Chapter 5 has 4 collective calls, so the BAR_BODY template must be instantiated
/// exactly 4 times (one per barrier-named call).
#[test]
fn test_collective_chapter_bar_body_count_matches_call_count() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let bar_body_count = output.lines().filter(|l| l.starts_with("BAR_BODY")).count();
    assert_eq!(
        bar_body_count, 4,
        "chapter 5 should produce 4 BAR_BODY entries, got: {output:?}"
    );
    Ok(())
}

/// BAR_INIT must appear exactly once, even when chapter 5 is the only chapter.
#[test]
fn test_bar_init_appears_exactly_once_for_chapter_5() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let bar_init_count = output.lines().filter(|l| l.starts_with("BAR_INIT")).count();
    assert_eq!(
        bar_init_count, 1,
        "BAR_INIT should appear exactly once for collective chapter 5, got: {output:?}"
    );
    Ok(())
}

/// The BAR_INIT line for chapter 5 must contain the chapter number, title, and
/// the name of the first barrier call ("Barrier_barrier").
#[test]
fn test_bar_init_content_for_chapter_5() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("BAR_INIT 5 Collective Communication Barrier_barrier\n"),
        "BAR_INIT line should contain chapter number, title, and first barrier call name, got: {output:?}"
    );
    Ok(())
}

/// Every BAR_BODY line must reference a call name that ends with "_barrier".
#[test]
fn test_bar_body_names_all_have_barrier_suffix() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    for line in output.lines().filter(|l| l.starts_with("BAR_BODY")) {
        assert!(
            line.contains("_barrier"),
            "BAR_BODY line should reference a '_barrier'-suffixed call name, got: {line:?}"
        );
    }
    Ok(())
}

/// A non-collective chapter (chapter 3, "Point-to-Point Communication") must
/// not produce any BAR_INIT or BAR_BODY entries.
#[test]
fn test_non_collective_chapter_produces_no_bar_output() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        !output.contains("BAR_INIT"),
        "non-collective chapter 3 should not produce BAR_INIT, got: {output:?}"
    );
    assert!(
        !output.contains("BAR_BODY"),
        "non-collective chapter 3 should not produce BAR_BODY, got: {output:?}"
    );
    Ok(())
}

/// When chapters 3 and 5 are combined, there must be exactly 1 BAR_INIT and
/// 4 BAR_BODY entries (only chapter 5 is collective).
#[test]
fn test_multiple_chapters_bar_section_counts() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![3, 5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let bar_init_count = output.lines().filter(|l| l.starts_with("BAR_INIT")).count();
    let bar_body_count = output.lines().filter(|l| l.starts_with("BAR_BODY")).count();
    assert_eq!(
        bar_init_count, 1,
        "combined chapters 3+5 should have 1 BAR_INIT entry, got: {output:?}"
    );
    assert_eq!(
        bar_body_count, 4,
        "combined chapters 3+5 should have 4 BAR_BODY entries (all from ch5), got: {output:?}"
    );
    Ok(())
}

/// Non-empty ch_decl/ch_msize templates must be emitted for all selected calls,
/// and ch_load_imb must only be appended for the collective chapter.
#[test]
fn test_decl_msize_and_load_imb_behavior_for_collective_chapter() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let mut config = make_config(vec![3, 5]);
    config.templates.ch_decl = "DECL %(chapter_number)d %(mpi_call_name)s\n".to_string();
    config.templates.ch_load_imb = "LOAD_IMB %(chapter_number)d %(mpi_call_name)s\n".to_string();
    config.templates.ch_msize = "MSIZE %(chapter_number)d %(mpi_call_name)s\n".to_string();

    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let decl_count = output.lines().filter(|l| l.starts_with("DECL ")).count();
    let msize_count = output.lines().filter(|l| l.starts_with("MSIZE ")).count();
    let load_imb_lines: Vec<&str> = output
        .lines()
        .filter(|l| l.starts_with("LOAD_IMB "))
        .collect();

    assert_eq!(
        decl_count, 7,
        "chapters 3+5 should emit DECL for all 7 calls, got: {output:?}"
    );
    assert_eq!(
        msize_count, 7,
        "chapters 3+5 should emit MSIZE for all 7 calls, got: {output:?}"
    );
    assert_eq!(
        load_imb_lines.len(),
        4,
        "only collective chapter 5 should emit LOAD_IMB (4 calls), got: {load_imb_lines:?}"
    );
    assert!(
        load_imb_lines.iter().all(|l| l.starts_with("LOAD_IMB 5 ")),
        "LOAD_IMB lines must only belong to chapter 5, got: {load_imb_lines:?}"
    );

    let decl_barrier_pos = output
        .find("DECL 5 Barrier")
        .expect("collective chapter should emit DECL for Barrier");
    let load_imb_barrier_pos = output
        .find("LOAD_IMB 5 Barrier")
        .expect("collective chapter should emit LOAD_IMB for Barrier");
    let msize_barrier_pos = output
        .find("MSIZE 5 Barrier")
        .expect("collective chapter should emit MSIZE for Barrier");
    assert!(
        decl_barrier_pos < load_imb_barrier_pos,
        "DECL must come before LOAD_IMB for collective chapter entries"
    );
    assert!(
        load_imb_barrier_pos < msize_barrier_pos,
        "LOAD_IMB must come before MSIZE for collective chapter entries"
    );
    Ok(())
}

// ── Excludes tests ────────────────────────────────────────────────────────────

/// A helper that builds a Config with an excludes list.
fn make_config_with_excludes(chapters: Vec<u32>, excludes: Vec<&str>) -> Config {
    let mut config = make_config(chapters);
    config.excludes = excludes.into_iter().map(str::to_string).collect();
    config
}

/// Excluding one call from chapter 3 must reduce the BODY count by one.
#[test]
fn test_exclude_one_call_reduces_body_count() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["Isend"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        body_count, 2,
        "excluding Isend from chapter 3 should leave 2 BODY entries (Send, Recv), got: {output:?}"
    );
    Ok(())
}

/// The excluded call name must not appear in any BODY line.
#[test]
fn test_excluded_call_name_absent_from_output() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["Isend"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_lines: Vec<&str> = output.lines().filter(|l| l.starts_with("BODY")).collect();
    assert!(
        !body_lines.iter().any(|l| l.contains("Isend")),
        "excluded call Isend must not appear in any BODY line, got: {body_lines:?}"
    );
    Ok(())
}

/// Non-excluded calls must still appear when one call is excluded.
#[test]
fn test_non_excluded_calls_still_present() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["Isend"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    assert!(
        output.contains("BODY Send\n"),
        "Send should still appear in output, got: {output:?}"
    );
    assert!(
        output.contains("BODY Recv\n"),
        "Recv should still appear in output, got: {output:?}"
    );
    Ok(())
}

/// Excluding multiple calls at once must remove all of them.
#[test]
fn test_exclude_multiple_calls() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["Send", "Isend"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        body_count, 1,
        "excluding Send and Isend should leave 1 BODY entry (Recv), got: {output:?}"
    );
    assert!(
        output.contains("BODY Recv\n"),
        "Recv should be the only remaining BODY entry, got: {output:?}"
    );
    Ok(())
}

/// Excluding all calls in a chapter must leave neither INIT nor BODY entries
/// for that chapter because the chapter is effectively skipped.
#[test]
fn test_exclude_all_calls_leaves_no_entries() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["Send", "Isend", "Recv"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    let init_count = output.lines().filter(|l| l.starts_with("INIT")).count();
    assert_eq!(
        body_count, 0,
        "excluding all calls should produce 0 BODY entries, got: {output:?}"
    );
    assert_eq!(
        init_count, 0,
        "excluding all calls should produce 0 INIT entries, got: {output:?}"
    );
    Ok(())
}

/// A name in the excludes list that does not appear in the chapter must have
/// no effect – all calls still appear in the output.
#[test]
fn test_exclude_nonexistent_call_has_no_effect() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["MPI_Nonexistent"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    assert_eq!(
        body_count, 3,
        "an unmatched exclude should leave all 3 BODY entries intact, got: {output:?}"
    );
    Ok(())
}

/// An empty excludes list must produce the same output as make_config (no excludes).
#[test]
fn test_empty_excludes_list_has_no_effect() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config_no_excludes = make_config(vec![3]);
    let config_empty_excludes = make_config_with_excludes(vec![3], vec![]);
    let output_no = single_output(extract_and_generate_listings(
        &doc,
        &quiet_args(),
        &config_no_excludes,
    )?);
    let output_empty = single_output(extract_and_generate_listings(
        &doc,
        &quiet_args(),
        &config_empty_excludes,
    )?);
    assert_eq!(
        output_no, output_empty,
        "empty excludes list should produce identical output to no excludes"
    );
    Ok(())
}

/// Excludes must work across multiple chapters: excluding Send from chapter 3
/// and Bcast from chapter 5 must reduce each chapter's BODY count by one.
#[test]
fn test_excludes_apply_across_multiple_chapters() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3, 5], vec!["Send", "Bcast"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_count = output.lines().filter(|l| l.starts_with("BODY")).count();
    // chapter 3: 3 - 1 = 2; chapter 5: 4 - 1 = 3  → total 5
    assert_eq!(
        body_count, 5,
        "excluding Send (ch3) and Bcast (ch5) should leave 5 BODY entries total, got: {output:?}"
    );
    Ok(())
}

/// Excludes should match case-insensitively and tolerate an optional MPI_ prefix.
#[test]
fn test_excludes_match_case_insensitively_with_optional_prefix() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![3], vec!["mpi_isend", "MPI_recv"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_lines: Vec<&str> = output.lines().filter(|l| l.starts_with("BODY")).collect();
    assert_eq!(
        body_lines,
        vec!["BODY Send"],
        "normalized excludes should leave only Send, got: {body_lines:?}"
    );
    Ok(())
}

/// Excluding a collective call should also remove its generated barrier wrapper.
#[test]
fn test_excludes_apply_to_barrier_section() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config_with_excludes(vec![5], vec!["bcast"]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);
    let body_lines: Vec<&str> = output.lines().filter(|l| l.starts_with("BODY")).collect();
    let bar_body_lines: Vec<&str> = output
        .lines()
        .filter(|l| l.starts_with("BAR_BODY"))
        .collect();
    assert!(
        !body_lines.iter().any(|line| line.contains("Bcast")),
        "excluded collective call must not appear in BODY lines, got: {body_lines:?}"
    );
    assert!(
        !bar_body_lines
            .iter()
            .any(|line| line.contains("Bcast_barrier")),
        "excluded collective call must not appear in BAR_BODY lines, got: {bar_body_lines:?}"
    );
    assert_eq!(
        body_lines.len(),
        3,
        "excluding Bcast should leave 3 BODY entries, got: {body_lines:?}"
    );
    assert_eq!(
        bar_body_lines.len(),
        3,
        "excluding Bcast should leave 3 BAR_BODY entries, got: {bar_body_lines:?}"
    );
    Ok(())
}

/// Verbose mode with an excluded call must still return the same output string
/// as quiet mode (the skip message goes to stdout only).
#[test]
fn test_verbose_excludes_produces_identical_output_to_quiet() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config_quiet = make_config_with_excludes(vec![3], vec!["Isend"]);
    let config_verbose = make_config_with_excludes(vec![3], vec!["Isend"]);
    let quiet_output = single_output(extract_and_generate_listings(
        &doc,
        &quiet_args(),
        &config_quiet,
    )?);
    let verbose_output = single_output(extract_and_generate_listings(
        &doc,
        &verbose_args(),
        &config_verbose,
    )?);
    assert_eq!(
        quiet_output, verbose_output,
        "verbose mode with excludes should not change the returned output string"
    );
    Ok(())
}

/// The output for chapter 5 must appear in the correct order:
/// HEADER → INIT → BODY × 4 → BAR_INIT → BAR_BODY × 4 → FOOTER.
#[test]
fn test_output_section_order_with_collective_chapter() -> Result<()> {
    let doc = Document::load(outline_pdf_path())?;
    let config = make_config(vec![5]);
    let output = single_output(extract_and_generate_listings(&doc, &quiet_args(), &config)?);

    let header_pos = output.find("HEADER").expect("HEADER must be present");
    let init_pos = output.find("INIT").expect("INIT must be present");
    let body_pos = output.find("BODY").expect("BODY must be present");
    let bar_init_pos = output.find("BAR_INIT").expect("BAR_INIT must be present");
    let bar_body_pos = output.find("BAR_BODY").expect("BAR_BODY must be present");
    let footer_pos = output.find("FOOTER").expect("FOOTER must be present");
    assert!(header_pos < init_pos, "HEADER must come before INIT");
    assert!(init_pos < body_pos, "INIT must come before first BODY");
    assert!(
        body_pos < bar_init_pos,
        "BODY entries must come before BAR_INIT"
    );
    assert!(
        bar_init_pos < bar_body_pos,
        "BAR_INIT must come before first BAR_BODY"
    );
    assert!(
        bar_body_pos < footer_pos,
        "BAR_BODY entries must come before FOOTER"
    );
    Ok(())
}
