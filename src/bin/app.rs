use claimly::Index;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Index>::new().render();
}
