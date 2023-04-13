use tide::{Request, Server};

use crate::config::Config;

static mut CONFIG: Option<Config> = None;

fn get_config() -> Config {
    unsafe {
        match &CONFIG {
            Some(s) => s.to_owned(),
            None => panic!("Config not initilised"),
        }
    }
}

pub fn setup(app: &mut Server<()>, config: Config) {
    unsafe {
        CONFIG = Some(config);
    }
    app.at("/").get(root);
}
async fn root(_: Request<()>) -> tide::Result {
    Ok(get_config().to_json().into())
}

async fn get_data() -> tide::Result{
    unimplemented!()
}
async fn set_data() -> tide::Result{
    unimplemented!()
}
async fn delete_data() -> tide::Result{
    unimplemented!()
}
