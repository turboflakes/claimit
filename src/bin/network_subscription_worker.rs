use claimeer::BlockSubscription;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    BlockSubscription::registrar().register();
}
