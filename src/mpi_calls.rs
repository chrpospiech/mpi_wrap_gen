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
// File: mpi_calls.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls.rs
// Author: Dr. Christoph Pospiech

use crate::chapters::{get_chapter_page, ChapterMap};
use anyhow::{Context, Result};
use lopdf::Document;
use regex::Regex;

#[cfg(test)]
mod mpi_call_xmp_tests;

#[cfg(test)]
mod wait_some_tests;

#[cfg(test)]
mod test_outline_tests;

#[cfg(test)]
mod create_barrier_names_tests;

#[cfg(test)]
mod mpi_calls_for_chapter_tests;

pub type MpiCallInfo = (String, Vec<String>);

/// Extracts MPI call information from the PDF document and returns a vector of MPI call names
/// and call parameters. The MPI calls are identified by searching for text elements that match
/// the pattern "int MPI_[A-Z]\w+\(.*\)" in the PDF content streams. The closing bracket may
/// be on the same line or on a subsequent line, so the function handles both cases.
///
/// Note: lopdf::extract_text often strips spaces, so the regex handles both
/// "int MPI_Send(" and "intMPI_Send(" patterns.
///
/// The MPI call name is returned without the "MPI_" prefix, and the parameters are
/// returned as a vector of strings.
///
/// # Arguments
/// * `doc` - A reference to the lopdf PDF document from which to extract MPI call information.
/// * `start_page` - The 1-based index of the page from which to start searching for MPI calls.
/// * `end_page` - The 1-based (exclusive) index of the page at which to stop searching for MPI calls.
///
/// # Returns
/// A Result containing a vector of [`MpiCallInfo`], where each entry is a tuple of
/// `(call_name, parameters)`: the MPI call name (without the "MPI_" prefix) and a vector
/// of its parameter strings. If an error occurs during PDF processing, the Result
/// will contain an error message.
fn mpi_calls_for_page_range(
    doc: &Document,
    start_page: u32,
    end_page: u32,
) -> Result<Vec<MpiCallInfo>> {
    let mut mpi_calls = Vec::new();
    // Use (?s) so . matches newlines; \s* to handle missing/present spaces
    let re = Regex::new(r"(?s)int\s*MPI_([A-Z]\w+)\s*\((.*?)\)")?;

    // Maximum number of bytes to carry over from the tail of the previous page
    // so that function signatures spanning a page boundary are still matched
    // without holding all page text in memory at once.
    const CARRY_OVER_SIZE: usize = 2048;

    // `carry_over` holds the tail of the previous page's cleaned text.  It is
    // prepended to the current page's text so that the regex can match
    // signatures whose opening "int MPI_…(" is on the previous page and whose
    // closing ")" is on the current page.
    let mut carry_over = String::new();

    for page_num in start_page..end_page {
        let text = doc.extract_text(&[page_num]).unwrap_or_default();

        // Remove line numbers that appear at start/end of lines (from PDF margins)
        let cleaned = clean_extracted_text(&text);

        // Append a newline to form the complete page text; reuse it for both
        // the search window and the next iteration's carry-over to avoid a
        // redundant allocation.
        let page_text = format!("{}\n", cleaned);

        // Build the search window: tail of the previous page + current page text.
        let window = format!("{}{}", carry_over, page_text);
        let carry_len = carry_over.len();

        // Collect matches. Skip any match whose full extent lies within the
        // carry-over region — those signatures were already yielded when the
        // previous page was processed.
        for cap in re.captures_iter(&window) {
            let m = cap.get(0).unwrap();
            if m.end() <= carry_len {
                continue;
            }
            // Skip typedef declarations (e.g., "typedef int MPI_Copy_function(...)").
            // lopdf may strip spaces, so "typedef int MPI_" can appear as
            // "typedefint MPI_", "typedef intMPI_", or "typedefintMPI_".
            // Trimming the text preceding the match and checking for the
            // "typedef" suffix covers all variants.
            if window[..m.start()].trim_end().ends_with("typedef") {
                continue;
            }
            let call_name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let params_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let params = parse_parameters(params_str);
            mpi_calls.push((call_name, params));
        }

        // Update carry-over to the tail of the current page text so the next
        // iteration can detect cross-boundary signatures.
        if page_text.len() > CARRY_OVER_SIZE {
            let mut split = page_text.len() - CARRY_OVER_SIZE;
            // Advance to the next UTF-8 character boundary if needed.
            // The bounds check guards against split advancing past the end.
            while split < page_text.len() && !page_text.is_char_boundary(split) {
                split += 1;
            }
            carry_over = page_text[split..].to_string();
        } else {
            carry_over = page_text;
        }
    }

    Ok(mpi_calls)
}

