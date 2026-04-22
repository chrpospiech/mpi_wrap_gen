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
// File: test_outline_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/test_outline_tests.rs
// Author: Dr. Christoph Pospiech

use super::*;
use lopdf::Document;
use std::path::PathBuf;

fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("test_outline.pdf")
}

#[test]
fn test_extract_mpi_calls_finds_calls() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    assert!(
        !calls.is_empty(),
        "Expected at least one MPI call to be found"
    );
    assert_eq!(
        calls.len(),
        7,
        "Expected to find 7 MPI calls, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_returns_call_names() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

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
fn test_extract_mpi_calls_returns_parameters() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

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
fn test_extract_mpi_calls_finds_send() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    let send_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Send").collect();
    assert!(
        !send_calls.is_empty(),
        "Expected to find MPI_Send in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &send_calls[0];
    assert!(
        params.iter().any(|p| p.contains("buf")),
        "MPI_Send should have a 'buf' parameter, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("comm")),
        "MPI_Send should have a 'comm' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_finds_recv() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

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
fn test_extract_mpi_calls_send_has_six_parameters() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

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
fn test_extract_mpi_calls_finds_barrier() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    let barrier_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Barrier").collect();
    assert!(
        !barrier_calls.is_empty(),
        "Expected to find MPI_Barrier in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &barrier_calls[0];
    assert_eq!(
        params.len(),
        1,
        "MPI_Barrier should have 1 parameter, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("comm")),
        "MPI_Barrier should have a 'comm' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_finds_bcast() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    let bcast_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Bcast").collect();
    assert!(
        !bcast_calls.is_empty(),
        "Expected to find MPI_Bcast in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &bcast_calls[0];
    assert_eq!(
        params.len(),
        5,
        "MPI_Bcast should have 5 parameters, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("buffer")),
        "MPI_Bcast should have a 'buffer' parameter, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("root")),
        "MPI_Bcast should have a 'root' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_finds_scatter() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    let scatter_calls: Vec<_> = calls.iter().filter(|(name, _)| name == "Scatter").collect();
    assert!(
        !scatter_calls.is_empty(),
        "Expected to find MPI_Scatter in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &scatter_calls[0];
    assert_eq!(
        params.len(),
        8,
        "MPI_Scatter should have 8 parameters, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("sendbuf")),
        "MPI_Scatter should have a 'sendbuf' parameter, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("recvbuf")),
        "MPI_Scatter should have a 'recvbuf' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_finds_scatterv() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let num_pages = doc.page_iter().count() as u32;
    let calls = extract_mpi_calls(&doc, 1, num_pages + 1)?;

    let scatterv_calls: Vec<_> = calls
        .iter()
        .filter(|(name, _)| name == "Scatterv")
        .collect();
    assert!(
        !scatterv_calls.is_empty(),
        "Expected to find MPI_Scatterv in test PDF, found: {:?}",
        calls.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );

    let (_, params) = &scatterv_calls[0];
    assert_eq!(
        params.len(),
        9,
        "MPI_Scatterv should have 9 parameters, got: {:?}",
        params
    );
    assert!(
        params.iter().any(|p| p.contains("displs")),
        "MPI_Scatterv should have a 'displs' parameter, got: {:?}",
        params
    );
    Ok(())
}

#[test]
fn test_extract_mpi_calls_empty_range() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let calls = extract_mpi_calls(&doc, 4, 4)?;

    assert!(calls.is_empty(), "Empty page range should return no calls");
    Ok(())
}

#[test]
fn test_extract_mpi_calls_invalid_page_range() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let calls = extract_mpi_calls(&doc, 100, 200)?;

    assert!(
        calls.is_empty(),
        "Out-of-range pages should return no calls"
    );
    Ok(())
}
