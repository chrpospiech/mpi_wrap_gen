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
// File: listings.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/listings.rs
// Author: Dr. Christoph Pospiech

use crate::chapters::{find_chapter_pages, get_chapter_title, ChapterMap};
use crate::cmdline::CliArgs;
use crate::config::{Config, Languages};
use crate::mpi_calls::{
    create_barrier_names, filter_excluded_mpi_calls, mpi_calls_for_chapter, MpiCallInfo,
};
use crate::template::generate_output;
use anyhow::{bail, Result};
use dynfmt2::{Format, PythonFormat};
use lopdf::Document;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod find_collective;

#[cfg(test)]
mod format_chapter_number_tests;

#[cfg(test)]
mod functional_tests;

pub type Output = Vec<(PathBuf, String)>;

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterTemplates {
    pub ch_init: String,
    #[serde(default)]
    pub ch_decl: String,
    #[serde(default)]
    pub ch_msize: String,
    pub ch_body: String,
}

/// Normalizes the MPI call name by removing the "mpi_" prefix and "_barrier" suffix, if present.
/// This function is used to create a set of excluded MPI call names for filtering purposes.
fn normalize_exclude_name(name: &str) -> String {
    let normalized = name.trim().to_ascii_lowercase();
    let without_prefix = normalized.strip_prefix("mpi_").unwrap_or(&normalized);
    let without_suffix = without_prefix
        .strip_suffix("_barrier")
        .unwrap_or(without_prefix);
    without_suffix.to_string()
}

/// Creates a set of excluded MPI call names based on the configuration.
/// This function normalizes the names by removing the "mpi_" prefix and "_barrier" suffix.
fn create_exclude_set(config: &Config) -> HashSet<String> {
    config
        .excludes
        .iter()
        .map(|name| normalize_exclude_name(name))
        .collect()
}

/// Checks if the given MPI call name is in the set of excluded names.
/// This function normalizes the name by removing the "mpi_" prefix
/// and "_barrier" suffix before checking.
fn is_excluded(excludes: &HashSet<String>, mpi_call_name: &str) -> bool {
    excludes.contains(&normalize_exclude_name(mpi_call_name))
}

/// Inspects the chapter headers provided by a `ChapterMap` for the list
/// `config.chapters` and returns the number of the chapter that contains
/// the word "collective" (case-insensitive).
/// If no chapter contains the word "collective" or if the list of chapters is empty,
/// the function returns `None`.
/// If multiple chapters contain the word "collective", the function returns an error.
/// If exactly one chapter contains the word "collective",
/// the function returns the chapter number.
fn find_collective_chapter(
    chapter_indices: &ChapterMap,
    args: &CliArgs,
    config: &Config,
) -> Result<Option<u32>> {
    let mut collective_chapter: Option<u32> = None;
    if args.verbose {
        println!(
            "Searching for collective chapter in chapters: {:?}",
            config.chapters
        );
    }

    for &chapter in &config.chapters {
        let title = get_chapter_title(chapter_indices, chapter)?;
        if title.to_lowercase().contains("collective") {
            if let Some(item) = collective_chapter {
                let ch_found: u32 = chapter;
                let title_found: &str = &title;
                let ch_second: u32 = item;
                let title_second: &str = &get_chapter_title(chapter_indices, ch_second)?;
                let bail_msg = [
                    "Multiple collective chapters found:".to_string(),
                    format!("  chapter {} (\"{}\")", ch_second, title_second),
                    format!("  chapter {} (\"{}\")", ch_found, title_found),
                    "Only one collective chapter is supported.".to_string(),
                ]
                .join("\n");
                bail!(bail_msg);
            }
            collective_chapter = Some(chapter);
        }
    }

    match collective_chapter {
        Some(chapter) => Ok(Some(chapter)),
        None => Ok(None),
    }
}

/// Helper function to take a `Path` that may contain a format string
/// `%(chapter_number)[...]s`, converts the path to a string and replaces
/// the format string with the actual chapter number.
/// The input string will be processed by the `dynfmt2` crate using
/// the `PythonFormat` formatter, including the width and alignment handling.
/// The function will return a Result containing a PathBuf with the formatted
/// string or an error message if the formatting fails.
/// This function is very similar to `generate_output` but
/// - only replaces the chapter number placeholder.
/// - typos in the placeholder name will result in an error
///   rather than silently leaving the raw text in the path.
/// - takes and returns PATHs instead of strings.
///
/// Args:
///     template: A `Path` containing the template with placeholders to be replaced.
///     chapter_number: The number of the chapter to be included in the output string.
///
/// Returns:
///     A Result containing a formatted PathBuf with placeholders replaced by actual values.
///
fn format_chapter_number(template: &Path, chapter_number: u32) -> Result<PathBuf> {
    let vars = std::collections::HashMap::<String, String>::from([(
        "chapter_number".to_string(),
        chapter_number.to_string(),
    )]);
    let output = PythonFormat
        .format(
            template
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
            &vars,
        )
        .map_err(|e| anyhow::anyhow!("{e}"))?
        .into_owned();
    Ok(output.into())
}

