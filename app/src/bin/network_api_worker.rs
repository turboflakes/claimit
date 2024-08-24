use claimeer_workers::Worker;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    Worker::registrar().register();
}
