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
// File: wait_some_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/wait_some_tests.rs
// Author: Dr. Christoph Pospiech

use super::*;
use lopdf::Document;
use std::path::PathBuf;

// Helper function to get the path to the test PDF file containing MPI call signatures.
// The test PDF should be located in the `test_data` directory of the project and named `wait_some_xmp.pdf`.
// The file `wait_some_xmp.pdf` is a subset of the MPI standard
// <https://www.mpi-forum.org/docs/mpi-3.0/mpi30-report.pdf>.
// The file `wait_some_xmp.pdf` contains the MPI call signatures for the `MPI_Waitsome` function, which
// is split across two pages in the MPI standard document.
fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("wait_some_xmp.pdf")
}

#[test]
fn test_mpi_calls_for_page_range_finds_all_calls() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    assert!(
        !calls.is_empty(),
        "Expected at least one MPI call to be found"
    );
    assert_eq!(
        calls.len(),
        3,
        "Expected to find 3 MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_finds_waitsome() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    let waitsome_calls: Vec<_> = calls
        .iter()
        .filter(|(name, _)| name == "Waitsome")
        .collect();
    assert!(
        !waitsome_calls.is_empty(),
        "Expected to find MPI_Waitsome in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &waitsome_calls[0];
    assert!(
        params
            .iter()
            .any(|p| p.contains("MPI_Request") && p.contains("array_of_requests")),
        "MPI_Waitsome should have a 'MPI_Request array_of_requests' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_finds_testsome() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    let testsome_calls: Vec<_> = calls
        .iter()
        .filter(|(name, _)| name == "Testsome")
        .collect();
    assert!(
        !testsome_calls.is_empty(),
        "Expected to find MPI_Testsome in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &testsome_calls[0];
    assert!(
        params
            .iter()
            .any(|p| p.contains("int") && p.contains("array_of_indices")),
        "MPI_Testsome should have an 'int array_of_indices' parameter, got: {:?}",
        params
    );
    Ok(())
}
