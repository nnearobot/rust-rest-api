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
        id SERIAL PRIMARY KEY,
        description TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS menu (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        time_to_cook_in_minutes INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS \"order\" (
        id SERIAL PRIMARY KEY,
        table_id INTEGER NOT NULL REFERENCES \"table\"(id),
        menu_id INTEGER NOT NULL REFERENCES menu(id),
        cooked_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
        is_deleted BOOLEAN,
        created_at TIMESTAMP WITHOUT TIME ZONE,
        updated_at TIMESTAMP WITHOUT TIME ZONE
    );
    CREATE INDEX IF NOT EXISTS idx_order_table_id ON \"order\"(table_id);
    CREATE INDEX IF NOT EXISTS idx_order_menu_id ON \"order\"(menu_id);

    INSERT INTO \"table\" (id, description)
    VALUES
        (1, 'Table 1'),
        (2, 'Table 2'),
        (3, 'Table 3'),
        (4, 'Table 4'),
        (5, 'Table 5'),
        (6, 'Table 6'),
        (7, 'Table for tests')
    ON CONFLICT (id) DO NOTHING;

    INSERT INTO menu (id, name, description, time_to_cook_in_minutes)
    VALUES
        (1, 'Classic Margherita Pizza', 'Description for Classic Margherita Pizza', 12),
        (2, 'Spaghetti Carbonara', 'Description for Spaghetti Carbonara', 5),
        (3, 'Vegetarian Lasagna', 'Description for Vegetarian Lasagna', 10),
        (4, 'Thai Green Curry', 'Description for Thai Green Curry', 5),
        (5, 'Chicken Caesar Salad', 'Description for Chicken Caesar Salad', 2),
        (6, 'Beef Stroganoff', 'Description for Beef Stroganoff', 9),
        (7, 'Grilled Salmon with Dill Sauce', 'Description for Grilled Salmon with Dill Sauce', 10),
        (8, 'Moroccan Chickpea Stew', 'Description for Moroccan Chickpea Stew', 10),
        (9, 'Szechuan Tofu Stir-Fry', 'Description for Szechuan Tofu Stir-Fry', 8),
        (10, 'Lamb Rogan Josh', 'Description for Lamb Rogan Josh', 14),
        (11, 'French Onion Soup', 'Description for French Onion Soup', 14),
        (12, 'Korean Bibimbap', 'Description for Korean Bibimbap', 10),
        (13, 'Mexican Chicken Fajitas', 'Description for Mexican Chicken Fajitas', 8),
        (14, 'Italian Risotto', 'Description for Italian Risotto', 8),
        (15, 'Greek Moussaka', 'Description for Greek Moussaka', 5),
        (16, 'Indian Butter Chicken', 'Description for Indian Butter Chicken', 13),
        (17, 'Spanish Paella', 'Description for Spanish Paella', 9),
        (18, 'American Cheeseburger', 'Description for American Cheeseburger', 7),
        (19, 'Japanese Sushi Rolls', 'Description for Japanese Sushi Rolls', 9),
        (20, 'English Fish and Chips', 'Description for English Fish and Chips', 5)
    ON CONFLICT (id) DO NOTHING;
    ")
    .map_err(|error| error.to_string())
}