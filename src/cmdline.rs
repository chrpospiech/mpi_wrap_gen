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
// File: cmdline.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/cmdline.rs
// Author: Dr. Christoph Pospiech

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
pub struct CliArgs {
    /// Verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Configuration file to process (must be provided by the user)
    pub config_file: String,
}

pub fn parse_args() -> Result<CliArgs> {
    let mut args = CliArgs::parse();
    if args.config_file.is_empty() {
        anyhow::bail!("Configuration file must be provided.");
    }
    if args.dry_run {
        println!("Dry-run mode: Switching on verbose mode.");
        args.verbose = true;
    }
    Ok(args)
}

pub fn echo_args(args: &CliArgs) -> Result<()> {
    if args.dry_run {
        println!("Performing a dry run");
    }
    if args.verbose || args.dry_run {
        println!("Verbose: {}", args.verbose);
        println!("Dry run: {}", args.dry_run);
        println!("Configuration file: {}", args.config_file);
    };
    Ok(())
}
