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
    // when resolving translation maps). The choice is persisted in a
    // `krynon.lang` cookie (so the next SSR renders the right language with
    // no English flash) and on the client in localStorage (so subsequent
    // client-side renders are immediate).
    #[allow(unused_mut)]
    let mut language = use_signal(|| {
        // On the client, synchronously read the saved language from
        // localStorage. The `use_signal` initializer runs on the client
        // *before* the first render, so the very first paint already shows
        // the user's preferred language. On the server (SSR) it falls back
        // to English; the SSR'd HTML is then upgraded by the cookie as soon
        // as the server function below resolves.
        #[cfg(target_family = "wasm")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(stored)) = window
                    .local_storage()
                    .map(|s| s.and_then(|s| s.get_item("krynon.language").ok().flatten()))
                {
                    if stored == "fr" {
                        return i18n::Language::French;
                    }
                    if stored == "en" {
                        return i18n::Language::English;
                    }
                }
            }
        }
        i18n::Language::English
    });

    use_context_provider(|| language);

    // Whenever the language changes, persist it both to localStorage and to
    // the `krynon.lang` cookie (so the next SSR sees it).
    #[cfg(target_family = "wasm")]
    {
        use wasm_bindgen::JsCast;

        use_effect(move || {
            let code = language().as_code().to_string();
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("krynon.language", &code);
                }
                // The cookie() / set_cookie() methods live on `HtmlDocument`,
                // not on the generic `Document` interface, so we downcast.
                if let Some(html_document) = window
                    .document()
                    .and_then(|d| d.dyn_into::<web_sys::HtmlDocument>().ok())
                {
                    let max_age = 60 * 60 * 24 * 365; // 1 year
                    let new_cookie = format!(
                        "krynon.lang={code}; path=/; max-age={max_age}; SameSite=Lax"
                    );
                    let current = html_document.cookie().ok();
                    let combined = match current {
                        Some(c) if c.is_empty() => new_cookie,
                        Some(c) if c.contains("krynon.lang=") => {
                            let parts: Vec<&str> = c.split(';').collect();
                            let mut kept: Vec<&str> = Vec::new();
                            for p in parts {
                                if !p.trim_start().starts_with("krynon.lang=") {
                                    kept.push(p);
                                }
                            }
                            let mut s = kept.join(";");
                            if !s.is_empty() {
                                s.push(';');
                            }
                            s.push_str(&new_cookie);
                            s
                        }
                        Some(c) => {
                            let mut s = c;
                            s.push(';');
                            s.push_str(&new_cookie);
                            s
                        }
                        None => new_cookie,
                    };
                    let _ = html_document.set_cookie(&combined);
                }
            }
        });
    }

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
