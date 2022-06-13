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

/// The settings and business logic of the resulting binary executable.
pub struct Application {
    /// The processes to be called.
    applications: Vec<String>,

    /// The command line arguments of the processes to be called.
    arguments: Vec<Vec<String>>,

    /// The error messages to show in case the processes should fail.
    error_messages: Vec<Option<String>>,

    /// The exit code to return by `main`.
    exit_codes: Vec<sysexits::ExitCode>,

    /// The verbosities of the respective processes.
    verbosities: Vec<crate::Verbosity>,
}

impl Application {
    /// Create a new application instance to run over the current project.
    #[must_use]
    pub fn new(
        applications: Vec<String>,
        arguments: Vec<Vec<String>>,
        error_messages: Vec<Option<String>>,
        exit_codes: Vec<sysexits::ExitCode>,
        verbosities: Vec<crate::Verbosity>,
    ) -> Self {
        Self {
            applications,
            arguments,
            error_messages,
            exit_codes,
            verbosities,
        }
    }

    /// Run the configured instance as binary executable.
    ///
    /// First of all, it will be checked whether the specified processes where
    /// defined uniformly.  This is, that for each process, there is a set of
    /// command line arguments, an error message -- just to be sure -- as well
    /// as an exit code from `sysexits` to be returned in case the error the
    /// message is configurated for should occur.  This sanity check is not only
    /// used for debugging but also ensures -- in case that this crate should be
    /// bound as a dependency -- that all the configured optimisation steps will
    /// have sufficient data to originate from.
    ///
    /// Then, the actual processes will be spawned and run using the settings of
    /// this instance.  The processes will be created in their order of
    /// appearance in `self.applications` such that the index position of the
    /// application to be called also returns the corresponding data from the
    /// other vectors held.  Every process is required to finish before the next
    /// one will be spawned.
    ///
    /// In case one process should not succeed, the corresponding error message
    /// will be written to `stderr` and this application will be aborted with
    /// the configured `sysexits::ExitCode`.  If each process succeeds, the
    /// return value is `sysexits::ExitCode::Ok`.
    #[must_use]
    pub fn run(&self) -> sysexits::ExitCode {
        if self.applications.len() != self.arguments.len()
            || self.applications.len() != self.error_messages.len()
            || self.applications.len() != self.exit_codes.len()
        {
            eprintln!(
                "Internal error:  \
                non-uniform count of process specification details!"
            );
            return sysexits::ExitCode::Software;
        }

        for i in 0..self.applications.len() {
            match crate::Process::new(
                &self.applications[i],
                self.arguments[i].clone(),
                self.verbosities[i],
            )
            .handle(self.error_messages[i].as_deref(), self.exit_codes[i])
            {
                None => (),
                Some(code) => return code,
            }
        }

        sysexits::ExitCode::Ok
    }
}

/******************************************************************************/
