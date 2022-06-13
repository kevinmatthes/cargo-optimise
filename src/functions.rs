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

//! Standalone auxillary functions of this crate.
//!
//! These functions cannot be associated with dedicated data structures and / or
//! enums such that they are collected here centrally as utility functions on
//! their own.

/// Show the license information.
pub fn license() {
    println!(
        "Copyright (C) 2022 Kevin Matthes\n\
        \n\
        This program is free software; you can redistribute it and/or modify\n\
        it under the terms of the GNU General Public License as published by\n\
        the Free Software Foundation; either version 2 of the License, or\n\
        (at your option) any later version.\n\
        \n\
        This program is distributed in the hope that it will be useful,\n\
        but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
        GNU General Public License for more details.\n\
        \n\
        You should have received a copy of the GNU General Public License along\n\
        with this program; if not, write to the Free Software Foundation, Inc.,\n\
        51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA."
    );
}

/******************************************************************************/
