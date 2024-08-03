use claimeer_app::StorageQueries;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    StorageQueries::registrar().register();
}
