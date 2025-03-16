use warp::Filter;
use hello_rust2::*;
use hello_rust2::database;
use hello_rust2::back_end::{api, routes};
use smol::block_on;

fn main() {
    block_on(async_main());
}

async fn async_main() {
    let pool = database::connect_db().await.expect("Không thể kết nối MySQL");
    let poem_data = route::get_poem_data(&pool).await;

    let hello_route = route::create_html_route(poem_data.clone());
    let api_route = route::create_api_route(poem_data.clone());
    let login_route = route::create_login_route(pool.clone());
    let register_route = route::create_register_route(pool.clone());
    let static_files = route::create_static_route();

    let call_login = login_route.with(warp::cors().allow_any_origin());

    println!("🚀 Server chạy tại http://localhost:8080/");

    let server_8080 = smol::spawn(async move {
        warp::serve(hello_route.or(api_route).or(register_route).or(static_files).or(call_login))
            .run(([127, 0, 0, 1], 8080))
            .await;
    });

    let pools = api::connect_db().await.expect("Không thể kết nối MySQL");
    let api_routes = routes::routess(pools.clone());

    println!("🚀 Server chạy tại http://localhost:9090/api/all");

    let server_9090 = smol::spawn(async move {
        warp::serve(api_routes).run(([127, 0, 0, 1], 9090)).await;
    });

    smol::future::block_on(async {
        server_8080.await;
        server_9090.await;
    });
}
