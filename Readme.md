# Restaurant API

This is a restaurant application which accepts menu items from various serving staff in the restaurant.
The application accepts up to 20 simultaneous incoming add/remove/query requests.

## Feature Highlights

- Create an order with one or more menu items and a table number.
- Store the item, the table number, and how long the item will take to cook.
- Remove a specified item for a specified table number.
- Show all items for a specified table.
- Show the items still remaining for a specified table number.
- Show a specified item for a specified table number.
- Show the items still remaining for a specified set of table numbers.


## About the Current Implementation

For educational purposes, I chose not to use any tools or frameworks for API building. This approach has helped me understand key concepts of Rust language such as ownership, borrowing, and lifetimes, among others.

Of course, we may use third-party high-level libraries such as Axum or Rocket to build the whole API, as they offer quite extensive API building capabilities.

## Versioning

Sometimes we need to change functionality, which could completely break the previous implementation. We must provide a reliable and efficient service for users. That's why we should always ensure that our API is up-to-date and bug-free. This is why we use versioning in our API.

In this application, all routes operate under the /v1 path. When another version of the application is needed, we should create a new router with a base like /v2.

## Development Mode
For development purposes, we need to install Rust and also the PostgreSQL database.

Then, from the root directory, run:
```bash
make run
```

A listing of all the endpoints should output to the console after the server starts (this is for testing purposes only, and should be removed or commented for a production application):

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

The development server works on port 7878: [http://localhost:7878/v1](http://localhost:7878/v1).

By reaching this endpoint, you should receive a message "Version 1 is running."

The configuration of the development server is stored in the `.cargo/config.toml` file.



## Building the Production Server

To quickly launch the production version, we use a Docker container based on the rust:1.74-buster image. The production server operates on port 8000.

The configuration of the production server is stored in the `.cargo/config.production.toml` file.


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
The client is able to add one or more items with a table number.

Returns a list of all the items still remaining for the specified `table_id`.

- **Show the items still remaining for all tables:** GET [http://localhost:8000/v1/orders](http://localhost:8000/v1/orders)

- **Show the specified order item:** GET [http://localhost:8000/v1/orders/1](http://localhost:8000/v1/orders/1)

- **Show the items still remaining for a specified list of tables:** GET [http://localhost:8000/v1/tables/1,2,3/orders/](http://localhost:8000/v1/tables/1,2,3/orders/)

*Note:* table list shoud be a comma-separated list of the table numbers.


- **Show a specified item for a specified table number:**  GET [http://localhost:8000/v1/tables/1/order/1](http://localhost:8000/v1/tables/1/order/1)

*Note:* as each order has its own unique ID, we may simplify the request by excluding a table number. But there is a requirenment in a task specification: "The application MUST, upon query request, show a specified item **for a specified table number**", therefore in current API we should specify a table number, too.

 This method has an advantage in additional checking of the request correctness: does the requested order ID belong to specified table or not.


- **Delete a specified item for a specified table number:**  DELETE [http://localhost:8000/v1/tables/1/order/1](http://localhost:8000/v1/tables/1/order/1)

Returns a list of all the items still remaining for the specified table.




## Testing

To run unit tests, execute:

```bash
make test
```

## Documentation

There is a documentation for some important parts of this package. To open it, run:

```bash
make doc
```



## Areas for Improvement

## Additional endpoints

For a real-world application, we may need additional functionality, requiring more endpoints:

- Show all orders for a table, including those already cooked - useful for final checkout, for example.
- Archive a table's orders (when a client is finished, we must clear the table for the next client).

## Pagination
The list of all orders may be too large if our restaurant has a large client capacity. Loading all orders at once can be a resource-intensive operation. To solve this problem, we should add pagination to the query for all orders.

## Authentication

The current task doesn't include client authentication, but it is good practice to authenticate clients for authorized requests. We could use JWT authentication for this purpose.

## Configuration Files

This implementation includes server and database configuration files directly in the git repository for testing purposes, which, in real life, is not a good practice. We should exclude the configuration file from the repository and create them directly on servers.
