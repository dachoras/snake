//! Snake Frontend Entrypoint
//!
//! This module declares the component submodules of the Yew client
//! and initializes the web application renderer with the main `App` component.
//! It serves as the application crate root.

mod api;
mod app;
mod components;
mod i18n;
mod storage;
mod types;

use app::App;

fn main() {
    // Render the App component into the root DOM element
    yew::Renderer::<App>::new().render();
}
