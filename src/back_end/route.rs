use warp::Filter;
use sqlx::mysql::MySqlPool;
use crate::back_end::api::get_all_tables;

pub fn routes(db: MySqlPool) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let db_filter = warp::any().map(move || db.clone());

    let all_data_route = warp::path!("api" / "all")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_all_tables);

    all_data_route
}
