//! S3 Profiles & X Battle Statistics.

use actix_web::{get, post, web, HttpResponse};
use entity::s3_profile::{self, Entity as S3Profile};
use entity::sea_orm::{
    prelude::*, ActiveModelTrait, ActiveValue, IntoActiveModel, TransactionTrait,
};
use entity::xbattle_stats::{self, Entity as XBattleStats};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FlattenedProfile {
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

    HttpResponse::Ok().json(FlattenedProfile { profile, xbattle })
}

#[post("/profile/{user_id}/edit")]
pub async fn edit_profile(
    path: web::Path<(u64,)>,
    data: web::Json<serde_json::Value>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let (user_id,) = path.into_inner();

    let (oldprofile, oldxbattle) = if let Some(x) = S3Profile::find_by_id(user_id)
        .find_also_related(XBattleStats)
        .one(&**db)
        .await
        .unwrap()
    {
        x
    } else {
        return HttpResponse::NotFound().finish();
    };

    let tx = db.begin().await.unwrap();

    let mut profile = oldprofile.into_active_model();
    profile.set_from_json((*data).clone()).unwrap();
    // Right now there's a bug where if no fields are changed, SeaORM will
    // generate invalid SQL. In the future, this'll be a no-op, but for now, a
    // simple workaround is to just reassign the primary key.
    profile.id = ActiveValue::Set(user_id);
    profile.update(&tx).await.unwrap();

    if let Some(oldxbattle) = oldxbattle {
        let mut xbattle = oldxbattle.into_active_model();
        xbattle.set_from_json((*data).clone()).unwrap();
        xbattle.id = ActiveValue::Set(user_id);
        xbattle.update(&tx).await.unwrap();
    }

    tx.commit().await.unwrap();

    HttpResponse::Ok().finish()
}
