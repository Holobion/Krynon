-- Create the criteria table
CREATE TABLE IF NOT EXISTS criteria (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    emoji TEXT
);

-- Create the categories table
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    emoji TEXT
);

-- Junction table for categories and criteria
CREATE TABLE IF NOT EXISTS category_criteria (
    category_id TEXT REFERENCES categories(id) ON DELETE CASCADE,
    criterion_id TEXT REFERENCES criteria(id) ON DELETE CASCADE,
    PRIMARY KEY (category_id, criterion_id)
);

-- Create the product_types table
CREATE TABLE IF NOT EXISTS product_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    emoji TEXT
);

-- Junction table for product_types and categories
CREATE TABLE IF NOT EXISTS product_type_categories (
    product_type_id TEXT REFERENCES product_types(id) ON DELETE CASCADE,
    category_id TEXT REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (product_type_id, category_id)
);

-- Junction table for product_types and specific criteria
CREATE TABLE IF NOT EXISTS product_type_specific_criteria (
    product_type_id TEXT REFERENCES product_types(id) ON DELETE CASCADE,
    criterion_id TEXT REFERENCES criteria(id) ON DELETE CASCADE,
    PRIMARY KEY (product_type_id, criterion_id)
);

-- Create the products table
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    product_type_id TEXT REFERENCES product_types(id) ON DELETE CASCADE
);

-- Table to store product scores for criteria
CREATE TABLE IF NOT EXISTS product_scores (
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    criterion_id TEXT REFERENCES criteria(id) ON DELETE CASCADE,
    score REAL NOT NULL,
    PRIMARY KEY (product_id, criterion_id)
);

-- Note: Weight profiles and presets are more complex and will be handled in a subsequent step or migration if necessary.
-- For now, we focus on the core entities and their relationships.