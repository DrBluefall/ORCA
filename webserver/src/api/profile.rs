//! S3 Profiles & X Battle Statistics.

use actix_web::{get, patch, web, HttpResponse};
use entity::s3_profile::{self, Entity as S3Profile};
use entity::sea_orm::prelude::*;
use entity::xbattle_stats::{self, Entity as XBattleStats};

#[derive(serde::Deserialize, serde::Serialize)]
struct ProfileReply {
    #[serde(flatten)]
    profile: s3_profile::Model,
    #[serde(flatten)]
    xbattle: Option<xbattle_stats::Model>,
}

#[get("/profile/{user_id}")]
pub async fn get_profile(
    path: web::Path<(u64,)>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let (user_id,) = path.into_inner();

    let (profile, xbattle) = if let Some(x) = S3Profile::find_by_id(user_id)
        .find_also_related(XBattleStats)
        .one(&**db)
        .await
        .unwrap()
    {
        x
    } else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().json(ProfileReply { profile, xbattle })
}
