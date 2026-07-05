-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    emoji VARCHAR(50) NOT NULL
);

-- Create criteria table
CREATE TABLE IF NOT EXISTS criteria (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    emoji VARCHAR(50) NOT NULL
);

-- Create category_criteria junction table
CREATE TABLE IF NOT EXISTS category_criteria (
    category_id VARCHAR(255) NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    criterion_id VARCHAR(255) NOT NULL REFERENCES criteria(id) ON DELETE CASCADE,
    PRIMARY KEY (category_id, criterion_id)
);

-- Create product_types table
CREATE TABLE IF NOT EXISTS product_types (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    emoji VARCHAR(50) NOT NULL
);

-- Create product_type_categories junction table
CREATE TABLE IF NOT EXISTS product_type_categories (
    product_type_id VARCHAR(255) NOT NULL REFERENCES product_types(id) ON DELETE CASCADE,
    category_id VARCHAR(255) NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (product_type_id, category_id)
);

-- Create product_type_criteria junction table
CREATE TABLE IF NOT EXISTS product_type_criteria (
    product_type_id VARCHAR(255) NOT NULL REFERENCES product_types(id) ON DELETE CASCADE,
    criterion_id VARCHAR(255) NOT NULL REFERENCES criteria(id) ON DELETE CASCADE,
    PRIMARY KEY (product_type_id, criterion_id)
);

-- Create products table
CREATE TABLE IF NOT EXISTS products (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    product_type_id VARCHAR(255) NOT NULL REFERENCES product_types(id) ON DELETE CASCADE,
    image_url TEXT
);

-- Create product_criteria_scores table
CREATE TABLE IF NOT EXISTS product_criteria_scores (
    product_id VARCHAR(255) NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    criterion_id VARCHAR(255) NOT NULL REFERENCES criteria(id) ON DELETE CASCADE,
    score DOUBLE PRECISION NOT NULL CHECK (score >= 0.0 AND score <= 10.0),
    PRIMARY KEY (product_id, criterion_id)
);

-- Create weight_profiles table
CREATE TABLE IF NOT EXISTS weight_profiles (
    id VARCHAR(255) PRIMARY KEY,
    product_type_id VARCHAR(255) NOT NULL REFERENCES product_types(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    weights JSONB NOT NULL,
    is_system BOOLEAN NOT NULL DEFAULT FALSE
);
