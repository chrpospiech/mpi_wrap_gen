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
use anyhow::{bail, Context, Result};
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
            let params = parse_parameters(params_str)
                .with_context(|| format!("Failed to parse parameters for MPI_{}", call_name))?;
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
fn parse_parameters(params_str: &str) -> Result<Vec<String>> {
    let params_str = params_str.trim();
    if params_str.is_empty() || params_str == "void" {
        return Ok(Vec::new());
    }
    params_str
        .split(',')
        .map(format_variable_declaration)
        .collect()
}

/// Parse a single parameter from an MPI call signature.
/// This function handles missing spaces and tries to extract
/// potential C type qualifiers, type modifiers, C data types
/// and the variable name.
///
/// The list of C type qualifiers are `const`, `volatile` and `restrict`.
/// The list of C type modifiers are `unsigned`, `signed`, `short` and `long`.
/// The list of C data types are `void*`, `int`, `char`, `float` and `double`.
/// The latter list needs to be extended by the MPI-specific data types, such as
/// `MPI_Comm`, `MPI_Datatype`, `MPI_Request`, `MPI_Status`, `MPI_File`,
/// `MPI_Aint`, `MPI_Offset`, `MPI_Count`, `MPI_Win`, `MPI_Group`, `MPI_Op`,
/// `MPI_Info`, `MPI_User_function` and `MPI_Message`.
///
/// For each of these categories, we create a group in a regular expression
/// by joining the keywords with the separator '|' and enclosing them in
/// parentheses.
///
/// To account for the variable name, three additional groups are appended
/// to the regular expression:
/// - One for the pointer operator (if present)
/// - One for the variable name
/// - One for any array brackets (if present)
///
/// Several of these groups are optional, as C type qualifiers, type modifiers,
/// and array brackets may not always be present in a parameter declaration.
/// We have the choice of `(group|entries)?` versus `(group|entries|)`
/// to make the group optional. Only in the latter case will the group
/// match an empty string if the corresponding component is absent.
///
/// Unfortunately, we cannot combine all groups into a single regular expression
/// because some groups are optional and and may deliver empty matches even when
/// the corresponding component is **not** absent. Hence, we start with the last
/// four groups corresponding to the C data type, pointer operator, variable name,
/// and array brackets, and work our way backward to handle optional components
/// correctly.
///
/// This is done by joining the last four groups together with a separator `\s*`
/// to form a first regular expression used for parsing MPI call signatures.
///
/// - If there is no match, an error is returned.
/// - If there is a match, the captured groups are joined to reconstruct the
///   correctly spaced variable declaration.
///
/// We also catch the part preceding the variable declaration, and check for
/// any C type qualifiers and type modifiers.
///
/// Arguments:
/// - `s`: The input string representing a single parameter from an MPI call signature.
///
/// Returns:
/// - `Ok(String)`: The correctly spaced variable declaration if parsing is successful.
/// - `Err`: An error message if parsing fails.
fn format_variable_declaration(s: &str) -> Result<String> {
    const QUALIFIERS: &[&str] = &["const", "volatile", "restrict"];
    const MODIFIERS: &[&str] = &["unsigned", "signed", "short", "long"];
    const DATA_TYPES: &[&str] = &[
        r"void\s*\*",
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
        "MPI_User_function",
        "MPI_Message",
    ];
    // Build regular expression fragments for parsing MPI call signatures using these lists.
    let data_types_re = format!("({})", DATA_TYPES.join("|"));
    let pointer_re = r"(\*+|)".to_string();
    let identifier_re = r"(\w+)".to_string();
    let array_re = r"((?:\[[^\]]*\])*)".to_string();
    // These regular expression pieces are joined together to form a complete
    // regular expression for parsing MPI call signatures.
    let typed_variable_signature_re = Regex::new(&format!(
        "(.*?){}\\s*{}\\s*{}\\s*{}.*",
        data_types_re, pointer_re, identifier_re, array_re,
    ))?;
    // If s matches the MPI call signature regular expression, return a formatted string
    // composed from joining the matching groups. We trim the resulting string to remove
    // any leading or trailing whitespace.
    if !typed_variable_signature_re.is_match(s) {
        bail!("Failed to extract typed variable declaration: {}", s)
    }
    let captures = typed_variable_signature_re.captures(s).unwrap();
    let mut leading_part = captures[1].to_string();
    let data_type = captures[2].split_whitespace().collect::<String>();
    let mut typed_variable = format!(
        "{} {}{}{}",
        data_type, &captures[3], &captures[4], &captures[5],
    );
    // Next we look for a C type qualifyiers and modifiers,
    // extract them from `leading_part` and prepend them to the typed variable.
    let chop_off_re = Regex::new(&format!(
        "(.*)({}|{})\\s*$",
        QUALIFIERS.join("|"),
        MODIFIERS.join("|")
    ))?;
    while let Some(caps) = chop_off_re.captures(&leading_part) {
        let qualifier_or_modifier = caps[2].to_string();
        let remaining = caps[1].to_string();
        typed_variable = format!("{} {}", qualifier_or_modifier, typed_variable);
        leading_part = remaining;
    }
    Ok(typed_variable)
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
