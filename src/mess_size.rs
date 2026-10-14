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

use crate::config::Languages;
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;
use regex::Regex;

#[cfg(test)]
mod generate_mess_size_code_tests;

/// Generates a string that contains C code for computing the size of an MPI message.
///
/// First, the parameter call_info.1 is joined to a single string with `,` as separator.
/// Then this string is searched for matches of the following regular expression:
///
/// ```regex
/// .*void\*\s+\w+,int\s+(\w+),MPI_Datatype\s+(\w+)
/// ```
///
/// If no matches are found, the string "MPI_Count message_size=0;" is returned.
/// If a match is found, the variable names captured by the regular expression are
/// placed into the generated C code to compute the message size.
/// Depending on the values of `language` and `underscores` the generated code
/// contains the appropriate call to MPI_Type_size to extract the size of the
/// MPI datatype into a variable type_size. Then the total message size is
/// computed as `MPI_Count message_size = (MPI_Count) count * type_size;`
///
/// # Arguments
/// * `language` - The programming language for which to generate the code.
/// * `underscores` - The number of underscores to use for Fortran calls.
/// * `chapter` - The chapter number of the MPI standard relevant to the call.
/// * `call_info` - Information about the MPI call.
///
/// # Returns
/// A `Result` containing the generated C code as a `String` if successful.
///
/// # Example
/// ```no_run
/// use mpi_wrap_gen::config::Languages;
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
/// let result = generate_mess_size_code(&Languages::C, 0, 3, &call_info);
/// match result {
///     Ok(code) => println!("Generated code: {}", code),
///     Err(e) => eprintln!("Error generating code: {}", e),
/// }
/// ```
///
pub fn generate_mess_size_code(
    language: &Languages,
    underscores: u8,
    chapter: u32,
    call_info: &MpiCallInfo,
) -> Result<String> {
    // No MPI calls with message sizes in chapter 4
    if chapter == 4 {
        return Ok("MPI_Count message_size=0;".to_string());
    }
    // Join call_info.1 into a single string with `,` as separator
    let joined_parameters = call_info.1.join(",");
    // Search for matches of the regular expression in the joined parameters string
    let re = Regex::new(r".*void\*\s+\w+,int\s+(\w+),MPI_Datatype\s+(\w+)")?;
    let captures = re.captures(&joined_parameters);
    // If no matches are found, return a default message size of 0.
    if captures.is_none() {
        return Ok("MPI_Count message_size=0;".to_string());
    }
    let count_var = &captures.as_ref().unwrap()[1];
    let datatype_var = &captures.as_ref().unwrap()[2];
    if matches!(language, Languages::C) {
        Ok(format!(
            "int type_size;\n\
            PMPI_Type_size({}, &type_size);\n\
            MPI_Count message_size = (MPI_Count) {} * type_size;",
            datatype_var, count_var
        ))
    } else {
        Ok(format!(
            "MPI_Count type_size;\n\
            int ierror_type_size;\n\
            pmpi_type_size{}({}, &type_size, &ierror_type_size);\n\
            MPI_Count message_size = (MPI_Count) *{} * type_size;",
            "_".repeat(underscores as usize),
            datatype_var,
            count_var
        ))
    }
}
