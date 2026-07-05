# Krynon: Database & Feature Evolution Roadmap

This document outlines the database architecture and feature roadmap for the Krynon classification engine, moving from a lightweight MVP to the production-ready 1.0 release.

---

## Phase 1: The Frontend Prototype (Current)
- **Data Store**: In-memory static Rust structures (`src/model.rs`).
- **Goal**: Establish the interactive UI, dynamic calculation algorithms, and styling system.
- **Benefits**: Instant page loads, zero database overhead, and perfect for initial visual design and user flow iteration.

---

## Phase 2: MVP Development (SQLite / SQLx)
For the MVP, we will introduce a local SQLite database embedded directly inside the Dioxus fullstack server target.

### 1. Database Architecture & Library
- **Library**: `sqlx` with the `sqlite` and `runtime-tokio` features. `sqlx` provides compile-time query checking which keeps Rust type-safety intact.
- **Data File**: Local file `krynon.db` stored in the workspace or application data directory.
- **Server Integration**: Dioxus server functions (`#[post]` / `#[get]`) will interact with the database on the server target (`#[cfg(feature = "server")]`).

### 2. SQLite Database Schema

```sql
-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    emoji TEXT NOT NULL
);

-- Criteria table
CREATE TABLE IF NOT EXISTS criteria (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    emoji TEXT NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- Criteria Scores table
CREATE TABLE IF NOT EXISTS product_criteria_scores (
    product_id TEXT NOT NULL,
    criterion_id TEXT NOT NULL,
    score REAL NOT NULL CHECK(score >= 0.0 AND score <= 10.0),
    PRIMARY KEY (product_id, criterion_id),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (criterion_id) REFERENCES criteria(id) ON DELETE CASCADE
);

-- Presets/Weight Profiles table
CREATE TABLE IF NOT EXISTS weight_profiles (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    weights TEXT NOT NULL, -- JSON string mapping: {"criterion_id": weight_value}
    is_system INTEGER NOT NULL DEFAULT 0, -- 1 for system presets, 0 for user presets
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
```

### 3. Migration to SQLite
- Write a database initializer that runs on server startup, creating tables if they do not exist and seeding default data.
- Expose server functions for:
  - `get_categories() -> Result<Vec<Category>, ServerFnError>`
  - `get_category_data(cat_id: String) -> Result<(Vec<Criterion>, Vec<Product>, Vec<WeightProfile>), ServerFnError>`
  - `save_user_profile(cat_id: String, name: String, weights: HashMap<String, f64>) -> Result<String, ServerFnError>`

---

## Phase 3: First Release (PostgreSQL / Dedicated DB)
As the platform scales to support concurrent users, collaborative datasets, and persistent authentication, we will transition to a dedicated PostgreSQL database.

### 1. Transition Path from SQLite to PostgreSQL
- **Query Compatibility**: Since `sqlx` query syntax is extremely similar between SQLite and Postgres, the transition will primarily involve swapping the crate features from `sqlite` to `postgres` in `Cargo.toml`.
- **Database URL**: Migrate database connection pool instantiation to read from the `DATABASE_URL` environment variable.
- **Type Conversions**: Update Rust mappings where necessary (e.g., SQLite `TEXT` columns holding JSON will map to Postgres `JSONB` native types, allowing index-optimized nested queries).

### 2. High-Priority Features for First Release
- **Production Postgres Deployment**: Database hosted on a managed service (Supabase, Neon, AWS RDS) with automated backups and connection pooling (`PgPool`).
- **User Authentication**: Simple cookie-based session authentication with Google and GitHub OAuth to isolate user profiles, custom products, and history.

---

## Phase 4: Low-Priority / Future Release Features
These features are scoped out of the MVP and the initial Postgres release to keep the development cycle fast and focused.

1. **Custom Categories & Product Submissions**:
   - Allow logged-in users to create new categories (e.g., "Electric Vehicles", "Mechanical Keyboards").
   - Enable users to submit products and rate them on defined criteria.
   - Requires a moderation/review workflow to prevent spam.

2. **Audit Logging & Collaborative History**:
   - A wiki-like edit log showing who changed what score and when.
   - Upvoting/downvoting of specific product criteria ratings.

3. **Advanced Analytical Visualizations**:
   - Dynamic SVG Radar Charts comparing up to 3 selected items side-by-side.
   - Price-to-Value Scatterplots mapping price vs. calculated weighted score.

4. **Public Shareable Configs**:
   - Short URL generator (e.g. `krynon.app/s/a7b8cd`) storing a snapshot of weight configuration.
   - Social card generation (OpenGraph images) showing the top product for a shared config.

5. **Developer API**:
   - REST or GraphQL endpoints to fetch ranked items based on weight inputs.
   - Embeddable widget (iframe or JS web component) for bloggers to include Krynon ranks on their reviews.
