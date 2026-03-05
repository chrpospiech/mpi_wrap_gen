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
use pdf_oxide::PdfDocument;

pub(crate) mod chapters;
pub(crate) mod cmdline;

fn main() -> Result<()> {
    let args = cmdline::parse_args()?;
    cmdline::echo_args(&args)?;

    if args.dry_run {
        println!("Dry-run mode: skipping PDF processing.");
        return Ok(());
    }

    let mut doc = PdfDocument::open(&args.input_file)?;
    if args.verbose {
        println!("PDF opened successfully: {:?}", &doc);
    }

    match chapters::get_page_indices_for_chapters(&mut doc) {
        Ok(chapter_indices) => println!("Chapter indices: {:?}", chapter_indices),
        Err(e) => println!("Failed to extract chapter indices: {}", e),
    }

    println!("Hello, world!");
    Ok(())
}
