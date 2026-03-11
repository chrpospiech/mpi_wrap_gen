use super::*;
use anyhow::Result;
use lopdf::Document;
use std::path::PathBuf;

fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("mpi_call_xmp.pdf")
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
        8,
        "Expected to find 8 MPI calls, found: {:?}",
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
    // lopdf may strip spaces, so check with contains
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
fn test_extract_mpi_calls_multiline_signature() -> Result<()> {
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
fn test_extract_mpi_calls_empty_range() -> Result<()> {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let calls = extract_mpi_calls(&doc, 5, 5)?;

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

#[test]
fn test_clean_extracted_text_removes_line_numbers() -> Result<()> {
    let input = "1\n2\n3\nint MPI_Send(void* buf)\n4\n5\n";
    let cleaned = clean_extracted_text(input);
    assert!(!cleaned.contains("\n1\n"), "Line numbers should be removed");
    assert!(
        cleaned.contains("int MPI_Send"),
        "MPI call text should be preserved"
    );
    Ok(())
}

#[test]
fn test_normalize_whitespace() -> Result<()> {
    let input = "  const void*   buf  ";
    assert_eq!(normalize_whitespace(input), "const void* buf");
    Ok(())
}

#[test]
fn test_insert_missing_spaces_pointer() -> Result<()> {
    // Space is inserted after * when immediately followed by a word character.
    assert_eq!(insert_missing_spaces("void*buf"), "void* buf");
    Ok(())
}

#[test]
fn test_insert_missing_spaces_const_void() -> Result<()> {
    // Full example from the docstring.
    let result = normalize_whitespace(&insert_missing_spaces("constvoid*buf"));
    assert_eq!(result, "const void* buf");
    Ok(())
}

#[test]
fn test_insert_missing_spaces_int_identifier() -> Result<()> {
    // Keyword at start of string followed directly by an identifier.
    let result = normalize_whitespace(&insert_missing_spaces("intcount"));
    assert_eq!(result, "int count");
    Ok(())
}

#[test]
fn test_parse_parameters_missing_spaces() -> Result<()> {
    // Verify parse_parameters applies space insertion across comma-separated params.
    let params = parse_parameters("constvoid*buf,intcount");
    assert_eq!(params, vec!["const void* buf", "int count"]);
    Ok(())
}

#[test]
fn test_insert_missing_spaces_already_spaced() -> Result<()> {
    // Already-correct input must not be changed.
    let result = normalize_whitespace(&insert_missing_spaces("const void* buf"));
    assert_eq!(result, "const void* buf");
    Ok(())
}
