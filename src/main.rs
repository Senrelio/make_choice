use make_choice::run_http_server;
use make_choice::init_pool;

#[async_std::main]
async fn main() {
    let pool = init_pool().await;
    run_http_server(&pool).await;
}
