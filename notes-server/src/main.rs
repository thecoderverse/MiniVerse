mod cache;
mod entity;
mod utility;
mod route;

use route::get_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST"]);

    let routes = get_routes(cors);

    println!("Server is running on http://localhost:5555");

    println!("Routes:");
    println!("- GET     /");
    println!("- GET     /note");
    println!("- POST    /note/create");
    println!("- DELETE  /note/delete/{{id}}");
    println!("Press CTRL+C to stop the server");
    warp::serve(routes).run(([0, 0, 0, 0], 5555)).await;

    Ok(())
}