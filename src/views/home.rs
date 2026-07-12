use crate::Route;
use dioxus::prelude::*;

// ==========================================
// Stippled SVG Logos (Gray dots, no solids/gradients)
// ==========================================

#[component]
fn CriterionLogo() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "w-36 h-36 text-kr-text-nucleus",
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
    }
}

#[component]
fn InheritanceLogo() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "w-36 h-36 text-kr-text-nucleus",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            
            // Root node (top)
            circle { cx: "50", cy: "20", r: "5.0" }
            circle { cx: "47", cy: "17", r: "3.0" }
            circle { cx: "53", cy: "23", r: "2.5" }
            
            // Left branch child
            circle { cx: "30", cy: "50", r: "4.0" }
            circle { cx: "27", cy: "47", r: "2.2" }
            circle { cx: "33", cy: "53", r: "2.0" }
            
            // Right branch child
            circle { cx: "70", cy: "50", r: "4.0" }
            circle { cx: "67", cy: "53", r: "2.2" }
            circle { cx: "73", cy: "47", r: "2.0" }
            
            // Bottom left children
            circle { cx: "15", cy: "75", r: "3.0" }
            circle { cx: "12", cy: "73", r: "1.5" }
            circle { cx: "40", cy: "75", r: "3.0" }
            circle { cx: "43", cy: "77", r: "1.5" }
            
            // Bottom right children
            circle { cx: "60", cy: "75", r: "3.0" }
            circle { cx: "57", cy: "77", r: "1.5" }
            circle { cx: "85", cy: "75", r: "3.0" }
            circle { cx: "88", cy: "73", r: "1.5" }
            
            // Connecting stippled lines
            // Top to Left child
            circle { cx: "45", cy: "28", r: "1.0" }
            circle { cx: "40", cy: "35", r: "0.9" }
            circle { cx: "35", cy: "43", r: "1.0" }
            // Top to Right child
            circle { cx: "55", cy: "28", r: "1.0" }
            circle { cx: "60", cy: "35", r: "0.9" }
            circle { cx: "65", cy: "43", r: "1.0" }
            // Left to Bottom Left 1
            circle { cx: "26", cy: "56", r: "0.8" }
            circle { cx: "22", cy: "63", r: "0.9" }
            circle { cx: "18", cy: "69", r: "0.8" }
            // Left to Bottom Left 2
            circle { cx: "33", cy: "56", r: "0.8" }
            circle { cx: "36", cy: "63", r: "0.9" }
            circle { cx: "38", cy: "69", r: "0.8" }
            // Right to Bottom Right 1
            circle { cx: "68", cy: "56", r: "0.8" }
            circle { cx: "65", cy: "63", r: "0.9" }
            circle { cx: "62", cy: "69", r: "0.8" }
            // Right to Bottom Right 2
            circle { cx: "74", cy: "56", r: "0.8" }
            circle { cx: "78", cy: "63", r: "0.9" }
            circle { cx: "82", cy: "69", r: "0.8" }
        }
    }
}

