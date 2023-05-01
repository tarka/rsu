/*
 * Copyright © 2022, Steve Smith <tarkasteve@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version
 * 3 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use argh::FromArgs;

/// Run commands as a user
#[derive(FromArgs)]
pub struct Opts {
    /// explain what is being done. Can be specified multiple times to
    /// increase logging.
    #[argh(option, short = 'v', default = "0")]
    pub verbose: u64,

    /// execute as this user (default 'root')
    #[argh(option, short = 'u')]
    pub user: Option<String>,

    /// switch to a login shell for the specified user
    #[argh(option, short = 'i', default = "false")]
    pub login: bool,

    // the command to execute
    #[argh(positional)]
    pub command: Vec<String>,
}

pub fn parse_opts() -> Opts {
    argh::from_env()
}
