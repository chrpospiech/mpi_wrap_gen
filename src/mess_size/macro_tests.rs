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
use crate::mess_size::{comm_partners, mess_size, var_name};
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

#[test]
fn test_comm_partners_macro_c() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Gather".to_string(),
        vec![
            "void* sendbuffer".to_string(),
            "int sendcount".to_string(),
            "MPI_Datatype sendtype".to_string(),
            "void* recvbuffer".to_string(),
            "int recvcount".to_string(),
            "MPI_Datatype recvtype".to_string(),
            "int root".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::C, 0);
    let result: Result<String> = comm_partners!(
        call_info,
        root_inx: 6,
        comm_inx: 7,
        xlanguage
    );
    assert!(result.is_ok());
    let result_str = &result.unwrap();
    let expected_str = "int is_intercomm;\n\
                int remote_size = 0;\n\
                PMPI_Comm_test_inter(comm, &is_intercomm);\n\
                if (is_intercomm) {\n\
                if (root == MPI_ROOT) {\n\
                PMPI_Comm_remote_size(comm, &remote_size);\n\
                } else if (root != MPI_PROC_NULL) {\n\
                remote_size = 1;\n\
                }\n\
                // End of inter-communicator code\n\
                } else {\n\
                // Intra-communicator code\n\
                int rank;\n\
                PMPI_Comm_rank(comm, &rank);\n\
                if (rank == root) {\n\
                PMPI_Comm_size(comm, &remote_size);\n\
                remote_size -= 1;\n\
                } else {\n\
                remote_size = 1;\n\
                }\n\
                // End of intra-communicator code\n\
                ";
    assert_eq!(result_str, expected_str);
    Ok(())
}

#[test]
fn test_comm_partners_macro_f() -> Result<()> {
    let call_info: &MpiCallInfo = &(
        "Gather".to_string(),
        vec![
            "void* sendbuffer".to_string(),
            "int sendcount".to_string(),
            "MPI_Datatype sendtype".to_string(),
            "void* recvbuffer".to_string(),
            "int recvcount".to_string(),
            "MPI_Datatype recvtype".to_string(),
            "int root".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    );
    let xlanguage = create_xlanguage(Languages::Fortran, 1);
    let result: Result<String> = comm_partners!(
        call_info,
        root_inx: 6,
        comm_inx: 7,
        xlanguage
    );
    assert!(result.is_ok());
    let result_str = &result.unwrap();
    let expected_str = "MPI_Fint is_intercomm;\n\
                MPI_Fint remote_size = 0;\n\
                MPI_Fint ierror_comm_test_inter;\n\
                pmpi_comm_test_inter_(comm, &is_intercomm, &ierror_comm_test_inter);\n\
                if (is_intercomm) {\n\
                if (*root == MPI_ROOT) {\n\
                MPI_Fint ierror_remote_size;\n\
                pmpi_comm_remote_size_(comm, &remote_size, &ierror_remote_size);\n\
                } else if (*root != MPI_PROC_NULL) {\n\
                remote_size = 1;\n\
                }\n\
                // End of inter-communicator code\n\
                } else {\n\
                // Intra-communicator code\n\
                MPI_Fint rank;\n\
                MPI_Fint ierror_comm_rank;\n\
                pmpi_comm_rank_(comm, &rank, &ierror_comm_rank);\n\
                if (rank == *root) {\n\
                MPI_Fint ierror_comm_size;\n\
                pmpi_comm_size_(comm, &remote_size, &ierror_comm_size);\n\
                remote_size -= 1;\n\
                } else {\n\
                remote_size = 1;\n\
                }\n\
                // End of intra-communicator code\n\
                ";
    assert_eq!(result_str, expected_str);
    Ok(())
}
