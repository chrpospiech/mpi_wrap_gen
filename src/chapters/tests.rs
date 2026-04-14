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
// File: chapters/tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/chapters/tests.rs
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
fn test_find_chapter_pages_loads_all_chapters() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    assert_eq!(
        chapters.len(),
        4,
        "Expected 4 chapters, got {}",
        chapters.len()
    );
    Ok(())
}

#[test]
fn test_find_chapter_pages_correct_page_numbers() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

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
    Ok(())
}

#[test]
fn test_find_chapter_pages_returns_hashmap_with_correct_keys() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    for i in 1..=4 {
        assert!(
            chapters.contains_key(&i),
            "Expected chapter {} to be present",
            i
        );
    }
    Ok(())
}

#[test]
fn test_get_chapter_page_valid() -> Result<()> {
    let mut map: ChapterMap = HashMap::new();
    map.insert(1, (1, "Introduction".to_string()));
    map.insert(3, (42, "Point-to-Point Communication".to_string()));

    assert_eq!(get_chapter_page(&map, 1).unwrap(), 1);
    assert_eq!(get_chapter_page(&map, 3).unwrap(), 42);
    Ok(())
}

#[test]
fn test_get_chapter_page_missing_chapter() -> Result<()> {
    let map: ChapterMap = HashMap::new();
    assert!(get_chapter_page(&map, 99).is_err());
    Ok(())
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_get_chapter_page_out_of_range() -> Result<()> {
    let mut map: ChapterMap = HashMap::new();
    map.insert(1, (usize::MAX, "Introduction".to_string()));
    assert!(get_chapter_page(&map, 1).is_err());
    Ok(())
}

#[test]
fn test_find_chapter_pages_no_extra_chapters() -> Result<()> {
    let doc = Document::load(test_pdf_path())?;
    let chapters = find_chapter_pages(&doc)?;

    assert!(!chapters.contains_key(&0), "Should not contain chapter 0");
    assert!(!chapters.contains_key(&5), "Should not contain chapter 5");
    Ok(())
}
