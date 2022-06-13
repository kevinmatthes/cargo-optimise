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

/// The verbosity level of an application.
///
/// There are the following major verbosity levels with the corresponding
/// meaning:
///
/// * `Verbosity::Chatty`:  be at most verbose
/// * `Verbosity::Monosyllabic`:  communicate only the most important data
/// * `Verbosity::Silent`:  do not communicate at all
///
/// Since these verbosity levels can be compared between each other, this enum
/// implements the `Ord` trait such that a given verbosity level can be compared
/// against a hard coded one comfortably.
///
/// In order to associate the configured verbosity levels with integer numbers,
/// each one has a corresponding `u8` value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Verbosity {
    /// Be at most verbose and communicate everything.
    Chatty = 2,

    /// Communicate only the most important information.
    Monosyllabic = 1,

    /// Do not communicate at all.
    Silent = 0,
}

impl Verbosity {
    /// Set the verbosity immediately to `Self::Chatty`.
    pub fn chatty(&mut self) {
        *self = Self::Chatty;
    }

    /// Downgrade the verbosity by one level.
    ///
    /// If the verbosity is already set to `Self::Silent`, it cannot be
    /// decreased any further.  This is not considered an error, the level just
    /// will not be changed.
    pub fn downgrade(&mut self) {
        *self = match self {
            Self::Chatty => Self::Monosyllabic,
            _ => Self::Silent,
        }
    }

    /// Set the verbosity immediately to `Self::Monosyllabic`.
    pub fn monosyllabic(&mut self) {
        *self = Self::Monosyllabic;
    }

    /// Set the verbosity immediately to `Self::Silent`.
    pub fn silent(&mut self) {
        *self = Self::Silent;
    }

    /// Upgrade the verbosity by one level.
    ///
    /// If the verbosity is already set to to `Self::Chatty`, it cannot be
    /// increased any further.  This is not considered an error, the level just
    /// will not be changed.
    pub fn upgrade(&mut self) {
        *self = match self {
            Self::Silent => Self::Monosyllabic,
            _ => Self::Chatty,
        }
    }
}

impl std::fmt::Display for Verbosity {
    /// Implements the `Display` trait.
    ///
    /// A verbosity level can be represented by its associated `u8`
    /// representation as well as its name in this enum.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "verbosity level {} ('{}')",
            *self as u8,
            String::from(self)
        )
    }
}

impl std::str::FromStr for Verbosity {
    /// The associated error type.
    type Err = ParseError;

    /// Implements the conversion from a string.
    ///
    /// Each configured level of verbosity can associated by either its integer
    /// representation or its name within the enum.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "2" | "chatty" => Ok(Self::Chatty),
            "1" | "monosyllabic" => Ok(Self::Monosyllabic),
            "0" | "silent" => Ok(Self::Silent),
            _ => Err(ParseError),
        }
    }
}

impl From<Verbosity> for String {
    /// Implements the string conversion.
    ///
    /// A verbosity level can represented as a string by its name within this
    /// enum.
    fn from(verbosity: Verbosity) -> Self {
        match verbosity {
            Verbosity::Chatty => "chatty",
            Verbosity::Monosyllabic => "monosyllabic",
            Verbosity::Silent => "silent",
        }
        .into()
    }
}

impl From<&Verbosity> for String {
    /// Implements the string conversion.
    ///
    /// A verbosity level can represented as a string by its name within this
    /// enum.
    fn from(verbosity: &Verbosity) -> Self {
        match verbosity {
            Verbosity::Chatty => "chatty",
            Verbosity::Monosyllabic => "monosyllabic",
            Verbosity::Silent => "silent",
        }
        .into()
    }
}

/// The verbosity level cannot be deduced.
#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    /// Implements the `Display` trait.
    ///
    /// In case this error should occur, the verbosity could be deduced from the
    /// input string.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "the verbosity level cannot be deduced!")
    }
}

/// This struct can be returned as an error.
impl std::error::Error for ParseError {}

/******************************************************************************/
