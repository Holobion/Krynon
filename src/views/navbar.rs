use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let route = use_route::<Route>();

    let home_class = if matches!(route, Route::Home {}) {
        "underline decoration-2 underline-offset-4"
    } else {
        "hover:underline hover:underline-offset-4"
    };
    let compare_class = if matches!(route, Route::Compare {}) {
        "underline decoration-2 underline-offset-4"
    } else {
        "hover:underline hover:underline-offset-4"
    };
    let about_class = if matches!(route, Route::About {}) {
        "underline decoration-2 underline-offset-4"
    } else {
        "hover:underline hover:underline-offset-4"
    };

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full bg-kr-membrane border-b-1.5 border-kr-text-nucleus px-6 py-4",
            div {
                class: "max-w-6xl mx-auto flex justify-between items-center",

                // Brand Logo (Space Grotesk, Bold, Minimalist)
                Link {
                    to: Route::Home {},
                    class: "flex items-center gap-3 group",
                    svg {
                        view_box: "0 0 100 100",
                        class: "w-12 h-12 text-kr-text-nucleus shrink-0 transition-transform duration-300 group-hover:scale-105",
                        fill: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        
                        // Left cluster (Separated facet A)
                        circle { cx: "35", cy: "50", r: "4.5" }
                        circle { cx: "30", cy: "45", r: "3.2" }
                        circle { cx: "40", cy: "42", r: "2.8" }
                        circle { cx: "32", cy: "55", r: "2.5" }
                        circle { cx: "38", cy: "58", r: "2.2" }
                        circle { cx: "25", cy: "50", r: "1.8" }
                        circle { cx: "45", cy: "50", r: "1.5" }
                        circle { cx: "28", cy: "40", r: "1.4" }
                        circle { cx: "42", cy: "62", r: "1.3" }
                        
                        // Right cluster (Separated facet B)
                        circle { cx: "65", cy: "50", r: "4.5" }
                        circle { cx: "70", cy: "55", r: "3.2" }
                        circle { cx: "60", cy: "58", r: "2.8" }
                        circle { cx: "68", cy: "45", r: "2.5" }
                        circle { cx: "62", cy: "42", r: "2.2" }
                        circle { cx: "75", cy: "50", r: "1.8" }
                        circle { cx: "55", cy: "50", r: "1.5" }
                        circle { cx: "72", cy: "60", r: "1.4" }
                        circle { cx: "58", cy: "38", r: "1.3" }
                        
                        // Floating scattered dots in between (Separation line parameter space)
                        circle { cx: "50", cy: "30", r: "0.8" }
                        circle { cx: "50", cy: "70", r: "0.8" }
                        circle { cx: "48", cy: "20", r: "0.6" }
                        circle { cx: "52", cy: "80", r: "0.6" }
                    }
                    span {
                        class: "text-lg font-bold tracking-widest text-kr-text-nucleus uppercase font-display",
                        "Krynon"
                    }
                    span {
                        class: "text-[9px] uppercase font-bold tracking-widest px-2 py-0.5 border border-kr-text-nucleus text-kr-text-nucleus font-display transition-colors group-hover:bg-kr-text-nucleus group-hover:text-kr-bg-membrane",
                        "Classification Engine"
                    }
                }

                // Nav Items
                nav {
                    class: "flex items-center gap-6 sm:gap-8 font-display text-xs uppercase font-bold tracking-wider",

                    // If we're on the home page, scroll directly to phylogeny/archive. 
                    // Otherwise, route to / first with anchor parameters.
                    a {
                        href: "/#phylogeny",
                        class: "text-kr-text-nucleus {home_class}",
                        "Concepts"
                    }
                    Link {
                        to: Route::Compare {},
                        class: "text-kr-text-nucleus {compare_class}",
                        "Workspace"
                    }
                    Link {
                        to: Route::About {},
                        class: "text-kr-text-nucleus {about_class}",
                        "Philosophy"
                    }
                }
            }
        }

        main {
            class: "min-h-[calc(100vh-73px)] bg-kr-membrane text-kr-text-nucleus flex flex-col justify-between relative",
            div {
                class: "flex-grow",
                Outlet::<Route> {}
            }

            // Shared Scientific Archival Footer
            footer {
                class: "border-t-1.5 border-kr-text-nucleus bg-kr-membrane py-8 px-6 text-xs text-kr-text-nucleus font-display",
                div {
                    class: "max-w-6xl mx-auto flex flex-col sm:flex-row justify-between items-center gap-4",
                    p {
                        class: "tracking-wider uppercase font-bold",
                        "© 2026 KRYNON. DEVELOPED BY THE HOLOBION ORGANISATION. ALL SYSTEMS INTEGRATED."
                    }
                    div {
                        class: "flex gap-6 uppercase font-bold tracking-wider",
                        Link { to: Route::About {}, class: "hover:underline", "Philosophy" }
                        a {
                            href: "https://github.com/Holobion/Krynon",
                            target: "_blank",
                            class: "hover:underline",
                            "GitHub Archive"
                        }
                    }
                }
            }
        }
    }
}
