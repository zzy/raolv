#[tokio::main]
async fn main() {
    raolv::db::init().await;
    raolv::db::schema::ensure_tables()
        .await
        .unwrap_or_else(|e| panic!("ensure tables: {e}"));

    raolv::db::arcs::seed_arcs()
        .await
        .unwrap_or_else(|e| panic!("seed arcs: {e}"));

    topcoat::start(raolv::app::router()).await.unwrap();
}
