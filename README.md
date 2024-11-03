# Recipe Manager

A Recipe Manager application built in Rust using the `iced` framework, allowing users to manage, add, edit, and delete recipes with JSON file storage.

## Features

- Add new recipes with name, ingredients, instructions, and servings.
- Edit existing recipes.
- Delete recipes.
- Save recipes to a JSON file.
- Load recipes from a JSON file.

## Project Structure

- **src/main.rs**: Entry point of the application and UI logic.
- **src/recipe.rs**: Defines the `Recipe` struct.
- **src/manager.rs**: Implements the `RecipeManager` struct for managing recipes.
- **src/ui.rs**: Contains the GUI components and logic.

## Requirements

- Rust and Cargo installed on your machine.

## Getting Started

1. Clone the repository:
   ```bash
   git clone {repository-url}.git
   cd recipe_manager
- **Note**: Fork this repository and replace `{repository-url}` with your own URL.
2. Build and run the application:

    ```bash
    cargo run --package recipe_manager --bin recipe_manager
## Usage
- Input recipe details in the provided fields.
- Use buttons to add, edit, or delete recipes.
- Save and load recipes using the corresponding buttons.
