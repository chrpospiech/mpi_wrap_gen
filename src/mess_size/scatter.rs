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
// File: scatter.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mess_size/scatter.rs
// Author: Dr. Christoph Pospiech

use super::*;
use crate::config::create_xlanguage;

fn scatter_call_info() -> MpiCallInfo {
    (
        "Scatter".to_string(),
        vec![
            "const void* sendbuf".to_string(),
            "int sendcount".to_string(),
            "MPI_Datatype sendtype".to_string(),
            "void* recvbuf".to_string(),
            "int recvcount".to_string(),
            "MPI_Datatype recvtype".to_string(),
            "int root".to_string(),
            "MPI_Comm comm".to_string(),
        ],
    )
}

#[test]
fn test_scatter_c() -> Result<()> {
    let call_info = scatter_call_info();
    let code = generate_mess_size_code(&call_info, &create_xlanguage(Languages::C, 0))?;
    assert_eq!(
        code,
        "int is_intercomm;\n\
        PMPI_Comm_test_inter(comm, &is_intercomm);\n\
        MPI_Count message_size = 0;\n\
        if (is_intercomm) {\n\
        if (root == MPI_ROOT) {\n\
        int remote_size;\n\
        PMPI_Comm_remote_size(comm, &remote_size);\n\
        int sendtype_size;\n\
        PMPI_Type_size(sendtype, &sendtype_size);\n\
        message_size += (MPI_Count) remote_size * sendcount * sendtype_size;\n\
        } else if (root != MPI_PROC_NULL) {\n\
        int recvtype_size;\n\
        PMPI_Type_size(recvtype, &recvtype_size);\n\
        message_size += (MPI_Count) recvcount * recvtype_size;\n\
        }\n\
        } else {\n\
        int rank;\n\
        PMPI_Comm_rank(comm, &rank);\n\
        if (rank == root) {\n\
        int comm_size;\n\
        PMPI_Comm_size(comm, &comm_size);\n\
        int sendtype_size;\n\
        PMPI_Type_size(sendtype, &sendtype_size);\n\
        message_size += (MPI_Count) (comm_size - 1) * sendcount * sendtype_size;\n\
        } else {\n\
        int recvtype_size;\n\
        PMPI_Type_size(recvtype, &recvtype_size);\n\
        message_size += (MPI_Count) recvcount * recvtype_size;\n\
        }\n\
        }"
    );
    Ok(())
}

#[test]
fn test_scatter_fortran() -> Result<()> {
    let call_info = scatter_call_info();
    let code = generate_mess_size_code(&call_info, &create_xlanguage(Languages::Fortran, 1))?;
    assert_eq!(
        code,
        "MPI_Fint is_intercomm;\n\
        MPI_Fint ierror_comm_test_inter;\n\
        pmpi_comm_test_inter_(comm, &is_intercomm, &ierror_comm_test_inter);\n\
        MPI_Count message_size = 0;\n\
        if (is_intercomm) {\n\
        if (*root == MPI_ROOT) {\n\
        MPI_Fint remote_size;\n\
        MPI_Fint ierror_remote_size;\n\
        pmpi_comm_remote_size_(comm, &remote_size, &ierror_remote_size);\n\
        MPI_Fint sendtype_size;\n\
        MPI_Fint ierror_sendtype_size;\n\
        pmpi_type_size_(sendtype, &sendtype_size, &ierror_sendtype_size);\n\
        message_size += (MPI_Count) remote_size * *sendcount * sendtype_size;\n\
        } else if (*root != MPI_PROC_NULL) {\n\
        MPI_Fint recvtype_size;\n\
        MPI_Fint ierror_recvtype_size;\n\
        pmpi_type_size_(recvtype, &recvtype_size, &ierror_recvtype_size);\n\
        message_size += (MPI_Count) *recvcount * recvtype_size;\n\
        }\n\
        } else {\n\
        MPI_Fint rank;\n\
        MPI_Fint ierror_rank;\n\
        pmpi_comm_rank_(comm, &rank, &ierror_rank);\n\
        if (rank == *root) {\n\
        MPI_Fint comm_size;\n\
        MPI_Fint ierror_comm_size;\n\
        pmpi_comm_size_(comm, &comm_size, &ierror_comm_size);\n\
        MPI_Fint sendtype_size;\n\
        MPI_Fint ierror_sendtype_size;\n\
        pmpi_type_size_(sendtype, &sendtype_size, &ierror_sendtype_size);\n\
        message_size += (MPI_Count) (comm_size - 1) * *sendcount * sendtype_size;\n\
        } else {\n\
        MPI_Fint recvtype_size;\n\
        MPI_Fint ierror_recvtype_size;\n\
        pmpi_type_size_(recvtype, &recvtype_size, &ierror_recvtype_size);\n\
        message_size += (MPI_Count) *recvcount * recvtype_size;\n\
        }\n\
        }"
    );
    Ok(())
}
