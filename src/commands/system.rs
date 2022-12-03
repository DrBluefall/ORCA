// Copyright (C) 2022 Alexander Bisono
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
use crate::{Context, Error};
use poise::serenity_prelude::{CacheHttp, Color};
use sysinfo::{ProcessExt, System, SystemExt};

#[poise::command(slash_command, owners_only, subcommands("system_info"))]
/// Commands intended to assist in the bot's administration.
pub async fn system(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "info")]
/// Inspect details about the bot.
pub async fn system_info(
    ctx: Context<'_>,
    #[description = "Display info publicly."]
    #[flag]
    public: bool,
) -> Result<(), Error> {
    if public {
        ctx.defer().await?;
    } else {
        ctx.defer_ephemeral().await?;
    }

    let core_name = format!("O.R.C.A. ver. {}", env!("CARGO_PKG_VERSION"));
    let revinfo = format!(
        "`{}` - Committed on `{}`",
        env!("ORCA_REVISION_HASH"),
        env!("ORCA_REVISION_DATE")
    );

    let dat = ctx.data();
    let uptime = std::time::Instant::now()
        .duration_since(dat.stats.start_time)
        .as_secs();
    let current_user = ctx.serenity_context().http().get_current_user().await?;

    let mut upstr = String::new();

    let mods = [(86400, "day"), (3600, "hour"), (60, "minute")];

    let mut i = uptime;
    for (div, unit) in &mods {
        let part = i / div;
        if part != 0 {
            upstr += &format!(
                "{} {}, ",
                part,
                if part != 1 {
                    format!("{}s", unit)
                } else {
                    unit.to_string()
                }
            );
        }
        i %= div;
    }
    upstr += &format!("{} {}", i, if i != 1 { "seconds" } else { "second" });

    let mut fields = vec![
        ("Intelligence Core", core_name, false),
        ("Revision Info", revinfo, true),
        (
            "Compiler Version",
            format!("`rustc` ver. `{}`", rustc_version_runtime::version()),
            false,
        ),
        ("System Uptime", upstr, true),
    ];

    if System::IS_SUPPORTED {
        let mut sys = dat.stats.sysinfo.write().await;

        fields.push(("Hostname", sys.host_name().unwrap(), false));
        fields.push((
            "Operating System",
            format!(
                "{} {}",
                sys.name().unwrap(),
                sys.os_version()
                    .unwrap_or_else(|| sys.kernel_version().unwrap())
            ),
            false,
        ));
        fields.push(("Host Target Triple", env!("ORCA_EXEC_TARGET").into(), false));

        if let Ok(pid) = sysinfo::get_current_pid() {
            sys.refresh_process(pid);
            let proc = sys.process(pid).unwrap();
            fields.push((
                "Memory Usage",
                format!(
                    "{} MiB",
                    f64::trunc((proc.memory() as f64 / ((1024 * 1024) as f64)) * 1000.0) / 1000.0
                ),
                true,
            ));
        }
    }

    ctx.send(|m| {
        m.embed(|em| {
            em.title("System Information")
                .thumbnail(
                    current_user
                        .avatar_url()
                        .unwrap_or_else(|| current_user.default_avatar_url()),
                )
                .fields(fields)
                .color(Color::from_rgb(158, 253, 56))
        })
    })
    .await?;

    Ok(())
}
