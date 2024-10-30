mod recipe;
mod manager;

use manager::RecipeManager;
use std::io::{self, Write};

fn main() {
    let mut manager = RecipeManager::new();
    loop {
        println!("\n--- Recipe Manager ---");
        println!("1. Add Recipe");
        println!("2. List Recipe");
        println!("3. Find Recipe by Id");
        println!("4. Update Recipe");
        println!("5. Delete Recipe");
        println!("6. Save Recipes to File");
        println!("7. Load Recipes from File");
        println!("0. Exit");

        print!("Choose an Option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option = option.trim();

        match option {
            "1" => add_recipe(&mut manager),
            "2" => list_recipes(&manager),
            "3" => view_recipe(&manager),
            "4" => update_recipe(&mut manager),
            "5" => delete_recipe(&mut manager),
            "6" => save_recipes(&manager),
            "7" => load_recipes(&mut manager),
            "0" => break,
            _ => println!("Invalid option"),
        }
    }
}

fn add_recipe(manager: &mut RecipeManager) {
    let name = prompt("Recipe name: ");
    let ingredients = prompt("Ingredients: (separeted by comma): ");
    let instructions = prompt("Instructions: (separated by comma): ");
    let servings: u32 = prompt("Servings: ").parse().unwrap_or(1);

    let ingredients_vec = ingredients.split(',').map(String::from).collect();
    let instructions_vec = instructions.split(',').map(String::from).collect();

    manager.add_recipe(name, ingredients_vec, instructions_vec, servings);
    println!("Recipe successfully added");
}

fn list_recipes(manager: &RecipeManager) {
    for recipe in manager.get_all_recipes() {
        println!("Id: {} - Name: {}", recipe.id, recipe.name);
    }
}

fn view_recipe(manager: &RecipeManager) {
    let id: u32 = prompt("Recipe id: ").parse().unwrap_or(0);
    if let Some(recipe) = manager.get_recipe(id) {
        println!("Name: {}", recipe.name);
        println!("Ingredients: {:?}", recipe.ingredients);
        println!("Instructions: {:?}", recipe.instructions);
        println!("Servings: {}", recipe.servings);
    } else {
        println!("Recipe not found");
    }
}

fn update_recipe(manager: &mut RecipeManager) {
    let id: u32 = prompt("Recipe id: ").parse().unwrap_or(0);

    if manager.get_recipe(id).is_none() {
        println!("Recipe not found");
        return;
    }

    let name = prompt("New Recipe name ");
    let ingredients = prompt("New ingredients(separetd by comma): ");
    let instructions = prompt("New instructions(separetad by comma): ");
    let servings: u32 = prompt("New servings: ").parse().unwrap_or(1);

    let ingredients_vec = ingredients.split(',').map(String::from).collect();
    let instructions_vec = instructions.split(',').map(String::from).collect();

    if manager.update_recipe(id, name, ingredients_vec, instructions_vec, servings) {
        println!("Recipe sussessfully updated ");
    } else {
        println!("Erro updating recipe.");
    }
}

fn delete_recipe(manager: &mut RecipeManager) {
    let id: u32 = prompt("Recipe id: ").parse().unwrap_or(0);
    if manager.delete_recipe(id) {
        println!("Recipe deleted");
    } else {
        println!("Error Deleting Recipe");
    }
}

fn save_recipes(manager: &RecipeManager) {
    let filename = prompt("Name of file to be saved ");
    match manager.save_to_file(&filename) {
        Ok(_) => println!("Recipes saved"),
        Err(e) => println!("Erro saving recipes: {}", e),
    }
}

fn load_recipes(manager: &mut RecipeManager) {
    let filename = prompt("File name to load ");
    match manager.load_from_file(&filename) {
        Ok(_) => println!("Recipes loaded"),
        Err(e) => println!("Erro loading: {}", e),
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}