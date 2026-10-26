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
// File: arguments.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/arguments.rs
// Author: Dr. Christoph Pospiech

use crate::config::Languages;
use crate::mpi_calls::{c_data_types_regex, MpiCallInfo};
use anyhow::{bail, Result};
use regex::Regex;

#[cfg(test)]
mod test_communicator_argument;

#[cfg(test)]
mod test_mpi_defs;

#[cfg(test)]
mod test_mpi_call;

/// Extracts the name of the (first) MPI communicator argument from the MPI
/// call parameters.
///
/// First, the parameter call_info.1 is joined to a single string with
/// `,` as separator. Then this string is searched for matches of the
/// following regular expression:
///
/// ```regex
/// .*?MPI_Comm\s+(\w+)
/// ```
///
/// If no matches are found, an empty string is returned. Otherwise, the value
/// captured by the regular expression, which corresponds to the name of the
/// first MPI communicator argument, is returned.
///
/// # Arguments
/// * `call_info` - A tuple containing the MPI call name and its parameters.
///
/// # Returns
/// A `Result` containing the name of the first MPI communicator argument as a string
/// or an empty string if no communicator argument is found.
pub fn extract_first_mpi_communicator_argument(call_info: &MpiCallInfo) -> Result<String> {
    let joined_parameters = call_info.1.join(",");
    let re = Regex::new(r".*?MPI_Comm\s+(\w+)")?;
    if let Some(captures) = re.captures(&joined_parameters) {
        return Ok(captures[1].to_string());
    }
    Ok(String::new())
}

/// Static set of C data types used in MPI calls that are passed through unchanged
/// to the C routine called by the corresponding MPI call in Fortran. Fortran calls
/// by name, hence all C data types are pointer types, most of them pointer to
/// MPI_Fint. This set lists the data types that are **not** mapped to MPI_Fint*.
static MPI_C_TO_C_TYPES: phf::Set<&'static str> = phf::phf_set! {
    "void*",
    "char",
    "MPI_Aint",
    "MPI_Count",
    "MPI_Offset",
    "MPI_Comm_copy_attr_function",
    "MPI_Comm_delete_attr_function",
    "MPI_Comm_errhandler_fn",
    "MPI_Comm_errhandler_function",
    "MPI_Copy_function",
    "MPI_Datarep_conversion_function",
    "MPI_Datarep_extent_function",
    "MPI_Delete_function",
    "MPI_File_errhandler_fn",
    "MPI_File_errhandler_function",
    "MPI_Grequest_cancel_function",
    "MPI_Grequest_free_function",
    "MPI_Grequest_query_function",
    "MPI_Handler_function",
    "MPI_Type_copy_attr_function",
    "MPI_Type_delete_attr_function",
    "MPI_User_function",
    "MPI_Win_copy_attr_function",
    "MPI_Win_delete_attr_function",
    "MPI_Win_errhandler_fn",
    "MPI_Win_errhandler_function",
};

/// Formats the extension arguments of an MPI function in the Fortran language.
/// The extension arguments are additional parameters required for Fortran
/// compatibility, such as
/// 1. The error code.
/// 2. The string length for character arguments.
///
/// In the special case of `MPI_Comm_spawn` and `MPI_Comm_spawn_multiple`,
/// a vector of strings (C type char* argv[]) representing the command-line
/// arguments is also included in the argument list. The MPI standard
/// requires that all strings in this array (or array of arrays) have the same length.
/// Hence a single string length argument is sufficient to represent the length of
/// all strings in the array. Hence no special handling is required for arrays of strings.
/// We only have to strip the `[]` suffix from the argument name.
///
/// Arguments
/// * `call_info` - A tuple containing the MPI call name and its parameters.
///
/// # Returns
/// A `Result` containing the formatted extension arguments as a vector of strings
/// (same format as the original argument list `call_info.1`)
/// or an error if formatting fails.
fn format_mpi_extension_arguments_fortran(call_info: &MpiCallInfo) -> Result<Vec<String>> {
    let mut extension_args: Vec<String> = Vec::new();
    // Add the error code argument
    extension_args.push("MPI_Fint *ierror".to_string());
    // Add string length arguments for each character argument
    for param in &call_info.1 {
        if let Some(captures) = c_data_types_regex()?.captures(param.trim()) {
            let data_type = captures[2].split_whitespace().collect::<String>();
            let pointer_part = &captures[3];
            if data_type == "char" && !pointer_part.is_empty() {
                extension_args.push(format!("MPI_Fint {}_len", &captures[4]));
            }
        }
    }
    Ok(extension_args)
}

