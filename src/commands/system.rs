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
use crate::{Context, Error};

#[poise::command(slash_command, owners_only, subcommands("system_info"))]
pub async fn system(_: Context<'_>) -> Result<(), Error> { Ok(()) }

#[poise::command(slash_command, rename = "info")]
pub async fn system_info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("foo").await?;
    Ok(())
}
