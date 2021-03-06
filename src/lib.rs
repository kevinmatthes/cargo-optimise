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
//!
//! It defines the crate settings and controls how the contained modules are
//! presented.

// The lint levels in order to ensure at most quality of the source code.
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::suspicious)]
#![deny(clippy::style)]

// Module imports.
mod application;
mod cli_options;
mod functions;
mod process;
mod verbosity;

// Module exports.
pub use crate::application::Application;
pub use crate::cli_options::CliOptions;
pub use crate::functions::license;
pub use crate::process::Process;
pub use crate::verbosity::{ParseError as VerbosityParseError, Verbosity};

/******************************************************************************/
