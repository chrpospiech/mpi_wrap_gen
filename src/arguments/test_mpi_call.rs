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
fn format_mpi_call_handles_c_complex_parameter_list() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Testall".to_string(),
        vec![
            "int count".to_string(),
            "MPI_Request array_of_requests[]".to_string(),
            "int *flag".to_string(),
            "MPI_Status array_of_statuses[]".to_string(),
        ],
    );

    let formatted = format_mpi_call(&Languages::C, 0, &call_info)?;

    assert_eq!(
        formatted,
        "MPI_Testall(count, array_of_requests, flag, array_of_statuses)"
    );
    Ok(())
}

#[test]
fn format_mpi_call_handles_empty_parameter_list_for_fortran() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec![]);

    let formatted = format_mpi_call(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(formatted, "mpi_barrier(ierror)");
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
            "char *name".to_string(),
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
            "char *argv[]".to_string(),
        ],
    );

    let formatted = format_mpi_call(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(
        formatted,
        "mpi_waitsome(array_of_requests, argv, ierror, argv_len)"
    );
    Ok(())
}

#[test]
fn format_mpi_call_fortran_handles_intent_in_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Comm_rank".to_string(),
        vec!["MPI_Comm comm".to_string(), "int *rank".to_string()],
    );

    let formatted = format_mpi_call(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(formatted, "mpi_comm_rank(comm, rank, ierror)");
    Ok(())
}

#[test]
fn format_mpi_call_fortran_handles_aint_arguments() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Get_address".to_string(),
        vec![
            "const void* location".to_string(),
            "MPI_Aint *address".to_string(),
        ],
    );

    let formatted = format_mpi_call(&Languages::Fortran, 0, &call_info)?;

    assert_eq!(formatted, "mpi_get_address(location, address, ierror)");
    Ok(())
}
