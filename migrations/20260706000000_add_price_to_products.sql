-- Alter products table to add price, quantity, and unit columns
ALTER TABLE products ADD COLUMN price DOUBLE PRECISION NOT NULL DEFAULT 0.0;
ALTER TABLE products ADD COLUMN quantity DOUBLE PRECISION;
ALTER TABLE products ADD COLUMN unit TEXT;

-- Update existing seeded products with price and optional quantity/unit
UPDATE products SET price = 999.00 WHERE id = 'iphone-15-pro';
UPDATE products SET price = 1299.00 WHERE id = 'galaxy-s24-ultra';
UPDATE products SET price = 699.00 WHERE id = 'fairphone-5';
UPDATE products SET price = 499.00 WHERE id = 'pixel-8a';

UPDATE products SET price = 24.50, quantity = 0.25, unit = 'kg' WHERE id = 'ethiopian-yirgacheffe';
UPDATE products SET price = 18.00, quantity = 0.25, unit = 'kg' WHERE id = 'colombian-supremo';
UPDATE products SET price = 22.00, quantity = 0.25, unit = 'kg' WHERE id = 'sumatran-mandheling';
UPDATE products SET price = 8.50, quantity = 0.50, unit = 'kg' WHERE id = 'mass-market-roast';

UPDATE products SET price = 15.00, quantity = 1.00, unit = 'kg' WHERE id = 'jasmine-rice';
UPDATE products SET price = 18.50, quantity = 1.00, unit = 'kg' WHERE id = 'basmati-rice';
UPDATE products SET price = 5.00, quantity = 2.00, unit = 'kg' WHERE id = 'cheap-white-rice';

UPDATE products SET price = 1599.00 WHERE id = 'herman-miller-aeron';
UPDATE products SET price = 1399.00 WHERE id = 'steelcase-gesture';
UPDATE products SET price = 89.00 WHERE id = 'budget-mesh-chair';
