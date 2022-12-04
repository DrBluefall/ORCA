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
use entity::prelude::*;
use poise::serenity_prelude::{Color, User};
use sea_orm::EntityTrait;

#[poise::command(slash_command, subcommands("dossier_get"))]
/// Commands intended to assist in the bot's administration.
pub async fn dossier(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "get")]
/// Fetch a Citizen's dossier.
pub async fn dossier_get(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    ctx.defer().await?;

    let db = &ctx.data().db;

    let citizen = user.as_ref().unwrap_or_else(|| ctx.author());

    if let Some(record) = S3Profile::find_by_id(*citizen.id.as_u64()).one(db).await? {
        ctx.send(|m| {
            m.embed(|e| {
                e.title(format!(
                    "Citizen Dossier :: {}#{}",
                    record.ign, record.discriminator
                ))
                .thumbnail(
                    citizen
                        .avatar_url()
                        .unwrap_or_else(|| citizen.default_avatar_url()),
                )
                .color(Color::from_rgb(158, 253, 56))
            })
        })
        .await?;
    } else {
        if user.is_none() {
            // TODO: Implement user registration on invocation.
            ctx.say("I could not find you in my datastore.").await?;
        } else {
            ctx.say("I could not find any citizen with the given ID.")
                .await?;
        }
    }

    Ok(())
}
