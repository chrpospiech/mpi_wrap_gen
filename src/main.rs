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
// File: main.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/main.rs
// Author: Dr. Christoph Pospiech

use anyhow::Result;
use chapters::ChapterMap;
use lopdf::Document;
use mpi_calls::MpiCallInfo;

pub(crate) mod chapters;
pub(crate) mod cmdline;
pub(crate) mod config;
pub(crate) mod mpi_calls;
pub(crate) mod template;

fn main() -> Result<()> {
    let args = cmdline::parse_args()?;
    cmdline::echo_args(&args)?;

    if args.dry_run {
        println!("Dry-run mode: skipping PDF processing.");
        return Ok(());
    }

    let config = config::read_config(&args.config_file)?;
    if args.verbose {
        config::print_config(&config);
    }

    let doc = Document::load(&config.mpi_pdf)?;
    if args.verbose {
        println!("PDF opened successfully.");
    }

    let chapter_indices: ChapterMap = chapters::find_chapter_pages(&doc)?;
    println!("Chapter indices: {:?}", chapter_indices);

    let ch3_start = chapters::get_chapter_page(&chapter_indices, 3)?;
    let ch4_start = chapters::get_chapter_page(&chapter_indices, 4)?;

    let mpi_calls: Vec<MpiCallInfo> = mpi_calls::extract_mpi_calls(&doc, ch3_start, ch4_start)?;

    let chapter_title = chapters::get_chapter_title(&chapter_indices, 3)?;
    let header_template =
        "\n\nExtracted MPI calls from chapter %(chapter_number)d: %(chapter_title)s\n";
    let output = template::generate_output(header_template, 3, &chapter_title, &mpi_calls[0])?;
    println!("{}", output);
    let item_template = "MPI call: MPI_%(mpi_call_name)s\n   with parameters: %(mpi_call_params)s";
    for mpi_call in mpi_calls {
        let output = template::generate_output(item_template, 3, &chapter_title, &mpi_call)?;
        println!("{}", output);
    }

    println!("Hello, world!");
    Ok(())
}
