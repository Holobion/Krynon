// The dioxus prelude contains a ton of common items used in dioxus apps.
use dioxus::prelude::*;

use views::{About, Compare, Home, Navbar};

/// Define a components module that contains all shared components for our app.
mod components;
/// Database module for server-side persistence.
mod db;
/// Define a model module that contains the core analytical logic and mock seed data.
mod model;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout (Navbar).
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/compare")]
        Compare {},
        #[route("/philosophy")]
        About {},
}

// We import assets in dioxus with the `asset!` macro.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Launch the application using Dioxus runtime.
    dioxus::launch(App);
}

/// App is the root component of the application.
#[component]
fn App() -> Element {
    let search_query = use_signal(|| "".to_string());
    use_context_provider(|| search_query);

    rsx! {
        // Inject favicon and Tailwind CSS stylesheet into the head
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Render the router using Route enum
        Router::<Route> {}
    }
}
