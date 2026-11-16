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

#[cfg(test)]
mod gather;

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
#[cfg(test)]
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
#[cfg(test)]
pub(crate) use mess_size;

/// Macro for creating a C code snippet for determining the number of communication partners
/// in an MPI call with a one to many communication pattern.
/// The macro takes a [`MpiCallInfo`] identifier and the indices of the root process and
/// the communicator within the MPI call arguments. It also takes the target programming language
/// of the call API as an argument.
/// The code snippet generated by this macro will first check whether the communicator is
/// an inter-communicator or intra-communicator and then determine the number of
/// communication partners accordingly. The value is assigned to a variable `remote_size`
/// representing the number of communication partners.
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
                "int is_intercomm;\n\
                int remote_size = 0;\n\
                PMPI_Comm_test_inter({comm}, &is_intercomm);\n\
                if (is_intercomm) {{\n\
                if ({root} == MPI_ROOT) {{\n\
                PMPI_Comm_remote_size({comm}, &remote_size);\n\
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
                }} else {{\n\
                remote_size = 1;\n\
                }}\n\
                // End of intra-communicator code\n\
                ",
                comm = var_name!($mpi_call_info, $comm_inx),
                root = var_name!($mpi_call_info, $root_inx),
            ))
        } else {
            Ok(format!(
                "MPI_Fint is_intercomm;\n\
                MPI_Fint remote_size = 0;\n\
                MPI_Fint ierror_comm_test_inter;\n\
                pmpi_comm_test_inter{underscore}({comm}, &is_intercomm, \
                                     &ierror_comm_test_inter);\n\
                if (is_intercomm) {{\n\
                if (*{root} == MPI_ROOT) {{\n\
                MPI_Fint ierror_remote_size;\n\
                pmpi_comm_remote_size{underscore}({comm}, &remote_size, &ierror_remote_size);\n\
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
                }} else {{\n\
                remote_size = 1;\n\
                }}\n\
                // End of intra-communicator code\n\
                ",
                comm = var_name!($mpi_call_info, $comm_inx),
                root = var_name!($mpi_call_info, $root_inx),
                underscore = $xlanguage.1,
            ))
        }
    };
}
#[cfg(test)]
pub(crate) use comm_partners;

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
/// Message size for sending and receiving in MPI_Sendrecv are added together,
/// taking into account that either the source or destination may be `MPI_PROC_NULL`.
fn generate_mess_size4sendrecv(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    let dest_var = var_name!(call_info, 3, xlanguage);
    let source_var = var_name!(call_info, 8, xlanguage);
    let send_part: Result<String> = mess_size!(
        "message_size +=",
        call_info,
        1, datatype_inx:2,
        xlanguage
    );
    let recv_part: Result<String> = mess_size!(
        "message_size +=",
        call_info,
        6, datatype_inx:7,
        xlanguage
    );
    Ok(format!(
        "MPI_Count message_size = 0;\n\
        if ({} != MPI_PROC_NULL) {{\n\
        {}\n\
        }}\n\
        if ({} != MPI_PROC_NULL) {{\n\
        {}\n\
        }}",
        dest_var, send_part?, source_var, recv_part?
    ))
}

/// [`SpecialMessageSizeGenerator`] for MPI_Scatter.
/// First, the function determines the communication partners for the scatter operation.
/// Then, it computes the message size for the receive buffer based on the number of elements
/// and the datatype and multiplies it by the remote group size to get the total message size.
/// The send buffer message size per communication partner must be the same as the receive
/// message size, which is implicitly accounted for by multiplying the receive message
/// size by the remote group size.
fn generate_mess_size4scatter(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    let comm_part: Result<String> = comm_partners!(
        call_info,
        root_inx: 6,
        comm_inx: 7,
        xlanguage
    );
    let recv_part: Result<String> = mess_size!(
        "MPI_Count message_size = remote_size *",
        call_info,
        4, datatype_inx:5,
        xlanguage
    );
    Ok(format!("{}{}", comm_part?, recv_part?))
}

/// [`SpecialMessageSizeGenerator`] for MPI_Gather.
/// First, the function determines the communication partners for the gather operation.
/// Then, it computes the message size for the send buffer based on the number of elements
/// and the datatype and multiplies it by the remote group size to get the total message size.
/// The receive buffer message size per communication partner must be the same as the send
/// message size, which is implicitly accounted for by multiplying the send message
/// size by the remote group size.
fn generate_mess_size4gather(call_info: &MpiCallInfo, xlanguage: &XLanguage) -> Result<String> {
    let comm_part: Result<String> = comm_partners!(
        call_info,
        root_inx: 6,
        comm_inx: 7,
        xlanguage
    );
    let send_part: Result<String> = mess_size!(
        "MPI_Count message_size = remote_size *",
        call_info,
        1, datatype_inx:2,
        xlanguage
    );
    Ok(format!("{}{}", comm_part?, send_part?))
}

static SPECIAL_GENERATORS: phf::Map<&'static str, SpecialMessageSizeGenerator> = phf_map! {
    "Sendrecv" => generate_mess_size4sendrecv,
    "Scatter" => generate_mess_size4scatter,
    "Gather" => generate_mess_size4gather,
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
