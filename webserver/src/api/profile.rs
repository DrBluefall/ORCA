//! S3 Profiles & X Battle Statistics.

use actix_web::{get, patch, web, HttpResponse};
use entity::sea_orm::prelude::*;
use entity::s3_profile::{self, Entity as S3Profile};
use entity::xbattle_stats::{self, Entity as XBattleStats};

#[get("/profile/{user_id}")]
pub async fn get_profile(path: web::Path<(u64,)>, db: web::Data<DatabaseConnection>) -> HttpResponse {
    let (user_id,) = path.into_inner();

    let (profile, xbattle) = if let Some (x) = S3Profile::find_by_id(user_id).find_also_related(XBattleStats).one(&**db).await.unwrap() {
        x
    } else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().json(serde_json::json! {{
        "profile": profile,
        "xbattle": xbattle,
    }})
}
