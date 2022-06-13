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

/// The configured command line options.
#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliOptions {
    /// Show the license information and quit.
    #[clap(short, long)]
    license: bool,

    /// The verbosity level for this run.
    #[clap(short, long, default_value = "monosyllabic")]
    verbosity: crate::Verbosity,
}

impl CliOptions {
    /// Retrieve the license information mode.
    #[must_use]
    pub const fn license(&self) -> bool {
        self.license
    }

    /// Retrieve the verbosity level.
    #[must_use]
    pub const fn verbosity(&self) -> crate::Verbosity {
        self.verbosity
    }
}

/******************************************************************************/
