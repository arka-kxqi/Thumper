use async_graphql::SimpleObject;
use linera_sdk::{
    base::Owner,
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};
use tic_tac_toe::Board;

/// The application state.
#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct GameState {
    /// The `Owner`s controlling players `One` and `Two`.
    pub game_players: RegisterView<Option<[Owner; 2]>>,
    /// The current game state.
    pub tic_tac_toe_board: RegisterView<Board>,
}
