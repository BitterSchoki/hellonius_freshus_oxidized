-- enable foreign keys
PRAGMA foreign_keys = ON;

-- create recipes table
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    serves INTEGER NOT NULL
);

-- create ingredients table
CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    replacement_flag VARCHAR(1) NOT NULL
    
);

-- create recipe_ingredients table
CREATE TABLE recipe_ingredients (
    id INTEGER PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    amount FLOAT NOT NULL,
    unit TEXT NOT NULL,
    UNIQUE (recipe_id, ingredient_id),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id)
);
