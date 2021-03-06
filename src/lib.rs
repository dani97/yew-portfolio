#![recursion_limit = "2048"]

mod app;
mod pages;
mod components;

use wasm_bindgen::prelude::*;
use yew_router::{prelude::*};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/skills/"]
    Skills,
    #[to = "/work/"]
    Work,
    #[to = "/!"]
    Home
}
type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;
type RouterButton = yew_router::components::RouterButton<AppRoute>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
