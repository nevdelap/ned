//
// ned, https://github.com/nevdelap/ned, colors.rs
//
// Copyright 2016-2019 Nev Delap (nevdelap at gmail)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
// 02110-1301, USA.
//

use crate::ned_error::StringError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Colors {
    Auto,
    Always,
    Never,
    Off,
}

impl FromStr for Colors {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Colors::Auto),
            "auto" => Ok(Colors::Auto),
            "always" => Ok(Colors::Always),
            "never" => Ok(Colors::Never),
            _ => Err(StringError {
                err: format!("invalid colors option {}", s),
            }),
        }
    }
}
