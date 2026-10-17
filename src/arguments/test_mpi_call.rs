use super::format_mpi_call;
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

    let formatted = format_mpi_call(&Languages::C, 0, &call_info)?;

    assert_eq!(formatted, "MPI_Send(buf, count, datatype)");
    Ok(())
}

#[test]
fn format_mpi_call_handles_empty_parameter_list() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec![]);

    let formatted = format_mpi_call(&Languages::C, 0, &call_info)?;

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

    let formatted = format_mpi_call(&Languages::Fortran, 2, &call_info)?;

    assert_eq!(
        formatted,
        format!(
            "mpi_send__({}, {})",
            "buf, count, datatype", "name, ierror, name_len"
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

    let formatted = format_mpi_call(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(
        formatted,
        "mpi_waitsome(array_of_requests, argv, ierror, argv_len)"
    );
    Ok(())
}
