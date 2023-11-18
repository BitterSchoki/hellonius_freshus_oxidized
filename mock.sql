--RECEIPS
insert into recipes values (1, 'Mushroom-Risotto', 'A creamy italian dish', 2);
insert into recipes values (2, 'Noodles with salmon cream sauce', 'A healthy noode receipe', 2);

--INGREDIENTS
--Ingridients Mushroom Risotto
insert into ingredients values (1, 'Mushrooms', 'Stone');
insert into ingredients values (2, 'Mushrooms', 'Normal');
insert into ingredients values (3, 'Water', 'Tap');
insert into ingredients values (4, 'Onions', '');
insert into ingredients values (5, 'Garlic', '');
insert into ingredients values (6, 'Rice', 'For risotto');
insert into ingredients values (7, 'Rice', 'Basmati');
insert into ingredients values (8, 'Frozen peas', '');
insert into ingredients values (9, 'Salt', '');
insert into ingredients values (10, 'Pepper', '');
insert into ingredients values (11, 'Parmesan', '');
insert into ingredients values (12, 'Butter', '');
insert into ingredients values (13, 'Plant Oil', '');
insert into ingredients values (14, 'Basil', '');

--Ingredients mushroom risotto vegan
insert into ingredients values (15, 'Vegan white wine', '');
insert into ingredients values (16, 'Vegan butter', '');
insert into ingredients values (17, 'Vegan Parmesan', '');

-- More ingredients
insert into ingredients values (18, 'Milk', '');
insert into ingredients values (19, 'Egg', '');
insert into ingredients values (20, 'Shrimps', '');
insert into ingredients values (21, 'Soy', '');
insert into ingredients values (22, 'Mehl', '');

-- AMOUNT AND ADD INGREDIENTS TO RECIPE
-- Unique, receipe id, ingridient id, amount, unit  
--Mushroom Risotto
insert into recipe_ingredients values (1, 1, 3, 500, 'ml');
insert into recipe_ingredients values (2, 2, 1, 200, 'g');
insert into recipe_ingredients values (3, 2, 2, 1.5, 'pcs');

-- FOOD GROUPS id, id ingredient, food group id (1, 'Lactose',
-- 2, 'Gluten' 3, 'Histamines', 4, 'Seafood')
insert into ingredient_food_groups values(1, 4, 3);
insert into ingredient_food_groups values(2, 5, 3);
insert into ingredient_food_groups values(3, 6, 3);
insert into ingredient_food_groups values(4, 8, 3);
insert into ingredient_food_groups values(5, 11, 3);
insert into ingredient_food_groups values(6, 11, 1);
insert into ingredient_food_groups values(7, 12, 1);
insert into ingredient_food_groups values(8, 15, 3);
insert into ingredient_food_groups values(11, 18, 1);
insert into ingredient_food_groups values(12, 19, 1);
insert into ingredient_food_groups values(13, 20, 4);
insert into ingredient_food_groups values(13, 21, 3);
insert into ingredient_food_groups values(14, 22, 1);


