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

fn main() -> sysexits::ExitCode {
    let mut check = cargo_optimise::Application::new("cargo", vec!["check"]);
    let mut clippy = cargo_optimise::Application::new(
        "cargo",
        vec!["clippy", "--fix", "--allow-dirty", "--allow-staged"],
    );
    let mut metadata = cargo_optimise::Application::new("cargo", vec!["metadata"]);
    let mut format = cargo_optimise::Application::new("cargo", vec!["fmt"]);

    match metadata.handle(
        Some("This is not a Rust project!"),
        sysexits::ExitCode::Usage,
    ) {
        None => (),
        Some(code) => return code,
    }

    match format.handle(None, sysexits::ExitCode::DataErr) {
        None => (),
        Some(code) => return code,
    }

    match check.handle(None, sysexits::ExitCode::DataErr) {
        None => (),
        Some(code) => return code,
    }

    match clippy.handle(None, sysexits::ExitCode::DataErr) {
        None => (),
        Some(code) => return code,
    }

    sysexits::ExitCode::Ok
}

/******************************************************************************/
