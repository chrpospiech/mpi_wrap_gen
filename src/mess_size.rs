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
// File: mess_size.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mess_size.rs
// Author: Dr. Christoph Pospiech

use crate::config::{Languages, XLanguage};
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;
use phf::phf_map;
use regex::Regex;

#[cfg(test)]
mod generate_mess_size_code_tests;

#[cfg(test)]
mod macro_tests;

#[cfg(test)]
mod sendrecv;

#[cfg(test)]
mod scatter;

/// Macro for extracting the variable name from a parameter declaration string.
/// It takes a name of a variable of type `MpiCallInfo` and a position in the parameter list,
/// and returns the variable name as a reference to str.
///
/// # Arguments
/// * `mpi_call_info` - The `MpiCallInfo` instance containing information about the MPI call.
/// * `position` - The position of the parameter in the parameter list.
///
/// # Returns
/// The name of the variable at the specified position as a `&str`.
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
/// ```
///
macro_rules! var_name {
    ($mpi_call_info:ident, $index:expr) => {
        $mpi_call_info.1[$index].split_whitespace().last().unwrap()
    };
}
#[cfg(test)]
pub(crate) use var_name;

/// Macro for creating a C code snippet to compute MPI message size.
/// The first argument specifies how the message size variable should be declared
/// or updated in the generated C code. It it a string expression, typically
/// some thing like `"int message_size = "` or `"message_size += "`.
/// The second and third arguments are the variable names of the count and
/// datatype parameters, respectively. The last argument is a variable name of
/// type XLanguage.
///
/// # Arguments
/// * `decl_or_update` - A string specifying how the message size variable
///   should be declared or updated.
/// * `count_var` - The variable name of the count parameter.
/// * `datatype_var` - The variable name of the datatype parameter.
/// * `xlanguage_var` - The variable name of type XLanguage.
///
/// # Returns
/// A `String` containing the generated C code snippet for computing the MPI message size.
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
    ($decl_or_update:expr, $count:ident, $datatype:ident, $xlanguage:ident) => {
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
}
#[cfg(test)]
pub(crate) use mess_size;

/// Macro for generating C code to set a boolean variable `is_root` indicating
/// whether a rank is the root in a collective operation in a communicator.
/// As it is strictly illegal to call a collective operation from a rank that
/// is not part of the communicator, this macro only generates code to check
/// if the current rank is the root.
///
/// # Arguments
/// * `root_var` - The variable name of the root rank.
/// * `comm_var` - The variable name of the communicator.
/// * `xlanguage_var` - The variable name of type XLanguage.
///
/// # Returns
/// A `String` containing the generated C code snippet for checking
/// if a rank is the root in the communicator.
///
/// # Example
/// ```no_run
/// let root_var = "root";
/// let comm_var = "comm";
/// let xlanguage_var = "xlanguage";
/// let code = is_root!(root_var, comm_var, xlanguage_var);
/// ```
///
macro_rules! is_root {
    ($root:ident, $comm:ident, $xlanguage:ident) => {
        if matches!($xlanguage.0, Languages::C) {
            Ok(format!(
                "int rank;\n\
                 MPI_Comm_rank({}, &rank);\n\
                 bool is_root = (rank == {});",
                $comm, $root
            ))
        } else {
            Ok(format!(
                "MPI_Fint rank;\n\
                 MPI_Fint ierror_rank;\n\
                 pmpi_comm_rank{}({}, &rank, &ierror_rank);\n\
                 bool is_root = (rank == *{});",
                $xlanguage.1, $comm, $root
            ))
        }
    };
}
#[cfg(test)]
pub(crate) use is_root;

/// Type alias for a function pointer that generates message size code for a specific MPI call.
/// This is used by [`SPECIAL_GENERATORS`] to override the default message size computation
/// in [`generate_mess_size_code`] for MPI calls whose message size can't be derived by the
/// generic single count/datatype pattern (e.g. because they have two message sizes, or because
/// the size depends on the calling rank being root in a collective operation).
///
/// # Arguments
/// * `call_info` - Information about the MPI call.
/// * `xlanguage` - A tuple containing the target programming language and\
///   the underscore string for Fortran calls.
///
/// # Returns
/// A `Result` containing the generated C code as a `String` if successful.
type SpecialMessageSizeGenerator = fn(&MpiCallInfo, &XLanguage) -> Result<String>;

