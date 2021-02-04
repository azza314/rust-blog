use dotenv::dotenv; 
use std::env; 

fn main() {
    dotenv().ok(); // set env vars based on content of .env

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    let app = blog_actis::Blog::new(8998);
    app.run(database_url)
}
