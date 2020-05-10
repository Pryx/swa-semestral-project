use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
extern crate config;
use std::collections::HashMap;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("AUTH_MICRO")).unwrap();

    let config = settings.try_into::<HashMap<String, String>>().unwrap();

    if config.contains_key("database_url") {
        let database_url = &config["database_url"];

        //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        init_pool(&database_url).expect(&format!("Error connecting to {}", database_url))
    } else{
        panic!("No connection specified!")
    }

}