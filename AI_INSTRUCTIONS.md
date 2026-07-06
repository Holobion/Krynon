# Krynon: AI Development Instructions

This document provides architectural patterns, tech stack rules, and styling guidelines for any AI assistant working on the Krynon codebase.

---

## 1. Dioxus 0.7 Rules & API Patterns
This project uses **Dioxus 0.7.1**. You must strictly adhere to the updated Dioxus 0.7 API:
- **No `cx` or `Scope`**: Do not use the legacy `cx: Scope` function parameter. Component props are passed directly as function arguments.
- **Signals instead of UseState**: Use `use_signal(closure)` for local state. Call the signal like a function `my_signal()` to clone/read, or use `.write()` to obtain mutable access.
- **Components Definition**: Use the `#[component]` attribute macro. Props should be owned values, implementing `PartialEq` and `Clone`.
- **Routable Enum**: The routing tree is defined in `Route` inside `src/main.rs`. Use the `Link` component and specify `to: Route::...`.
- **Fullstack & Server Functions**: Use `#[post]` or `#[get]` for server functions. Put server-only dependencies inside `#[cfg(feature = "server")]` blocks.
- **Use `use_server_future`**: When pre-rendering or doing initial server loads, use `use_server_future` to avoid hydration mismatches.

---

## 2. Tailwind CSS Styling Guidelines
Krynon uses **Tailwind CSS v4** for all styles:
- **Automatic Compilation**: The compiler is integrated with `dx serve`. Do not manually write heavy raw CSS files. Edit classes directly inside components.
- **Modern Premium Design System**:
  - **Colors**: Use deep slate backgrounds (`bg-slate-900`, `bg-slate-950`), glowing indigo/violet accents (`text-indigo-400`, `bg-violet-600`), and semantic indicators (emerald for sustainability/success, amber for warnings/durability, rose for high price).
  - **Glassmorphism**: Use `bg-slate-800/60 backdrop-blur-md border border-slate-700/50` for cards and layouts to create a premium, futuristic look.
  - **Typography**: Inter/Outfit style clean sans-serif. Utilize readable line-heights and letter-spacing (`tracking-wide`).
  - **Micro-Animations**: Add subtle transition effects (`transition-all duration-300 ease-out hover:scale-[1.02] hover:shadow-lg hover:shadow-indigo-500/10`) to interactive cards, tabs, and buttons.

---

## 3. Database & Server Functions
- **Production (PostgreSQL)**: Integrated via `sqlx` with the `postgres` driver. Database operations reside inside server functions, executed on the backend target.
- Ensure all SQL queries are generic or standard-compliant. Do not use SQLite-specific syntax that might break when migrating to PostgreSQL.
- **Client/Server Isolation**: Place all imports related to database drivers, connection pools, or server crates within `#[cfg(feature = "server")]` blocks or inside server-only functions to prevent compilation failures on the WASM client target.

---

## 4. Multi-Criteria Engine Logic
- Keep core math simple and predictable:
  $$\text{Weighted Score} = \frac{\sum_{i} (\text{Criterion Score}_i \times \text{Weight}_i)}{\sum_{i} \text{Weight}_i}$$
- Weights ranges are integers `0` to `10` or floats `0.0` to `10.0`.
- Handlers that update weights should update the parent/local signals, triggering a reactive re-calculation of the sorted product list.
- Keep calculations memoized using `use_memo` where possible to optimize re-renders.