/// Generates the chapter output string for a given chapter, applying the chapter
/// init template once and the chapter body template for each MPI call.
///
/// Args:
///     chapter_template: References to the chapter init and body templates.
///     chapter: The chapter number.
///     chapter_title: The title of the chapter.
///     mpi_calls: The filtered MPI calls for this chapter (must be non-empty).
///     language: The programming language to be used for generating the chapter output.
///     verbose: Whether to print progress messages.
///
/// Returns:
///     A `Result<String>` containing the generated chapter output.
fn generate_chapter_output(
    chapter_template: &ChapterTemplates,
    chapter: u32,
    chapter_title: &str,
    mpi_calls: Vec<MpiCallInfo>,
    language: &Languages,
    verbose: bool,
) -> Result<String> {
    let mut ch_output = String::new();

    // Add chapter init template
    if !chapter_template.ch_init.is_empty() {
        if verbose {
            println!(
                "Adding chapter init template for chapter {} to output.",
                chapter
            );
        }

        let first_call = mpi_calls.first().ok_or_else(|| {
            anyhow::anyhow!(
                "generate_chapter_output requires at least one MPI call for chapter {}.",
                chapter
            )
        })?;

        ch_output += &generate_output(
            &chapter_template.ch_init,
            chapter,
            chapter_title,
            first_call,
            language,
        )?;
    }
    for mpi_call in mpi_calls {
        // Add function declaration template with MPI call information
        if verbose && !chapter_template.ch_decl.is_empty() {
            println!(
                "Adding function declaration template for MPI call {} to output.",
                mpi_call.0
            );
        }
        if !chapter_template.ch_decl.is_empty() {
            ch_output += &generate_output(
                &chapter_template.ch_decl,
                chapter,
                chapter_title,
                &mpi_call,
                language,
            )?;
        }
        if verbose && !chapter_template.ch_msize.is_empty() {
            println!(
                "Adding message size template for MPI call {} to output.",
                mpi_call.0
            );
        }
        if !chapter_template.ch_msize.is_empty() {
            ch_output += &generate_output(
                &chapter_template.ch_msize,
                chapter,
                chapter_title,
                &mpi_call,
                language,
            )?;
        }
        // Add chapter body template with MPI call information
        if verbose && !chapter_template.ch_body.is_empty() {
            println!(
                "Adding body template for MPI call {} to output.",
                mpi_call.0
            );
        }
        ch_output += &generate_output(
            &chapter_template.ch_body,
            chapter,
            chapter_title,
            &mpi_call,
            language,
        )?;
    }

    Ok(ch_output)
}