/// Formats the definition of an MPI function in the Fortran language.
/// The wrapper itself is written in C. The argument list is adapted to
/// match the Fortran calling conventions. This is done in the following steps:
/// 1. Each argument in the call_info tuple is examined and split into its type
///    and name components. This step is done using the regular expression
///    returned by `c_data_types_regex()`.
/// 2. The argument types are checked against the MPI_C_TO_C_TYPES set.
/// 3. If the type of the argument is not found in the MPI_C_TO_C_TYPES set,
///    it is set to "MPI_Fint"; otherwise it is left unchanged.
/// 4. If the argument is neither an array nor a pointer, an additional `*` is
///    prepended to the argument name. In all other cases, the argument name is left as is.
/// 5. An additional argument "MPI_Fint *ierror" is appended to the argument list
///    for Fortran compatibility.
/// 6. For each argument of type "char*", an additional "MPI_Fint" argument is appended
///    to represent the string length.
/// 7. The formatted arguments are then combined into a single string representing the MPI call
///    and returned as the result of the function.
///
/// # Arguments
/// * `underscores` - The number of underscores to use for Fortran calls.
/// * `call_info` - A tuple containing the MPI call name and its parameters.
///
/// # Returns
/// A `Result` containing the formatted MPI call as a string, or an error if formatting fails.
///
fn format_mpi_definition_fortran(underscores: u8, call_info: &MpiCallInfo) -> Result<String> {
    let call_name = format!(
        "mpi_{}{}",
        call_info.0.to_lowercase(),
        "_".repeat(underscores as usize)
    );
    let mut formatted_args: Vec<String> = Vec::new();

    for param in &call_info.1 {
        let s = param.trim();
        // parameter void means "no argument" => leave alone.
        if s == "void" {
            continue;
        }
        let typed_variable_signature_re = c_data_types_regex()?;
        // If s matches the MPI call signature regular expression, return a formatted string
        // composed from joining the matching groups.
        if !typed_variable_signature_re.is_match(s) {
            bail!("Failed to extract typed variable declaration: {}", s)
        }
        let captures = typed_variable_signature_re.captures(s).unwrap();
        let data_type = captures[2].to_string();
        let pointer_part = captures[3].to_string();
        let name_part = captures[4].to_string();
        let array_part = captures[5].to_string();
        // Handle the data type
        let mapped_type = if MPI_C_TO_C_TYPES.contains(data_type.as_str()) {
            data_type.as_str()
        } else {
            "MPI_Fint"
        };
        let pointer_part =
            if mapped_type != "void*" && pointer_part.is_empty() && array_part.is_empty() {
                "*"
            } else {
                &pointer_part
            };
        formatted_args.push(format!(
            "{} {}{}{}",
            mapped_type, pointer_part, name_part, array_part
        ));
    }

    formatted_args.extend(format_mpi_extension_arguments_fortran(call_info)?);

    Ok(format!("{}({})", call_name, formatted_args.join(", ")))
}

