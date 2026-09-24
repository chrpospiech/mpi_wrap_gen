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

    let formatted = format_mpi_call(&Languages::C, &call_info)?;

    assert_eq!(
        formatted,
        "Send(const void* buf, int count, MPI_Datatype datatype)"
    );
    Ok(())
}

#[test]
fn format_mpi_call_handles_empty_parameter_list() -> Result<()> {
    let call_info: MpiCallInfo = ("Barrier".to_string(), vec![]);

    let formatted = format_mpi_call(&Languages::C, &call_info)?;

    assert_eq!(formatted, "Barrier()");
    Ok(())
}

#[test]
fn format_mpi_call_rejects_unsupported_language() -> Result<()> {
    let call_info: MpiCallInfo = (
        "Bcast".to_string(),
        vec!["void* buffer".to_string(), "int count".to_string()],
    );

    let result = format_mpi_call(&Languages::Fortran, &call_info);

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported language"));
    Ok(())
}
