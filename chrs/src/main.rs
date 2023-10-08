#![allow(warnings, unused)]
mod app;
mod board;
mod cache;
mod ui;
mod util;

use app::App;
use pixels::Error;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("error initializing logger");
        wasm_bindgen_futures::spawn_local(App::run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::set_var("RUST_BACKTRACE", "1");
        pretty_env_logger::init();
        pollster::block_on(App::run());
    }
}


#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => (crate::util::log(&format_args!($($t)*).to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => (println!($($t)*))
}