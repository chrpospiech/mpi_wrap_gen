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

    let mut doc = PdfDocument::open(&args.input_file)?;
    if args.dry_run || args.verbose {
        println!("PDF opened successfully: {:?}", &doc);
    }

    let text = doc.extract_text(55)?;
    println!("{text}");

    if let Some(outline) = doc.get_outline()? {
        let page_ref_to_index = chapters::build_page_ref_to_index_map(&mut doc)?;
        println!("PDF Outline:");
        chapters::print_outline_items(&mut doc, &outline, &page_ref_to_index, 0)?;
    } else {
        println!("No outline found in the PDF.");
    }

    println!("Hello, world!");
    Ok(())
}
