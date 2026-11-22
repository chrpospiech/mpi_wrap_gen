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
use crate::macros::{comm_partners, mess_size, var_name};
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;
use phf::phf_map;
use regex::Regex;

#[cfg(test)]
mod generate_mess_size_code_tests;

#[cfg(test)]
mod sendrecv;

#[cfg(test)]
mod scatter;

#[cfg(test)]
mod gather;
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
fn generate_mess_size4scattergather(
    call_info: &MpiCallInfo,
    xlanguage: &XLanguage,
) -> Result<String> {
    let comm_part: Result<String> = comm_partners!(
        call_info,
        root_inx: 6,
        comm_inx: 7,
        xlanguage
    );
    let send_part: Result<String> = mess_size!(
        "message_size = remote_size *",
        call_info,
        1, datatype_inx:2,
        xlanguage
    );
    let recv_part: Result<String> = mess_size!(
        "message_size = remote_size *",
        call_info,
        4, datatype_inx:5,
        xlanguage
    );
    if call_info.0 == "Scatter" || call_info.0 == "Iscatter" {
        Ok(format!(
            "{}\n\
            MPI_Count message_size = 0;\n\
            if (is_root) {{\n\
            {}\n\
            }} else {{\n\
            {}\n\
            }}",
            comm_part?, send_part?, recv_part?
        ))
    } else {
        Ok(format!(
            "{}\n\
            MPI_Count message_size = 0;\n\
            if (is_root) {{\n\
            {}\n\
            }} else {{\n\
            {}\n\
            }}",
            comm_part?, recv_part?, send_part?
        ))
    }
}

static SPECIAL_GENERATORS: phf::Map<&'static str, SpecialMessageSizeGenerator> = phf_map! {
    "Sendrecv" => generate_mess_size4sendrecv,
    "Scatter" => generate_mess_size4scattergather,
    "Iscatter" => generate_mess_size4scattergather,
    "Gather" => generate_mess_size4scattergather,
    "Igather" => generate_mess_size4scattergather,
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
