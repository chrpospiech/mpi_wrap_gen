use super::format_mpi_definition;
use crate::config::Languages;
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;

#[test]
fn format_mpi_definition_c_formats_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
        ],
    );

    let formatted = format_mpi_definition(&Languages::C, 0, &call_info)?;

    assert_eq!(
        formatted,
        "MPI_Send(const void* buf, int count, MPI_Datatype datatype)"
    );
    Ok(())
}

#[test]
fn format_mpi_definition_handles_empty_parameter_list() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec![]);

    let formatted = format_mpi_definition(&Languages::C, 0, &call_info)?;

    assert_eq!(formatted, "MPI_Barrier()");
    Ok(())
}

#[test]
fn format_mpi_definition_formats_fortran_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Send".to_string(),
        vec![
            "const void* buf".to_string(),
            "int count".to_string(),
            "MPI_Datatype datatype".to_string(),
            "char* name".to_string(),
        ],
    );

    let formatted = format_mpi_definition(&Languages::Fortran, 2, &call_info)?;

    assert_eq!(
        formatted,
        format!(
            "mpi_send__({}, {})",
            "void* buf, MPI_Fint *count, MPI_Fint *datatype",
            "char *name, MPI_Fint *ierror, MPI_Fint name_len"
        )
    );
    Ok(())
}

#[test]
fn format_mpi_definition_normalizes_fortran_handle_arrays() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Waitsome".to_string(),
        vec![
            "MPI_Request array_of_requests[]".to_string(),
            "char* argv[]".to_string(),
        ],
    );

    let formatted = format_mpi_definition(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(
        formatted,
        "mpi_waitsome(MPI_Fint array_of_requests[], char *argv[], MPI_Fint *ierror, MPI_Fint argv_len)"
    );
    Ok(())
}

#[test]
fn format_mpi_definition_fortran_handles_intent_in_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Comm_rank".to_string(),
        vec!["MPI_Comm comm".to_string(), "int *rank".to_string()],
    );

    let formatted = format_mpi_definition(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(
        formatted,
        "mpi_comm_rank(MPI_Fint *comm, MPI_Fint *rank, MPI_Fint *ierror)"
    );
    Ok(())
}

#[test]
fn format_mpi_definition_fortran_handles_aint_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Get_address".to_string(),
        vec![
            "const void* location".to_string(),
            "MPI_Aint *address".to_string(),
        ],
    );

    let formatted = format_mpi_definition(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(
        formatted,
        "mpi_get_address(void* location, MPI_Aint *address, MPI_Fint *ierror)"
    );
    Ok(())
}