/// [`SpecialMessageSizeGenerator`] for MPI_Sendrecv.
/// Message size for sending and receiving in MPI_Sendrecv are added together.
fn generate_mess_size4sendrecv(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    let send_count_var = var_name!(call_info, 1);
    let send_datatype_var = var_name!(call_info, 2);
    let dest_var = var_name!(call_info, 3);
    let recv_count_var = var_name!(call_info, 6);
    let recv_datatype_var = var_name!(call_info, 7);
    let source_var = var_name!(call_info, 8);
    if matches!(xlanguage.0, Languages::C) {
        Ok(format!(
            "MPI_Count message_size = 0;\n\
             if ({dest} != MPI_PROC_NULL) {{\n\
             int {send_type}_size;\n\
             PMPI_Type_size({send_type}, &{send_type}_size);\n\
             message_size += (MPI_Count) {send_count} * {send_type}_size;\n\
             }}\n\
             if ({source} != MPI_PROC_NULL) {{\n\
             int {recv_type}_size;\n\
             PMPI_Type_size({recv_type}, &{recv_type}_size);\n\
             message_size += (MPI_Count) {recv_count} * {recv_type}_size;\n\
             }}",
            dest = dest_var,
            send_type = send_datatype_var,
            send_count = send_count_var,
            source = source_var,
            recv_type = recv_datatype_var,
            recv_count = recv_count_var
        ))
    } else {
        Ok(format!(
            "MPI_Count message_size = 0;\n\
             if (*{dest} != MPI_PROC_NULL) {{\n\
             MPI_Fint {send_type}_size;\n\
             MPI_Fint ierror_{send_type}_size;\n\
             pmpi_type_size{underscore}({send_type}, &{send_type}_size, &ierror_{send_type}_size);\n\
             message_size += (MPI_Count) *{send_count} * {send_type}_size;\n\
             }}\n\
             if (*{source} != MPI_PROC_NULL) {{\n\
             MPI_Fint {recv_type}_size;\n\
             MPI_Fint ierror_{recv_type}_size;\n\
             pmpi_type_size{underscore}({recv_type}, &{recv_type}_size, &ierror_{recv_type}_size);\n\
             message_size += (MPI_Count) *{recv_count} * {recv_type}_size;\n\
             }}",
            dest = dest_var,
            send_type = send_datatype_var,
            send_count = send_count_var,
            source = source_var,
            recv_type = recv_datatype_var,
            recv_count = recv_count_var,
            underscore = xlanguage.1
        ))
    }
}

