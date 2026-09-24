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
// File: generate_output.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/template/generate_output.rs
// Author: Dr. Christoph Pospiech

use super::generate_output as generate_output_with_language;
use crate::config::Languages;
use anyhow::Result;

fn generate_output(
    template: &str,
    chapter_number: u32,
    chapter_title: &str,
    mpi_call_info: &(String, Vec<String>),
) -> Result<String> {
    generate_output_with_language(
        template,
        chapter_number,
        chapter_title,
        mpi_call_info,
        &Languages::C,
    )
}

fn sample_call_info() -> (String, Vec<String>) {
    (
        "Send".to_string(),
        vec![
            "buf".to_string(),
            "count".to_string(),
            "datatype".to_string(),
            "dest".to_string(),
            "tag".to_string(),
            "comm".to_string(),
        ],
    )
}

#[test]
fn test_chapter_number_substitution() -> Result<()> {
    let template = "Chapter %(chapter_number)d";
    let info = sample_call_info();
    let output = generate_output(template, 3, "Title", &info)?;
    assert_eq!(output, "Chapter 3");
    Ok(())
}

#[test]
fn test_chapter_title_substitution() -> Result<()> {
    let template = "%(chapter_title)s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Point-to-Point Communication", &info)?;
    assert_eq!(output, "Point-to-Point Communication");
    Ok(())
}

#[test]
fn test_mpi_call_name_substitution() -> Result<()> {
    let template = "MPI_%(mpi_call_name)s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "MPI_Send");
    Ok(())
}

#[test]
fn test_mpi_call_params_substitution() -> Result<()> {
    let template = "%(mpi_call_params)s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(
        output,
        r#"["buf", "count", "datatype", "dest", "tag", "comm"]"#
    );
    Ok(())
}

#[test]
fn test_all_placeholders() -> Result<()> {
    let template =
        "Chapter %(chapter_number)d: %(chapter_title)s - %(mpi_call_name)s(%(mpi_call_params)s)";
    let info = sample_call_info();
    let output = generate_output(template, 3, "Point-to-Point Communication", &info)?;
    assert_eq!(
        output,
        r#"Chapter 3: Point-to-Point Communication - Send(["buf", "count", "datatype", "dest", "tag", "comm"])"#
    );
    Ok(())
}

#[test]
fn test_template_without_placeholders() -> Result<()> {
    let template = "no placeholders here";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "no placeholders here");
    Ok(())
}

#[test]
fn test_empty_template() -> Result<()> {
    let template = "";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "");
    Ok(())
}

#[test]
fn test_empty_params() -> Result<()> {
    let template = "%(mpi_call_params)s";
    let info = ("Barrier".to_string(), vec![]);
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "[]");
    Ok(())
}

#[test]
fn test_mpi_call_name_lc_substitution() -> Result<()> {
    let template = "%(mpi_call_name_lc)s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "send");
    Ok(())
}

#[test]
fn test_mpi_call_name_id_substitution() -> Result<()> {
    let template = "%(mpi_call_name_id)s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    assert_eq!(output, "send_id,");
    Ok(())
}

#[test]
fn test_mpi_call_name_id_left_aligned_padding() -> Result<()> {
    let template = "%(mpi_call_name_id)-20s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "send_id," is 8 chars, padded on the right to width 20
    assert_eq!(output, "send_id,            ");
    assert_eq!(output.len(), 20);
    Ok(())
}

#[test]
fn test_mpi_call_name_id_right_aligned_padding() -> Result<()> {
    let template = "%(mpi_call_name_id)20s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "send_id," is 8 chars, padded on the left to width 20
    assert_eq!(output, "            send_id,");
    assert_eq!(output.len(), 20);
    Ok(())
}

#[test]
fn test_mpi_call_name_id_exact_width() -> Result<()> {
    let template = "%(mpi_call_name_id)8s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "send_id," is exactly 8 chars — no padding applied
    assert_eq!(output, "send_id,");
    Ok(())
}

#[test]
fn test_mpi_call_name_id_width_less_than_value() -> Result<()> {
    let template = "%(mpi_call_name_id)5s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // width < actual length: no truncation, value passed through unchanged
    assert_eq!(output, "send_id,");
    Ok(())
}

#[test]
fn test_mpi_call_name_lc_left_aligned_padding() -> Result<()> {
    let template = "%(mpi_call_name_lc)-10s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "send" is 4 chars, padded on the right to width 10
    assert_eq!(output, "send      ");
    assert_eq!(output.len(), 10);
    Ok(())
}

#[test]
fn test_mpi_call_name_left_aligned_padding() -> Result<()> {
    let template = "%(mpi_call_name)-8s";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "Send" is 4 chars, padded on the right to width 8
    assert_eq!(output, "Send    ");
    assert_eq!(output.len(), 8);
    Ok(())
}

#[test]
fn test_left_aligned_padding_in_context() -> Result<()> {
    let template = "id=%(mpi_call_name_id)-10s;";
    let info = sample_call_info();
    let output = generate_output(template, 1, "Title", &info)?;
    // "send_id," is 8 chars, padded on the right to width 10, wrapped in context
    assert_eq!(output, "id=send_id,  ;");
    Ok(())
}

#[test]
fn test_invalid_placeholder_returns_error() {
    let template = "%(unknown_key)s";
    let info = sample_call_info();
    let result = generate_output(template, 1, "Title", &info);
    assert!(result.is_err());
}
