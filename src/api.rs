const API_HOST: &str = "http://localhost:3000";

#[derive(Debug)]
pub enum Status {
  ETC,
  STARTED,
  WAITING,
}

#[tokio::main]
pub async fn fetch_status() -> Result<Status, Box<dyn std::error::Error>> {
  let result = reqwest::get(API_HOST).await?.text().await?;

  let status = match result.as_str() {
    "waiting" => Status::WAITING,
    "started" => Status::STARTED,
    _ => Status::ETC,
  };

  println!("{:?}", status);

  Ok(status)
}

#[tokio::main]
pub async fn join() -> Result<Status, Box<dyn std::error::Error>> {
  let result = reqwest::get(API_HOST).await?.text().await?;

  let status = match result.as_str() {
    "waiting" => Status::WAITING,
    "started" => Status::STARTED,
    _ => Status::ETC,
  };

  println!("{:?}", status);

  Ok(status)
}
