use super::*;
use crate::queue::QueueUpdate;
use splendor_arena::*;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct MockEnv {
    pub queue_reciever: UnboundedReceiver<QueueUpdate>,
    pub queue_sender: AsyncQueue,
    pub games: AsyncGames,
    pub arenas: AsyncArenas,
    pub ids: Vec<Uuid>,
}

fn default_game_update() -> GameUpdate {
    GameUpdate {
        update_num: 0,
        info: SmallClientInfo {
            board: Board {
                deck_counts: [16, 26, 36],
                available_cards: vec![vec![0, 1, 2, 3], vec![5, 6, 7, 8], vec![10, 11, 12]],
                nobles: vec![0, 1, 2, 3, 4],
                gems: Gems::empty(),
            },
            players: vec![
                PlayerPublicInfo {
                    gems: Gems::empty(),
                    developments: Cost::from_gems(&Gems::empty()),
                    num_reserved: 0,
                    points: 0,
                },
                PlayerPublicInfo {
                    gems: Gems::empty(),
                    developments: Cost::from_gems(&Gems::empty()),
                    num_reserved: 0,
                    points: 0,
                },
                PlayerPublicInfo {
                    gems: Gems::empty(),
                    developments: Cost::from_gems(&Gems::empty()),
                    num_reserved: 0,
                    points: 0,
                },
            ],
            current_player_num: 0,
        },
    }
}

async fn create_mock_env() -> MockEnv {
    let (qtx, qrx) = tokio::sync::mpsc::unbounded_channel();
    let games = std::sync::Arc::new(std::sync::Mutex::new(Games::new()));
    let arenas = std::sync::Arc::new(std::sync::Mutex::new(Arenas::new()));

    let id0 = Uuid::new_v4();
    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let id3 = Uuid::new_v4();

    let arena_default = ArenaState {
        authenticated: false,
        initialized: false,
        id: id0,
        num_successful_updates: 0,
    };

    let arena_authenticated = ArenaState {
        authenticated: true,
        initialized: false,
        id: id1,
        num_successful_updates: 0,
    };

    let arena_initialized = ArenaState {
        authenticated: true,
        initialized: true,
        id: id2,
        num_successful_updates: 0,
    };

    let arena_with_updates = ArenaState {
        authenticated: true,
        initialized: true,
        id: id3,
        num_successful_updates: 5,
    };

    arenas.lock().unwrap().insert(id0, arena_default);
    arenas.lock().unwrap().insert(id1, arena_initialized);
    arenas.lock().unwrap().insert(id2, arena_authenticated);
    arenas.lock().unwrap().insert(id3, arena_with_updates);

    games.lock().unwrap().insert(id0, vec![]);
    games.lock().unwrap().insert(id1, vec![]);
    games.lock().unwrap().insert(id2, vec![]);
    games.lock().unwrap().insert(id3, vec![]);

    MockEnv {
        queue_reciever: qrx,
        queue_sender: qtx,
        games,
        arenas,
        ids: vec![id0, id1, id2, id3],
    }
}

#[test]
pub fn empty_api_key_does_not_authenticate() {
    let secret = "";
    assert_eq!(
        verify(secret),
        false,
        "an empty secret should not authenticate"
    );
    let message = handle_authenticate(secret, &mut ArenaState::default());
    assert!(message.is_err(), "expected error on empty secret, got Ok");
}

#[test]
pub fn correct_api_key_authenticates() {
    let secret = std::env::var("TEST_API_KEY")
        .expect("TEST_API_KEY must be set to a valid secret for this test to run");
    assert_eq!(
        verify(&secret),
        true,
        "a correct secret should authenticate"
    );
    let message = handle_authenticate(&secret, &mut ArenaState::default());
    let message = message.expect("unexpected error on correct secret authentication, expected Ok");
    assert!(matches!(
        message,
        GlobalServerResponse::Authenticated(Authenticated::Success)
    ));
}

#[test]
pub fn correct_api_key_updates_state() {
    let secret =
        std::env::var("TEST_API_KEY").expect("TEST_API_KEY must be set for this test to run");
    assert_eq!(
        verify(&secret),
        true,
        "TEST_API_KEY secret should authenticate"
    );
    let mut state = ArenaState::default();
    handle_authenticate(&secret, &mut state)
        .expect("unexpected error on correct secret authentication, expected Ok");

    assert_eq!(
        state.authenticated, true,
        "expected state to be authenticated"
    );
}

#[test]
pub fn handle_reconnect_fails_on_invalid_uuid() {
    let mut state = ArenaState::default();
    let games = Arc::new(Mutex::new(Games::new()));
    let message = handle_reconnect("this_id_does_not_exist", &mut state, games);
    assert!(message.is_err(), "expected error on invalid id, got Ok");
}

#[test]
pub fn handle_reconnect_fails_on_unknown_uuid() {
    let mut state = ArenaState::default();
    let games = Arc::new(Mutex::new(Games::new()));
    let uuid = Uuid::new_v4();
    let message = handle_reconnect(&uuid.to_string(), &mut state, games);
    assert!(message.is_err(), "expected error on unknown id, got Ok");
}

#[tokio::test]
pub async fn handle_reconnect_succeeds_on_valid_id() {
    let mock = create_mock_env().await;

    for id in mock.ids {
        let arena = mock.arenas.lock().unwrap().get(&id).unwrap().clone();
        let message = handle_reconnect(&id.to_string(), &mut arena.clone(), mock.games.clone());
        let message = message.expect("unexpected error on valid id, expected Ok");
        println!("{:?}", message);
        assert!(matches!(
            message,
            GlobalServerResponse::Reconnected(Reconnected::Success)
        ));
    }
}

#[tokio::test]
pub async fn handle_game_update_adds_to_queue() {
    let mock = create_mock_env().await;
    let id = mock.ids[0];
    let state = mock.arenas.lock().unwrap().get(&id).unwrap().clone();

    handle_game_update(
        &mut state.clone(),
        mock.queue_sender.clone(),
        &vec![default_game_update()],
    )
    .expect("unexpected error on game update, expected Ok");

    let mut qrx = mock.queue_reciever;
    qrx.try_recv()
        .expect("expected game update to be added to the queue");

    handle_game_update(
        &mut state.clone(),
        mock.queue_sender.clone(),
        &vec![default_game_update()],
    )
    .expect("unexpected error on game update, expected Ok");

    handle_game_update(
        &mut state.clone(),
        mock.queue_sender.clone(),
        &vec![default_game_update()],
    )
    .expect("unexpected error on game update, expected Ok");

    qrx.try_recv()
        .expect("expected message to be added to the queue (twice)");
    qrx.try_recv()
        .expect("expected message to be added to the queue (thrice)");

    assert!(
        qrx.try_recv().is_err(),
        "expected no more messages in the queue"
    );
}

#[tokio::test]
pub async fn handle_game_over_updates_queue() {
    let mock = create_mock_env().await;
    let id = mock.ids[0];
    let state = mock.arenas.lock().unwrap().get(&id).unwrap().clone();

    handle_game_over(&mut state.clone(), mock.queue_sender.clone())
        .expect("unexpected error on game over, expected Ok");

    let mut qrx = mock.queue_reciever;
    qrx.try_recv()
        .expect("expected one message to be added to the queue, but queue is empty");

    assert!(
        qrx.try_recv().is_err(),
        "expected no more messages in the queue"
    );
}
