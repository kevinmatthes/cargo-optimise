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
    if !match std::process::Command::new("cargo").arg("metadata").output() {
        Ok(output) => output,
        Err(_) => {
            eprintln!("It cannot be checked whether this is a Rust project!");
            return sysexits::ExitCode::Unavailable;
        }
    }
    .status
    .success()
    {
        eprintln!("This is not a Rust project!");
        return sysexits::ExitCode::Usage;
    }

    print!("Formatting the code ... ");

    let format = match std::process::Command::new("cargo").arg("fmt").output() {
        Ok(output) => output,
        Err(_) => {
            eprintln!("The code cannt be formatted!");
            return sysexits::ExitCode::Unavailable;
        }
    };

    if !format.status.success() {
        eprintln!(
            "Failure:\n{}",
            String::from_utf8(format.stderr)
                .unwrap_or(format!("{}", sysexits::ExitCode::IoErr as u32))
        );
        return sysexits::ExitCode::DataErr;
    } else {
        println!("Okay.")
    }

    print!("Checking whether the code compiles ... ");

    let check = match std::process::Command::new("cargo").arg("check").output() {
        Ok(output) => output,
        Err(_) => {
            eprintln!("It cannot be checked whether this code compiles!");
            return sysexits::ExitCode::Unavailable;
        }
    };

    if !check.status.success() {
        eprintln!(
            "Failure:\n{}",
            String::from_utf8(check.stderr)
                .unwrap_or(format!("{}", sysexits::ExitCode::IoErr as u32))
        );
        return sysexits::ExitCode::DataErr;
    } else {
        println!("Okay.")
    }

    print!("Linting the code ... ");

    let clippy = match std::process::Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-dirty")
        .arg("--allow-staged")
        .output()
    {
        Ok(output) => output,
        Err(_) => {
            eprintln!("The code cannot be linted!");
            return sysexits::ExitCode::Unavailable;
        }
    };

    if !clippy.status.success() {
        eprintln!(
            "Failure:\n{}",
            String::from_utf8(clippy.stderr)
                .unwrap_or(format!("{}", sysexits::ExitCode::IoErr as u32))
        );
        return sysexits::ExitCode::DataErr;
    } else {
        println!("Okay.");
    }

    sysexits::ExitCode::Ok
}

/******************************************************************************/
