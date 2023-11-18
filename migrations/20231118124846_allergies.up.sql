CREATE TABLE food_groups (
    id INTEGER PRIMARY KEY,
    designation TEXT NOT NULL
);

INSERT INTO food_groups (id, designation) VALUES
    (1, 'Lactose'),
    (2, 'Gluten'),
    (3, 'Histamines'),
    (4, 'Seafood');

CREATE TABLE diet_goals (
    id INTEGER PRIMARY KEY,
    designation TEXT NOT NULL
);

INSERT INTO diet_goals (id, designation) VALUES
    (1, 'Low Carb'),
    (2, 'High Carb'),
    (3, 'High Fat'),
    (4, 'Clean Eating');

CREATE TABLE special_diets (
    id INTEGER PRIMARY KEY,
    designation TEXT NOT NULL
);

INSERT INTO special_diets (id, designation) VALUES
    (1, 'Vegan'),
    (2, 'Vegetarian'),
    (3, 'Pescetarian'),
    (4, 'Keto');
    
-- create relationship tables to ingredients

CREATE TABLE ingredient_food_groups (
    id INTEGER PRIMARY KEY,
    ingredient_id INTEGER NOT NULL,
    food_group_id INTEGER NOT NULL,
    UNIQUE (ingredient_id, food_group_id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id),
    FOREIGN KEY (food_group_id) REFERENCES food_groups (id)
);

CREATE TABLE ingredient_diet_goals (
    id INTEGER PRIMARY KEY,
    ingredient_id INTEGER NOT NULL,
    diet_goal_id INTEGER NOT NULL,
    UNIQUE (ingredient_id, diet_goal_id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id),
    FOREIGN KEY (diet_goal_id) REFERENCES diet_goals (id)
);

CREATE TABLE ingredient_special_diets (
    id INTEGER PRIMARY KEY,
    ingredient_id INTEGER NOT NULL,
    special_diet_id INTEGER NOT NULL,
    UNIQUE (ingredient_id, special_diet_id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id),
    FOREIGN KEY (special_diet_id) REFERENCES special_diets (id)
);