/// [`SpecialMessageSizeGenerator`] for MPI_Scatter.
/// For intracommunicators, non-root processes use the receive message size and
/// the root uses the send size for all other processes. Intercommunicator root
/// processes use the send size for the remote group, while receiving processes
/// use the receive message size.
fn generate_mess_size4scatter(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    let send_count_var = var_name!(call_info, 1);
    let send_datatype_var = var_name!(call_info, 2);
    let recv_count_var = var_name!(call_info, 4);
    let recv_datatype_var = var_name!(call_info, 5);
    let root_var = var_name!(call_info, 6);
    let comm_var = var_name!(call_info, 7);
    Ok(if matches!(xlanguage.0, Languages::C) {
        format!(
            "int is_intercomm;\n\
             PMPI_Comm_test_inter({comm}, &is_intercomm);\n\
             MPI_Count message_size = 0;\n\
             if (is_intercomm) {{\n\
             if ({root} == MPI_ROOT) {{\n\
             int remote_size;\n\
             PMPI_Comm_remote_size({comm}, &remote_size);\n\
             int {send_datatype}_size;\n\
             PMPI_Type_size({send_datatype}, &{send_datatype}_size);\n\
             message_size += (MPI_Count) remote_size * {send_count} * {send_datatype}_size;\n\
             }} else if ({root} != MPI_PROC_NULL) {{\n\
             int {recv_datatype}_size;\n\
             PMPI_Type_size({recv_datatype}, &{recv_datatype}_size);\n\
             message_size += (MPI_Count) {recv_count} * {recv_datatype}_size;\n\
             }}\n\
             }} else {{\n\
             int rank;\n\
             PMPI_Comm_rank({comm}, &rank);\n\
             if (rank == {root}) {{\n\
             int comm_size;\n\
             PMPI_Comm_size({comm}, &comm_size);\n\
             int {send_datatype}_size;\n\
             PMPI_Type_size({send_datatype}, &{send_datatype}_size);\n\
             message_size += (MPI_Count) (comm_size - 1) * {send_count} * {send_datatype}_size;\n\
             }} else {{\n\
             int {recv_datatype}_size;\n\
             PMPI_Type_size({recv_datatype}, &{recv_datatype}_size);\n\
             message_size += (MPI_Count) {recv_count} * {recv_datatype}_size;\n\
             }}\n\
             }}",
            comm = comm_var,
            root = root_var,
            send_datatype = send_datatype_var,
            send_count = send_count_var,
            recv_datatype = recv_datatype_var,
            recv_count = recv_count_var
        )
    } else {
        format!(
            "MPI_Fint is_intercomm;\n\
             MPI_Fint ierror_comm_test_inter;\n\
             pmpi_comm_test_inter{underscore}({comm}, &is_intercomm, &ierror_comm_test_inter);\n\
             MPI_Count message_size = 0;\n\
             if (is_intercomm) {{\n\
             if (*{root} == MPI_ROOT) {{\n\
             MPI_Fint remote_size;\n\
             MPI_Fint ierror_remote_size;\n\
             pmpi_comm_remote_size{underscore}({comm}, &remote_size, &ierror_remote_size);\n\
             MPI_Fint {send_datatype}_size;\n\
             MPI_Fint ierror_{send_datatype}_size;\n\
             pmpi_type_size{underscore}({send_datatype}, &{send_datatype}_size, &ierror_{send_datatype}_size);\n\
             message_size += (MPI_Count) remote_size * *{send_count} * {send_datatype}_size;\n\
             }} else if (*{root} != MPI_PROC_NULL) {{\n\
             MPI_Fint {recv_datatype}_size;\n\
             MPI_Fint ierror_{recv_datatype}_size;\n\
             pmpi_type_size{underscore}({recv_datatype}, &{recv_datatype}_size, &ierror_{recv_datatype}_size);\n\
             message_size += (MPI_Count) *{recv_count} * {recv_datatype}_size;\n\
             }}\n\
             }} else {{\n\
             MPI_Fint rank;\n\
             MPI_Fint ierror_rank;\n\
             pmpi_comm_rank{underscore}({comm}, &rank, &ierror_rank);\n\
             if (rank == *{root}) {{\n\
             MPI_Fint comm_size;\n\
             MPI_Fint ierror_comm_size;\n\
             pmpi_comm_size{underscore}({comm}, &comm_size, &ierror_comm_size);\n\
             MPI_Fint {send_datatype}_size;\n\
             MPI_Fint ierror_{send_datatype}_size;\n\
             pmpi_type_size{underscore}({send_datatype}, &{send_datatype}_size, &ierror_{send_datatype}_size);\n\
             message_size += (MPI_Count) (comm_size - 1) * *{send_count} * {send_datatype}_size;\n\
             }} else {{\n\
             MPI_Fint {recv_datatype}_size;\n\
             MPI_Fint ierror_{recv_datatype}_size;\n\
             pmpi_type_size{underscore}({recv_datatype}, &{recv_datatype}_size, &ierror_{recv_datatype}_size);\n\
             message_size += (MPI_Count) *{recv_count} * {recv_datatype}_size;\n\
             }}\n\
             }}",
            underscore = xlanguage.1,
            comm = comm_var,
            root = root_var,
            send_datatype = send_datatype_var,
            send_count = send_count_var,
            recv_datatype = recv_datatype_var,
            recv_count = recv_count_var
        )
    })
}

