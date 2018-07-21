use getopts::{Matches, Options};
use ned_error::NedResult;
use std::env;

pub struct OptionsWithDefaults {
    opts: Options,
    arg_matches: Matches,
    default_matches: Matches,
}

impl OptionsWithDefaults {
    pub fn new(opts: Options, args: &[String]) -> NedResult<OptionsWithDefaults> {
        Ok(OptionsWithDefaults {
            arg_matches: opts.parse(args)?,
            default_matches: opts.parse(if let Ok(mut default_args) = env::var("NED_DEFAULTS") {
                // This replace of ASCII RS character (what the?) is special - it is for
                // if when using fish shell someone has done "set NED_DEFAULTS -u -c" rather
                // than this "set NED_DEFAULTS '-u -c'" they don't get a cryptic complaint.
                default_args = default_args.replace("\u{1e}", " ");
                default_args
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            } else {
                vec![]
            })?,
            opts: opts,
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
