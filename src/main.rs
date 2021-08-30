#[macro_use]
extern crate rocket;

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    tokio::sync::RwLock,
    State,
};

type ID = usize;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: ID,
    name: String,
}

impl Todo {
    fn new(id: ID, dto: TodoDto) -> Todo {
        Todo {
            id: id,
            name: dto.name,
        }
    }
}

#[derive(Debug, Deserialize, FromForm)]
struct TodoDto {
    name: String,
}

type TodoVec = RwLock<Vec<Todo>>;

#[get("/")]
fn index() -> &'static str {
    "Hello Todos"
}

#[get("/todos")]
async fn find_all(todo_vec: &State<TodoVec>) -> Json<Vec<Todo>> {
    let todos = todo_vec.read().await;
    Json(todos.to_vec())
}

#[post("/todos", format = "json", data = "<dto>")]
async fn create(dto: Json<TodoDto>, todo_vec: &State<TodoVec>) -> Json<Todo> {
    let todos = todo_vec.read().await;
    let id = todos.len() + 1;
    drop(todos);
    let todo = Todo::new(id, dto.0);
    todo_vec.write().await.push(todo.clone());
    Json(todo)
}

#[get("/todos/<id>")]
async fn find_one(id: ID, todo_vec: &State<TodoVec>) -> Option<Json<Todo>> {
    let todos = todo_vec.read().await;
    let todo = todos.iter().find(|t| t.id == id);
    todo.map(|t| Json(t.clone()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, find_all, create, find_one])
        .manage(TodoVec::new(vec![]))
}
