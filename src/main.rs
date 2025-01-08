mod db;
mod models;

fn main() {
    let aux = models::Status::InProgress;
    println!("Welcome To My-Jira: {aux}!");
    let aux = models::Epic::new("Epic name".to_string(), "Epic description".to_string());
    println!("Epic structure display: {aux}!");
    let aux = models::Story::new("Story name".to_string(), "Epic description".to_string());
    println!("Epic structure display: {aux}!");
}