#[component]
fn WeightingLogo() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "w-36 h-36 text-kr-text-nucleus",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            
            // Center fulcrum
            circle { cx: "50", cy: "70", r: "4.0" }
            circle { cx: "47", cy: "73", r: "2.5" }
            circle { cx: "53", cy: "73", r: "2.5" }
            
            // Central pillar
            circle { cx: "50", cy: "60", r: "1.2" }
            circle { cx: "50", cy: "50", r: "1.2" }
            circle { cx: "50", cy: "40", r: "1.2" }
            circle { cx: "50", cy: "30", r: "1.5" }
            
            // Balance beam (tilted slightly for dynamism)
            circle { cx: "50", cy: "30", r: "2.5" }
            circle { cx: "42", cy: "28", r: "1.8" }
            circle { cx: "34", cy: "26", r: "1.8" }
            circle { cx: "26", cy: "24", r: "2.2" }
            circle { cx: "58", cy: "32", r: "1.8" }
            circle { cx: "66", cy: "34", r: "1.8" }
            circle { cx: "74", cy: "36", r: "2.2" }
            
            // Left scale pan (higher up because it's lighter)
            circle { cx: "26", cy: "45", r: "3.5" }
            circle { cx: "22", cy: "43", r: "1.5" }
            circle { cx: "30", cy: "47", r: "1.5" }
            // Left scale strings
            circle { cx: "26", cy: "31", r: "0.8" }
            circle { cx: "26", cy: "38", r: "0.8" }
            
            // Right scale pan (lower down because it's heavier)
            circle { cx: "74", cy: "65", r: "4.5" }
            circle { cx: "70", cy: "63", r: "2.0" }
            circle { cx: "78", cy: "67", r: "2.0" }
            // Right scale strings
            circle { cx: "74", cy: "45", r: "0.8" }
            circle { cx: "74", cy: "55", r: "0.8" }
        }
    }
}

#[component]
fn ConsensusLogo() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "w-36 h-36 text-kr-text-nucleus",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            
            // Central target/bullseye (high density)
            circle { cx: "50", cy: "50", r: "5.5" }
            circle { cx: "47", cy: "47", r: "3.5" }
            circle { cx: "53", cy: "53", r: "3.5" }
            circle { cx: "53", cy: "47", r: "3.0" }
            circle { cx: "47", cy: "53", r: "3.0" }
            
            // Inner concentric ring
            circle { cx: "50", cy: "35", r: "1.8" }
            circle { cx: "65", cy: "50", r: "1.8" }
            circle { cx: "50", cy: "65", r: "1.8" }
            circle { cx: "35", cy: "50", r: "1.8" }
            circle { cx: "39", cy: "39", r: "1.2" }
            circle { cx: "61", cy: "39", r: "1.2" }
            circle { cx: "61", cy: "61", r: "1.2" }
            circle { cx: "39", cy: "61", r: "1.2" }
            
            // Outer incoming vectors (converging dots)
            circle { cx: "50", cy: "20", r: "1.5" }
            circle { cx: "50", cy: "10", r: "1.0" }
            
            circle { cx: "80", cy: "50", r: "1.5" }
            circle { cx: "90", cy: "50", r: "1.0" }
            
            circle { cx: "50", cy: "80", r: "1.5" }
            circle { cx: "50", cy: "90", r: "1.0" }
            
            circle { cx: "20", cy: "50", r: "1.5" }
            circle { cx: "10", cy: "50", r: "1.0" }
            
            // Diagonal vectors
            circle { cx: "28", cy: "28", r: "1.4" }
            circle { cx: "20", cy: "20", r: "0.9" }
            
            circle { cx: "72", cy: "28", r: "1.4" }
            circle { cx: "80", cy: "20", r: "0.9" }
            
            circle { cx: "72", cy: "72", r: "1.4" }
            circle { cx: "80", cy: "80", r: "0.9" }
            
            circle { cx: "28", cy: "72", r: "1.4" }
            circle { cx: "20", cy: "80", r: "0.9" }
        }
    }
}

