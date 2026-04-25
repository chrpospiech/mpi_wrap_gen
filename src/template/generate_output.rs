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

use super::generate_output;
use anyhow::Result;

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
    assert_eq!(output, "send_id");
    Ok(())
}

#[test]
fn test_invalid_placeholder_returns_error() {
    let template = "%(unknown_key)s";
    let info = sample_call_info();
    let result = generate_output(template, 1, "Title", &info);
    assert!(result.is_err());
}
