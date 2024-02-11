use rocket::get;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;

#[get("/")]
async fn index() -> Result<String, String> {
    let api_key = "YOUR_API_KEY";
    let city = "New York";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);

    match reqwest::get(&url).await {
        Ok(response) => {
            let body = response.text().await.unwrap();
            Ok(body)
        }
        Err(error) => {
            Err(format!("Error: {}", error))
        }
    }
}

pub async fn create_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[tokio::main]
async fn main() {
    create_rocket().await.launch().await.unwrap();
}