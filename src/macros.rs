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
// File: macros.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/macros.rs
// Author: Dr. Christoph Pospiech

#[cfg(test)]
mod macro_tests;

/// Macro for extracting the variable name from a parameter declaration string.
/// It takes a name of a variable of type `MpiCallInfo` and a position in the parameter list,
/// and returns the variable name as a reference to str.
///
/// An optional third argument of type XLanguage can be given. If the language
/// is Fortran, the returned variable name is prefixed with `*` (since Fortran
/// MPI calls pass this parameter by reference) and returned as an owned `String`
/// instead of a `&str`.
///
/// # Arguments
/// * `mpi_call_info` - The `MpiCallInfo` instance containing information about the MPI call.
/// * `position` - The position of the parameter in the parameter list.
/// * `xlanguage` - (optional) An expression of type XLanguage.
///
/// # Returns
/// The name of the variable at the specified position, as a `&str` if `xlanguage`
/// is not given, or as a `String` (prefixed with `*` for Fortran) if it is.
///
/// # Example
/// ```no_run
/// let call_info: MpiCallInfo = (
///     "Send".to_string(),
///     vec![
///         "const void* buf".to_string(),
///         "int count".to_string(),
///         "MPI_Datatype datatype".to_string(),
///     ],
/// );
/// let variable_name = var_name!(call_info, 1);
/// let variable_name_f = var_name!(call_info, 1, xlanguage);
/// ```
///
macro_rules! var_name {
    ($mpi_call_info:ident, $index:expr) => {
        $mpi_call_info.1[$index].split_whitespace().last().unwrap()
    };
    ($mpi_call_info:ident, $index:expr, $xlanguage:ident) => {
        if matches!($xlanguage.0, Languages::C) {
            var_name!($mpi_call_info, $index).to_string()
        } else {
            format!("*{}", var_name!($mpi_call_info, $index))
        }
    };
}
pub(crate) use var_name;

/// Macro for creating a C code snippet to compute MPI message size.
/// The first argument specifies how the message size variable should be declared
/// or updated in the generated C code. It it a string expression, typically
/// some thing like `"int message_size = "` or `"message_size += "`.
/// The last argument is an expression of type XLanguage.
///
/// There are two forms:
/// * `mess_size!(decl_or_update, count, datatype, xlanguage)` - `count` and
///   `datatype` are expressions evaluating to the variable names of the count
///   and datatype parameters, respectively.
/// * `mess_size!(decl_or_update, mpi_call_info, count_inx, datatype_inx:datatype_inx, xlanguage)` -
///   `count` and `datatype` variable names are extracted from `mpi_call_info`
///   via [`var_name!`] using the given argument indices. The literal token
///   `datatype_inx:` is required to disambiguate the two index expressions.
///
/// # Arguments
/// * `decl_or_update` - A string specifying how the message size variable
///   should be declared or updated.
/// * `count` - An expression evaluating to the variable name of the count parameter.
/// * `datatype` - An expression evaluating to the variable name of the datatype parameter.
/// * `mpi_call_info` - A [`MpiCallInfo`] identifier to extract `count` and
///   `datatype` variable names from, by argument index.
/// * `count_inx` - The argument index of the count parameter in `mpi_call_info`.
/// * `datatype_inx` - The argument index of the datatype parameter in `mpi_call_info`.
/// * `xlanguage` - An expression of type XLanguage.
///
/// # Returns
/// A `Result<String>` containing the generated C code snippet for computing the MPI message size.
///
/// # Example
/// ```no_run
/// let decl_or_update = "int message_size = ";
/// let count_var = "count";
/// let datatype_var = "datatype";
/// let xlanguage_var = "xlanguage";
/// let code = mess_size!(decl_or_update, count_var, datatype_var, xlanguage_var);
/// ```
///
macro_rules! mess_size {
    // Count and datatype based message size computation for MPI calls.
    ($decl_or_update:expr, $count:expr, $datatype:expr, $xlanguage:ident) => {
        if matches!($xlanguage.0, Languages::C) {
            Ok(format!(
                "int {type}_size;\n\
                 PMPI_Type_size({type}, &{type}_size);\n\
                 {} (MPI_Count) {} * {type}_size;",
                $decl_or_update, $count,
                type = $datatype
            ))
        } else {
            Ok(format!(
                "MPI_Fint {type}_size;\n\
                 MPI_Fint ierror_{type}_size;\n\
                 pmpi_type_size{}({type}, &{type}_size, &ierror_{type}_size);\n\
                 {} (MPI_Count) *{} * {type}_size;",
                $xlanguage.1, $decl_or_update, $count,
                type = $datatype
            ))
        }
    };
    // Count and datatype are extracted from $mpi_call_info
    ($decl_or_update:expr, $mpi_call_info:ident,
        $count_inx:expr, datatype_inx:$datatype_inx:expr,
        $xlanguage:ident) => {
        mess_size!(
            $decl_or_update,
            var_name!($mpi_call_info, $count_inx),
            var_name!($mpi_call_info, $datatype_inx),
            $xlanguage
        )
    };
}
pub(crate) use mess_size;

