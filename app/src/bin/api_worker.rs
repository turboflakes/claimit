use claimit_workers::Worker;
use yew_agent::Registrable;

fn main() {
    // TODO: make logger to only print info logs when depployed live
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    tracing_wasm::set_as_global_default();
    Worker::registrar().register();
}
