#![recursion_limit = "512"]

#[cfg(feature = "doc")]
extern crate self as yewprint;

#[cfg(feature = "doc")]
mod app;
mod buttons;
mod card;
mod collapse;
#[cfg(feature = "doc")]
mod example;
mod icon;
mod menu;
mod switch;
mod tree;
mod elevation;

#[cfg(feature = "doc")]
pub use app::*;
pub use buttons::*;
pub use card::*;
pub use collapse::*;
pub use elevation::*;
#[cfg(feature = "doc")]
pub use example::*;
pub use icon::*;
pub use id_tree;
pub use menu::*;
pub use switch::*;
pub use tree::*;

use yew::virtual_dom::Classes;

#[cfg(feature = "doc")]
#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[cfg(feature = "doc")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Intent {
    Primary,
    Success,
    Warning,
    Danger,
}

impl From<Intent> for Classes {
    fn from(intent: Intent) -> Self {
        Classes::from(intent.as_ref())
    }
}

impl AsRef<str> for Intent {
    fn as_ref(&self) -> &'static str {
        match self {
            Intent::Primary => "bp3-intent-primary",
            Intent::Success => "bp3-intent-success",
            Intent::Warning => "bp3-intent-warning",
            Intent::Danger => "bp3-intent-danger",
        }
    }
}
