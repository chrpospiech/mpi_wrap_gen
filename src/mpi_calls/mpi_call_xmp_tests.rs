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
// File: tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/tests.rs
// Author: Dr. Christoph Pospiech

use super::*;
use lopdf::Document;
use std::path::PathBuf;

// Helper function to get the path to the test PDF file containing MPI call signatures.
// The test PDF should be located in the `test_data` directory of the project and named `mpi_call_xmp.pdf`.
// The file `mpi_call_xmp.pdf` is a subset of the MPI standard
// <https://www.mpi-forum.org/docs/mpi-3.1/mpi31-report.pdf>.
fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("mpi_call_xmp.pdf")
}

#[test]
fn test_mpi_calls_for_page_range_finds_calls() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    assert!(
        !calls.is_empty(),
        "Expected at least one MPI call to be found"
    );
    assert_eq!(
        calls.len(),
        23,
        "Expected to find 23 MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_returns_call_names() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    assert!(!calls.is_empty(), "Need at least one call for this test");
    for (name, _) in &calls {
        assert!(!name.is_empty(), "MPI call name should not be empty");
        assert!(
            name.chars().next().unwrap().is_uppercase(),
            "MPI call name '{}' should start with uppercase",
            name
        );
    }
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_returns_parameters() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    assert!(!calls.is_empty(), "Need at least one call for this test");
    for (name, params) in &calls {
        assert!(
            !params.is_empty(),
            "MPI call '{}' should have at least one parameter",
            name
        );
        for param in params {
            assert!(
                !param.is_empty(),
                "Parameters of '{}' should not be empty",
                name
            );
        }
    }
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_finds_send() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    let send_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Send").collect();
    assert!(
        !send_calls.is_empty(),
        "Expected to find MPI_Send in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &send_calls[0];
    // lopdf may strip spaces, but we need the exact spaces
    assert_eq!(
        params[0], "const void* buf",
        "MPI_Send should have a 'buf' parameter, got: {:?}",
        params[0]
    );
    assert_eq!(
        params[2], "MPI_Datatype datatype",
        "MPI_Send should have a 'datatype' parameter, got: {:?}",
        params[2]
    );
    assert_eq!(
        params[5], "MPI_Comm comm",
        "MPI_Send should have a 'comm' parameter, got: {:?}",
        params[5]
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_finds_recv() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    let recv_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Recv").collect();
    assert!(
        !recv_calls.is_empty(),
        "Expected to find MPI_Recv in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &recv_calls[0];
    assert!(
        params.iter().any(|p| p.contains("status")),
        "MPI_Recv should have a 'status' parameter, got: {:?}",
        params
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
    // lopdf may strip spaces, but we need the exact spaces
    assert_eq!(
        params[0], "int incount",
        "MPI_Waitsome should have 'incount' parameter, got: {:?}",
        params[0]
    );
    assert_eq!(
        params[1], "MPI_Request array_of_requests[]",
        "MPI_Waitsome should have 'array_of_requests' parameter, got: {:?}",
        params[1]
    );
    assert_eq!(
        params[2], "int *outcount",
        "MPI_Waitsome should have 'outcount' parameter, got: {:?}",
        params[2]
    );
    assert_eq!(
        params[3], "int array_of_indices[]",
        "MPI_Waitsome should have 'array_of_indices' parameter, got: {:?}",
        params[3]
    );
    assert_eq!(
        params[4], "MPI_Status array_of_statuses[]",
        "MPI_Waitsome should have 'array_of_statuses' parameter, got: {:?}",
        params[4]
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_multiline_signature() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let num_pages = doc.page_iter().count() as u32;
    let calls = mpi_calls_for_page_range(&doc, 1, num_pages + 1)?;

    let send_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Send").collect();
    assert!(!send_calls.is_empty(), "Expected to find MPI_Send");
    let (_, params) = &send_calls[0];
    assert_eq!(
        params.len(),
        6,
        "MPI_Send should have 6 parameters, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_empty_range() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let calls = mpi_calls_for_page_range(&doc, 5, 5)?;

    assert!(calls.is_empty(), "Empty page range should return no calls");
    Ok(())
}

#[test]
fn test_mpi_calls_for_page_range_invalid_page_range() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let calls = mpi_calls_for_page_range(&doc, 100, 200)?;

    assert!(
        calls.is_empty(),
        "Out-of-range pages should return no calls"
    );
    Ok(())
}

#[test]
fn test_clean_extracted_text_removes_line_numbers() -> Result<()> {
    let input = "1\n2\n3\nint MPI_Send(void* buf)\n4\n5\n";
    let cleaned = clean_extracted_text(input);
    assert!(!cleaned.contains("\n1\n"), "Line numbers should be removed");
    assert!(
        cleaned.contains("int MPI_Send(void* buf)"),
        "MPI call text should be preserved"
    );
    Ok(())
}

#[test]
fn test_normalize_whitespace() -> Result<()> {
    let input = "  const void*   buf  ";
    assert_eq!(format_variable_declaration(input)?, "const void* buf");
    Ok(())
}

#[test]
fn test_parse_parameters_missing_spaces() -> Result<()> {
    // Verify parse_parameters applies space insertion across comma-separated params.
    let params = parse_parameters("constvoid*buf,intcount")?;
    assert_eq!(params, vec!["const void* buf", "int count"]);
    Ok(())
}

#[test]
fn test_parse_parameters_returns_error_for_unrecognized_declaration() -> Result<()> {
    let error = parse_parameters("int count,unknown_type value").unwrap_err();
    assert!(error
        .to_string()
        .contains("Failed to extract typed variable declaration"));
    assert!(error.to_string().contains("unknown_type value"));
    Ok(())
}
