use postgres::{ Client, NoTls };

const DB_HOST: &str = "localhost";
const DB_PORT: usize = 5433;
const DB_USER: &str = "postgres";
const DB_PASS: &str = "postgres";
const DB_NAME: &str = "postgres";

pub const CONECTION_ERROR: &str = "Database connection error";

pub mod models;
pub mod model;


fn db_url() -> String {
    format!("postgres://{}:{}@{}:{}/{}", DB_USER, DB_PASS, DB_HOST, DB_PORT, DB_NAME)
}

/// Establishes a connection with a database
pub fn client() -> Result<Client, String> {
    match Client::connect(db_url().as_str(), NoTls) {
        Ok(client) => Ok(client),
        _ => Err(CONECTION_ERROR.to_string()),
    }
}

/// Warms up a database
pub fn set_database() -> Result<(), String> {
    let mut client = client()?;
    client.batch_execute("
    CREATE TABLE \"table\" (
        id SERIAL PRIMARY KEY,
        description TEXT NOT NULL
    );
    INSERT INTO \"table\" (description)
    VALUES
    ('Table 1'),
    ('Table 2'),
    ('Table 3'),
    ('Table 4'),
    ('Table 5'),
    ('Table 6'),
    ('Table 7'),
    ('Table 8'),
    ('Table 9'),
    ('Table 10');

    CREATE TABLE menu (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        time_to_cook_in_minutes INTEGER NOT NULL
    );
    INSERT INTO menu (name, description, time_to_cook_in_minutes)
    VALUES
    ('Classic Margherita Pizza', 'Description for Classic Margherita Pizza', 12),
    ('Spaghetti Carbonara', 'Description for Spaghetti Carbonara', 5),
    ('Vegetarian Lasagna', 'Description for Vegetarian Lasagna', 10),
    ('Thai Green Curry', 'Description for Thai Green Curry', 5),
    ('Chicken Caesar Salad', 'Description for Chicken Caesar Salad', 2),
    ('Beef Stroganoff', 'Description for Beef Stroganoff', 9),
    ('Grilled Salmon with Dill Sauce', 'Description for Grilled Salmon with Dill Sauce', 10),
    ('Moroccan Chickpea Stew', 'Description for Moroccan Chickpea Stew', 10),
    ('Szechuan Tofu Stir-Fry', 'Description for Szechuan Tofu Stir-Fry', 8),
    ('Lamb Rogan Josh', 'Description for Lamb Rogan Josh', 14),
    ('French Onion Soup', 'Description for French Onion Soup', 14),
    ('Korean Bibimbap', 'Description for Korean Bibimbap', 10),
    ('Mexican Chicken Fajitas', 'Description for Mexican Chicken Fajitas', 8),
    ('Italian Risotto', 'Description for Italian Risotto', 8),
    ('Greek Moussaka', 'Description for Greek Moussaka', 5),
    ('Indian Butter Chicken', 'Description for Indian Butter Chicken', 13),
    ('Spanish Paella', 'Description for Spanish Paella', 9),
    ('American Cheeseburger', 'Description for American Cheeseburger', 7),
    ('Japanese Sushi Rolls', 'Description for Japanese Sushi Rolls', 9),
    ('English Fish and Chips', 'Description for English Fish and Chips', 5);

    CREATE TABLE \"order\" (
        id SERIAL PRIMARY KEY,
        table_id INTEGER NOT NULL REFERENCES table(id),
        menu_id INTEGER NOT NULL REFERENCES menu(id),
        cooked_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
        is_deleted BOOLEAN,
        created_at TIMESTAMP WITHOUT TIME ZONE,
        updated_at TIMESTAMP WITHOUT TIME ZONE
    );
    CREATE INDEX idx_order_table_id ON \"order\"(table_id);
    CREATE INDEX idx_order_menu_id ON \"order\"(menu_id);
    ")
    .map_err(|error| error.to_string())
}