/// Formats an MPI call definition as a string based on the target programming language.
///
/// This function takes the target language and MPI call information, and returns
/// a string representation of the MPI call suitable for the specified language.
///
/// # Arguments
/// * `language` - The target programming language for which to format the MPI call.
/// * `underscores` - The number of underscores to use for Fortran calls.
/// * `call_info` - A tuple containing the MPI call name and its parameters.
///
/// # Returns
/// A `Result` containing the formatted MPI call as a string, or an error if formatting fails.
///
/// # Example
///
/// ```rust
/// use crate::config::Languages;
/// use crate::mpi_calls::MpiCallInfo;
/// use crate::arguments::format_mpi_definition;
///
/// let call_info: MpiCallInfo = ("Send".to_string(),
///                      vec!["const void* buf".to_string(),
///                           "int count".to_string(),
///                           "MPI_Datatype datatype".to_string(),
///                           "int dest".to_string(),
///                           "int tag".to_string(),
///                           "MPI_Comm comm".to_string()])
/// ;
/// let formatted_call = format_mpi_definition(&Languages::C, 0, &call_info).unwrap();
/// println!("{}", formatted_call);
/// ```
///
pub fn format_mpi_definition(
    language: &Languages,
    underscores: u8,
    call_info: &MpiCallInfo,
) -> Result<String> {
    match language {
        Languages::C => {
            let call_name = format!("MPI_{}", call_info.0);
            let params = call_info.1.join(", ");
            Ok(format!("{}({})", call_name, params))
        }
        Languages::Fortran => format_mpi_definition_fortran(underscores, call_info),
    }
}

/// Formats the call of an MPI function as a string for the C programming language.
/// This function generates a string representation of an MPI function call in C,
/// including the function name and its parameters, but without the data type definitions.
/// This is done in the following steps:
/// 1. Each argument in the call_info tuple is examined and split into its type
///    and name components.
/// 2. The argument names are extracted from the type-name pairs by taking the
///    last component and stripping any potential `[]` suffix.
/// 3. The formatted argument list is constructed by combining the argument names.
/// 4. The function name created according to the language and underscore conventions.
/// 5. The final MPI call string is generated by concatenating the function name
///    with the formatted argument list.
///
/// # Arguments
/// * `language` - The target programming language for which to format the MPI call.
/// * `underscores` - The number of underscores to use for Fortran calls.
/// * `call_info` - A tuple containing the MPI call name and its parameters.
///
/// # Returns
/// A `Result` containing the formatted MPI call as a string, or an error if formatting fails.
///
/// # Example
///
/// ```rust
/// use crate::config::Languages;
/// use crate::mpi_calls::MpiCallInfo;
/// use crate::arguments::format_mpi_call;
///
/// let call_info: MpiCallInfo = ("Send".to_string(),
///                      vec!["const void* buf".to_string(),
///                           "int count".to_string(),
///                           "MPI_Datatype datatype".to_string(),
///                           "int dest".to_string(),
///                           "int tag".to_string(),
///                           "MPI_Comm comm".to_string()])
/// ;
/// let formatted_call = format_mpi_call(&Languages::C, 0, &call_info).unwrap();
/// println!("{}", formatted_call);
/// ```
///
pub fn format_mpi_call(
    language: &Languages,
    underscores: u8,
    call_info: &MpiCallInfo,
) -> Result<String> {
    let mut ext_params = call_info.1.clone();
    if let Languages::Fortran = language {
        ext_params.extend(format_mpi_extension_arguments_fortran(call_info)?);
    }
    let pointer_re = r"(\*+|)".to_string();
    let identifier_re = r"(\w+)".to_string();
    let array_re = r"(?:(?:\[[^\]]*\])*)".to_string();
    let param_re = Regex::new(&format!(
        r".*\s+{}{}{}$",
        pointer_re, identifier_re, array_re
    ))?;
    let extended_params = ext_params
        .iter()
        .filter(|arg| arg.trim() != "void")
        .map(|arg| -> Result<String> {
            let caps = param_re
                .captures(arg)
                .ok_or_else(|| anyhow::anyhow!("failed to parse parameter: {}", arg))?;
            Ok(caps[2].to_string())
        })
        .collect::<Result<Vec<String>>>()?
        .join(", ");
    let call_name = match language {
        Languages::C => format!("MPI_{}", call_info.0),
        Languages::Fortran => format!(
            "mpi_{}{}",
            call_info.0.to_lowercase(),
            "_".repeat(underscores as usize)
        ),
    };
    Ok(format!("{}({})", call_name, extended_params))
}
