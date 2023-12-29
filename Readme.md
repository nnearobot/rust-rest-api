# Restaurant API

This is a restaurant application which accepts menu items from various serving staff in the restaurant.
The application accepts up to 20 simultaneous incoming add/remove/query requests.


## Feature highlight

- Create an order with one or more menu items and a table number.
- Store the item, the table number, and how long the item will take to cook.
- Remove a specified item for a specified table number.
- Show all items for a specified table.
- Show the items still remaining for a specified table number.
- Show a specified item for a specified table number.
- Show the items still remaining for a specified set of table numbers.


## Versioning

Sometimes we need to change a functionality, so the prevoius functionally is totally breaks. We must to provide a reliable and efficient service for users. That is why we always should be sure that our API is up-to-date and bug-free. This is why we use versioning in our API.

In this application all the routes work under `/v1` path. When we need another version of the application, we should create another router with `/v2` (or something like) base.


## Developing mode
For developing purpose we should install Rust, and also PostgreSQL database.

Then from the root directory run:
```bash
make run
```

Listing of all the endpoints should be output to the console after server run:

```
Server has started on port 7878
/v1
  -GET
  /tables
    -GET
    /:dyn
      /orders
        -GET
        /:dyn
          -GET
          -DELETE
  /orders
    -GET
    -POST
    /:dyn
      -GET
  /menu
    -GET
```

Developing server works on 7878 port: [http://127.0.0.1:7878/v1](http://127.0.0.1:7878/v1).

By reaching this endpoint you should get a message "Version 1 is running".



## Building the Production Server

To quickly launch the production version, we use a Docker container based on the rust:1.74-buster image. The production server operates on port 8000.


### Initial Launch

For building an image and start the server, execute:

```bash
make start
```


### Stopping the Server

To stop the server, execute:

```bash
make stop
```


## Testing Endpoints

You can test this API using tools like Postman.


### Endpoints:

- **Version 1 API health check:** GET [http://localhost:8000/v1](http://localhost:8000/v1)

- **List menu:** GET [http://localhost:8000/v1/menu](http://localhost:8000/v1/menu)

- **List tables:** GET [http://localhost:8000/v1/tables](http://localhost:8000/v1/tables)

- **Create an order:** POST http://localhost:8000/v1/orders

Body: raw
JSON params: 
```
{
    "table_id": 1,
    "menu_id": [9, 2, 5, 3, 6, 8, 4, 4, 8, 3]
}
```
Returns a list of all the items still remaining for the specified `table_id`.

- **Show the items still remaining for all tables:** GET [http://localhost:8000/v1/orders](http://localhost:8000/v1/orders)

- **Show the specified order item:** GET [http://localhost:8000/v1/orders/1](http://localhost:8000/v1/orders/1)

- **Show the items still remaining for a specified list of tables:** GET [http://localhost:8000/v1/tables/1,2,3/orders/](http://localhost:8000/v1/tables/1,2,3/orders/)

Table list shoud be a comma-separated list of the table numbers.


- **Show a specified item for a specified table number:**  GET [http://localhost:8000/v1/tables/1/order/1](http://localhost:8000/v1/tables/1/order/1)

- **Delete a specified item for a specified table number:**  DELETE [http://localhost:8000/v1/tables/1/order/1](http://localhost:8000/v1/tables/1/order/1)

Returns a list of all the items still remaining for the specified table.




## Testing

To run unit tests, execute:

```bash
make test
```



## Potential Improvements
//TODO