/// Extracts MPI call information from the PDF document for a given chapter
/// and returns a vector of MPI call names and call parameters.
/// The MPI calls are identified by searching for text elements that match
/// the pattern "int MPI_[A-Z]\w+\(.*\)" in the PDF content streams. The closing bracket may
/// be on the same line or on a subsequent line, so the function handles both cases.
///
/// The MPI call name is returned without the "MPI_" prefix, and the parameters are
/// returned as a vector of strings.
///
/// The function calls `mpi_calls_for_page_range` to perform the extraction, and it is
/// intended to be used for extracting MPI calls from a specific chapter of the PDF document,
/// where the chapter is defined by its start and end pages.
///
/// # Arguments
/// * `doc` - A reference to the lopdf PDF document from which to extract MPI call information.
/// * `chapter_indices` - ChapterMap to identify the start and end pages for each chapter.
/// * `chapter` - the chapter number (1-based) for which to extract MPI calls.
///
/// # Returns
/// A Result containing a vector of [`MpiCallInfo`], where each entry is a tuple of
/// `(call_name, parameters)`: the MPI call name (without the "MPI_" prefix) and a vector
/// of its parameter strings. If an error occurs during PDF processing, the Result
/// will contain an error message.
///
pub fn mpi_calls_for_chapter(
    doc: &Document,
    chapter_indices: &ChapterMap,
    chapter: u32,
) -> Result<Vec<MpiCallInfo>> {
    let ch_start = get_chapter_page(chapter_indices, chapter)
        .with_context(|| format!("Chapter {} not found in PDF document.", chapter))?;
    let ch_end = chapter_indices
        .keys()
        .filter_map(|&candidate_chapter| get_chapter_page(chapter_indices, candidate_chapter).ok())
        .filter(|&page| page > ch_start)
        .min()
        .unwrap_or_else(|| (doc.get_pages().len() + 1) as u32); // If no later chapter is found, use end of document
    mpi_calls_for_page_range(doc, ch_start, ch_end)
}

/// Calls mpi_calls_for_chapter but maps the result of type `Vec<MpiCallInfo>` to a new
/// `Vec<MpiCallInfo>` with the first tuple entry being the concatenation of
/// the original tuple entry and the string "_barrier".
///
/// # Arguments
/// * `doc` - A reference to the lopdf PDF document from which to extract MPI call information.
/// * `chapter_indices` - ChapterMap to identify the start and end pages for each chapter.
/// * `chapter` - the chapter number (1-based) for which to extract MPI calls.
///
/// # Returns
/// A Result containing a vector of [`MpiCallInfo`], where each entry is a tuple of
/// `(call_name, parameters)`: the MPI call name (without the "MPI_" prefix) and a vector
/// of its parameter strings. If an error occurs during PDF processing, the Result
/// will contain an error message.
pub fn create_barrier_names(
    doc: &Document,
    chapter_indices: &ChapterMap,
    chapter: u32,
) -> Result<Vec<MpiCallInfo>> {
    let barrier_calls = mpi_calls_for_chapter(doc, chapter_indices, chapter)?
        .into_iter()
        .map(|(call_name, params)| (format!("{}_barrier", call_name), params))
        .collect::<Vec<MpiCallInfo>>();
    Ok(barrier_calls)
}

/// Filters a slice of MPI calls using a caller-supplied exclusion predicate.
/// The predicate receives the call name and returns `true` if the call should be excluded.
///
/// # Arguments
/// * `calls` - A slice of [`MpiCallInfo`] to filter (original slice remains usable after the call).
/// * `exclude_pred` - A closure `Fn(&str) -> bool` that returns `true` for calls to exclude.
///
/// # Returns
/// A new `Vec<MpiCallInfo>` containing only the calls for which `exclude_pred` returns `false`.
pub fn filter_excluded_mpi_calls<F>(calls: &[MpiCallInfo], exclude_pred: F) -> Vec<MpiCallInfo>
where
    F: Fn(&str) -> bool,
{
    calls
        .iter()
        .filter(|(call_name, _)| !exclude_pred(call_name))
        .cloned()
        .collect()
}

