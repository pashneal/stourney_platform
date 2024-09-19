use crate::database;
use serde::{Deserialize, Serialize};
use splendor_arena::models::GameUpdate;
use splendor_arena::*;
use sqlx::sqlite::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Success {
    #[serde(rename = "game_update")]
    GameUpdate(DetailedGameUpdate),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    #[serde(rename = "success")]
    Success(Success),
    #[serde(rename = "failure")]
    Failure { reason: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateRequest {
    pub uuid: String,
    #[serde(rename = "turnNumber")]
    pub turn_number: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GemDescription {
    #[serde(rename = "onyx")]
    Onyx,
    #[serde(rename = "sapphire")]
    Sapphire,
    #[serde(rename = "emerald")]
    Emerald,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "diamond")]
    Diamond,
    #[serde(rename = "gold")]
    Gold,
}

impl From<Gem> for GemDescription {
    fn from(gem: Gem) -> Self {
        match gem {
            Gem::Onyx => GemDescription::Onyx,
            Gem::Sapphire => GemDescription::Sapphire,
            Gem::Emerald => GemDescription::Emerald,
            Gem::Ruby => GemDescription::Ruby,
            Gem::Diamond => GemDescription::Diamond,
            Gem::Gold => GemDescription::Gold,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardDescription {
    pub id: CardId,
    pub cost: Cost,
    pub points: usize,
    pub color: GemDescription,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NobleDescription {
    pub cost: Cost,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardDescription {
    #[serde(rename = "deckCounts")]
    pub deck_counts: [usize; 3],
    #[serde(rename = "availableCards")]
    pub available_cards: Vec<Vec<CardDescription>>,
    pub nobles: Vec<NobleDescription>,
    pub bank: Gems,
}

impl BoardDescription {
    pub fn from_board(board: &Board) -> Self {
        let nobles = board
            .nobles
            .iter()
            .map(|n| Noble::from_id(*n))
            .map(|n| NobleDescription {
                cost: Cost::from_gems(n.requirements()),
            })
            .collect();

        let all_cards = Card::all_const();
        let available_cards = board
            .available_cards
            .iter()
            .map(|cards| {
                cards
                    .iter()
                    .map(|&id| all_cards[id as usize].clone())
                    .map(|card| CardDescription {
                        id: card.id(),
                        cost: card.cost(),
                        points: card.points() as usize,
                        color: card.gem().into(),
                    })
                    .collect()
            })
            .collect();

        BoardDescription {
            deck_counts: board.deck_counts,
            available_cards,
            nobles,
            bank: board.gems.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerDescription {
    pub bank: Gems,
    pub developments: Cost,
    #[serde(rename = "numReservedCards")]
    pub num_reserved_cards: usize,
    #[serde(rename = "totalPoints")]
    pub total_points: usize,
}

impl PlayerDescription {
    pub fn from_player(player: &PlayerPublicInfo) -> Self {
        PlayerDescription {
            bank: player.gems,
            developments: player.developments,
            num_reserved_cards: player.num_reserved,
            total_points: player.points as usize,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetailedGameUpdate {
    #[serde(rename = "turnNumber")]
    pub turn_number: usize,
    pub board: BoardDescription,
    pub players: Vec<PlayerDescription>,
    #[serde(rename = "currentPlayer")]
    pub current_player : usize,
}

impl DetailedGameUpdate {
    pub fn from_game_update(game_update: &GameUpdate) -> Self {
        let board = BoardDescription::from_board(&game_update.info.board);
        let players = game_update
            .info
            .players
            .iter()
            .map(|player| PlayerDescription::from_player(player))
            .collect();

        DetailedGameUpdate {
            turn_number: game_update.update_num as usize,
            board,
            players,
            current_player: game_update.info.current_player_num as usize,
        }
    }
}

pub fn json_body() -> impl Filter<Extract = (UpdateRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::json()
}

/// POST /api/load_game
/// given an update request, load the requested game from the database,
/// and transform it to a form appropriate for the client
pub async fn load_game(
    update: UpdateRequest,
    db_pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let slug = update.uuid;
    let turn_id = update.turn_number;

    let uuid = database::load_uuid_from_slug(&db_pool, &slug).await.map_err(|_| warp::reject::not_found())?;
    let game = database::load_game_update(&db_pool, uuid, turn_id as i32).await;
    let game = game.map(|game| DetailedGameUpdate::from_game_update(&game));

    if let Some(game) = game {
        Ok(warp::reply::json(&Response::Success(Success::GameUpdate(
            game,
        ))))
    } else {
        Err(warp::reject::not_found())
    }
}
