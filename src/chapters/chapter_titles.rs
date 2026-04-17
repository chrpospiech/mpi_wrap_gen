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
// File: chapter_titles.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/chapters/chapter_titles.rs
// Author: Dr. Christoph Pospiech

use crate::chapters::ChapterMap;
use anyhow::Result;

use super::get_chapter_title;

#[test]
fn test_get_chapter_title_valid() -> Result<()> {
    let mut map: ChapterMap = std::collections::HashMap::new();
    map.insert(1, (1, "Introduction".to_string()));
    map.insert(3, (42, "Point-to-Point Communication".to_string()));

    assert_eq!(get_chapter_title(&map, 1)?, "Introduction");
    assert_eq!(get_chapter_title(&map, 3)?, "Point-to-Point Communication");
    Ok(())
}

#[test]
fn test_get_chapter_title_missing_chapter() {
    let map: ChapterMap = std::collections::HashMap::new();
    assert!(get_chapter_title(&map, 99).is_err());
}

#[test]
fn test_get_chapter_title_empty_title() -> Result<()> {
    let mut map: ChapterMap = std::collections::HashMap::new();
    map.insert(1, (1, String::new()));

    assert_eq!(get_chapter_title(&map, 1)?, "");
    Ok(())
}

#[test]
fn test_get_chapter_title_returns_clone() -> Result<()> {
    let mut map: ChapterMap = std::collections::HashMap::new();
    map.insert(1, (1, "Introduction".to_string()));

    let mut title = get_chapter_title(&map, 1)?;
    title.push_str(" modified");

    // Calling again should return the original value, proving independence.
    let title2 = get_chapter_title(&map, 1)?;
    assert_eq!(title2, "Introduction");
    assert_ne!(title, title2);
    Ok(())
}
