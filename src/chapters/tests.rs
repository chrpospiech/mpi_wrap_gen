use super::*;
use lopdf::Document;
use std::path::PathBuf;

fn test_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("test_outline.pdf")
}

#[test]
fn test_find_chapter_pages_loads_all_chapters() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to extract chapters");

    assert_eq!(
        chapters.len(),
        4,
        "Expected 4 chapters, got {}",
        chapters.len()
    );
}

#[test]
fn test_find_chapter_pages_correct_page_numbers() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to extract chapters");

    assert_eq!(chapters.get(&1), Some(&1), "Chapter 1 should be on page 1");
    assert_eq!(chapters.get(&2), Some(&2), "Chapter 2 should be on page 2");
    assert_eq!(chapters.get(&3), Some(&3), "Chapter 3 should be on page 3");
    assert_eq!(chapters.get(&4), Some(&4), "Chapter 4 should be on page 4");
}

#[test]
fn test_find_chapter_pages_returns_hashmap_with_correct_keys() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to extract chapters");

    for i in 1..=4 {
        assert!(
            chapters.contains_key(&i),
            "Expected chapter {} to be present",
            i
        );
    }
}

#[test]
fn test_find_chapter_pages_no_extra_chapters() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to extract chapters");

    assert!(!chapters.contains_key(&0), "Should not contain chapter 0");
    assert!(!chapters.contains_key(&5), "Should not contain chapter 5");
}
