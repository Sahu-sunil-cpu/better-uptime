use std::env;
use dotenvy::dotenv;
pub struct Config {
    pub db_url: String,
}

impl Default for  Config {
 fn default() -> Self {
 println!("{}", dotenv().unwrap().display());
    let db_url = env::var("DATABASE_URL").
    unwrap_or_else(|_|panic!( "provide environment db-url" ));
    

    Self {
        db_url
    }
  }
}