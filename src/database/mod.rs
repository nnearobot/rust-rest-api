use postgres::{ Client, NoTls };
use std::env;

const DB_HOST: &str = env!("DB_HOST");
const DB_PORT: &str = env!("DB_PORT");
const DB_USER: &str = env!("DB_USER");
const DB_PASS: &str = env!("DB_PASS");
const DB_NAME: &str = env!("DB_NAME");

pub mod models;
pub mod model;


pub fn db_url() -> String {
    format!("postgres://{}:{}@{}:{}/{}", DB_USER, DB_PASS, DB_HOST, DB_PORT, DB_NAME)
}

/// Establishes a connection with a database
pub fn client() -> Result<Client, String> {
    match Client::connect(db_url().as_str(), NoTls) {
        Ok(client) => Ok(client),
        Err(error) => Err(format!("{}", error)),
    }
}

/// Warms up a database
pub fn set_database() -> Result<(), String> {
    let mut client = client()?;
    client.batch_execute( "
    CREATE TABLE IF NOT EXISTS \"table\" (
        table_id SERIAL PRIMARY KEY,
        table_description TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS menu (
        menu_id SERIAL PRIMARY KEY,
        menu_name VARCHAR(255) NOT NULL,
        menu_description TEXT NOT NULL,
        time_to_cook_in_minutes INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS \"order\" (
        order_id SERIAL PRIMARY KEY,
        table_id INTEGER NOT NULL REFERENCES \"table\"(table_id),
        menu_id INTEGER NOT NULL REFERENCES menu(menu_id),
        cooked_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
        is_deleted BOOLEAN,
        created_at TIMESTAMP WITHOUT TIME ZONE,
        updated_at TIMESTAMP WITHOUT TIME ZONE
    );
    CREATE INDEX IF NOT EXISTS idx_order_table_id ON \"order\"(table_id);
    CREATE INDEX IF NOT EXISTS idx_order_menu_id ON \"order\"(menu_id);

    INSERT INTO \"table\" (table_id, table_description)
    VALUES
        (1, 'Table 1'),
        (2, 'Table 2'),
        (3, 'Table 3'),
        (4, 'Table 4'),
        (5, 'Table 5'),
        (6, 'Table 6'),
        (7, 'Table 7'),
        (8, 'Table 8'),
        (9, 'Table 9'),
        (10, 'Table 10'),
        (21, 'Table for tests')
    ON CONFLICT (table_id) DO NOTHING;

    INSERT INTO menu (menu_id, menu_name, menu_description, time_to_cook_in_minutes)
    VALUES
        (1, 'Classic Margherita Pizza', 'Description for Classic Margherita Pizza', 1),
        (2, 'Spaghetti Carbonara', 'Description for Spaghetti Carbonara', 5),
        (3, 'Vegetarian Lasagna', 'Description for Vegetarian Lasagna', 3),
        (4, 'Thai Green Curry', 'Description for Thai Green Curry', 4),
        (5, 'Chicken Caesar Salad', 'Description for Chicken Caesar Salad', 2),
        (6, 'Beef Stroganoff', 'Description for Beef Stroganoff', 9),
        (7, 'Grilled Salmon with Dill Sauce', 'Description for Grilled Salmon with Dill Sauce', 3),
        (8, 'Moroccan Chickpea Stew', 'Description for Moroccan Chickpea Stew', 2),
        (9, 'Szechuan Tofu Stir-Fry', 'Description for Szechuan Tofu Stir-Fry', 1),
        (10, 'Lamb Rogan Josh', 'Description for Lamb Rogan Josh', 1),
        (11, 'French Onion Soup', 'Description for French Onion Soup', 2),
        (12, 'Korean Bibimbap', 'Description for Korean Bibimbap', 5),
        (13, 'Mexican Chicken Fajitas', 'Description for Mexican Chicken Fajitas', 2),
        (14, 'Italian Risotto', 'Description for Italian Risotto', 1),
        (15, 'Greek Moussaka', 'Description for Greek Moussaka', 4),
        (16, 'Indian Butter Chicken', 'Description for Indian Butter Chicken', 6),
        (17, 'Spanish Paella', 'Description for Spanish Paella', 2),
        (18, 'American Cheeseburger', 'Description for American Cheeseburger', 1),
        (19, 'Japanese Sushi Rolls', 'Description for Japanese Sushi Rolls', 4),
        (20, 'English Fish and Chips', 'Description for English Fish and Chips', 5)
    ON CONFLICT (menu_id) DO NOTHING;
    ")
    .map_err(|error| error.to_string())
}