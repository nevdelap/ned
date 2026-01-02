//
// ned, https://github.com/nevdelap/ned, options_with_defaults.rs
//
// Copyright 2016-2026 Nev Delap (nevdelap at gmail)
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

use crate::ned_error::NedResult;
use getopts::{Matches, Options};
use std::env;

pub struct OptionsWithDefaults {
    opts: Options,
    arg_matches: Matches,
    default_matches: Matches,
}

impl OptionsWithDefaults {
    pub fn new(opts: Options, args: &[String]) -> NedResult<OptionsWithDefaults> {
        let default_vec: Vec<String> = match env::var("NED_DEFAULTS") {
            Ok(mut s) => {
                // Normalize ASCII RS and apply POSIX shell-style splitting.
                s = s.replace("\u{1e}", " ");
                shell_words::split(&s).map_err(|e| e.to_string())?
            }
            Err(_) => vec![],
        };
        Ok(OptionsWithDefaults {
            arg_matches: opts.parse(args)?,
            default_matches: opts.parse(default_vec)?,
            opts,
        })
    }

    #[cfg(test)]
    /// Test-only constructor used to inject a simulated `NED_DEFAULTS` value.
    /// Applies the same POSIX shell-style splitting (`shell_words::split`) as
    /// production, while avoiding reads/writes to the process environment.
    /// This keeps tests isolated and deterministic for defaults parsing.
    /// Also normalizes ASCII RS (U+001E) to spaces to mirror runtime behavior.
    pub fn new_with_defaults_string(
        opts: Options,
        args: &[String],
        defaults: Option<&str>,
    ) -> NedResult<OptionsWithDefaults> {
        let default_vec: Vec<String> = match defaults {
            Some(s) => {
                let s = s.replace("\u{1e}", " ");
                shell_words::split(&s).map_err(|e| e.to_string())?
            }
            None => vec![],
        };
        Ok(OptionsWithDefaults {
            arg_matches: opts.parse(args)?,
            default_matches: opts.parse(default_vec)?,
            opts,
        })
    }

    pub fn get_opts(&self) -> &Options {
        &self.opts
    }

    pub fn opt_present(&self, s: &str) -> bool {
        self.arg_matches.opt_present(s) || self.default_matches.opt_present(s)
    }

    pub fn opt_str(&self, s: &str) -> Option<String> {
        self.arg_matches
            .opt_str(s)
            .or_else(|| self.default_matches.opt_str(s))
    }

    pub fn opt_strs(&self, s: &str) -> Vec<String> {
        let mut strs = self.arg_matches.opt_strs(s);
        strs.extend(self.default_matches.opt_strs(s));
        strs
    }

    pub fn free(&self) -> Vec<String> {
        let mut free = Vec::<String>::new();
        free.extend(self.arg_matches.free.iter().cloned());
        free.extend(self.default_matches.free.iter().cloned());
        free
    }
}
