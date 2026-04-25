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
// File: template.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/template.rs
// Author: Dr. Christoph Pospiech

use crate::mpi_calls::MpiCallInfo;
use anyhow::Result;
use dynfmt::{Format, PythonFormat};

#[cfg(test)]
mod generate_output;

/// Generates an output string based on a template. The template is defined as a
/// string with placeholders that can be replaced with actual values. The function
/// takes a template string, a chapter number, a chapter title and one item of MPI call
/// information (name and parameters) and returns a formatted string where the placeholders
/// in the template are replaced.
///
/// The placeholders in the template should follow the old Python string formatting syntax
/// as described in <https://docs.python.org/2/library/stdtypes.html#string-formatting-operations>
/// `%(placeholder_name)[...]<d|i|o|x|s|...>` and will be processed by the `dynfmt` crate using
/// the `PythonFormat` struct. The function will return a Result containing the formatted string.
///
/// Args:
///     template: A string containing the template with placeholders to be replaced.
///     chapter_number: The number of the chapter to be included in the output string.
///     chapter_title: The title of the chapter to be included in the output string.
///     mpi_call_info: A tuple containing the MPI call name and its parameters.
///
/// Returns:
///    A Result containing the formatted string with placeholders replaced by actual values.
///   If an error occurs during formatting, the Result will contain an error message.
///
/// Example:
/// ```no_run
/// use mpi_wrap_gen::template::generate_output;
///
/// let template = "Chapter %(chapter_number)d: %(chapter_title)s ";
/// let chapter_number = 3;
/// let chapter_title = "Point-to-Point Communication";
/// let mpi_call_info = ("Send".to_string(),
///                      vec!["buf".to_string(),
///                           "count".to_string(),
///                           "datatype".to_string(),
///                           "dest".to_string(),
///                           "tag".to_string(),
///                           "comm".to_string()]);
/// let result = generate_output(template, chapter_number, chapter_title, &mpi_call_info);
/// match result {
///     Ok(output) => println!("Generated output: {}", output),
///     Err(e) => eprintln!("Error generating output: {}", e),
/// }
/// ```
pub fn generate_output(
    template: &str,
    chapter_number: u32,
    chapter_title: &str,
    mpi_call_info: &MpiCallInfo,
) -> Result<String> {
    let vars = std::collections::HashMap::<String, String>::from([
        ("chapter_number".to_string(), chapter_number.to_string()),
        ("chapter_title".to_string(), chapter_title.to_string()),
        ("mpi_call_name".to_string(), mpi_call_info.0.clone()),
        (
            "mpi_call_name_lc".to_string(),
            mpi_call_info.0.to_lowercase(),
        ),
        (
            "mpi_call_name_id".to_string(),
            format!("{}_id", mpi_call_info.0.to_lowercase()),
        ),
        (
            "mpi_call_params".to_string(),
            format!("{:?}", mpi_call_info.1),
        ),
    ]);
    let output = PythonFormat
        .format(template, &vars)
        .map_err(|e| anyhow::anyhow!("{e}"))?
        .into_owned();
    Ok(output)
}
