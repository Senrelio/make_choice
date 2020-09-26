use node_travel::config;
use node_travel::database;

fn main() {
    let setting = config::Setting::init().expect("read setting error.");
    let _pool = database::init_pool(setting.database_url());
}
