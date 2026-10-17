use super::format_mpi_definition;
use crate::config::Languages;
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;

#[test]
fn format_mpi_call_c_formats_arguments() -> Result<()> {
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
fn format_mpi_call_handles_empty_parameter_list() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec![]);

    let formatted = format_mpi_definition(&Languages::C, 0, &call_info)?;

    assert_eq!(formatted, "MPI_Barrier()");
    Ok(())
}

#[test]
fn format_mpi_call_formats_fortran_arguments() -> Result<()> {
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
            "void* buf, MPI_Fint* count, MPI_Fint* datatype",
            "char* name, MPI_Fint* ierror, MPI_Fint name_len"
        )
    );
    Ok(())
}

#[test]
fn format_mpi_call_normalizes_fortran_handle_arrays() -> Result<()> {
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
        "mpi_waitsome(MPI_Fint* array_of_requests, char** argv, MPI_Fint* ierror, MPI_Fint argv_len)"
    );
    Ok(())
}
