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
// File: mpi_calls.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/mpi_calls.rs
// Author: Dr. Christoph Pospiech

use anyhow::Result;
use lopdf::Document;

/// Extracts MPI call information from the PDF document and returns a vector of MPI call names
/// and call parameters. The MPI calls are identified by searching for text elements that match
/// the pattern "int MPI_[A-Z]\w+\(.*\)" in the PDF content streams. The closing bracket may
/// be on the same line or on a subsequent line, so the function handles both cases.
///
/// Args:
///     doc: A reference to the lopdf PDF document from which to extract MPI call information.
///     start_page: The 1-based index of the page from which to start searching for MPI calls.
///     end_page: The 1-based (exclusive) index of the page at which to stop searching for MPI calls.
///
/// Returns:
///     A Result containing a vector of tuples, where each tuple consists of the MPI call name
///     and a vector of its parameters. If an error occurs during PDF processing, the Result
///     will contain an error message.
pub fn extract_mpi_calls(
    doc: &Document,
    start_page: u32,
    end_page: u32,
) -> Result<Vec<(String, Vec<String>)>> {
    let mut mpi_calls = Vec::new();
    let re = regex::Regex::new(r"int\s+MPI_([A-Z]\w+)\s*\((.*?)\)")?;

    for page_num in start_page..end_page {
        let text = doc.extract_text(&[page_num]).unwrap_or_default();

        for cap in re.captures_iter(&text) {
            let call_name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let params_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            let params: Vec<String> = params_str
                .split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect();

            mpi_calls.push((call_name, params));
        }
    }

    Ok(mpi_calls)
}
