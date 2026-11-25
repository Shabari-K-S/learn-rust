use std::env;
use std::fs::read_to_string;

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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_config(user_relative_path: &str) -> Result<(std::ffi::OsString, String)> {
    let home_dir = std::env::var_os("HOME").ok_or("no home directory")?;
    let mut config_path = std::path::PathBuf::new();
    config_path.push(home_dir);
    config_path.push(user_relative_path);
    let config_content = if let Ok(content) = std::fs::read(&config_path) {
        String::from_utf8_lossy(&content).to_string()
    } else {
        "".to_owned()
    };
    Ok((config_path.into_os_string(), config_content))
}

fn save_config(config_path: &std::ffi::OsStr, config_content: &str) -> Result<()> {
    let dir_name = std::path::Path::new(&config_path)
        .parent()
        .ok_or("incorrect directory")?;
    std::fs::create_dir_all(dir_name)?;
    std::fs::write(&config_path, config_content)?;
    Ok(())
}

fn add_item(content: &str, config_content: &str, config_path: &std::ffi::OsStr) {}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (config_path, mut config_content) = load_config(".todo/list.txt").expect("");

    if args.len() == 1 {
        println!("Use 'todo help' to print the help menu.");
        return;
    }

    if args[1] == "help" {
        help_menu();
        return;
    } else if args[1] == "add" {
        let id = config_content.lines().count();
        config_content += &format!("{}, {}, 0\n", id, args[2]);
        save_config(&config_path, &config_content);
    } else if args[1] == "complete" {
        // mark_completed(&args[2], &config_content);
    } else if args[1] == "list" {
        println!(" Status  ID   Task ");
        for line in read_to_string(config_path).unwrap().lines() {
            let chunks: Vec<String> = line
                .to_string()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            if chunks[2] == " 0" {
                print!("   -    ");
            } else {
                print!("   x    ");
            }
            println!("  {}  {}", chunks[0], chunks[1]);
        }
        // } else if args[1] == "list-completed" {
        //     list_down_completed(&config_content);
        // } else if args[1] == "delete" {
        //     delete_item(&args[2], &config_content);
    }

    // let bar: Vec<List> = Vec::new();
    // print!(bar.)
}
