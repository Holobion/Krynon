use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let route = use_route::<Route>();
    let navigator = use_navigator();
    let mut search_query = use_context::<Signal<String>>();

    let home_class = if matches!(route, Route::Home {}) { "text-hb-primary" } else { "text-hb-matrix hover:text-hb-nucleus" };
    let compare_class = if matches!(route, Route::Compare {}) { "text-hb-primary" } else { "text-hb-matrix hover:text-hb-nucleus" };
    let about_class = if matches!(route, Route::About {}) { "text-hb-primary" } else { "text-hb-matrix hover:text-hb-nucleus" };

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full hb-glass border-b border-hb-matrix/10 px-6 py-4",
            div {
                class: "max-w-6xl mx-auto flex justify-between items-center",
                
                // Brand Logo
                Link {
                    to: Route::Home {},
                    class: "flex items-center gap-2 group",
                    span {
                        class: "text-2xl font-black bg-clip-text text-transparent bg-gradient-to-r from-hb-primary to-teal-500 tracking-wider",
                        "Krynon"
                    }
                    span {
                        class: "text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded bg-hb-membrane text-hb-matrix group-hover:text-hb-primary group-hover:bg-hb-membrane transition-colors border border-hb-matrix/10",
                        "v0.1"
                    }
                }

                // Search Input in Navbar
                div {
                    class: "hidden md:flex items-center gap-2 bg-hb-membrane border border-hb-matrix/10 px-3 py-1.5 rounded-xl text-hb-matrix focus-within:border-hb-primary focus-within:text-hb-nucleus transition-colors w-72 lg:w-96",
                    // Magnifying glass icon
                    svg {
                        class: "w-4 h-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                        }
                    }
                    input {
                        class: "bg-transparent border-none outline-none text-xs w-full text-hb-nucleus placeholder-hb-matrix/60",
                        value: "{search_query}",
                        placeholder: "Search products, categories, or criteria...",
                        oninput: move |e| {
                            let val = e.value();
                            *search_query.write() = val.clone();
                            if !matches!(route, Route::Compare {}) {
                                navigator.push(Route::Compare {});
                            }
                        }
                    }
                    if !search_query.read().is_empty() {
                        button {
                            class: "hover:text-hb-primary transition-colors",
                            onclick: move |_| {
                                search_query.set("".to_string());
                            },
                            // Close icon
                            svg {
                                class: "w-3.5 h-3.5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }
                    }
                }
                
                // Nav Items
                nav {
                    class: "flex items-center gap-5 sm:gap-8",
                    
                    Link {
                        to: Route::Home {},
                        class: "text-xs sm:text-sm font-bold uppercase tracking-wider transition-colors {home_class}",
                        "Home"
                    }
                    Link {
                        to: Route::Compare {},
                        class: "text-xs sm:text-sm font-bold uppercase tracking-wider transition-colors {compare_class}",
                        "Workspace"
                    }
                    Link {
                        to: Route::About {},
                        class: "text-xs sm:text-sm font-bold uppercase tracking-wider transition-colors {about_class}",
                        "Philosophy"
                    }
                }
            }
        }
        
        main {
            class: "min-h-[calc(100vh-73px)] bg-hb-membrane text-hb-nucleus flex flex-col justify-between",
            div {
                class: "flex-grow",
                Outlet::<Route> {}
            }
            
            // Shared Footer
            footer {
                class: "border-t border-hb-matrix/10 bg-hb-membrane py-8 px-6 text-center text-xs text-hb-matrix",
                div {
                    class: "max-w-6xl mx-auto flex flex-col sm:flex-row justify-between items-center gap-4",
                    p {
                        "© 2026 Krynon. A project by "
                        a {
                            href: "https://github.com/Holobion",
                            class: "font-semibold underline hover:text-hb-primary transition-colors",
                            "Holobion"
                        }
                        ". Built with Dioxus 0.7 & Tailwind CSS."
                    }
                    div {
                        class: "flex gap-4",
                        Link { to: Route::About {}, class: "hover:text-hb-nucleus transition-colors", "Philosophy" }
                        a {
                            href: "https://github.com/Holobion/Krynon",
                            class: "hover:text-hb-nucleus transition-colors",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}
