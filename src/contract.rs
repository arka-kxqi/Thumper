#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;
use env_logger;
use linera_sdk::{
    base::{CryptoHash, Owner, PublicKey, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use log::{error, info, warn};
use state::GameState;
use std::sync::Once;
use tic_tac_toe::{Board, GameOutcome, InstantiationArgument, Operation, TicTacToeAbi};
static INIT: Once = Once::new();

pub struct TicTacToeContract {
    state: GameState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(TicTacToeContract);

impl WithContractAbi for TicTacToeContract {
    type Abi = TicTacToeAbi;
}

impl Contract for TicTacToeContract {
    type Message = ();
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = GameState::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        TicTacToeContract { state, runtime }
    }

    async fn instantiate(&mut self, arg: Self::InstantiationArgument) {
        self.runtime.application_parameters();
        self.state.tic_tac_toe_board.set(Board::new());
        info!("Owners: {:?}", arg.players);
        self.state.game_players.set(Some(arg.players.clone()));
    }

    async fn execute_operation(&mut self, operation: Operation) -> GameOutcome {
        let active = self.state.tic_tac_toe_board.get().get_active_player();
        info!("Active player: {:?}", active);
        let (x, y) = match operation {
            Operation::MarkBoard { x, y } => (x, y),
        };
        info!(
            "Authenticated signer: {:?}",
            self.runtime.authenticated_signer()
        );
        info!("Owners: {:?}", self.state.game_players.get());
        assert_eq!(
            self.runtime.authenticated_signer(),
            Some(self.state.game_players.get().unwrap()[active.index()]),
            "Move must be signed by the player whose turn it is."
        );
        self.state.tic_tac_toe_board.get_mut().mark_board(x, y)
    }

    async fn execute_message(&mut self, _message: ()) {
        panic!("The Tic Tac Toe application doesn't support any cross-chain messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
