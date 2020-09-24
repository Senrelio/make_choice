mod config;
mod database;
mod graph;
mod playground;

fn main() {
    let setting = config::Setting::init().expect("read setting error.");
    let _pool = database::init_pool(setting.database_url());
}
