// Copyright (C) 2022, 2023 Alexander Bisono
//
// This file is part of O.R.C.A., a Splatoon 3 Discord Bot.
//
// O.R.C.A is free software: you can redistribute it and/or modify it under the
// terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// O.R.C.A is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with O.R.C.A. If not, see <https://www.gnu.org/licenses/>.
use std::process::Command;

fn main() {
    /*
     * This will set the environment variables ORCA_REVISION_HASH &
     * ORCA_REVISION_DATE during compilation. Mainly for use with /system info.
     */

    let mut orca_rev = "<UNKNOWN>".to_string();
    // The last day of the Callie vs. Marie Splatfest.
    // No commit made on this day will ever be the active HEAD.
    let mut orca_date = "2016-07-24".to_string();

    let git = Command::new("git")
        .args([
            "--no-optional-locks",
            "log",
            "-n 1",
            "--format=%h %cd",
            "--date=short",
        ])
        .output();

    match git {
        Ok(out) => {
            let stdout = String::from_utf8(out.stdout).unwrap();
            let mut rev_info = stdout.split(' ');

            orca_rev = rev_info.next().unwrap().to_string();
            orca_date = rev_info.next().unwrap().to_string();
        }
        Err(_) => {
            println!("cargo:warning=Failed to invoke git for the revision hash & date. Default values will be used.");
        }
    }

    println!("cargo:rustc-env=ORCA_REVISION_HASH={}", orca_rev);
    println!("cargo:rustc-env=ORCA_REVISION_DATE={}", orca_date);

    /* also exporting TARGET from the build script to the crate itself */

    println!("cargo:rustc-env=ORCA_EXEC_TARGET={}", std::env::var("TARGET").unwrap());
}