static SPECIAL_GENERATORS: phf::Map<&'static str, SpecialMessageSizeGenerator> = phf_map! {
    "Sendrecv" => generate_mess_size4sendrecv,
    "Scatter" => generate_mess_size4scatter,
};

/// Generates a string that contains C code for computing the size of an MPI message.
///
/// First, the function looks up the MPI call (i.e. `call_info.0`) in the
/// special generators map.
///
///  * If a special generator is found for the MPI call, it is invoked to
///    generate the message size code.
///  * If no special generator is found, the function proceeds with the default
///    message size computation logic.
///
/// The default message size computation logic starts by joining the parameter
/// `call_info.1` is joined to a single string with `,` as separator.
/// Then this string is searched for matches of the following regular expression:
///
/// ```regex
/// .*void\*\s+\w+,int\s+(\w+),MPI_Datatype\s+(\w+)
/// ```
///
/// If no matches are found, the string "int message_size = -1;" is returned.
/// The default message size of -1 indicates that this MPI call does not have a
/// meaningful message size.
///
/// If a match is found, the variable names captured by the regular expression are
/// placed into the generated C code to compute the message size.
/// Depending on the values of `language` and `underscores` the generated code
/// contains the appropriate call to MPI_Type_size to extract the size of the
/// MPI datatype into a variable type_size. Then the total message size is
/// computed as `int message_size = count * type_size;`
///
/// The data type of `message_size` has to be `int` to match the
///
/// # Arguments
/// * `call_info` - Information about the MPI call.
/// * `xlanguage` - A tuple containing the target programming language and\
///   the underscore string for Fortran calls.
///
/// # Returns
/// A `Result` containing the generated C code as a `String` if successful.
///
/// # Example
/// ```no_run
/// use mpi_wrap_gen::config::{Languages, XLanguage, create_xlanguage};
/// use mpi_wrap_gen::mess_size::generate_mess_size_code;
/// use mpi_wrap_gen::mpi_calls::MpiCallInfo;
///
/// let call_info: MpiCallInfo = ("Send".to_string(),
///     vec!["const void* buf".to_string(),
///          "int count".to_string(),
///          "MPI_Datatype datatype".to_string(),
///          "int dest".to_string(),
///          "int tag".to_string(),
///          "MPI_Comm comm".to_string()]);
/// let xlanguage = create_xlanguage(Languages::C, 0);
/// let result = generate_mess_size_code(&call_info, &xlanguage);
/// match result {
///     Ok(code) => println!("Generated code: {}", code),
///     Err(e) => eprintln!("Error generating code: {}", e),
/// }
/// ```
///
pub fn generate_mess_size_code(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    // Check if there is a special generator for this MPI call
    if let Some(generator) = SPECIAL_GENERATORS.get(call_info.0.as_str()) {
        return generator(call_info, xlanguage);
    }
    // Join call_info.1 into a single string with `,` as separator
    let joined_parameters = call_info.1.join(",");
    // Search for matches of the regular expression in the joined parameters string
    let re = Regex::new(r".*void\*\s+\w+,int\s+(\w+),MPI_Datatype\s+(\w+)")?;
    let captures = re.captures(&joined_parameters);
    // If no matches are found, return a default message size of -1.
    if captures.is_none() {
        return Ok("MPI_Count message_size = -1;".to_string());
    }
    let count_var = &captures.as_ref().unwrap()[1];
    let datatype_var = &captures.as_ref().unwrap()[2];
    // dummy call to the var_name macro to extract the variable name for the count parameter
    let _count_var = var_name!(call_info, 1);
    mess_size!(
        "MPI_Count message_size =",
        count_var,
        datatype_var,
        xlanguage
    )
}
