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
use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;

#[cfg(test)]
mod test_mpi_call;

/// function to compose a string representation of MPI call name(arguments) based
/// on the provided language and call information.
///
/// # Arguments
/// * `language` - The target programming language for which to format the MPI call.
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
/// let formatted_call = format_mpi_call(&Languages::C, &call_info).unwrap();
/// println!("{}", formatted_call);
/// ```
///
pub fn format_mpi_call(language: &Languages, call_info: &MpiCallInfo) -> Result<String> {
    match language {
        &Languages::C => {
            let call_name = &call_info.0;
            let params = call_info.1.join(", ");
            Ok(format!("{}({})", call_name, params))
        }
        _ => Err(anyhow::anyhow!("Unsupported language")),
    }
}
