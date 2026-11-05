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
use regex::Regex;

#[cfg(test)]
mod generate_mess_size_code_tests;

#[cfg(test)]
mod macro_tests;

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
                "int type_size;\n\
                 PMPI_Type_size({}, &type_size);\n\
                 {} (MPI_Count) {} * type_size;",
                $datatype, $decl_or_update, $count
            ))
        } else {
            Ok(format!(
                "MPI_Fint type_size;\n\
                 MPI_Fint ierror_type_size;\n\
                 pmpi_type_size{}({}, &type_size, &ierror_type_size);\n\
                 {} (MPI_Count) *{} * type_size;",
                $xlanguage.1, $datatype, $decl_or_update, $count
            ))
        }
    };
}
#[cfg(test)]
pub(crate) use mess_size;

/// Generates a string that contains C code for computing the size of an MPI message.
///
/// First, the parameter call_info.1 is joined to a single string with `,` as separator.
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
