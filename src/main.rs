/*********************** GNU General Public License 2.0 ***********************\
|                                                                              |
|  Copyright (C) 2022 Kevin Matthes                                            |
|                                                                              |
|  This program is free software; you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation; either version 2 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License along     |
|  with this program; if not, write to the Free Software Foundation, Inc.,     |
|  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.                 |
|                                                                              |
\******************************************************************************/

// Third party libraries.
use clap::Parser;

/// The main function.
///
/// It composes the required functionalities and takes care for the error event
/// handling as well as the return status.
fn main() -> sysexits::ExitCode {
    const PROCESSES: usize = 5;

    let args = rs_optimise::CliOptions::parse();

    if args.license() {
        rs_optimise::license();
        return sysexits::ExitCode::Ok;
    }

    let applications = vec!["cargo".into(); PROCESSES];

    let mut arguments = vec![vec!["metadata".into()]];
    arguments.push(vec![
        "clippy".into(),
        "--fix".into(),
        "--allow-dirty".into(),
        "--allow-staged".into(),
    ]);
    arguments.push(vec!["fmt".into()]);
    arguments.push(vec!["check".into()]);
    arguments.push(vec![
        "clippy".into(),
        "--".into(),
        "-D".into(),
        "clippy::all".into(),
        "-D".into(),
        "clippy::cargo".into(),
        "-D".into(),
        "clippy::complexity".into(),
        "-D".into(),
        "clippy::correctness".into(),
        "-D".into(),
        "clippy::nursery".into(),
        "-D".into(),
        "clippy::perf".into(),
        "-D".into(),
        "clippy::pedantic".into(),
        "-D".into(),
        "clippy::suspicious".into(),
        "-D".into(),
        "clippy::style".into(),
    ]);
    let arguments = arguments;

    let mut error_messages = vec![None; PROCESSES];
    error_messages[0] = Some("This is not a Cargo maintained Rust project".into());
    let error_messages = error_messages;

    let mut exit_codes = vec![sysexits::ExitCode::DataErr; PROCESSES];
    exit_codes[0] = sysexits::ExitCode::Usage;
    let exit_codes = exit_codes;

    let mut verbosities = vec![args.verbosity(); PROCESSES];
    verbosities[0].silent();
    let verbosities = verbosities;

    rs_optimise::Application::new(
        applications,
        arguments,
        error_messages,
        exit_codes,
        verbosities,
    )
    .run()
}

/******************************************************************************/
