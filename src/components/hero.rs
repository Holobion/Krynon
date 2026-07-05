use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "relative overflow-hidden pt-12 pb-16 md:py-24",
            div {
                class: "max-w-6xl mx-auto px-6 grid grid-cols-1 md:grid-cols-12 gap-12 items-center",
                
                // Left Column: Hero Text
                div {
                    class: "md:col-span-7 flex flex-col items-start text-left space-y-6",
                    
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-hb-primary/10 border border-hb-primary/20 text-hb-primary text-xs font-semibold uppercase tracking-wider",
                        span { "🎯 Objective & Data-Driven" }
                    }
                    
                    h1 {
                        class: "font-black tracking-tight text-5xl md:text-6xl text-hb-nucleus leading-[1.1] font-display",
                        "Unveil the perfect choice with "
                        span {
                            class: "bg-clip-text text-transparent bg-gradient-to-r from-hb-primary via-emerald-400 to-teal-500",
                            "Krynon"
                        }
                    }
                    
                    p {
                        class: "text-hb-matrix text-lg leading-relaxed max-w-xl",
                        "Say goodbye to arbitrary 5-star reviews. Krynon is an analytical classification engine that sorts products based on details that actually matter to you. Customize criteria weights to find what fits your life."
                    }
                    
                    div {
                        class: "flex flex-col sm:flex-row gap-4 w-full sm:w-auto pt-2",
                        Link {
                            to: Route::Compare {},
                            class: "hb-btn-pill px-8 py-3.5 text-center shadow-lg active:scale-[0.98]",
                            "Open Engine Workspace"
                        }
                        Link {
                            to: Route::About {},
                            class: "inline-flex items-center justify-center px-8 py-3.5 bg-hb-cytoplasm hover:bg-hb-membrane border border-hb-matrix/20 text-hb-nucleus font-semibold rounded-full transition-all duration-200 hover:scale-[1.01] active:scale-[0.99] text-center shadow-sm",
                            "Read Our Philosophy"
                        }
                    }
                }
                
                // Right Column: Hero Graphic (Interactive/Animated Mockup)
                div {
                    class: "md:col-span-5 flex justify-center relative w-full py-8 md:py-0",
                    // Glowing background blobs
                    div { class: "absolute w-72 h-72 bg-hb-primary/15 rounded-full blur-3xl -z-10" }
                    div { class: "absolute w-48 h-48 -bottom-8 -left-8 bg-teal-400/10 rounded-full blur-3xl -z-10" }
                    
                    // Main Container for overlapping cards
                    div {
                        class: "relative w-full max-w-sm aspect-[4/3] flex items-center justify-center",
                        
                        // Back Card: Criteria Weights Editor
                        div {
                            class: "absolute top-0 left-4 w-[90%] bg-slate-900 text-white rounded-2xl p-5 shadow-lg border border-slate-800 -rotate-3 translate-y-[-10px] opacity-90 transition-all duration-300 hover:rotate-0 hover:translate-y-[-15px] hover:z-20",
                            div {
                                class: "flex items-center justify-between mb-4 border-b border-slate-800 pb-2",
                                span { class: "text-xs font-bold tracking-wide uppercase text-slate-400", "Criteria Weights" }
                                span { class: "text-[10px] bg-slate-800 px-2 py-0.5 rounded text-emerald-400 font-mono", "w_sum = 1.0" }
                            }
                            div {
                                class: "space-y-2.5",
                                // Weight 1
                                div {
                                    class: "space-y-1",
                                    div {
                                        class: "flex justify-between text-[11px] font-medium text-slate-300",
                                        span { "Antioxidants" }
                                        span { "0.50" }
                                    }
                                    div { class: "h-1 bg-slate-800 rounded-full", div { class: "h-full bg-emerald-500 rounded-full w-1/2" } }
                                }
                                // Weight 2
                                div {
                                    class: "space-y-1",
                                    div {
                                        class: "flex justify-between text-[11px] font-medium text-slate-300",
                                        span { "Price/Value" }
                                        span { "0.30" }
                                    }
                                    div { class: "h-1 bg-slate-800 rounded-full", div { class: "h-full bg-emerald-500 rounded-full w-[30%]" } }
                                }
                                // Weight 3
                                div {
                                    class: "space-y-1",
                                    div {
                                        class: "flex justify-between text-[11px] font-medium text-slate-300",
                                        span { "Organic Cert." }
                                        span { "0.20" }
                                    }
                                    div { class: "h-1 bg-slate-800 rounded-full", div { class: "h-full bg-emerald-500 rounded-full w-1/5" } }
                                }
                            }
                        }

                        // Front Card: Classification Results
                        div {
                            class: "absolute bottom-0 right-4 w-[90%] bg-white/95 backdrop-blur-md rounded-2xl border border-slate-200/60 p-5 shadow-xl rotate-3 translate-y-[10px] transition-all duration-300 hover:rotate-0 hover:translate-y-[5px] hover:z-20",
                            
                            div {
                                class: "flex items-center justify-between mb-4 border-b border-slate-100 pb-2",
                                div {
                                    class: "flex items-center gap-2",
                                    div { class: "w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse" }
                                    span { class: "text-xs font-bold text-slate-800", "Top Classified Picks" }
                                }
                                span { class: "text-[10px] text-slate-500", "Matcha Engine" }
                            }
                            
                            div {
                                class: "space-y-3",
                                // Winner
                                div {
                                    class: "flex items-center justify-between p-2 rounded-lg bg-emerald-50/50 border border-emerald-100",
                                    div {
                                        class: "flex items-center gap-2",
                                        span { class: "text-xs font-bold text-emerald-700", "1st" }
                                        span { class: "text-xs font-semibold text-slate-700", "Matcha Premium" }
                                    }
                                    span { class: "text-xs font-bold text-emerald-600", "9.4" }
                                }
                                // Runner up
                                div {
                                    class: "flex items-center justify-between p-2 rounded-lg bg-slate-50/50 border border-slate-100",
                                    div {
                                        class: "flex items-center gap-2",
                                        span { class: "text-xs font-bold text-slate-400", "2nd" }
                                        span { class: "text-xs font-semibold text-slate-700", "Sencha Green" }
                                    }
                                    span { class: "text-xs font-bold text-slate-600", "8.1" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
