--RECEIPS
insert into recipes values (1, 'Mushroom-Risotto', 'A creamy italian dish', 2);
insert into recipes values (2, 'Dumplings', 'A Bavarian receipe', 2);
insert into recipes values (3, 'Leberkaese Burger', 'A Bavarian receipe', 4);

--INGREDIENTS
--Ingridients Mushroom Risotto
insert into ingredients values (1, 'Mushrooms', 'Stone','mushrooms');
insert into ingredients values (2, 'Mushrooms', 'Normal', 'mushrooms');
insert into ingredients values (3, 'Water', 'Tap','x');
insert into ingredients values (4, 'Onions', '','onions');
insert into ingredients values (5, 'Garlic', '', 'garlic');
insert into ingredients values (6, 'Rice', 'For risotto', 'rice');
insert into ingredients values (7, 'Rice', 'Basmati','rice');
insert into ingredients values (8, 'Frozen peas', '', 'legume');
insert into ingredients values (9, 'Salt', '','salt');
insert into ingredients values (10, 'Pepper', '', 'pepper');
insert into ingredients values (11, 'Parmesan', '', 'parmesan');
insert into ingredients values (12, 'Butter', 'Animal based', 'butter');
insert into ingredients values (13, 'Plant Oil', '', 'oil');
insert into ingredients values (14, 'Basil', '', 'basil');

--Ingredients mushroom risotto vegan
insert into ingredients values (15, 'Vegan white wine', 'wine');
insert into ingredients values (16, 'Vegan butter', 'butter');
insert into ingredients values (17, 'Vegan Parmesan', 'parmesan');

-- More ingredients
insert into ingredients values (18, 'Milk', '', 'milk');
insert into ingredients values (19, 'Egg', '', 'egg');
insert into ingredients values (20, 'Shellfish', '', 'fish');
insert into ingredients values (21, 'Soy', '', 'meat');
insert into ingredients values (22, 'Flour', '', 'flour');

--Alternative legumes
insert into ingredients values (23, 'Kidney beans', '', 'legumes');
insert into ingredients values (24, 'Chickpeas', '', 'legumes');
insert into ingredients values (25, 'Lentils', '', 'legumes');
insert into ingredients values (26, 'Testparmesan2', '', 'parmesan');

--Disliked ingridients
insert into ingredients values (27, 'Coriander', '', 'coriander');
insert into ingredients values (28, 'Olives', '', 'olives');
insert into ingredients values (29, 'Blue Cheese', '', 'cheese');
insert into ingredients values (30, 'Coffee', '', 'coffee');
insert into ingredients values (31, 'Peanuts', '', 'nuts');
insert into ingredients values (32, 'Tree nuts', '', 'nuts');


--Leberkaese
insert into ingredients values (33, 'Senf', '', 'senf');
insert into ingredients values (34, 'Coleslaw', '', 'salat');
insert into ingredients values (35, 'Radishes', '', 'radishes');
insert into ingredients values (36, 'Leberkaese', '', 'leberkaese');
insert into ingredients values (37, 'Lagenbroetchen', '', 'roll');

--Leberkaese Ersatz 
insert into ingredients values (38, 'Gluten free roll', '', 'roll');
--- + meat replacement, soy

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
insert into ingredient_food_groups values(15, 26, 1);


