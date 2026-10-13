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
// File: generate_mess_size_code_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mess_size/generate_mess_size_code_tests.rs
// Author: Dr. Christoph Pospiech

use super::*;

fn send_call_info() -> MpiCallInfo {
    (
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
            "int dest".to_string(),
            "int tag".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    )
}

#[test]
fn test_no_match_returns_default_message_size() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec!["MPI_Comm comm".to_string()]);
    let code = generate_mess_size_code(&Languages::C, 0, &call_info)?;
    assert_eq!(code, "MPI_Count message_size=0;");
    Ok(())
}

#[test]
fn test_c_language_uses_pmpi_type_size_call() -> Result<()> {
    let call_info = send_call_info();
    let code = generate_mess_size_code(&Languages::C, 0, &call_info)?;
    assert!(code.contains("PMPI_Type_size(datatype, &type_size);"));
    assert!(code.contains("MPI_Count message_size = count * type_size;"));
    Ok(())
}

#[test]
fn test_fortran_language_uses_pmpi_type_size_with_underscores() -> Result<()> {
    let call_info = send_call_info();
    let code = generate_mess_size_code(&Languages::Fortran, 2, &call_info)?;
    assert!(code.contains("pmpi_type_size__(datatype, &type_size, &ierror_type_size);"));
    assert!(code.contains("MPI_Count message_size = *count * type_size;"));
    Ok(())
}

#[test]
fn test_fortran_language_with_zero_underscores() -> Result<()> {
    let call_info = send_call_info();
    let code = generate_mess_size_code(&Languages::Fortran, 0, &call_info)?;
    assert!(code.contains("pmpi_type_size(datatype, &type_size, &ierror_type_size);"));
    Ok(())
}
