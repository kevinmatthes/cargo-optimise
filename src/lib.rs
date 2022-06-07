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

//! The library root of this crate.

pub struct Application {
    application: String,
    arguments: Vec<String>,
    exit: i32,
    stderr: String,
    stdout: String,
}

impl Application {
    pub fn failure(&self, error: &str) -> bool {
        let ret = !self.success();

        if ret {
            eprintln!("{error}")
        }

        ret
    }

    pub fn handle(
        &mut self,
        error: Option<&str>,
        ret: sysexits::ExitCode,
    ) -> Option<sysexits::ExitCode> {
        match self.run() {
            sysexits::ExitCode::DataErr => {
                eprintln!("Failed to convert output of '{self}'!");
                Some(sysexits::ExitCode::DataErr)
            }
            sysexits::ExitCode::Ok => {
                match self.failure(match error {
                    Some(string) => string,
                    None => self.stderr(),
                }) {
                    true => Some(ret),
                    false => None,
                }
            }
            sysexits::ExitCode::Software => {
                eprintln!("'{self}' was terminated unexpectedly by a signal!");
                Some(sysexits::ExitCode::Software)
            }
            sysexits::ExitCode::Unavailable => {
                eprintln!("Failed to launch '{self}'!");
                Some(sysexits::ExitCode::Unavailable)
            }
            code => {
                eprintln!(
                    "Unknown exit code {} originating from running {self}!",
                    code as i32
                );
                Some(sysexits::ExitCode::Config)
            }
        }
    }

    pub fn new(app: &str, args: Vec<&str>) -> Self {
        let mut arguments = Vec::new();

        for string in args {
            arguments.push(String::from(string))
        }

        Self {
            application: String::from(app),
            arguments,
            exit: 0,
            stderr: String::new(),
            stdout: String::new(),
        }
    }

    pub fn run(&mut self) -> sysexits::ExitCode {
        let mut process = std::process::Command::new(&self.application);

        for argument in &self.arguments {
            process.arg(argument);
        }

        let output = match process.output() {
            Ok(output) => output,
            Err(_) => return sysexits::ExitCode::Unavailable,
        };

        self.exit = match output.status.code() {
            Some(number) => number,
            None => return sysexits::ExitCode::Software,
        };
        self.stderr = match String::from_utf8(output.stderr) {
            Ok(string) => string,
            Err(_) => return sysexits::ExitCode::DataErr,
        };
        self.stdout = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_) => return sysexits::ExitCode::DataErr,
        };

        sysexits::ExitCode::Ok
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn success(&self) -> bool {
        self.exit == 0
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from(&self.application);

        for argument in &self.arguments {
            string.push_str(&format!(" {argument}"))
        }

        write!(f, "{string}")
    }
}

/******************************************************************************/