/// This function extracts the MPI call information from the PDF document for
/// the specified list of chapters as listed in the configuration. Depending
/// on the configuration, this information is written to a single string variable
/// or to separate string variables for each chapter. The generated output is based
/// on the templates provided in the configuration. The function returns a Result
/// containing the generated output as a vector of tuples (PathBuf, String) or an error.
/// The PathBuf in the tuple represents the output file path for each chapter, and
/// the String contains the generated output for that chapter. The file paths are
/// generated by replacing the `%(chapter_number)[...]s` placeholder in the output
/// file path template with the actual chapter number. If there is no `%(chapter_number)[...]s`
/// placeholder in the output file path template, the same output file path is used
/// for all chapters and the output vector will contain a single tuple with the
/// generated output for all chapters combined.
///
/// The function also handles the case where a chapter contains collective operations,
/// in which case a separate section for collective operations is generated and included
/// in the output. This is only supported for the case of a single output file path, as
/// the collective operations section is not associated with a specific chapter.
///
/// Arguments:
/// - `doc`: The PDF document to extract the MPI call information from.
/// - `args`: The command line arguments containing the verbose flag for debugging purposes.
/// - `config`: The configuration containing the list of chapters to extract and
///   the templates to use for generating the output.
///
/// Returns:
/// - A vector of tuples (PathBuf, String) containing the generated output based
///   on the extracted MPI call information and the templates from the configuration.
/// - If the list of chapters is empty, the function will return an error.
/// - If a chapter is not found in the PDF document, the function will return an error.
pub fn extract_and_generate_listings(
    doc: &Document,
    args: &CliArgs,
    config: &Config,
) -> Result<Output> {
    if config.chapters.is_empty() {
        bail!("No chapters specified in the configuration.");
    }

    if args.verbose {
        println!(
            "Extracting MPI call information for chapters: {:?}",
            config.chapters
        );
    }
    let chapter_indices: ChapterMap = find_chapter_pages(doc)?;
    let collective_chapter = find_collective_chapter(&chapter_indices, args, config)?;
    let excludes = create_exclude_set(config);
    let mut output: Output = Vec::new();

    // Determine whether we have a single output file or multiple
    // output files based on the presence of the
    // `%(chapter_number)[...]s` placeholder in the output file path template.
    // This is determined by comparing input and output paths after
    // formatting the chapter number. The result is stored in a
    // boolean variable `single_output_file`.
    let single_output_file = {
        let formatted_path = format_chapter_number(&config.output_file, 1)?;
        formatted_path == config.output_file
    };

    // If we have a single output file, we will generate the output for all chapters
    // and return a single tuple (PathBuf, String) in the output vector.
    // Only in this case we are processing the header and footer templates,
    // as they are only relevant for a single output file.
    let mut output_str = String::new();
    if single_output_file {
        // Add header template
        if args.verbose {
            println!("Adding header template to output.");
        }
        output_str.push_str(&config.templates.header);
    }

    for &chapter in &config.chapters {
        if args.verbose {
            println!("Extracting MPI calls for chapter {}", chapter);
        }
        let chapter_title = get_chapter_title(&chapter_indices, chapter)?;

        let mpi_calls: Vec<MpiCallInfo> = mpi_calls_for_chapter(doc, &chapter_indices, chapter)?;

        let filtered_mpi_calls =
            filter_excluded_mpi_calls(&mpi_calls, |name| is_excluded(&excludes, name));

        if filtered_mpi_calls.is_empty() {
            if args.verbose {
                println!(
                    "All MPI calls in chapter {} are excluded by the configuration.",
                    chapter
                );
            }
            continue; // Skip to next chapter if all MPI calls are excluded
        }

        let is_collective_chapter = collective_chapter == Some(chapter);

        let chapter_templates = ChapterTemplates {
            ch_init: config.templates.ch_init.clone(),
            ch_decl: if is_collective_chapter {
                config.templates.ch_decl.clone() + &config.templates.ch_load_imb
            } else {
                config.templates.ch_decl.clone()
            },
            ch_msize: config.templates.ch_msize.clone(),
            ch_body: config.templates.ch_body.clone(),
        };

        let ch_output = generate_chapter_output(
            &chapter_templates,
            chapter,
            &chapter_title,
            filtered_mpi_calls,
            &config.language,
            args.verbose,
        )?;
        if single_output_file {
            output_str += &ch_output;
        } else {
            let output_path = format_chapter_number(&config.output_file, chapter)?;
            output.push((output_path, ch_output));
        }
    }

    if !single_output_file {
        return Ok(output);
    }

    // If a collective chapter was found, add the collective operations section
    if let Some(collective_chapter) = collective_chapter {
        if args.verbose {
            println!(
                "Extracting barrier MPI calls for chapter {}",
                collective_chapter
            );
        }
        let mpi_calls: Vec<MpiCallInfo> =
            create_barrier_names(doc, &chapter_indices, collective_chapter)?;
        let filtered_mpi_calls =
            filter_excluded_mpi_calls(&mpi_calls, |name| is_excluded(&excludes, name));

        if !filtered_mpi_calls.is_empty() {
            let chapter_title = get_chapter_title(&chapter_indices, collective_chapter)?;

            let chapter_templates = ChapterTemplates {
                ch_init: config.templates.bar_init.clone(),
                ch_decl: String::new(),
                ch_msize: String::new(),
                ch_body: config.templates.bar_body.clone(),
            };

            output_str += &generate_chapter_output(
                &chapter_templates,
                collective_chapter,
                &chapter_title,
                filtered_mpi_calls,
                &config.language,
                args.verbose,
            )?;
        } else if args.verbose {
            println!("No MPI calls found in chapter {}.", collective_chapter);
        }
    } else if args.verbose {
        println!("No collective chapter found, skipping collective operations section.");
    }

    // Add footer template
    if args.verbose {
        println!("Adding footer template to output.");
    }
    output_str.push_str(&config.templates.footer);

    if single_output_file {
        output.push((PathBuf::from(&config.output_file), output_str));
    }

    Ok(output)
}
