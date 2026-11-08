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
// File: sendrecv.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mess_size/sendrecv.rs
// Author: Dr. Christoph Pospiech

use super::*;
use crate::config::create_xlanguage;

fn sendrecv_call_info() -> MpiCallInfo {
    (
        "Sendrecv".to_string(),
        vec![
            "const void* buf".to_string(),
            "int sendcount".to_string(),
            "MPI_Datatype sendtype".to_string(),
            "int dest".to_string(),
            "int sendtag".to_string(),
            "void* recvbuf".to_string(),
            "int recvcount".to_string(),
            "MPI_Datatype recvtype".to_string(),
            "int source".to_string(),
            "int recvtag".to_string(),
            "MPI_Comm comm".to_string(),
            "MPI_Status *status".to_string(),
        ],
    )
}

#[test]
fn test_sendrecv_c() -> Result<()> {
    let call_info = sendrecv_call_info();
    let code = generate_mess_size_code(&call_info, &create_xlanguage(Languages::C, 0))?;
    assert_eq!(
        code,
        "MPI_Count message_size = 0;\n\
        if (dest != MPI_PROC_NULL) {\n\
        int sendtype_size;\n\
        PMPI_Type_size(sendtype, &sendtype_size);\n\
        message_size += (MPI_Count) sendcount * sendtype_size;\n\
        }\n\
        if (source != MPI_PROC_NULL) {\n\
        int recvtype_size;\n\
        PMPI_Type_size(recvtype, &recvtype_size);\n\
        message_size += (MPI_Count) recvcount * recvtype_size;\n\
        }"
    );
    Ok(())
}

#[test]
fn test_sendrecv_fortran() -> Result<()> {
    let call_info = sendrecv_call_info();
    let code = generate_mess_size_code(&call_info, &create_xlanguage(Languages::Fortran, 1))?;
    assert_eq!(
        code,
        "MPI_Count message_size = 0;\n\
        if (*dest != MPI_PROC_NULL) {\n\
        MPI_Fint sendtype_size;\n\
        MPI_Fint ierror_sendtype_size;\n\
        pmpi_type_size_(sendtype, &sendtype_size, &ierror_sendtype_size);\n\
        message_size += (MPI_Count) *sendcount * sendtype_size;\n\
        }\n\
        if (*source != MPI_PROC_NULL) {\n\
        MPI_Fint recvtype_size;\n\
        MPI_Fint ierror_recvtype_size;\n\
        pmpi_type_size_(recvtype, &recvtype_size, &ierror_recvtype_size);\n\
        message_size += (MPI_Count) *recvcount * recvtype_size;\n\
        }"
    );
    Ok(())
}
