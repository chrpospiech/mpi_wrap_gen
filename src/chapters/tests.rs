use super::*;
use anyhow::Result;
use pdf_oxide::PdfDocument;

/// Helper: open the test PDF with 4 chapters (chapter.1 .. chapter.4).
fn open_test_pdf() -> Result<PdfDocument> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/test_outline.pdf");
    Ok(PdfDocument::open(path)?)
}

// ── get_page_indices_for_chapters (public API) ──────────────────────────────

#[test]
fn test_get_page_indices_for_chapters_finds_all_four() -> Result<()> {
    let mut doc = open_test_pdf()?;
    let indices = get_page_indices_for_chapters(&mut doc)?;

    assert_eq!(
        indices.len(),
        4,
        "Expected 4 chapter entries, got {}",
        indices.len()
    );
    for chapter_num in 1..=4 {
        assert!(
            indices.contains_key(&chapter_num),
            "Missing chapter.{chapter_num}"
        );
    }
    Ok(())
}

#[test]
fn test_chapter_page_indices_are_sequential() -> Result<()> {
    let mut doc = open_test_pdf()?;
    let indices = get_page_indices_for_chapters(&mut doc)?;

    // Each chapter sits on its own page: chapter N -> page N-1
    for chapter_num in 1..=4i32 {
        let page_index = indices[&chapter_num];
        assert_eq!(
            page_index,
            (chapter_num - 1) as usize,
            "chapter.{chapter_num} should map to page index {}",
            chapter_num - 1
        );
    }
    Ok(())
}

// ── build_page_ref_to_index_map ─────────────────────────────────────────────

#[test]
fn test_build_page_ref_to_index_map_has_four_pages() -> Result<()> {
    let mut doc = open_test_pdf()?;
    let map = build_page_ref_to_index_map(&mut doc)?;

    assert_eq!(map.len(), 4, "Test PDF should have exactly 4 pages");
    // Indices should be 0..3
    let mut indices: Vec<usize> = map.values().copied().collect();
    indices.sort();
    assert_eq!(indices, vec![0, 1, 2, 3]);
    Ok(())
}

// ── resolve_named_destination_page_index ─────────────────────────────────────

#[test]
fn test_resolve_named_dest_existing() -> Result<()> {
    let mut doc = open_test_pdf()?;
    let page_map = build_page_ref_to_index_map(&mut doc)?;

    for i in 1..=4 {
        let name = format!("chapter.{i}");
        let idx = resolve_named_destination_page_index(&mut doc, &page_map, &name)?;
        assert!(idx.is_some(), "Named dest '{name}' should resolve");
        assert_eq!(idx.unwrap(), (i - 1) as usize);
    }
    Ok(())
}

#[test]
fn test_resolve_named_dest_nonexistent_returns_none() -> Result<()> {
    let mut doc = open_test_pdf()?;
    let page_map = build_page_ref_to_index_map(&mut doc)?;

    let idx = resolve_named_destination_page_index(&mut doc, &page_map, "chapter.99")?;
    assert!(idx.is_none(), "Non-existent destination should return None");
    Ok(())
}

// ── find_named_destination_object ────────────────────────────────────────────

#[test]
fn test_find_named_dest_object_existing() -> Result<()> {
    let mut doc = open_test_pdf()?;

    for i in 1..=4 {
        let name = format!("chapter.{i}");
        let obj = find_named_destination_object(&mut doc, &name)?;
        assert!(obj.is_some(), "Should find named dest object for '{name}'");
    }
    Ok(())
}

#[test]
fn test_find_named_dest_object_missing() -> Result<()> {
    let mut doc = open_test_pdf()?;

    let obj = find_named_destination_object(&mut doc, "nonexistent.dest")?;
    assert!(obj.is_none(), "Missing dest should return None");
    Ok(())
}

// ── object_to_name_key ──────────────────────────────────────────────────────

#[test]
fn test_object_to_name_key_from_string() {
    use pdf_oxide::object::Object;

    let obj = Object::String(b"chapter.1".to_vec());
    let key = object_to_name_key(&obj);
    assert_eq!(key.as_deref(), Some("chapter.1"));
}

#[test]
fn test_object_to_name_key_from_name() {
    use pdf_oxide::object::Object;

    let obj = Object::Name("chapter.2".to_string());
    let key = object_to_name_key(&obj);
    assert_eq!(key.as_deref(), Some("chapter.2"));
}

#[test]
fn test_object_to_name_key_from_integer_returns_none() {
    use pdf_oxide::object::Object;

    let obj = Object::Integer(42);
    assert!(object_to_name_key(&obj).is_none());
}
