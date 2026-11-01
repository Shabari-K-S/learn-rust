use std::env;
// use std::{io::{self, Write}};
//
// struct Item {
//     id: i32,
//     completed: bool,
//     task: String,
// }

fn help_menu() {
    println!("Command: todo [option] [others]....\n");
    println!("Options:");
    println!("    help           \t Display the help menu");
    println!("    add [string]   \t Add item in the todo list");
    println!("    complete [id]  \t Mark item as a complete.");
    println!("    list           \t List all todo items");
    println!("    list-completed \t List only the completed items");
    println!("    delete [id]    \t Delete the todo item.");
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Use 'todo help' to print the help menu.");
        return;
    }

    if args[1] == "help" {
        help_menu();
        return;
    } else if args[1] == "add" {
        println!("Add TODO LIST Item");
    } else if args[1] == "complete" {
        println!("Mark ID as Completed");
    } else if args[1] == "list" {
        println!("List all the todo iteams");
    } else if args[1] == "list-completed" {
        println!("List all the completed Item");
    } else if args[1] == "delete" {
        println!("Remove todo item based on the id");
    }

    // let bar: Vec<List> = Vec::new();
    // print!(bar.)
}
