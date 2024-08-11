# The B∀CKROOMS

This project is a maze game developed in Rust, inspired by the concept of navigating through a labyrinth. The game features a first-person perspective where the player must find their way through the maze while avoiding a roaming enemy. The game includes sound effects and background music to enhance the experience.

## Features

- **First-person maze exploration:** Navigate through a complex maze and find the exit.
- **Enemy AI:** An enemy sprite roams the maze, attempting to catch the player.
- **Dynamic sound effects:** Background music and sound effects for victories and losses are included.
- **Mini-map:** A mini-map is provided to help you track your position within the maze.
- **Optimized performance:** The game can be run in both debug and release modes, offering better performance when needed.

## Gameplay

[![Watch the video](./assets/The%20B∀CKROOMS.png)](https://www.youtube.com/watch?v=qv5wwNgyxZY)

## Getting Started

### Prerequisites

To build and run this project, you need to have Rust installed. If you don't have Rust installed, you can download it from [here](https://www.rust-lang.org/tools/install).

### Clone the Repository

First, clone the repository to your local machine using the following command:

```bash
git clone <your-repository-url>

cd <your-repository-folder>
```

### Running the Game

You can run the game in two modes:

1. **Debug Mode:** This is the default mode and is useful for development and testing.

```bash
cargo run
```

2. **Release Mode:** This mode compiles the game with optimizations, providing better performance. Use this mode when you're ready to play the game in its final form.

```bash
cargo run --release
```

### Controls

- **WASD/Arrow Keys:** Move forward, backward, and turn left or right.
- **Mouse Movement:** Control the camera's direction.
- **ESC:** Exit the game.

### Project Structure

- **`src/`**: Contains all the Rust source files for the game.
- **`assets/`**: Contains all the assets like images and sound files used in the game.

### Branches

The project is organized into several branches:

- **`main`**: The primary branch for the project.
- **`add-dependencies`**: Contains the dependencies required for the project.
- **`add-maze-and-font`**: Adds the maze layout and font used in the game.
- **`add-media-files`**: Adds all the image and sound files used in the project.
- **`videogame`**: Implements the core gameplay features, including enemy AI and player controls.

## Acknowledgements

- Inspired by various maze games and first-person exploration games.
- Background music and sound effects enhance the gameplay experience.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
