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

/// The settings for a process to be invoked.
pub struct Process {
    /// The application to call.
    application: String,

    /// The command line arguments to pass.
    arguments: Vec<String>,

    /// The exit code after returning to the caller.
    exit: i32,

    /// The output written to `stderr` during the execution.
    stderr: String,

    /// The output written to `stdout` during the execution.
    stdout: String,

    /// The verbosity level.
    ///
    /// This field controls how much information shall be written to `stdout`:
    ///
    /// * `Verbosity::Silent`:  important error messages only
    /// * `Verbosity::Monosyllabic`:  important error messages and call line
    /// * `Verbosity::Chatty`:  entire output and call line
    verbosity: crate::Verbosity,
}

impl Process {
    /// The error message to show in case of an error.
    ///
    /// If the called application should exit non-zero, the given message will
    /// be written to `stderr`.  In case that no message should be given, the
    /// error messages of the application will be shown instead.
    #[must_use]
    pub fn failure(&self, error: Option<&str>) -> bool {
        let message = match error {
            Some(string) => format!("{string}\n"),
            None => self.stderr.to_string(),
        };
        let ret = !self.success();

        if ret {
            eprint!("{message}");
        }

        ret
    }

    /// Run the configured process and handle all occurring errors.
    ///
    /// The configured process will be executed with `run` method and thereby
    /// resulting `sysexits::ExitCode` will be propagated to the `main` function
    /// such that it can immediately return it.
    ///
    /// If running the configured process should not succeed, indicated by a
    /// `sysexits::ExitCode` other than `sysexits::ExitCode::Ok` originating
    /// from the `run` method, this exit code will be handed over to the caller
    /// after an according default error message was written to `stderr`.
    ///
    /// Even if the process terminated itself, it might have failed due to some
    /// reasons.  Then, the given error message will be written to `stderr` and
    /// the given exit code will be propagated to the caller.
    ///
    /// If the configured process should succeed, nothing would be written to
    /// `stderr` by this method and it would return `None`.  Hence, this method
    /// will only write the most important error messages to the default error
    /// stream.
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
                if self.failure(error) {
                    Some(ret)
                } else {
                    None
                }
            }
            sysexits::ExitCode::OsErr => {
                eprintln!("'{self}' was terminated unexpectedly by a signal!");
                Some(sysexits::ExitCode::OsErr)
            }
            sysexits::ExitCode::Unavailable => {
                eprintln!("Failed to launch '{self}'!");
                Some(sysexits::ExitCode::Unavailable)
            }
            code => {
                eprintln!("Unknown exit status {code} originating from '{self}'!");
                Some(sysexits::ExitCode::Config)
            }
        }
    }

    /// Configure a new process.
    ///
    /// Since some information are unavailable before the process was executed,
    /// the respective fields are initialised with sane default values such as
    /// zero for the exit code -- the process is expected to succeed -- and
    /// empty strings for the output.
    #[must_use]
    pub fn new(application: &str, arguments: Vec<String>, verbosity: crate::Verbosity) -> Self {
        let mut buffer = Vec::new();

        for argument in arguments {
            buffer.push(argument);
        }

        Self {
            application: application.into(),
            arguments: buffer,
            exit: 0,
            stderr: String::new(),
            stdout: String::new(),
            verbosity,
        }
    }

    /// Run the configured process.
    ///
    /// The configured process will be spawned by this method.  As soon as it
    /// returns, its exit code an any data written to both `stdout` and `stderr`
    /// will be stored in this instance for later evaluation.
    ///
    /// Depending on the set verbosity level, some additional information might
    /// be written to `stdout`, as well:
    ///
    /// * When set to `Verbosity::Silent`, absolutely nothing will be shown to
    ///   the user.
    /// * When set to `Verbosity::Monosyllabic`, the application call line will
    ///   be written to `stdout`.
    /// * When set to `Verbosity::Chatty`, the application call line will be
    ///   written to `stdout` as well as the entire output of the called process
    ///   to their respective streams if they are not empty.
    ///
    /// At the end, this method will return a `sysexits::ExitCode` to be
    /// propagated to the `main` function:
    ///
    /// * `sysexits::ExitCode::DataErr` if the output of the process could not
    ///   be saved in this instance for later usage.
    /// * `sysexits::ExitCode::Ok` if the configured process terminated itself.
    /// * `sysexits::ExitCode::Software` if the configured process did not
    ///   terminate itself but due to a signal.
    /// * `sysexits::ExitCode::Unavailable` if the configured process was unable
    ///   to be spawned.
    pub fn run(&mut self) -> sysexits::ExitCode {
        let mut process = std::process::Command::new(&self.application);

        if self.verbosity > crate::Verbosity::Silent {
            println!("{self}");
        }

        for argument in &self.arguments {
            process.arg(argument);
        }

        let output = match process.output() {
            Ok(output) => output,
            Err(_) => return sysexits::ExitCode::Unavailable,
        };

        self.exit = match output.status.code() {
            Some(number) => number,
            None => return sysexits::ExitCode::OsErr,
        };
        self.stderr = match String::from_utf8(output.stderr) {
            Ok(string) => string,
            Err(_) => return sysexits::ExitCode::DataErr,
        };
        self.stdout = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_) => return sysexits::ExitCode::DataErr,
        };

        if self.verbosity == crate::Verbosity::Chatty && !self.stdout.is_empty() {
            println!("{}", self.stdout);
        }

        if self.verbosity == crate::Verbosity::Chatty && !self.stderr.is_empty() {
            eprint!("{}", self.stderr);
        }

        sysexits::ExitCode::Ok
    }

    /// Whether the called application returned with code zero.
    ///
    /// By convention, the exit code zero is assumed to indicate the success of
    /// the called application.  Any other value is assumed to be a failure.
    #[must_use]
    pub const fn success(&self) -> bool {
        self.exit == 0
    }
}

impl std::fmt::Display for Process {
    /// Implements the `Display` trait.
    ///
    /// A `Process` instance will be formatted by naming the called application
    /// as well as all command line options passed to it in their order of
    /// appearance, joined by one space character each.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = self.application.to_string();

        for argument in &self.arguments {
            buffer.push_str(&format!(" {argument}"));
        }

        write!(f, "{buffer}")
    }
}

/******************************************************************************/
