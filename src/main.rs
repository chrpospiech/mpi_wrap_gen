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
use lopdf::Document;

pub(crate) mod chapters;
pub(crate) mod cmdline;
pub(crate) mod mpi_calls;

fn main() -> Result<()> {
    let args = cmdline::parse_args()?;
    cmdline::echo_args(&args)?;

    if args.dry_run {
        println!("Dry-run mode: skipping PDF processing.");
        return Ok(());
    }

    let doc = Document::load(&args.input_file)?;
    if args.verbose {
        println!("PDF opened successfully.");
    }

    match chapters::find_chapter_pages(&doc) {
        Ok(chapter_indices) => println!("Chapter indices: {:?}", chapter_indices),
        Err(e) => println!("Failed to extract chapter indices: {}", e),
    }

    let mpi_calls = mpi_calls::extract_mpi_calls(&doc, 54, 60)?;
    println!("Extracted MPI calls: {:?}", mpi_calls);

    println!("Hello, world!");
    Ok(())
}