/// Parse the parameter string from an MPI call signature.
/// Handles missing spaces (e.g., "constvoid*buf" -> "const void* buf")
/// by inserting spaces before C type keywords and after pointer operators,
/// then splitting on commas and normalizing whitespace.
fn parse_parameters(params_str: &str) -> Vec<String> {
    params_str
        .split(',')
        .map(|p| normalize_whitespace(&insert_missing_spaces(p)))
        .filter(|p| !p.is_empty())
        .collect()
}

/// Insert spaces that may be missing due to PDF text extraction stripping
/// whitespace. Handles pointer operators and common C type qualifiers,
/// type modifiers and C data types.
///
/// The list of C type qualifiers are `const`, `volatile` and `restrict`.
/// The list of C type modifiers are `unsigned`, `signed`, `short` and `long`.
/// The list of C data types are `void*`, `void`, `int`, `char`, `float` and `double`.
/// The latter list needs to be extended by the MPI-specific data types, such as
/// `MPI_Comm`, `MPI_Datatype`, `MPI_Request`, `MPI_Status`, `MPI_File`,
/// `MPI_Aint`, `MPI_Offset`, `MPI_Count`, `MPI_Win`, `MPI_Group`, `MPI_Op`, `MPI_Info`.
///
/// For each of these categories, the function will attempt to insert spaces
/// where they are missing. This is done by creating a regular expression
/// by joining the keywords with the separator '|' and enclosing them in
/// parentheses. This regular expression is then extended with (\w+).
/// For the C type qualifiers for example, the resulting regular expression
/// would be `(const|volatile|restrict)(\w+)`.
/// If the regular expression matches, a space is inserted between the two groups.
///
/// Per the MPI standard, `*` is placed next to the variable name rather than
/// the type, so a trailing space is inserted before any remaining `*` that
/// directly joins two identifiers. The `void*` data type is the exception:
/// since it is matched as a single keyword above, it keeps its `*` attached
/// to `void` (e.g., `void* buf`).
///
/// Examples:
/// - `"constvoid*buf"` → `"const void*buf"` → `"const void* buf"`
/// - `"intcount"` → `"int count"`
/// - `"MPI_Commcomm"` → `"MPI_Comm comm"`
/// - `"MPI_Request*req"` → `"MPI_Request *req"`
///
/// Arguments:
/// - `s`: The input string potentially missing spaces.
///
/// Returns:
/// A new string with spaces inserted where necessary.
fn insert_missing_spaces(s: &str) -> String {
    const QUALIFIERS: &[&str] = &["const", "volatile", "restrict"];
    const MODIFIERS: &[&str] = &["unsigned", "signed", "short", "long"];
    const DATA_TYPES: &[&str] = &[
        "void*",
        "void",
        "int",
        "char",
        "float",
        "double",
        "MPI_Comm",
        "MPI_Datatype",
        "MPI_Request",
        "MPI_Status",
        "MPI_File",
        "MPI_Aint",
        "MPI_Offset",
        "MPI_Count",
        "MPI_Win",
        "MPI_Group",
        "MPI_Op",
        "MPI_Info",
    ];

    // For each category, build "(keyword1|keyword2|...)(\w+)" and insert a
    // space between the matched keyword and the word characters that follow it.
    let mut result = s.to_string();
    for keywords in [QUALIFIERS, MODIFIERS, DATA_TYPES] {
        let alternation = keywords
            .iter()
            .map(|k| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|");
        let re = Regex::new(&format!("({})(\\w+)", alternation)).unwrap();
        result = re.replace_all(&result, "$1 $2").to_string();
    }

    // Any remaining "*" directly joining two identifiers belongs to a
    // pointer whose type was not matched above (e.g., "MPI_Request*req");
    // attach it to the variable name on its right.
    let re_before_star = Regex::new(r"(\w)\*(\w)").unwrap();
    re_before_star.replace_all(&result, "$1 *$2").to_string()
}

/// Remove line numbers and normalize the text extracted from the PDF.
/// The MPI standard PDF has line numbers in the margins that appear as
/// standalone numbers on lines or at the start/end of text lines.
fn clean_extracted_text(text: &str) -> String {
    let mut lines: Vec<&str> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        // Skip lines that are just a number (line numbers in margins)
        if trimmed.parse::<u32>().is_ok() {
            continue;
        }
        lines.push(trimmed);
    }
    lines.join("\n")
}

/// Normalize whitespace: collapse all whitespace (including newlines) into
/// single spaces, then trim.
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
