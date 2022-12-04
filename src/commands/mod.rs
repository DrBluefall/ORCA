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

pub mod system;
pub mod dossier;

#[poise::command(prefix_command, owners_only)]
/// Register all of the bot's slash-commands.
pub async fn register_commands(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
