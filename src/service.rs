#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    base::{Owner, WithServiceAbi},
    graphql::GraphQLMutationRoot,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};
use tic_tac_toe::{Board, Operation, Player};

use self::state::GameState;

#[derive(Clone)]
pub struct TicTacToeService {
    state: Arc<GameState>,
}

linera_sdk::service!(TicTacToeService);

impl WithServiceAbi for TicTacToeService {
    type Abi = tic_tac_toe::TicTacToeAbi;
}

impl Service for TicTacToeService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = GameState::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        TicTacToeService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema =
            Schema::build(self.clone(), Operation::mutation_root(), EmptySubscription).finish();
        schema.execute(request).await
    }
}

#[Object]
impl TicTacToeService {
    async fn winner(&self) -> Option<Player> {
        if let Some(winner) = self.state.tic_tac_toe_board.get().get_winner() {
            return Some(winner);
        }
        None
    }

    async fn game_players(&self) -> [Owner; 2] {
        self.state.game_players.get().unwrap()
    }

    async fn tic_tac_toe_board(&self) -> &Board {
        self.state.tic_tac_toe_board.get()
    }
}
