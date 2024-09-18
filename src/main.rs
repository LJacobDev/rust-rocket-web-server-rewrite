#[macro_use]
extern crate rocket;

use core::num;
use std::result;

use rocket::{http::Status, serde::{json::Json, Deserialize, Serialize}, State};

#[launch]
fn launch_rocket() -> _ {
    //create a sqlite database and make sure a 'todo_list' table exists on it
    {

        println!("Initializing server");

        let db_connection = rusqlite::Connection::open("data.sqlite")
            .expect("Database connection failed.  Terminating program.");

        let result = db_connection.execute(
            "CREATE TABLE if not exists todo_items
                (
                    id integer primary key,
                    item varchar(255) not null
                );",
            (),
        );

        println!("{:?}", result);
    } //database connection is made in this inner scope so that it is dropped after creating it.  Will reconnect on each request for this application.

    rocket::build().mount(
        "/",
        routes![index, fetch_todo_items, add_todo_item, delete_todo_item],
    )
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the rewritten rocket server!"
}

#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

#[get("/todo")]
fn fetch_todo_items() -> Result<Json<ToDoList>, String> {


    let db_connection = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Unable to connect to database"))
    };

    let mut statement = match db_connection.prepare("select * from todo_items;") {
        Ok(statement) => statement,
        Err(message) => return Err(String::from("Error creating SQL statement") + &format!("{}", message))
    };

    let results = statement.query_map((), |row| {
        Ok(
            ToDoItem {
                id: row.get(0)?,
                item: row.get(1)?
            }
        )
    });


    //I had to look at the original code for this one line, as it wasn't working from writing it like this from memory
    // let results: rusqlite::Result<Vec<ToDoItem>> = rows.collect();

    match results {
        Ok(rows) => {

            let collection: rusqlite::Result<Vec<ToDoItem>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList{ items })),
                Err(_) => Err(String::from("Error gathering rows into a vector of ToDoItems"))
            }
        }
        Err(_) => Err(String::from("Unable to parse rows from results"))
        
    }

}


#[post("/todo", format = "json", data = "<item>")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Error connecting to database"))
    };

    let mut statement = match db_connection.prepare("insert into todo_items values (null, $1);") {
        Ok(statement) => statement,
        Err(_) => return Err(String::from("Failed to prepare SQL statement"))
    };

    let result = statement.execute(&[&item.0]);

    match result {
        Ok(number_rows) => Ok(Json(StatusMessage { message: format!("{} rows inserted", number_rows)})),
        Err(_) => return Err(String::from("Error inserting values"))
    }
}

#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Error connecting to database"))
    };

    let mut statement = match db_connection.prepare("delete from todo_items where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err(String::from("Error preparing statement"))
    };

    let result = statement.execute(&[&id]);

    match result {
        Ok(number_rows) => Ok(Json(StatusMessage{ message: format!("{} rows deleted", number_rows)})),
        Err(_) => return Err(String::from("Error deleting rows"))
    }
}