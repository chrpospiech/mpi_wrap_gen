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
use crate::config::Config;
use crate::mpi_calls::{create_barrier_names, mpi_calls_for_chapter, MpiCallInfo};
use crate::template::generate_output;
use anyhow::{bail, Result};
use lopdf::Document;
use std::collections::HashSet;

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod find_collective;

#[cfg(test)]
mod functional_tests;

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
                    format!("  chapter {} (\"{}\")", ch_second, &title_second),
                    format!("  chapter {} (\"{}\")", ch_found, &title_found),
                    "Only one collective chapter is supported.".to_string(),
                ]
                .join("\n");
                bail!(bail_msg);
            }
            if let Some(col_item) = collective_chapter {
                let ch_found: u32 = chapter;
                let title_found: &str = &title;
                let ch_second: u32 = col_item;
                let title_second: &str = &get_chapter_title(chapter_indices, ch_second)?;
                let bail_msg = [
                    "Multiple collective chapters found:".to_string(),
                    format!("  chapter {} (\"{}\")", ch_second, &title_second),
                    format!("  chapter {} (\"{}\")", ch_found, &title_found),
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

/// This function extracts the MPI call information from the PDF document for
/// the specified list of chapters as listed in the configuration. Using the
/// templates from the configuration, a single document is generated and written
/// to a string variable. If the list of chapters contain a chapter dealing
/// with collective operations, the generated output will include a section for
/// collective operations. The generated output is returned as a string.
/// This string can then be written to a file or printed to the console.
///
/// Arguments:
/// - `doc`: The PDF document to extract the MPI call information from.
/// - `args`: The command line arguments containing the verbose flag for debugging purposes.
/// - `config`: The configuration containing the list of chapters to extract and
///   the templates to use for generating the output.
///
/// Returns:
/// - A string containing the generated output based on the extracted MPI call
///   information and the templates from the configuration.
/// - If the list of chapters is empty, the function will return an error.
/// - If a chapter is not found in the PDF document, the function will return an error.
pub fn extract_and_generate_listings(
    doc: &Document,
    args: &CliArgs,
    config: &Config,
) -> Result<String> {
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
    let excludes = create_exclude_set(config);
    let mut output = String::new();

    // Add header template
    if args.verbose {
        println!("Adding header template to output.");
    }
    output.push_str(&config.templates.header);

    for &chapter in &config.chapters {
        if args.verbose {
            println!("Extracting MPI calls for chapter {}", chapter);
        }
        let mpi_calls: Vec<MpiCallInfo> = mpi_calls_for_chapter(doc, &chapter_indices, chapter)?;

        if mpi_calls.is_empty() {
            if args.verbose {
                println!("No MPI calls found in chapter {}.", chapter);
            }
            continue; // Skip to next chapter if no MPI calls found
        }
        let filtered_mpi_calls: Vec<MpiCallInfo> = mpi_calls
            .iter()
            .filter(|mpi_call| !is_excluded(&excludes, &mpi_call.0))
            .cloned()
            .collect();

        let chapter_title = get_chapter_title(&chapter_indices, chapter)?;

        // Add chapter init template
        if args.verbose {
            println!(
                "Adding chapter init template for chapter {} to output.",
                chapter
            );
        }
        output += &generate_output(
            &config.templates.ch_init,
            chapter,
            &chapter_title,
            filtered_mpi_calls.first().unwrap_or(&mpi_calls[0]),
        )?;

        for mpi_call in filtered_mpi_calls {
            // Add chapter body template with MPI call information
            if args.verbose {
                println!(
                    "Adding body template for MPI call {} to output.",
                    mpi_call.0
                );
            }
            output += &generate_output(
                &config.templates.ch_body,
                chapter,
                &chapter_title,
                &mpi_call,
            )?;
        }
    }

    // If a collective chapter was found, add the collective operations section
    let collective_chapter = find_collective_chapter(&chapter_indices, args, config)?;
    if let Some(collective_chapter) = collective_chapter {
        if args.verbose {
            println!(
                "Extracting barrier MPI calls for chapter {}",
                collective_chapter
            );
        }
        let mpi_calls: Vec<MpiCallInfo> =
            create_barrier_names(doc, &chapter_indices, collective_chapter)?;
        let filtered_mpi_calls: Vec<MpiCallInfo> = mpi_calls
            .iter()
            .filter(|mpi_call| !is_excluded(&excludes, &mpi_call.0))
            .cloned()
            .collect();

        if !mpi_calls.is_empty() {
            let chapter_title = get_chapter_title(&chapter_indices, collective_chapter)?;

            // Add barrier chapter init template
            if args.verbose {
                println!(
                    "Adding barrier chapter init template for chapter {} to output.",
                    collective_chapter
                );
            }
            output += &generate_output(
                &config.templates.bar_init,
                collective_chapter,
                &chapter_title,
                filtered_mpi_calls.first().unwrap_or(&mpi_calls[0]),
            )?;

            for mpi_call in filtered_mpi_calls {
                // Add barrier chapter body template with MPI call information
                if args.verbose {
                    println!(
                        "Adding barrier body template for MPI call {} to output.",
                        mpi_call.0
                    );
                }
                output += &generate_output(
                    &config.templates.bar_body,
                    collective_chapter,
                    &chapter_title,
                    &mpi_call,
                )?;
            }
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
    output.push_str(&config.templates.footer);

    Ok(output)
}
