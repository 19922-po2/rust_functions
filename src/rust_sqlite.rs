use sqlite::Connection;
use std::io;

fn connect() -> Connection {
    let connection = sqlite::open("todos.db").unwrap();
    return connection;
}

fn create_table(connection: &Connection) {
    let query = "CREATE TABLE todos (name TEXT, status INTEGER);";
    connection.execute(query).unwrap();
}

fn get_todos(connection: Connection) {
    let query = "SELECT * FROM todos WHERE status = 0";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                print!("{} = {} | ", name, value.unwrap());
            }
            println!();
            true
        })
        .unwrap();
}

fn add_todo(todo: String, connection: Connection) {
    let query = "INSERT INTO todos VALUES ('".to_string() + &todo + "', 0);";
    connection.execute(query).unwrap();
}

fn delete_todo(todo: String, connection: Connection) {
    let query = "DELETE FROM todos WHERE name = '".to_string() + &todo + "';";
    println!("{}", query);
    connection.execute(query).unwrap();
}

fn ask_user_input() -> bool {
    let connection: Connection = connect();

    let mut input: String = String::new(); // create a mutable empty String
    println!(
        "Choose: \n [1] View todos list.\n [2] Add new todo.\n [3] Delete todo.\n [4] Create new table\n [5] Exit"
    );
    io::stdin() // access standard input
        .read_line(&mut input) // read into the string
        .expect("Failed to read line"); // handle errors

    // read_line leaves a newline `\n` at the end, so trim it
    let input: usize = input.trim().parse().expect("Please type a number!\n");

    match input {
        1 => {
            println!("Todos list: ");
            get_todos(connection);
        }
        2 => {
            println!("Add new todo: ");
            let mut input: String = String::new(); // create a mutable empty String
            io::stdin() // access standard input
                .read_line(&mut input) // read into the string
                .expect("Failed to read line"); // handle errors

            let input = input.trim();
            add_todo(input.to_string(), connection);
        }
        3 => {
            println!("Delete todo: ");
            let mut input: String = String::new(); // create a mutable empty String
            io::stdin() // access standard input
                .read_line(&mut input) // read into the string
                .expect("Failed to read line"); // handle errors

            let input = input.trim();
            delete_todo(input.to_string(), connection);
        }
        4 => {
            println!("Create Table: ");
            create_table(&connection)
        }
        5 => {
            println!("Exiting todos... ");
            return false;
        }
        _ => println!("Ending"),
    }
    true // keep looping
}

#[allow(unused)]
pub fn todo_list() {
    println!("TODOS with sqilte: ");

    loop {
        if !ask_user_input() {
            break; // exit loop on option 5
        }
    }
}
