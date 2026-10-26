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
// File: test_communicator_argument.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/arguments/test_communicator_argument.rs
// Author: Dr. Christoph Pospiech

use super::extract_first_mpi_communicator_argument;
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;

#[test]
fn extract_first_mpi_communicator_argument_finds_single_comm() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
            "int dest".to_string(),
            "int tag".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    );

    let name = extract_first_mpi_communicator_argument(&call_info)?;

    assert_eq!(name, "comm");
    Ok(())
}

#[test]
fn extract_first_mpi_communicator_argument_returns_first_of_multiple() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Intercomm_create".to_string(),
        vec![
            "MPI_Comm local_comm".to_string(),
            "int local_leader".to_string(),
            "MPI_Comm peer_comm".to_string(),
            "int remote_leader".to_string(),
        ],
    );

    let name = extract_first_mpi_communicator_argument(&call_info)?;

    assert_eq!(name, "local_comm");
    Ok(())
}

#[test]
fn extract_first_mpi_communicator_argument_ignores_similar_names() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Comm_test".to_string(),
        vec![
            "MPI_Comm_errhandler_function* function".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    );

    let name = extract_first_mpi_communicator_argument(&call_info)?;

    assert_eq!(name, "comm");
    Ok(())
}

#[test]
fn extract_first_mpi_communicator_argument_returns_empty_when_absent() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Barrier".to_string(),
        vec!["int count".to_string(), "MPI_Datatype datatype".to_string()],
    );

    let name = extract_first_mpi_communicator_argument(&call_info);

    assert!(name.is_err());
    Ok(())
}
