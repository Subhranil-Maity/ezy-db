use config::Config;
use endpoints::setup;
mod config;
mod db;
mod endpoints;

#[tokio::main]
async fn main() {
    let mut config = Config::load_config();
    let endpoint = config.get_end_point();
    let mut app = tide::new();
    setup(&mut app, config);
    app.listen(endpoint).await.unwrap();
}
