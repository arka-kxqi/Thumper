/*!
# Tic Tac Toe

Tic Tac Toe is a game where two players take turns marking a 3x3 grid. The first player to align
three marks horizontally, vertically, or diagonally wins.

*/
use async_graphql::{Enum, Request, Response, SimpleObject};
use linera_sdk::{
    base::{ContractAbi, Owner, ServiceAbi, TimeDelta},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

// Define the ABI (Application Binary Interface) for the Tic Tac Toe contract
pub struct TicTacToeAbi;

// Enum representing the possible outcomes of the game
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameOutcome {
    Winner(Player), // A player has won the game
    Continue,       // The game should continue
}

// Enum representing the two players in the game
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Player {
    #[default]
    One, // Player One
    Two, // Player Two
}

impl Player {
    // Method to get the opponent of the current player
    pub fn opponent(self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    // Method to get the index of the player (0 for Player One, 1 for Player Two)
    pub fn index(&self) -> usize {
        match self {
            Player::One => 0,
            Player::Two => 1,
        }
    }
}

// Struct representing a cell on the Tic Tac Toe board
#[derive(Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, SimpleObject)]
pub struct Cell {
    mark: Option<Player>, // The player who has marked this cell, if any
}

// Enum representing the possible operations in the game
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    MarkBoard { x: u16, y: u16 }, // Operation to mark the board at position (x, y)
}

// Implement the ContractAbi trait for TicTacToeAbi
impl ContractAbi for TicTacToeAbi {
    type Operation = Operation;
    type Response = GameOutcome;
}

// Implement the ServiceAbi trait for TicTacToeAbi
impl ServiceAbi for TicTacToeAbi {
    type Query = Request;
    type QueryResponse = Response;
}

// Wrapper struct for TimeDelta
pub struct TimeDeltaWrapper(pub TimeDelta);

// Struct representing the arguments needed to instantiate the game
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct InstantiationArgument {
    pub players: [Owner; 2], // The two players in the game
}

// Struct representing the Tic Tac Toe board
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct Board {
    cells: Vec<Cell>,      // The cells on the board
    size: u16,             // The size of the board (e.g., 3 for a 3x3 board)
    active_player: Player, // The player whose turn it is
}

impl Board {
    // Method to create a new Tic Tac Toe board
    pub fn new() -> Self {
        let size = 3; // The board is 3x3
        let cell_count = size * size;
        let cells = vec![Cell::default(); cell_count];
        Board {
            size: size.try_into().unwrap(),
            cells,
            active_player: Player::One, // Player One starts the game
        }
    }

    // Method to get the active player
    pub fn get_active_player(&self) -> Player {
        self.active_player
    }

    // Method to mark the board at position (x, y)
    pub fn mark_board(&mut self, x: u16, y: u16) -> GameOutcome {
        assert!(self.get_winner().is_none(), "Game has ended."); // Ensure the game hasn't ended
        assert!(x < self.size && y < self.size, "Invalid coordinates."); // Ensure the coordinates are valid
        assert!(self.get_cell(x, y).mark.is_none(), "Cell is not empty."); // Ensure the cell is empty
        self.place_mark(x, y); // Place the mark
        if let Some(winner) = self.get_winner() {
            return GameOutcome::Winner(winner); // Return the winner if there is one
        }
        GameOutcome::Continue // Continue the game if there is no winner
    }

    // Method to place a mark at position (x, y)
    fn place_mark(&mut self, x: u16, y: u16) {
        let player = self.active_player;
        self.get_cell_mut(x, y).mark = Some(player); // Mark the cell with the active player
        self.active_player = player.opponent(); // Switch to the opponent
    }

    // Method to get the winner of the game, if any
    pub fn get_winner(&self) -> Option<Player> {
        // Define the winning lines on the board
        let lines = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        // Check if any line is fully marked by Player One
        for &line in &lines {
            if line
                .iter()
                .all(|&(x, y)| self.get_cell(x, y).mark == Some(Player::One))
            {
                return Some(Player::One);
            }
            // Check if any line is fully marked by Player Two
            if line
                .iter()
                .all(|&(x, y)| self.get_cell(x, y).mark == Some(Player::Two))
            {
                return Some(Player::Two);
            }
        }
        None // No winner yet
    }

    // Method to get a reference to a cell at position (x, y)
    fn get_cell(&self, x: u16, y: u16) -> &Cell {
        &self.cells[x as usize + (y as usize) * (self.size as usize)]
    }

    // Method to get a mutable reference to a cell at position (x, y)
    fn get_cell_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        &mut self.cells[x as usize + (y as usize) * (self.size as usize)]
    }
}
