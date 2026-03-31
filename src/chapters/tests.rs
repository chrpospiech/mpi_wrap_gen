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

    assert_eq!(
        chapters.get(&1).map(|v| v.0),
        Some(1),
        "Chapter 1 should be on page 1"
    );
    assert_eq!(
        chapters.get(&2).map(|v| v.0),
        Some(2),
        "Chapter 2 should be on page 2"
    );
    assert_eq!(
        chapters.get(&3).map(|v| v.0),
        Some(3),
        "Chapter 3 should be on page 3"
    );
    assert_eq!(
        chapters.get(&4).map(|v| v.0),
        Some(4),
        "Chapter 4 should be on page 4"
    );

    // Also verify that the chapter titles are correctly extracted.
    assert_eq!(
        chapters.get(&1).map(|v| v.1.as_str()),
        Some("Introduction"),
        "Chapter 1 should have title 'Introduction'"
    );
    assert_eq!(
        chapters.get(&2).map(|v| v.1.as_str()),
        Some("Terms and Conventions"),
        "Chapter 2 should have title 'Terms and Conventions'"
    );
    assert_eq!(
        chapters.get(&3).map(|v| v.1.as_str()),
        Some("Point-to-Point Communication"),
        "Chapter 3 should have title 'Point-to-Point Communication'"
    );
    assert_eq!(
        chapters.get(&4).map(|v| v.1.as_str()),
        Some("Datatypes"),
        "Chapter 4 should have title 'Datatypes'"
    );
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
fn test_get_chapter_page_valid() {
    let mut map: ChapterMap = HashMap::new();
    map.insert(1, (1, "Introduction".to_string()));
    map.insert(3, (42, "Point-to-Point Communication".to_string()));

    assert_eq!(get_chapter_page(&map, 1).unwrap(), 1);
    assert_eq!(get_chapter_page(&map, 3).unwrap(), 42);
}

#[test]
fn test_get_chapter_page_missing_chapter() {
    let map: ChapterMap = HashMap::new();
    assert!(get_chapter_page(&map, 99).is_err());
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_get_chapter_page_out_of_range() {
    let mut map: ChapterMap = HashMap::new();
    map.insert(1, (usize::MAX, "Introduction".to_string()));
    assert!(get_chapter_page(&map, 1).is_err());
}

#[test]
fn test_find_chapter_pages_no_extra_chapters() {
    let doc = Document::load(test_pdf_path()).expect("Failed to load test PDF");
    let chapters = find_chapter_pages(&doc).expect("Failed to extract chapters");

    assert!(!chapters.contains_key(&0), "Should not contain chapter 0");
    assert!(!chapters.contains_key(&5), "Should not contain chapter 5");
}
