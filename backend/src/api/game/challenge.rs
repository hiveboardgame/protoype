use actix_web::{post, web, HttpResponse, delete};
use hive_lib::game_type::GameType;
use serde::{Deserialize, Serialize};

use crate::server_error::ServerError;
use crate::model::challenge::GameChallenge;
use crate::{db::util::DbPool, extractors::auth::AuthenticatedUser};

#[derive(Deserialize)]
pub struct NewGameRequest {
    // Whether this challenge should be listed publicly
    pub public: bool,

    // Whether the game will be ranked
    pub ranked: bool,

    // Whether the game follows the "tournament" rules, i.e. the queen
    // cannot be played first. Always true for now
    pub tournament_queen_rule: bool,

    pub game_type: GameType,
}

#[derive(Serialize)]
pub struct NewGameResponse {
    challenge_url: String,
}

#[post("/game/challenge")]
pub async fn create_game_challenge(
    game: web::Json<NewGameRequest>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::create(&auth_user.uid, &game, &pool).await?;
    let challenge_url = format!("/game/challenge/{}", challenge.id);
    Ok(HttpResponse::Created().json(NewGameResponse { challenge_url }))
}

#[post("/game/challenge/{id}/accept")]
pub async fn accept_game_challenge(
    id: web::Path<i32>,
    _auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let _challenge = GameChallenge::get(*id, &pool).await?;
    // TODO: create a new game between auth_user and the challenger, then
    // delete the challenge
    Err(ServerError::Unimplemented)
}

#[delete("/game/challenge/{id}")]
pub async fn delete_game_challenge(
    id: web::Path<i32>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let challenge = GameChallenge::get(*id, &pool).await?;
    auth_user.authorize(&challenge.challenger_uid)?;
    challenge.delete(&pool).await?;
    Ok(HttpResponse::NoContent().finish())
}
