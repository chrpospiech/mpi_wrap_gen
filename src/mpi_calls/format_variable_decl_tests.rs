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
// File: format_variable_decl_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls/format_variable_decl_tests.rs
// Author: Dr. Christoph Pospiech

use super::*;

#[test]
fn test_format_variable_declaration_pointer() -> Result<()> {
    // Space is inserted after * when immediately followed by a word character.
    assert_eq!(format_variable_declaration("void*buf")?, "void* buf");
    Ok(())
}

#[test]
fn test_format_variable_declaration_const_void() -> Result<()> {
    // Full example from the docstring.
    let result = format_variable_declaration("constvoid*buf")?;
    assert_eq!(result, "const void* buf");
    Ok(())
}

#[test]
fn test_format_variable_declaration_int_identifier() -> Result<()> {
    // Keyword at start of string followed directly by an identifier.
    let result = format_variable_declaration("intcount")?;
    assert_eq!(result, "int count");
    Ok(())
}

#[test]
fn test_format_variable_declaration_wrong_spacing() -> Result<()> {
    // Keyword at start of string followed a * in the wrong spacing.
    let result = format_variable_declaration("int* count")?;
    assert_eq!(result, "int *count");
    Ok(())
}

#[test]
fn test_format_variable_declaration_already_spaced() -> Result<()> {
    // Already-correct input must not be changed.
    let result = format_variable_declaration("const void* buf")?;
    assert_eq!(result, "const void* buf");
    Ok(())
}

#[test]
fn test_format_variable_declaration_additional_mpi_types_and_pointer_forms() -> Result<()> {
    assert_eq!(
        format_variable_declaration("MPI_User_function*user_fn")?,
        "MPI_User_function *user_fn"
    );
    assert_eq!(
        format_variable_declaration("MPI_Message *message")?,
        "MPI_Message *message"
    );
    assert_eq!(
        format_variable_declaration("const void *buf")?,
        "const void* buf"
    );
    assert_eq!(format_variable_declaration("char ***argv")?, "char ***argv");
    Ok(())
}

#[test]
fn test_format_variable_declaration_array() -> Result<()> {
    assert_eq!(format_variable_declaration("int arr[]")?, "int arr[]");
    assert_eq!(
        format_variable_declaration("constchar*str[]")?,
        "const char *str[]"
    );
    Ok(())
}

#[test]
fn test_format_variable_declaration_nested_array() -> Result<()> {
    assert_eq!(
        format_variable_declaration("intranges[][3]")?,
        "int ranges[][3]"
    );
    Ok(())
}

#[test]
fn test_format_variable_declaration_greedy_leading_part() -> Result<()> {
    // Leading part should be empty as opposed to
    // "MPI_A" and variable "int disp" being incorrectly split.
    assert_eq!(
        format_variable_declaration("MPI_Aintdisp ")?,
        "MPI_Aint disp"
    );
    // Similar for int array_of_integers[]
    assert_eq!(
        format_variable_declaration("intarray_of_integers[]")?,
        "int array_of_integers[]"
    );
    Ok(())
}

#[test]
fn test_format_variable_declaration_cannot_parse() -> Result<()> {
    // Ensure that parsing an unknown text returns an error.
    assert!(format_variable_declaration("unknowntext").is_err());
    // Check the error message.
    if let Err(e) = format_variable_declaration("unknowntext") {
        assert!(e
            .to_string()
            .contains("Failed to extract typed variable declaration"));
    }
    Ok(())
}
