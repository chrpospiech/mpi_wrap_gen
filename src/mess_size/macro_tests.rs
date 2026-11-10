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
// File: macro_tests.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mess_size/macro_tests.rs
// Author: Dr. Christoph Pospiech

use crate::config::{create_xlanguage, Languages};
use crate::mess_size::{mess_size, var_name};
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;

#[test]
fn test_var_name_macro() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
        ],
    );
    let variable_name = var_name!(call_info, 1);
    assert_eq!(variable_name, "count");
    Ok(())
}

#[test]
fn test_var_name_macro_with_xlanguage() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::Fortran, 0);
    let variable_name = var_name!(call_info, 1, xlanguage);
    assert_eq!(variable_name, "*count");
    Ok(())
}

#[test]
fn test_mess_size_macro_c() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype type".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::C, 0);
    let count_var = var_name!(call_info, 1);
    let datatype_var = var_name!(call_info, 2);
    let result: Result<String> = mess_size!(
        "MPI_Count message_size =",
        count_var,
        datatype_var,
        xlanguage
    );
    assert!(result.is_ok());
    let result_str = &result.unwrap();
    assert_eq!(
        result_str,
        "int type_size;\n\
        PMPI_Type_size(type, &type_size);\n\
        MPI_Count message_size = (MPI_Count) count * type_size;"
    );
    Ok(())
}

#[test]
fn test_mess_size_macro_ext_c() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype type".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::C, 0);
    let result: Result<String> = mess_size!(
        "MPI_Count message_size =",
        call_info,
        1, datatype_inx:2,
        xlanguage
    );
    assert!(result.is_ok());
    let result_str = &result.unwrap();
    assert_eq!(
        result_str,
        "int type_size;\n\
        PMPI_Type_size(type, &type_size);\n\
        MPI_Count message_size = (MPI_Count) count * type_size;"
    );
    Ok(())
}

#[test]
fn test_mess_size_macro_f() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype type".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::Fortran, 1);
    let count_var = var_name!(call_info, 1);
    let datatype_var = var_name!(call_info, 2);
    let result: Result<String> = mess_size!(
        "MPI_Count message_size =",
        count_var,
        datatype_var,
        xlanguage
    );
    assert!(result.is_ok());
    let result_str = &result.unwrap();
    assert_eq!(
        result_str,
        "MPI_Fint type_size;\n\
        MPI_Fint ierror_type_size;\n\
        pmpi_type_size_(type, &type_size, &ierror_type_size);\n\
        MPI_Count message_size = (MPI_Count) *count * type_size;"
    );
    Ok(())
}
