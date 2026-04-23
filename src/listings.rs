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

use crate::chapters::{find_chapter_pages, get_chapter_page, get_chapter_title, ChapterMap};
use crate::cmdline::CliArgs;
use crate::config::Config;
use crate::mpi_calls::{extract_mpi_calls, MpiCallInfo};
use crate::template::generate_output;
use anyhow::{Context, Result};
use lopdf::Document;

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod functional_tests;

/// This function extracts the MPI call information from the PDF document for
/// the specified list of chapters as listed in the configuration. Using the
/// templates from the configuration, a single document is generated and written
/// to a string variable. This string can then be written to a file or printed
/// to the console.
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
        anyhow::bail!("No chapters specified in the configuration.");
    }

    if args.verbose {
        println!(
            "Extracting MPI call information for chapters: {:?}",
            config.chapters
        );
    }
    let chapter_indices: ChapterMap = find_chapter_pages(doc)?;
    let mut output = String::new();

    // Add header template
    if args.verbose {
        println!("Adding header template to output.");
    }
    output.push_str(&config.templates.header);

    for &chapter in &config.chapters {
        let ch_start = get_chapter_page(&chapter_indices, chapter)
            .with_context(|| format!("Chapter {} not found in PDF document.", chapter))?;
        let ch_end = chapter_indices
            .keys()
            .filter_map(|&candidate_chapter| {
                get_chapter_page(&chapter_indices, candidate_chapter).ok()
            })
            .filter(|&page| page > ch_start)
            .min()
            .unwrap_or_else(|| (doc.get_pages().len() + 1) as u32); // If no later chapter is found, use end of document

        if args.verbose {
            println!(
                "Extracting MPI calls for chapter {}: pages {} to {}",
                chapter,
                ch_start,
                ch_end - 1
            );
        }
        let mpi_calls: Vec<MpiCallInfo> = extract_mpi_calls(doc, ch_start, ch_end)?;

        if mpi_calls.is_empty() {
            if args.verbose {
                println!("No MPI calls found in chapter {}.", chapter);
            }
            continue; // Skip to next chapter if no MPI calls found
        }

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
            &mpi_calls[0],
        )?;

        for mpi_call in mpi_calls {
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

    // Add footer template
    if args.verbose {
        println!("Adding footer template to output.");
    }
    output.push_str(&config.templates.footer);

    Ok(output)
}
