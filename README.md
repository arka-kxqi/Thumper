# Thumper The Game

## Overview
"Thumper The Game" is a classic Tic Tac Toe game developed using the Linera SDK. Designed for two players, this game brings the traditional Tic Tac Toe experience to the blockchain. Players compete by marking a 3x3 grid, with the game determining the winner when a player aligns three marks horizontally, vertically, or diagonally.

## Prerequisites
- Rust and Cargo installed
- Linera SDK installed
- Bash shell

## Building the Project
To build the project, use the following command in your terminal:
```bash
cargo build
```
This command compiles the Rust code and prepares it for deployment.

## Deployment
Deploy "Thumper The Game" using the `deploy.sh` script provided in the repository. This script automates the deployment process by setting up the Linera network, publishing the application, and starting the necessary services.

### Steps to Deploy
1. Ensure the `deploy.sh` script is executable. If not, make it executable with:
    ```bash
    chmod +x deploy.sh
    ```
2. Execute the deployment script:
    ```bash
    ./deploy.sh
    ```

## `deploy.sh` Script
The `deploy.sh` script is responsible for:
- Building the project using Cargo to generate the executable and WebAssembly files.
- Updating the system PATH to include the `target/debug` directory.
- Initializing the Linera network and configuring wallet variables.
- Defining chain and owner variables for the Linera network setup.
- Publishing the application bytecode and capturing the `APP_ID` for further use.
- Starting Linera services on the specified ports to run the game.

## Running the Game
Once deployed, the game services will be available on ports 8080 and 8081. You can interact with "Thumper The Game" through:
- Linera SDK commands for blockchain interactions.
- A web interface (if provided), where you can start a new game and make moves.

## Inspiration
The project was inspired by the desire to merge classic game mechanics with modern blockchain technology. By leveraging the Linera SDK, we aimed to demonstrate the potential of Rust and blockchain in creating engaging and interactive applications beyond NFTs.

## What it does
"Thumper The Game" offers a classic Tic Tac Toe experience where two players take turns marking a 3x3 grid. The game checks for a winner after every move, declaring the first player to align three marks in a row, column, or diagonal as the winner. The game’s state is managed on the blockchain, ensuring integrity and persistence.

## How we built it
The game was developed using Rust and the Linera SDK, which provided the following components:
- **Game Contract:** Handles the core game logic, including move validation and win condition checks.
- **GraphQL API Service:** Facilitates interactions with the game, allowing players to make moves and query the game state.
- **Deployment Script:** Automates the setup and configuration of the Linera network and services, ensuring a smooth deployment process.

## Challenges we ran into
- **State Management:** Ensuring the game state was consistently managed and persisted across different sessions was challenging.
- **Game Logic:** Implementing game logic to handle various edge cases, such as invalid moves and detecting win conditions, required thorough testing and debugging.
- **SDK Integration:** Integrating the Linera SDK with Rust and ensuring secure, blockchain-based state management presented technical challenges.

## Accomplishments that we're proud of
- Successfully implementing a fully functional Tic Tac Toe game with blockchain integration.
- Creating a robust game logic system that accurately handles all possible game scenarios and maintains game integrity.
- Developing a seamless deployment process with the `deploy.sh` script, simplifying setup and configuration for users.

## What we learned
- Gained a deep understanding of Rust programming and its applications in blockchain development.
- Acquired experience with the Linera SDK and its capabilities for building decentralized applications.
- Learned how to design and implement secure and efficient game logic that operates on a blockchain.

## What's next for Thumper The Game
- **Web Interface:** Developing a user-friendly web interface to enhance accessibility and provide a more engaging experience.
- **Additional Features:** Implementing player statistics, leaderboards, and other features to increase game depth and user interaction.
- **Exploration:** Investigating the development of other classic games using the Linera SDK and Rust to further explore blockchain-based gaming opportunities.

## Built With
- **Linera SDK:** For blockchain integration and state management.
- **Rust:** For game logic and backend development.