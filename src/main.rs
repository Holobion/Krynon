// The dioxus prelude contains a ton of common items used in dioxus apps.
use dioxus::prelude::*;

use views::{About, CategoryPage, Compare, Home, Navbar, WorkspacePage};

/// Define a components module that contains all shared components for our app.
mod components;
/// Database module for server-side persistence.
mod db;
/// Internationalization translation module.
pub mod i18n;
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
        #[route("/category/:id")]
        CategoryPage { id: String },
        #[route("/product-type/:id")]
        WorkspacePage { id: String },
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

    // UI language selection (used for static UI labels and the active locale
    // when resolving translation maps).
    let language = use_signal(|| i18n::Language::English);
    use_context_provider(|| language);

    // BCP 47 locale strings (database-backed). The default locale is used as
    // the fallback when an entity is missing a translation for the active
    // language.
    let mut default_locale = use_signal(|| "en".to_string());
    use_context_provider(|| default_locale);
    let mut enabled_locales = use_signal(Vec::<String>::new);
    use_context_provider(|| enabled_locales);

    // Load locale settings on mount.
    let locale_settings = use_resource(|| async { model::get_supported_languages().await });
    use_effect(move || {
        if let Some(Ok(list)) = locale_settings() {
            let mut new_default = "en".to_string();
            for lang in &list {
                if lang.is_default {
                    new_default = lang.code.clone();
                }
            }
            default_locale.set(new_default);
            let enabled: Vec<String> = list
                .into_iter()
                .filter(|l| l.is_enabled)
                .map(|l| l.code)
                .collect();
            enabled_locales.set(enabled);
        }
    });

    rsx! {
        // Inject favicon and Tailwind CSS stylesheet into the head
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Render the router using Route enum
        Router::<Route> {}
    }
}