// ==========================================
// Main Home View
// ==========================================

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "pb-24 space-y-24",

            // ------------------------------------------
            // HERO SECTION: Separate criteria. Decide with clarity.
            // ------------------------------------------
            section {
                class: "max-w-6xl mx-auto px-6 pt-16 md:pt-24",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-center",

                    // Left Column: Massive geometric text
                    div {
                        class: "lg:col-span-8 space-y-8 text-left",
                        div {
                            class: "font-mono text-xs uppercase tracking-widest text-kr-text-matrix border-l-2 border-kr-text-nucleus pl-3 py-1",
                            "Krynon Scientific Document // Ref. No. 8023-F"
                        }
                        h1 {
                            class: "text-5xl md:text-7xl font-black text-kr-text-nucleus tracking-tight leading-none uppercase font-display",
                            "Separate criteria."
                            br {}
                            "Decide with clarity."
                        }
                        p {
                            class: "font-serif italic text-lg md:text-2xl text-kr-text-matrix leading-relaxed max-w-2xl",
                            "\"Krynon is designed to embody its etymological root—krinein. By separating products into their fundamental components and evaluating them systematically, it empowers users to make objective, data-driven decisions based on their exact priorities.\""
                        }
                        
                        // Metadata detail blocks (archival style)
                        div {
                            class: "grid grid-cols-2 sm:grid-cols-3 gap-6 pt-4 font-mono text-[11px] uppercase tracking-wider text-kr-text-nucleus border-t-1.5 border-kr-text-nucleus/20",
                            div {
                                span { class: "block text-kr-text-matrix", "Engine Status" }
                                span { class: "font-bold", "Sorting Core Active" }
                            }
                            div {
                                span { class: "block text-kr-text-matrix", "Active Matrices" }
                                span { class: "font-bold", "Inherited Tree Loaded" }
                            }
                            div {
                                span { class: "block text-kr-text-matrix", "Calculation Mode" }
                                span { class: "font-bold", "Weighted Average" }
                            }
                        }
                    }

                    // Right Column: Bento Box with the stippled 'Criterion' logo
                    div {
                        class: "lg:col-span-4 flex justify-center items-center",
                        div {
                            class: "w-full max-w-sm aspect-square kr-grid-box flex flex-col justify-between items-center text-center",
                            div {
                                class: "w-full text-left font-mono text-[10px] text-kr-text-matrix border-b border-kr-text-nucleus/20 pb-2 mb-4",
                                "FIG 01. THE CRITERION // EVALUATION FACET"
                            }
                            div {
                                class: "flex-grow flex items-center justify-center py-6",
                                CriterionLogo {}
                            }
                            div {
                                class: "w-full border-t border-kr-text-nucleus/20 pt-3 text-center",
                                p { class: "font-serif italic text-xs text-kr-text-matrix", "Mathematical separation of independent parameter scores." }
                            }
                        }
                    }
                }
            }

            // ------------------------------------------
            // CONCEPTS SECTION (4-column grid of logo variations)
            // ------------------------------------------
            section {
                id: "phylogeny",
                class: "max-w-6xl mx-auto px-6",
                div {
                    class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-8 flex justify-between items-end",
                    h2 {
                        class: "text-2xl font-bold tracking-widest uppercase font-display",
                        "Taxonomy // Core Concepts"
                    }
                    span {
                        class: "font-mono text-xs text-kr-text-matrix hidden sm:inline",
                        "ENGINE_MATRIX: V0.7.1"
                    }
                }
                
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6",

                    // Criterion Bento Box
                    div {
                        class: "kr-grid-box flex flex-col justify-between p-0",
                        div { class: "kr-header-bar bg-kr-text-nucleus/5", "Concept 01 // Criterion" }
                        div {
                            class: "border-b border-kr-text-nucleus/20 aspect-square flex items-center justify-center p-8 bg-kr-bg-membrane/30",
                            CriterionLogo {}
                        }
                        div {
                            class: "p-4 space-y-2",
                            h3 { class: "font-display text-sm font-bold uppercase tracking-wider", "Criterion" }
                            p {
                                class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed",
                                "The fundamental unit of judgment. Separates items into independent, measurable facets rather than merging everything into a single arbitrary score."
                            }
                        }
                    }

                    // Inheritance Bento Box
                    div {
                        class: "kr-grid-box flex flex-col justify-between p-0",
                        div { class: "kr-header-bar bg-kr-text-nucleus/5", "Concept 02 // Inheritance" }
                        div {
                            class: "border-b border-kr-text-nucleus/20 aspect-square flex items-center justify-center p-8 bg-kr-bg-membrane/30",
                            InheritanceLogo {}
                        }
                        div {
                            class: "p-4 space-y-2",
                            h3 { class: "font-display text-sm font-bold uppercase tracking-wider", "Inheritance" }
                            p {
                                class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed",
                                "Hierarchical structure. Criteria are inherited from parent categories down to specific products, allowing unified comparisons at any abstraction level."
                            }
                        }
                    }

                    // Weighting Bento Box
                    div {
                        class: "kr-grid-box flex flex-col justify-between p-0",
                        div { class: "kr-header-bar bg-kr-text-nucleus/5", "Concept 03 // Weighting" }
                        div {
                            class: "border-b border-kr-text-nucleus/20 aspect-square flex items-center justify-center p-8 bg-kr-bg-membrane/30",
                            WeightingLogo {}
                        }
                        div {
                            class: "p-4 space-y-2",
                            h3 { class: "font-display text-sm font-bold uppercase tracking-wider", "Weighting" }
                            p {
                                class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed",
                                "Subjective scaling. Users define the priority of each criterion in real-time, allowing the engine's sorting calculations to align with individual needs."
                            }
                        }
                    }

                    // Consensus Bento Box
                    div {
                        class: "kr-grid-box flex flex-col justify-between p-0",
                        div { class: "kr-header-bar bg-kr-text-nucleus/5", "Concept 04 // Consensus" }
                        div {
                            class: "border-b border-kr-text-nucleus/20 aspect-square flex items-center justify-center p-8 bg-kr-bg-membrane/30",
                            ConsensusLogo {}
                        }
                        div {
                            class: "p-4 space-y-2",
                            h3 { class: "font-display text-sm font-bold uppercase tracking-wider", "Consensus" }
                            p {
                                class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed",
                                "The synthesized decision. Evaluates the multi-criteria scores using a weighted average model, outputting the optimal match value."
                            }
                        }
                    }
                }
            }

            // ------------------------------------------
            // REPOSITORIES SECTION (GitHub Repositories)
            // ------------------------------------------
            section {
                id: "archive",
                class: "max-w-6xl mx-auto px-6",
                div {
                    class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-8 flex justify-between items-end",
                    h2 {
                        class: "text-2xl font-bold tracking-widest uppercase font-display",
                        "Engine Repositories // Open Source Components"
                    }
                    span {
                        class: "font-mono text-xs text-kr-text-matrix hidden sm:inline",
                        "KRYNON_CORE // REPOS"
                    }
                }
                
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6",

                    // Card 1: K-CORE (Sage Header)
                    div {
                        class: "kr-grid-box p-0 flex flex-col justify-between overflow-hidden",
                        div {
                            style: "background-color: var(--color-kr-sage);",
                            class: "kr-header-bar border-b-1.5 border-kr-text-nucleus text-kr-text-nucleus flex justify-between items-center",
                            span { "REPOSITORY: HOLOBION/K-CORE" }
                            span { class: "font-mono text-[9px] px-1 border border-kr-text-nucleus bg-kr-membrane/60", "RUST" }
                        }
                        div {
                            class: "p-6 flex flex-col justify-between flex-grow h-[220px]",
                            div {
                                class: "space-y-3",
                                p {
                                    class: "font-serif italic text-sm text-kr-text-nucleus leading-relaxed",
                                    "The core rust-based classification engine. Orchestrates sorting algorithms, computes weighted score normalization, and manages criteria registries."
                                }
                            }
                            div {
                                class: "flex justify-between items-center text-[10px] font-mono border-t border-kr-text-nucleus/15 pt-4 text-kr-text-matrix",
                                span { "v0.7.1" }
                                span { "100% COVERAGE" }
                            }
                        }
                    }

                    // Card 2: K-UI (Clay Header)
                    div {
                        class: "kr-grid-box p-0 flex flex-col justify-between overflow-hidden",
                        div {
                            style: "background-color: var(--color-kr-clay);",
                            class: "kr-header-bar border-b-1.5 border-kr-text-nucleus text-kr-text-nucleus flex justify-between items-center",
                            span { "REPOSITORY: HOLOBION/K-UI" }
                            span { class: "font-mono text-[9px] px-1 border border-kr-text-nucleus bg-kr-membrane/60", "DIOXUS" }
                        }
                        div {
                            class: "p-6 flex flex-col justify-between flex-grow h-[220px]",
                            div {
                                class: "space-y-3",
                                p {
                                    class: "font-serif italic text-sm text-kr-text-nucleus leading-relaxed",
                                    "The frontend client interface. Implements the high-performance Swiss grid layout, interactive weight sliders, and criteria creation forms."
                                }
                            }
                            div {
                                class: "flex justify-between items-center text-[10px] font-mono border-t border-kr-text-nucleus/15 pt-4 text-kr-text-matrix",
                                span { "v0.7.0" }
                                span { "WASM HYDRATED" }
                            }
                        }
                    }

                    // Card 3: K-DB (Slate Header)
                    div {
                        class: "kr-grid-box p-0 flex flex-col justify-between overflow-hidden",
                        div {
                            style: "background-color: var(--color-kr-slate);",
                            class: "kr-header-bar border-b-1.5 border-kr-text-nucleus text-kr-text-nucleus flex justify-between items-center",
                            span { "REPOSITORY: HOLOBION/K-DB" }
                            span { class: "font-mono text-[9px] px-1 border border-kr-text-nucleus bg-kr-membrane/60", "POSTGRES" }
                        }
                        div {
                            class: "p-6 flex flex-col justify-between flex-grow h-[220px]",
                            div {
                                class: "space-y-3",
                                p {
                                    class: "font-serif italic text-sm text-kr-text-nucleus leading-relaxed",
                                    "The persistent database layer. Manages product schemas, relational category hierarchies, and criteria definitions using PostgreSQL."
                                }
                            }
                            div {
                                class: "flex justify-between items-center text-[10px] font-mono border-t border-kr-text-nucleus/15 pt-4 text-kr-text-matrix",
                                span { "v0.4.2" }
                                span { "ASYNC STYLED" }
                            }
                        }
                    }
                }
            }

            // ------------------------------------------
            // WORKSPACE CALL TO ACTION SECTION (Bento box)
            // ------------------------------------------
            section {
                class: "max-w-6xl mx-auto px-6",
                div {
                    class: "kr-grid-box p-0 overflow-hidden grid grid-cols-1 md:grid-cols-12 items-stretch",
                    
                    // Left bento cell: Title & Info
                    div {
                        class: "md:col-span-8 p-8 md:p-12 space-y-6 flex flex-col justify-center",
                        div {
                            class: "font-mono text-xs uppercase tracking-widest text-kr-text-matrix border-l-2 border-kr-text-nucleus pl-3 py-0.5",
                            "Analytical Workspace // Criteria Filter"
                        }
                        h2 {
                            class: "text-3xl md:text-4xl font-bold tracking-tight text-kr-text-nucleus uppercase font-display",
                            "Execute Criteria-Based Sorting"
                        }
                        p {
                            class: "font-serif italic text-base text-kr-text-matrix leading-relaxed max-w-xl",
                            "Initialize the comparison workspace to compute weight matrices, evaluate product fitness scores, and discover the optimal choices."
                        }
                    }

                    // Right bento cell: Navigation Button
                    div {
                        class: "md:col-span-4 p-8 md:p-12 border-t-1.5 md:border-t-0 md:border-l-1.5 border-kr-text-nucleus flex items-center justify-center bg-kr-text-nucleus/5",
                        Link {
                            to: Route::Compare {},
                            class: "kr-btn-pill px-8 py-4 text-center text-sm font-bold w-full uppercase transition-all duration-300",
                            "Initialize Workspace →"
                        }
                    }
                }
            }
        }
    }
}
