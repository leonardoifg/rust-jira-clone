mod db;
mod models;

fn main() {
    let aux = models::Status::InProgress;
    println!("Welcome To My-Jira: {aux}!");
}