/// Macro for creating a C code snippet for determining the number of communication partners
/// in an MPI call with a one to many communication pattern.
/// The macro takes a [`MpiCallInfo`] identifier and the indices of the root process and
/// the communicator within the MPI call arguments. It also takes the target programming language
/// of the call API as an argument.
/// The code snippet generated by this macro will first check whether the communicator is
/// an inter-communicator or intra-communicator and then determine the number of
/// communication partners accordingly. The value is assigned to a variable `remote_size`
/// representing the number of communication partners. The code snippet also sets an integer
/// variable `is_root` to indicate whether the process is the root.
///
///
/// # Arguments
/// * `call_info` - Information about the MPI call.
/// * `root_inx` - Index of the root process in the MPI call arguments.
/// * `comm_inx` - Index of the communicator in the MPI call arguments.
/// * `xlanguage` - A tuple containing the target programming language and\
///   the underscore string for Fortran calls.
///
/// # Returns
/// A `Result` containing the generated C code as a `String` if successful.
macro_rules! comm_partners {
    ( $mpi_call_info:ident, root_inx:$root_inx:expr,
        comm_inx:$comm_inx:expr, $xlanguage:ident) => {
        if matches!($xlanguage.0, Languages::C) {
            Ok(format!(
                "int remote_size = 0;\n\
                int is_root = 0;\n\
                int is_intercomm;\n\
                PMPI_Comm_test_inter({comm}, &is_intercomm);\n\
                if (is_intercomm) {{\n\
                if ({root} == MPI_ROOT) {{\n\
                PMPI_Comm_remote_size({comm}, &remote_size);\n\
                is_root = 1;\n\
                }} else if ({root} != MPI_PROC_NULL) {{\n\
                remote_size = 1;\n\
                }}\n\
                // End of inter-communicator code\n\
                }} else {{\n\
                // Intra-communicator code\n\
                int rank;\n\
                PMPI_Comm_rank({comm}, &rank);\n\
                if (rank == {root}) {{\n\
                PMPI_Comm_size({comm}, &remote_size);\n\
                remote_size -= 1;\n\
                is_root = 1;\n\
                }} else {{\n\
                remote_size = 1;\n\
                }}\n\
                }}\n\
                // End of intra-communicator code",
                comm = var_name!($mpi_call_info, $comm_inx),
                root = var_name!($mpi_call_info, $root_inx),
            ))
        } else {
            Ok(format!(
                "MPI_Fint remote_size = 0;\n\
                MPI_Fint is_root = 0;\n\
                MPI_Fint is_intercomm;\n\
                MPI_Fint ierror_comm_test_inter;\n\
                pmpi_comm_test_inter{underscore}({comm}, &is_intercomm, \
                                     &ierror_comm_test_inter);\n\
                if (is_intercomm) {{\n\
                if (*{root} == MPI_ROOT) {{\n\
                MPI_Fint ierror_remote_size;\n\
                pmpi_comm_remote_size{underscore}({comm}, &remote_size, &ierror_remote_size);\n\
                is_root = 1;\n\
                }} else if (*{root} != MPI_PROC_NULL) {{\n\
                remote_size = 1;\n\
                }}\n\
                // End of inter-communicator code\n\
                }} else {{\n\
                // Intra-communicator code\n\
                MPI_Fint rank;\n\
                MPI_Fint ierror_comm_rank;\n\
                pmpi_comm_rank{underscore}({comm}, &rank, &ierror_comm_rank);\n\
                if (rank == *{root}) {{\n\
                MPI_Fint ierror_comm_size;\n\
                pmpi_comm_size{underscore}({comm}, &remote_size, &ierror_comm_size);\n\
                remote_size -= 1;\n\
                is_root = 1;\n\
                }} else {{\n\
                remote_size = 1;\n\
                }}\n\
                }}\n\
                // End of intra-communicator code",
                comm = var_name!($mpi_call_info, $comm_inx),
                root = var_name!($mpi_call_info, $root_inx),
                underscore = $xlanguage.1,
            ))
        }
    };
}
pub(crate) use comm_partners;
