use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{types::Json, PgPool, Row};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// Multilingual data model
// ----------------------------------------------------------------------------
// All user-facing text (name, description) is stored ONLY in the typed
// translation tables, keyed by BCP 47 locale code. Missing translations are
// never fabricated; the frontend performs a deterministic fallback at render
// time. Each entity carries a `translations: HashMap<Locale, LocalizedText>`
// map and helpers to query the localized text for a requested locale.
// ============================================================================

/// BCP 47 locale tag (e.g. `en`, `fr`, `fr-CA`).
pub type Locale = String;

/// Localized content for a single locale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalizedText {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// All translations available for an entity, keyed by BCP 47 locale code.
pub type Translations = HashMap<Locale, LocalizedText>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    pub id: String,
    pub emoji: String,
    pub translations: Translations,
}

impl Criterion {
    /// Returns the localized `name` for the requested locale, falling back to
    /// the entity's default locale, then to the first available translation.
    /// Returns `None` only if no translations exist.
    #[allow(dead_code)]
    pub fn localized_name(
        &self,
        requested: &str,
        default_locale: &str,
    ) -> Option<String> {
        localized_value(&self.translations, requested, default_locale, |t| {
            t.name.clone()
        })
    }

    #[allow(dead_code)]
    pub fn localized_description(
        &self,
        requested: &str,
        default_locale: &str,
    ) -> Option<String> {
        localized_value(&self.translations, requested, default_locale, |t| {
            t.description.clone().unwrap_or_default()
        })
        .filter(|s: &String| !s.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub emoji: String,
    pub translations: Translations,
    pub criteria: Vec<Criterion>,
}

impl Category {
    #[allow(dead_code)]
    pub fn localized_name(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| t.name.clone())
    }
    #[allow(dead_code)]
    pub fn localized_description(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| {
            t.description.clone().unwrap_or_default()
        })
        .filter(|s: &String| !s.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductType {
    pub id: String,
    pub emoji: String,
    pub translations: Translations,
    pub category_ids: Vec<String>,
    pub specific_criteria: Vec<Criterion>,
    pub presets: Vec<WeightProfile>,
}

impl ProductType {
    #[allow(dead_code)]
    pub fn localized_name(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| t.name.clone())
    }
    #[allow(dead_code)]
    pub fn localized_description(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| {
            t.description.clone().unwrap_or_default()
        })
        .filter(|s: &String| !s.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub price: f64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub product_type_id: String,
    pub translations: Translations,
    pub scores: HashMap<String, f64>,
}

impl Product {
    #[allow(dead_code)]
    pub fn localized_name(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| t.name.clone())
    }
    #[allow(dead_code)]
    pub fn localized_description(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| {
            t.description.clone().unwrap_or_default()
        })
        .filter(|s: &String| !s.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightProfile {
    pub translations: Translations,
    pub weights: HashMap<String, f64>,
}

impl WeightProfile {
    #[allow(dead_code)]
    pub fn localized_name(&self, requested: &str, default: &str) -> Option<String> {
        localized_value(&self.translations, requested, default, |t| t.name.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppData {
    pub categories: Vec<Category>,
    pub product_types: Vec<ProductType>,
    pub products: Vec<Product>,
    pub default_locale: String,
    pub enabled_locales: Vec<Locale>,
}

// ----------------------------------------------------------------------------
// Multilingual inputs
// ----------------------------------------------------------------------------

/// Translation entry submitted alongside a new entity. Only the active UI
/// language is required; other locales are optional and stored only when
/// non-empty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationEntry {
    pub locale: Locale,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewCategoryInput {
    pub emoji: String,
    pub translations: Vec<TranslationEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewProductTypeInput {
    pub emoji: String,
    pub translations: Vec<TranslationEntry>,
    pub category_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewProductInput {
    pub product_type_id: String,
    pub price: f64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub translations: Vec<TranslationEntry>,
    pub scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportedLanguage {
    pub code: Locale,
    pub english_name: String,
    pub native_name: String,
    pub is_default: bool,
    pub is_enabled: bool,
}

// ----------------------------------------------------------------------------
// Fallback resolution (used by the frontend; the backend keeps raw maps)
// ----------------------------------------------------------------------------

/// Resolve a single localized value using the standardized fallback chain:
/// 1. exact requested locale (e.g. `fr-CA`);
/// 2. base-language match (e.g. `fr` from `fr-CA`);
/// 3. the database default locale;
/// 4. the first available translation (any locale);
/// 5. `None` if no translation exists.
pub fn localized_value<F>(
    translations: &Translations,
    requested: &str,
    default_locale: &str,
    pick: F,
) -> Option<String>
where
    F: Fn(&LocalizedText) -> String,
{
    if let Some(t) = translations.get(requested) {
        let v = pick(t);
        if !v.is_empty() {
            return Some(v);
        }
    }
    let base = requested.split('-').next().unwrap_or(requested);
    if base != requested {
        if let Some(t) = translations.get(base) {
            let v = pick(t);
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    if let Some(t) = translations.get(default_locale) {
        let v = pick(t);
        if !v.is_empty() {
            return Some(v);
        }
    }
    for t in translations.values() {
        let v = pick(t);
        if !v.is_empty() {
            return Some(v);
        }
    }
    None
}

/// Helper: convenience to resolve a missing-content label for UI use only.
pub fn missing_label() -> &'static str {
    "—"
}

// ============================================================================
// Frontend-only convenience wrappers
// ----------------------------------------------------------------------------
// The frontend wants a single expression that resolves a localized name
// regardless of which entity it is working with. The wrappers below make
// call-sites short while still performing the full fallback chain.
// ============================================================================

pub fn tr_name(translations: &Translations, requested: &str, default_locale: &str) -> String {
    localized_value(translations, requested, default_locale, |t| t.name.clone())
        .unwrap_or_else(|| missing_label().to_string())
}

pub fn tr_description(
    translations: &Translations,
    requested: &str,
    default_locale: &str,
) -> String {
    localized_value(translations, requested, default_locale, |t| {
        t.description.clone().unwrap_or_default()
    })
    .unwrap_or_default()
}
// ============================================================================
// Server queries
// ============================================================================
#[cfg(feature = "server")]
fn slugify(value: &str) -> String {
    let slug = value
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();
    slug.trim_matches('-').to_string()
}

#[cfg(feature = "server")]
fn unique_id_from_name(name: &str, fallback: &str) -> String {
    let slug = slugify(name);
    let base = if slug.is_empty() { fallback.to_string() } else { slug };
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{}-{}", base, timestamp)
}

#[cfg(feature = "server")]
async fn generate_unique_category_id(pool: &PgPool, name: &str) -> Result<String, sqlx::Error> {
    loop {
        let candidate = unique_id_from_name(name, "category");
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)")
            .bind(&candidate)
            .fetch_one(pool)
            .await?;
        if !exists {
            return Ok(candidate);
        }
    }
}

#[cfg(feature = "server")]
async fn generate_unique_product_type_id(
    pool: &PgPool,
    name: &str,
) -> Result<String, sqlx::Error> {
    loop {
        let candidate = unique_id_from_name(name, "product-type");
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM product_types WHERE id = $1)")
                .bind(&candidate)
                .fetch_one(pool)
                .await?;
        if !exists {
            return Ok(candidate);
        }
    }
}

#[cfg(feature = "server")]
async fn generate_unique_product_id(pool: &PgPool, name: &str) -> Result<String, sqlx::Error> {
    let base_slug = slugify(name);
    let base = if base_slug.is_empty() { "product".to_string() } else { base_slug };
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut counter: u32 = 0;
    loop {
        let candidate = if counter == 0 {
            format!("{}-{}", base, timestamp)
        } else {
            format!("{}-{}-{}", base, timestamp, counter)
        };
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM products WHERE id = $1)")
            .bind(&candidate)
            .fetch_one(pool)
            .await?;
        if !exists {
            return Ok(candidate);
        }
        counter = counter.saturating_add(1);
    }
}

#[cfg(feature = "server")]
async fn load_locale_settings(pool: &PgPool) -> Result<(String, Vec<Locale>), sqlx::Error> {
    let rows = sqlx::query(
        "SELECT code, is_default, is_enabled FROM locales ORDER BY is_default DESC, code",
    )
    .fetch_all(pool)
    .await?;
    let mut default_locale = "en".to_string();
    let mut enabled = Vec::new();
    for row in rows {
        let code: String = row.get("code");
        let is_default: bool = row.get("is_default");
        let is_enabled: bool = row.get("is_enabled");
        if is_default {
            default_locale = code.clone();
        }
        if is_enabled {
            enabled.push(code);
        }
    }
    Ok((default_locale, enabled))
}

#[cfg(feature = "server")]
async fn fetch_category_translations(
    pool: &PgPool,
    category_ids: &[String],
) -> Result<HashMap<String, Translations>, sqlx::Error> {
    let mut out: HashMap<String, Translations> = HashMap::new();
    if category_ids.is_empty() {
        return Ok(out);
    }
    let rows = sqlx::query(
        "SELECT category_id, locale, name, description FROM category_translations WHERE category_id = ANY($1)",
    )
    .bind(category_ids)
    .fetch_all(pool)
    .await?;
    for row in rows {
        let id: String = row.get("category_id");
        let locale: String = row.get("locale");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        out.entry(id)
            .or_default()
            .insert(locale, LocalizedText { name, description });
    }
    Ok(out)
}

#[cfg(feature = "server")]
async fn fetch_criterion_translations(
    pool: &PgPool,
    criterion_ids: &[String],
) -> Result<HashMap<String, Translations>, sqlx::Error> {
    let mut out: HashMap<String, Translations> = HashMap::new();
    if criterion_ids.is_empty() {
        return Ok(out);
    }
    let rows = sqlx::query(
        "SELECT criterion_id, locale, name, description FROM criterion_translations WHERE criterion_id = ANY($1)",
    )
    .bind(criterion_ids)
    .fetch_all(pool)
    .await?;
    for row in rows {
        let id: String = row.get("criterion_id");
        let locale: String = row.get("locale");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        out.entry(id)
            .or_default()
            .insert(locale, LocalizedText { name, description });
    }
    Ok(out)
}

#[cfg(feature = "server")]
async fn fetch_product_type_translations(
    pool: &PgPool,
    product_type_ids: &[String],
) -> Result<HashMap<String, Translations>, sqlx::Error> {
    let mut out: HashMap<String, Translations> = HashMap::new();
    if product_type_ids.is_empty() {
        return Ok(out);
    }
    let rows = sqlx::query(
        "SELECT product_type_id, locale, name, description FROM product_type_translations WHERE product_type_id = ANY($1)",
    )
    .bind(product_type_ids)
    .fetch_all(pool)
    .await?;
    for row in rows {
        let id: String = row.get("product_type_id");
        let locale: String = row.get("locale");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        out.entry(id)
            .or_default()
            .insert(locale, LocalizedText { name, description });
    }
    Ok(out)
}

#[cfg(feature = "server")]
async fn fetch_product_translations(
    pool: &PgPool,
    product_ids: &[String],
) -> Result<HashMap<String, Translations>, sqlx::Error> {
    let mut out: HashMap<String, Translations> = HashMap::new();
    if product_ids.is_empty() {
        return Ok(out);
    }
    let rows = sqlx::query(
        "SELECT product_id, locale, name, description FROM product_translations WHERE product_id = ANY($1)",
    )
    .bind(product_ids)
    .fetch_all(pool)
    .await?;
    for row in rows {
        let id: String = row.get("product_id");
        let locale: String = row.get("locale");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        out.entry(id)
            .or_default()
            .insert(locale, LocalizedText { name, description });
    }
    Ok(out)
}

#[cfg(feature = "server")]
async fn fetch_weight_profile_translations(
    pool: &PgPool,
    weight_profile_ids: &[String],
) -> Result<HashMap<String, Translations>, sqlx::Error> {
    let mut out: HashMap<String, Translations> = HashMap::new();
    if weight_profile_ids.is_empty() {
        return Ok(out);
    }
    let rows = sqlx::query(
        "SELECT weight_profile_id, locale, name FROM weight_profile_translations WHERE weight_profile_id = ANY($1)",
    )
    .bind(weight_profile_ids)
    .fetch_all(pool)
    .await?;
    for row in rows {
        let id: String = row.get("weight_profile_id");
        let locale: String = row.get("locale");
        let name: String = row.get("name");
        out.entry(id).or_default().insert(
            locale,
            LocalizedText { name, description: None },
        );
    }
    Ok(out)
}

// ============================================================================
// Data fetch helpers (server-only)
// ============================================================================

#[cfg(feature = "server")]
pub async fn fetch_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, emoji FROM categories ORDER BY id")
        .fetch_all(pool)
        .await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let emoji: String = row.get("emoji");
        out.push(Category {
            id,
            emoji,
            translations: Translations::new(),
            criteria: Vec::new(),
        });
    }
    let ids: Vec<String> = out.iter().map(|c| c.id.clone()).collect();
    let translations = fetch_category_translations(pool, &ids).await?;
    for cat in &mut out {
        if let Some(map) = translations.get(&cat.id) {
            cat.translations = map.clone();
        }
    }
    for cat in &mut out {
        cat.criteria = fetch_criteria_for_category(pool, &cat.id).await?;
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_criteria_for_category(
    pool: &PgPool,
    category_id: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT c.id, c.emoji FROM criteria c JOIN category_criteria cc ON c.id = cc.criterion_id WHERE cc.category_id = $1 ORDER BY c.id",
    )
    .bind(category_id)
    .fetch_all(pool)
    .await?;
    let ids: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("id"))
        .collect();
    let translations = fetch_criterion_translations(pool, &ids).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let emoji: String = row.get("emoji");
        let mut translations_map = Translations::new();
        if let Some(map) = translations.get(&id) {
            translations_map = map.clone();
        }
        out.push(Criterion {
            id,
            emoji,
            translations: translations_map,
        });
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_specific_criteria_for_product_type(
    pool: &PgPool,
    product_type_id: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT c.id, c.emoji FROM criteria c JOIN product_type_specific_criteria pts ON c.id = pts.criterion_id WHERE pts.product_type_id = $1 ORDER BY c.id",
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await?;
    let ids: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("id"))
        .collect();
    let translations = fetch_criterion_translations(pool, &ids).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let emoji: String = row.get("emoji");
        let mut translations_map = Translations::new();
        if let Some(map) = translations.get(&id) {
            translations_map = map.clone();
        }
        out.push(Criterion {
            id,
            emoji,
            translations: translations_map,
        });
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_all_product_types(pool: &PgPool) -> Result<Vec<ProductType>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, emoji FROM product_types ORDER BY id")
        .fetch_all(pool)
        .await?;
    let ids: Vec<String> = rows.iter().map(|r| r.get::<String, _>("id")).collect();
    let translations = fetch_product_type_translations(pool, &ids).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let emoji: String = row.get("emoji");
        let translations_map = translations.get(&id).cloned().unwrap_or_default();
        out.push(ProductType {
            id,
            emoji,
            translations: translations_map,
            category_ids: Vec::new(),
            specific_criteria: Vec::new(),
            presets: Vec::new(),
        });
    }
    for pt in &mut out {
        let rows = sqlx::query(
            "SELECT category_id FROM product_type_categories WHERE product_type_id = $1",
        )
        .bind(&pt.id)
        .fetch_all(pool)
        .await?;
        pt.category_ids = rows
            .iter()
            .map(|r| r.get::<String, _>("category_id"))
            .collect();
        pt.specific_criteria = fetch_specific_criteria_for_product_type(pool, &pt.id).await?;
        pt.presets = fetch_weight_profiles_for_product_type(pool, &pt.id).await?;
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_weight_profiles_for_product_type(
    pool: &PgPool,
    product_type_id: &str,
) -> Result<Vec<WeightProfile>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, weights FROM weight_profiles WHERE product_type_id = $1 ORDER BY id",
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await?;
    let ids: Vec<String> = rows.iter().map(|r| r.get::<String, _>("id")).collect();
    let translations = fetch_weight_profile_translations(pool, &ids).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let weights: Json<HashMap<String, f64>> = row.get("weights");
        let translations_map = translations.get(&id).cloned().unwrap_or_default();
        out.push(WeightProfile {
            translations: translations_map,
            weights: weights.0,
        });
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, price, quantity, unit, product_type_id FROM products ORDER BY id",
    )
    .fetch_all(pool)
    .await?;
    let ids: Vec<String> = rows.iter().map(|r| r.get::<String, _>("id")).collect();
    let translations = fetch_product_translations(pool, &ids).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: String = row.get("id");
        let price: f64 = row.get("price");
        let quantity: Option<f64> = row.get("quantity");
        let unit: Option<String> = row.get("unit");
        let product_type_id: String = row.get("product_type_id");
        let translations_map = translations.get(&id).cloned().unwrap_or_default();
        let scores = fetch_scores_for_product(pool, &id).await?;
        out.push(Product {
            id,
            price,
            quantity,
            unit,
            product_type_id,
            translations: translations_map,
            scores,
        });
    }
    Ok(out)
}

#[cfg(feature = "server")]
pub async fn fetch_scores_for_product(
    pool: &PgPool,
    product_id: &str,
) -> Result<HashMap<String, f64>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT criterion_id, score FROM product_scores WHERE product_id = $1",
    )
    .bind(product_id)
    .fetch_all(pool)
    .await?;
    let mut out = HashMap::new();
    for row in rows {
        let criterion_id: String = row.get("criterion_id");
        let score: f32 = row.get("score");
        out.insert(criterion_id, f64::from(score));
    }
    Ok(out)
}

// ============================================================================
// Public API endpoints (Dioxus server functions)
// ============================================================================

#[get("/api/app-data/{language}")]
pub async fn load_app_data(language: String) -> Result<AppData, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let (default_locale, enabled_locales) = load_locale_settings(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    // The `language` query parameter is accepted for backwards compatibility
    // but the backend now returns raw translation maps; the frontend does the
    // fallback resolution. We still normalize an empty value to the default.
    let _ = if language.is_empty() { default_locale.clone() } else { language };

    let categories = fetch_all_categories(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let product_types = fetch_all_product_types(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let products = fetch_all_products(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(AppData {
        categories,
        product_types,
        products,
        default_locale,
        enabled_locales,
    })
}

#[get("/api/supported-languages")]
pub async fn get_supported_languages() -> Result<Vec<SupportedLanguage>, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let rows = sqlx::query(
        "SELECT code, english_name, native_name, is_default, is_enabled FROM locales ORDER BY is_default DESC, code",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;
    Ok(rows
        .into_iter()
        .map(|row| SupportedLanguage {
            code: row.get("code"),
            english_name: row.get("english_name"),
            native_name: row.get("native_name"),
            is_default: row.get("is_default"),
            is_enabled: row.get("is_enabled"),
        })
        .collect())
}

// ----------------------------------------------------------------------------
// Validation helpers
// ----------------------------------------------------------------------------

#[cfg(feature = "server")]
async fn enabled_locale_set(
    pool: &PgPool,
) -> Result<std::collections::HashSet<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT code, is_enabled FROM locales")
        .fetch_all(pool)
        .await?;
    let mut set = std::collections::HashSet::new();
    for row in rows {
        let code: String = row.get("code");
        let is_enabled: bool = row.get("is_enabled");
        if is_enabled {
            set.insert(code);
        }
    }
    Ok(set)
}

#[cfg(feature = "server")]
async fn active_locale_for_request(pool: &PgPool) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT code FROM locales WHERE is_default = TRUE LIMIT 1")
        .fetch_optional(pool)
        .await?;
    Ok(match row {
        Some(r) => r.get("code"),
        None => "en".to_string(),
    })
}

#[cfg(feature = "server")]
fn is_valid_bcp47(tag: &str) -> bool {
    // Lightweight BCP 47 validation: language, optional script, optional region.
    // Accepts "en", "fr", "fr-CA", "zh-Hans-CN", etc.
    if tag.is_empty() || tag.len() > 35 {
        return false;
    }
    let parts: Vec<&str> = tag.split('-').collect();
    if parts.is_empty() {
        return false;
    }
    // Primary language subtag: 2-3 ASCII letters
    let primary = parts[0];
    if !(2..=3).contains(&primary.len()) || !primary.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    for part in &parts[1..] {
        if part.is_empty() {
            return false;
        }
        let len = part.len();
        // Script: 4 letters; Region: 2 letters or 3 digits
        if len == 4 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        if (len == 2 || len == 3) && part.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        if len == 2 && part.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        return false;
    }
    true
}

#[cfg(feature = "server")]
fn clean_translation(
    entry: &TranslationEntry,
    enabled: &std::collections::HashSet<String>,
    default_locale: &str,
) -> Result<Option<(String, String, Option<String>)>, String> {
    let locale = entry.locale.trim().to_string();
    if locale.is_empty() {
        return Err("Empty locale in translation entry".to_string());
    }
    if !is_valid_bcp47(&locale) {
        return Err(format!("Invalid BCP 47 locale: {locale}"));
    }
    if !enabled.contains(&locale) {
        return Err(format!("Locale '{locale}' is not enabled"));
    }
    let name = entry.name.trim().to_string();
    if name.is_empty() {
        // Empty optional translations are silently dropped, not stored.
        return Ok(None);
    }
    let description = entry
        .description
        .as_ref()
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty());
    // The default-locale name must always be present; if the client sent only
    // an optional locale, that is acceptable, but the default must be filled
    // elsewhere. We do NOT manufacture a name here.
    if locale == default_locale {
        // Make sure description is at least NULL when missing.
    }
    Ok(Some((locale, name, description)))
}
// ============================================================================
// Create endpoints (multilingual-aware)
// ============================================================================

#[cfg(feature = "server")]
async fn insert_category_translations(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    category_id: &str,
    entries: &[(String, String, Option<String>)],
) -> Result<(), sqlx::Error> {
    for (locale, name, description) in entries {
        sqlx::query(
            "INSERT INTO category_translations (category_id, locale, name, description)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (category_id, locale) DO UPDATE
             SET name = EXCLUDED.name, description = EXCLUDED.description",
        )
        .bind(category_id)
        .bind(locale)
        .bind(name)
        .bind(description.as_deref())
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

#[cfg(feature = "server")]
async fn insert_product_type_translations(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    product_type_id: &str,
    entries: &[(String, String, Option<String>)],
) -> Result<(), sqlx::Error> {
    for (locale, name, description) in entries {
        sqlx::query(
            "INSERT INTO product_type_translations (product_type_id, locale, name, description)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (product_type_id, locale) DO UPDATE
             SET name = EXCLUDED.name, description = EXCLUDED.description",
        )
        .bind(product_type_id)
        .bind(locale)
        .bind(name)
        .bind(description.as_deref())
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

#[cfg(feature = "server")]
async fn insert_product_translations(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    product_id: &str,
    entries: &[(String, String, Option<String>)],
) -> Result<(), sqlx::Error> {
    for (locale, name, description) in entries {
        sqlx::query(
            "INSERT INTO product_translations (product_id, locale, name, description)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (product_id, locale) DO UPDATE
             SET name = EXCLUDED.name, description = EXCLUDED.description",
        )
        .bind(product_id)
        .bind(locale)
        .bind(name)
        .bind(description.as_deref())
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

#[cfg(feature = "server")]
fn process_translations(
    entries: &[TranslationEntry],
    enabled: &std::collections::HashSet<String>,
    default_locale: &str,
) -> Result<Vec<(String, String, Option<String>)>, String> {
    let mut out = Vec::new();
    let mut seen_locales: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut has_default = false;
    for entry in entries {
        if !seen_locales.insert(entry.locale.clone()) {
            return Err(format!(
                "Duplicate locale '{}' in translations",
                entry.locale
            ));
        }
        if let Some(cleaned) = clean_translation(entry, enabled, default_locale)? {
            if cleaned.0 == default_locale {
                has_default = true;
            }
            out.push(cleaned);
        }
    }
    if !has_default {
        return Err(format!(
            "Translations must include the default locale '{default_locale}' with a non-empty name"
        ));
    }
    Ok(out)
}

#[post("/api/categories")]
pub async fn create_category(input: NewCategoryInput) -> Result<Category, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let default_locale = active_locale_for_request(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let enabled = enabled_locale_set(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let cleaned = process_translations(&input.translations, &enabled, &default_locale)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    // The slug is derived from the current-locale (default) name only. We do
    // NOT mix other locales' content into the id.
    let current_name = cleaned
        .iter()
        .find(|(loc, _, _)| loc == &default_locale)
        .map(|(_, n, _)| n.clone())
        .ok_or_else(|| ServerFnError::new("Missing default-locale name".to_string()))?;
    let category_id = generate_unique_category_id(pool, &current_name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    sqlx::query("INSERT INTO categories (id, emoji) VALUES ($1, $2)")
        .bind(&category_id)
        .bind(&input.emoji)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    insert_category_translations(&mut tx, &category_id, &cleaned)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut translations = Translations::new();
    for (locale, name, description) in cleaned {
        translations.insert(locale, LocalizedText { name, description });
    }
    Ok(Category {
        id: category_id,
        emoji: input.emoji,
        translations,
        criteria: Vec::new(),
    })
}

#[post("/api/product-types")]
pub async fn create_product_type(input: NewProductTypeInput) -> Result<ProductType, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let default_locale = active_locale_for_request(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let enabled = enabled_locale_set(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let cleaned = process_translations(&input.translations, &enabled, &default_locale)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let current_name = cleaned
        .iter()
        .find(|(loc, _, _)| loc == &default_locale)
        .map(|(_, n, _)| n.clone())
        .ok_or_else(|| ServerFnError::new("Missing default-locale name".to_string()))?;
    let product_type_id = generate_unique_product_type_id(pool, &current_name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    sqlx::query("INSERT INTO product_types (id, emoji) VALUES ($1, $2)")
        .bind(&product_type_id)
        .bind(&input.emoji)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    for category_id in &input.category_ids {
        sqlx::query(
            "INSERT INTO product_type_categories (product_type_id, category_id)
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(&product_type_id)
        .bind(category_id)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    }
    insert_product_type_translations(&mut tx, &product_type_id, &cleaned)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut translations = Translations::new();
    for (locale, name, description) in cleaned {
        translations.insert(locale, LocalizedText { name, description });
    }
    Ok(ProductType {
        id: product_type_id,
        emoji: input.emoji,
        translations,
        category_ids: input.category_ids,
        specific_criteria: Vec::new(),
        presets: Vec::new(),
    })
}

#[post("/api/products")]
pub async fn create_product(input: NewProductInput) -> Result<Product, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let default_locale = active_locale_for_request(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let enabled = enabled_locale_set(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let cleaned = process_translations(&input.translations, &enabled, &default_locale)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let current_name = cleaned
        .iter()
        .find(|(loc, _, _)| loc == &default_locale)
        .map(|(_, n, _)| n.clone())
        .ok_or_else(|| ServerFnError::new("Missing default-locale name".to_string()))?;
    let product_id = generate_unique_product_id(pool, &current_name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    sqlx::query(
        "INSERT INTO products (id, price, quantity, unit, product_type_id)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&product_id)
    .bind(input.price)
    .bind(input.quantity)
    .bind(input.unit.as_deref())
    .bind(&input.product_type_id)
    .execute(&mut *tx)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;
    for (criterion_id, score) in &input.scores {
        sqlx::query(
            "INSERT INTO product_scores (product_id, criterion_id, score)
             VALUES ($1, $2, $3)",
        )
        .bind(&product_id)
        .bind(criterion_id)
        .bind(*score as f32)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    }
    insert_product_translations(&mut tx, &product_id, &cleaned)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let mut translations = Translations::new();
    for (locale, name, description) in cleaned {
        translations.insert(locale, LocalizedText { name, description });
    }
    Ok(Product {
        id: product_id,
        price: input.price,
        quantity: input.quantity,
        unit: input.unit,
        product_type_id: input.product_type_id,
        translations,
        scores: input.scores,
    })
}

// ============================================================================
// Client-side helpers (no server-only deps)
// ============================================================================

pub fn calculate_score(product: &Product, weights: &HashMap<String, f64>) -> f64 {
    let mut total_score = 0.0;
    let mut total_weight = 0.0;
    for (criterion_id, score) in &product.scores {
        let weight = weights.get(criterion_id).copied().unwrap_or(5.0);
        total_score += score * weight;
        total_weight += weight;
    }
    if total_weight > 0.0 {
        total_score / total_weight
    } else {
        0.0
    }
}

pub fn get_combined_criteria(
    product_type: &ProductType,
    categories: &[Category],
) -> Vec<Criterion> {
    let mut list = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for criterion in &product_type.specific_criteria {
        if seen.insert(criterion.id.clone()) {
            list.push(criterion.clone());
        }
    }
    for category_id in &product_type.category_ids {
        if let Some(category) = categories.iter().find(|c| c.id == *category_id) {
            for criterion in &category.criteria {
                if seen.insert(criterion.id.clone()) {
                    list.push(criterion.clone());
                }
            }
        }
    }
    list.sort_by(|a, b| a.id.cmp(&b.id));
    list
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn map(entries: &[(&str, &str, Option<&str>)]) -> Translations {
        let mut t = Translations::new();
        for (loc, name, desc) in entries {
            t.insert(
                loc.to_string(),
                LocalizedText {
                    name: name.to_string(),
                    description: desc.map(|s| s.to_string()),
                },
            );
        }
        t
    }

    #[test]
    fn fallback_exact_locale_wins() {
        let t = map(&[
            ("en", "English", None),
            ("fr", "Français", Some("desc FR")),
        ]);
        let v = localized_value(&t, "fr", "en", |x| x.name.clone());
        assert_eq!(v.as_deref(), Some("Français"));
    }

    #[test]
    fn fallback_to_base_when_region_missing() {
        let t = map(&[("en", "English", None), ("fr", "Français", None)]);
        let v = localized_value(&t, "fr-CA", "en", |x| x.name.clone());
        assert_eq!(v.as_deref(), Some("Français"));
    }

    #[test]
    fn fallback_to_default_locale() {
        let t = map(&[("en", "English", None)]);
        let v = localized_value(&t, "es", "en", |x| x.name.clone());
        assert_eq!(v.as_deref(), Some("English"));
    }

    #[test]
    fn fallback_to_any_available() {
        let t = map(&[("de", "Deutsch", None)]);
        let v = localized_value(&t, "es", "en", |x| x.name.clone());
        assert_eq!(v.as_deref(), Some("Deutsch"));
    }

    #[test]
    fn fallback_returns_none_when_empty() {
        let t: Translations = HashMap::new();
        let v = localized_value(&t, "en", "en", |x| x.name.clone());
        assert_eq!(v, None);
    }

    #[test]
    fn bcp47_validates_tags() {
        assert!(is_valid_bcp47("en"));
        assert!(is_valid_bcp47("fr"));
        assert!(is_valid_bcp47("fr-CA"));
        assert!(is_valid_bcp47("zh-Hans-CN"));
        assert!(!is_valid_bcp47(""));
        assert!(!is_valid_bcp47("e"));
        assert!(!is_valid_bcp47("en_US"));
        assert!(!is_valid_bcp47("en-"));
    }
}
