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
use entity::{prelude::*, s3_profile::AnarchyRank, xbattle_stats::XBattleDivision};
use poise::serenity_prelude::{Color, User};
use sea_orm::{ActiveEnum, EntityTrait, ModelTrait};

#[poise::command(slash_command, subcommands("dossier_get"))]
/// Commands intended to assist in the bot's administration.
pub async fn dossier(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "get")]
#[tracing::instrument(skip(ctx))]
/// Fetch a Citizen's dossier.
pub async fn dossier_get(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    ctx.defer().await?;

    let db = &ctx.data().db;

    let citizen = user.as_ref().unwrap_or_else(|| ctx.author());

    if let Some(record) = S3Profile::find_by_id(*citizen.id.as_u64()).one(db).await? {
        let fcdashed = format!(
            "{}-{}-{}",
            &record.friend_code[..4],
            &record.friend_code[4..8],
            &record.friend_code[8..],
        );

        let mut fields = vec![
            (
                "Friend Code".to_string(),
                if let Some(ref token) = record.fclink_token {
                    format!(
                        "[SW-{fc}](https://lounge.nintendo.com/friendcode/{fc}/{tok})",
                        fc = fcdashed,
                        tok = token
                    )
                } else {
                    format!("SW-{}", fcdashed)
                },
                true,
            ),
            ("Level".to_string(), record.level.to_string(), true),
            (
                "Turf Inked".to_string(),
                format!("{}p", record.turf_inked),
                false,
            ),
            (
                "Recorded Victories".to_string(),
                record.total_wins.to_string(),
                false,
            ),
            (
                "Anarchy Battle Rank (Current/Best)".to_string(),
                format!(
                    "{}/{}",
                    record.anarchy_rank_current.to_value(),
                    record.anarchy_rank_best.to_value()
                ),
                false,
            ),
        ];

        if let AnarchyRank::SPlus(_) = record.anarchy_rank_current {
            if let Some(xstats) = record.find_related(XBattleStats).one(db).await? {
                let assets = &ctx.data().conf.assets;
                let head = format!(
                    "X Battle Ratings | {} Division",
                    match xstats.division {
                        XBattleDivision::Takaroka =>
                            format!("{} Takaroka", assets.xbattle.takaroka),
                        XBattleDivision::Tentatek =>
                            format!("{} Tentatek", assets.xbattle.tentatek),
                    }
                );

                let body = format!(
                    "{} *Splat Zones* : {}\n\
                     {} *Tower Control* : {}\n\
                     {} *Rainmaker* : {}\n\
                     {} *Clam Blitz* : {}",
                    assets.xbattle.splat_zones,
                    xstats.splat_zones,
                    assets.xbattle.tower_control,
                    xstats.tower_control,
                    assets.xbattle.rainmaker,
                    xstats.rainmaker,
                    assets.xbattle.clam_blitz,
                    xstats.clam_blitz,
                );

                fields.push((head, body, false));
            }
        }

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
                .fields(fields)
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
