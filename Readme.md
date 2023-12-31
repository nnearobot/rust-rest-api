# Restaurant API

This is a restaurant application which accepts menu items from various serving staff in the restaurant.
The application accepts up to 20 simultaneous incoming add/remove/query requests.

## Feature Highlights

- Create an order with one or more menu items and a table number.
- Store the item, the table number, and how long the item will take to cook.
- Remove a specified item for a specified table.
- Show all items for all tables.
- Show all items for a specified table.
- Show a specified item for a specified table.
- Show all items for a specified set of table numbers.


## About the Current Implementation

For educational purposes, I chose not to use any tools or frameworks for API building. This approach has helped me understand key concepts of Rust language such as ownership, borrowing, and lifetimes, among others.

Of course, we may use third-party high-level libraries such as Axum or Rocket to build the whole API, as they offer quite extensive API building capabilities.

All order requests retrieve a complete set of orders, including those that are deleted and completed. Clients are responsible for hiding any deleted orders from their display.

The time left to complete the current order is given in seconds, which simplifies the implementation of a timer on the client's side.


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

- **Show the items of all tables:** GET [http://localhost:8000/v1/orders](http://localhost:8000/v1/orders)

- **Show the specified order item:** GET [http://localhost:8000/v1/orders/1](http://localhost:8000/v1/orders/1)

- **Show the items of a specified list of tables:** GET [http://localhost:8000/v1/tables/1,2,3/orders/](http://localhost:8000/v1/tables/1,2,3/orders/)

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


## Simulating a clients

“Clients” are simulated as simple threads in a main() function calling the main server application with a variety of requests.

There are 10 clients running at any one time.

When `cargo run is executed, the following output appears in the console (the output from this application might be a little different every time, but it will look similar to the following):

<details>
  <summary>Click here to expand</summary>
  
```bash
Table 1 client started
Table 2 client started
Table 3 client started
Table 4 client started
Table 5 client started
Table 6 client started
Table 8 client started
Table 10 client started
Table 9 client started
Table 7 client started
Table 6 menu: 20 items
Table 7 menu: 20 items
Table 5 menu: 20 items
Table 10 menu: 20 items
Table 1 menu: 20 items
Table 9 menu: 20 items
Table 8 menu: 20 items
Table 4 menu: 20 items
Table 2 menu: 20 items
Table 3 menu: 20 items
Table 6 orders: [
    Order 422 for table 6: Vegetarian Lasagna (3)[600 sec left],
    Order 425 for table 6: Greek Moussaka (15)[300 sec left],
    Order 426 for table 6: Lamb Rogan Josh (10)[840 sec left],
]
Table 7 orders: [
    Order 421 for table 7: Beef Stroganoff (6)[540 sec left],
    Order 423 for table 7: Beef Stroganoff (6)[540 sec left],
    Order 424 for table 7: Greek Moussaka (15)[300 sec left],
]
Table 10 orders: [
    Order 427 for table 10: Mexican Chicken Fajitas (13)[480 sec left],
    Order 428 for table 10: Indian Butter Chicken (16)[780 sec left],
    Order 429 for table 10: Vegetarian Lasagna (3)[600 sec left],
]
Table 5 orders: [
    Order 439 for table 5: Lamb Rogan Josh (10)[840 sec left],
    Order 440 for table 5: Thai Green Curry (4)[300 sec left],
    Order 441 for table 5: Mexican Chicken Fajitas (13)[480 sec left],
]
Table 1 orders: [
    Order 436 for table 1: Classic Margherita Pizza (1)[720 sec left],
    Order 437 for table 1: Lamb Rogan Josh (10)[840 sec left],
    Order 438 for table 1: Szechuan Tofu Stir-Fry (9)[480 sec left],
]
Table 8 orders: [
    Order 433 for table 8: Szechuan Tofu Stir-Fry (9)[480 sec left],
    Order 434 for table 8: Vegetarian Lasagna (3)[600 sec left],
    Order 435 for table 8: Vegetarian Lasagna (3)[600 sec left],
]
Table 4 orders: [
    Order 430 for table 4: American Cheeseburger (18)[420 sec left],
    Order 431 for table 4: Chicken Caesar Salad (5)[120 sec left],
    Order 432 for table 4: Classic Margherita Pizza (1)[720 sec left],
]
Table 9 orders: [
    Order 442 for table 9: Spanish Paella (17)[540 sec left],
    Order 443 for table 9: Korean Bibimbap (12)[600 sec left],
    Order 444 for table 9: Lamb Rogan Josh (10)[840 sec left],
]
Table 2 orders: [
    Order 445 for table 2: Szechuan Tofu Stir-Fry (9)[480 sec left],
    Order 446 for table 2: American Cheeseburger (18)[420 sec left],
    Order 447 for table 2: Szechuan Tofu Stir-Fry (9)[480 sec left],
]
Table 3 orders: [
    Order 448 for table 3: Vegetarian Lasagna (3)[600 sec left],
    Order 449 for table 3: English Fish and Chips (20)[300 sec left],
    Order 450 for table 3: Indian Butter Chicken (16)[780 sec left],
]
Table 6 orders after deletion: [
    Order 422 for table 6: Vegetarian Lasagna (3)[594 sec left],
    Order 425 for table 6: Greek Moussaka (15) [deleted],
    Order 426 for table 6: Lamb Rogan Josh (10)[834 sec left],
]
Table 7 orders after deletion: [
    Order 421 for table 7: Beef Stroganoff (6)[534 sec left],
    Order 423 for table 7: Beef Stroganoff (6) [deleted],
    Order 424 for table 7: Greek Moussaka (15)[294 sec left],
]
Table 10 orders after deletion: [
    Order 427 for table 10: Mexican Chicken Fajitas (13)[474 sec left],
    Order 428 for table 10: Indian Butter Chicken (16)[774 sec left],
    Order 429 for table 10: Vegetarian Lasagna (3) [deleted],
]
Table 1 orders after deletion: [
    Order 436 for table 1: Classic Margherita Pizza (1)[714 sec left],
    Order 437 for table 1: Lamb Rogan Josh (10)[834 sec left],
    Order 438 for table 1: Szechuan Tofu Stir-Fry (9) [deleted],
]
Table 8 orders after deletion: [
    Order 433 for table 8: Szechuan Tofu Stir-Fry (9)[474 sec left],
    Order 434 for table 8: Vegetarian Lasagna (3) [deleted],
    Order 435 for table 8: Vegetarian Lasagna (3)[594 sec left],
]
Table 5 orders after deletion: [
    Order 439 for table 5: Lamb Rogan Josh (10)[834 sec left],
    Order 440 for table 5: Thai Green Curry (4) [deleted],
    Order 441 for table 5: Mexican Chicken Fajitas (13)[474 sec left],
]
Table 4 orders after deletion: [
    Order 430 for table 4: American Cheeseburger (18) [deleted],
    Order 431 for table 4: Chicken Caesar Salad (5)[114 sec left],
    Order 432 for table 4: Classic Margherita Pizza (1)[714 sec left],
]
Table 9 orders after deletion: [
    Order 442 for table 9: Spanish Paella (17)[534 sec left],
    Order 443 for table 9: Korean Bibimbap (12) [deleted],
    Order 444 for table 9: Lamb Rogan Josh (10)[834 sec left],
]
Table 2 orders after deletion: [
    Order 445 for table 2: Szechuan Tofu Stir-Fry (9)[474 sec left],
    Order 446 for table 2: American Cheeseburger (18)[414 sec left],
    Order 447 for table 2: Szechuan Tofu Stir-Fry (9) [deleted],
]
Table 3 orders after deletion: [
    Order 448 for table 3: Vegetarian Lasagna (3)[594 sec left],
    Order 449 for table 3: English Fish and Chips (20)[294 sec left],
    Order 450 for table 3: Indian Butter Chicken (16) [deleted],
]
Order 426 for table 6: Lamb Rogan Josh (10)[829 sec left]
Order 421 for table 7: Beef Stroganoff (6)[529 sec left]
Order 429 for table 10: Vegetarian Lasagna (3) [deleted]
Order 435 for table 8: Vegetarian Lasagna (3)[589 sec left]
Order 440 for table 5: Thai Green Curry (4) [deleted]
Order 437 for table 1: Lamb Rogan Josh (10)[829 sec left]
Order 431 for table 4: Chicken Caesar Salad (5)[109 sec left]
Order 443 for table 9: Korean Bibimbap (12) [deleted]
Order 446 for table 2: American Cheeseburger (18)[409 sec left]
Order 450 for table 3: Indian Butter Chicken (16) [deleted]
Table 6 orders after 200 sec: [
    Order 422 for table 6: Vegetarian Lasagna (3)[389 sec left],
    Order 425 for table 6: Greek Moussaka (15) [deleted],
    Order 426 for table 6: Lamb Rogan Josh (10)[629 sec left],
]
Table 7 orders after 200 sec: [
    Order 421 for table 7: Beef Stroganoff (6)[329 sec left],
    Order 423 for table 7: Beef Stroganoff (6) [deleted],
    Order 424 for table 7: Greek Moussaka (15)[89 sec left],
]
Table 10 orders after 200 sec: [
    Order 427 for table 10: Mexican Chicken Fajitas (13)[269 sec left],
    Order 428 for table 10: Indian Butter Chicken (16)[569 sec left],
    Order 429 for table 10: Vegetarian Lasagna (3) [deleted],
]
Table 5 orders after 200 sec: [
    Order 439 for table 5: Lamb Rogan Josh (10)[629 sec left],
    Order 440 for table 5: Thai Green Curry (4) [deleted],
    Order 441 for table 5: Mexican Chicken Fajitas (13)[269 sec left],
]
Table 8 orders after 200 sec: [
    Order 433 for table 8: Szechuan Tofu Stir-Fry (9)[269 sec left],
    Order 434 for table 8: Vegetarian Lasagna (3) [deleted],
    Order 435 for table 8: Vegetarian Lasagna (3)[389 sec left],
]
Table 1 orders after 200 sec: [
    Order 436 for table 1: Classic Margherita Pizza (1)[509 sec left],
    Order 437 for table 1: Lamb Rogan Josh (10)[629 sec left],
    Order 438 for table 1: Szechuan Tofu Stir-Fry (9) [deleted],
]
Table 4 orders after 200 sec: [
    Order 430 for table 4: American Cheeseburger (18) [deleted],
    Order 431 for table 4: Chicken Caesar Salad (5) [completed],
    Order 432 for table 4: Classic Margherita Pizza (1)[509 sec left],
]
Table 9 orders after 200 sec: [
    Order 442 for table 9: Spanish Paella (17)[329 sec left],
    Order 443 for table 9: Korean Bibimbap (12) [deleted],
    Order 444 for table 9: Lamb Rogan Josh (10)[629 sec left],
]
Table 2 orders after 200 sec: [
    Order 445 for table 2: Szechuan Tofu Stir-Fry (9)[269 sec left],
    Order 446 for table 2: American Cheeseburger (18)[209 sec left],
    Order 447 for table 2: Szechuan Tofu Stir-Fry (9) [deleted],
]
Table 3 orders after 200 sec: [
    Order 448 for table 3: Vegetarian Lasagna (3)[389 sec left],
    Order 449 for table 3: English Fish and Chips (20)[89 sec left],
    Order 450 for table 3: Indian Butter Chicken (16) [deleted],
]

```
</details>


## Areas for Improvement

### Additional endpoints

For a real-world application, we may need additional functionality, requiring more endpoints:

- Show all orders for a table, including those already cooked - useful for final checkout, for example.
- Archive a table's orders (when a client is finished, we must clear the table for the next client).

### Pagination
The list of all orders may be too large if our restaurant has a large client capacity. Loading all orders at once can be a resource-intensive operation. To solve this problem, we should add pagination to the query for all orders.

### Authentication

The current task doesn't include client authentication, but it is good practice to authenticate clients for authorized requests. We could use JWT authentication for this purpose.

### Configuration Files

This implementation includes server and database configuration files directly in the git repository for testing purposes, which, in real life, is not a good practice. We should exclude the configuration file from the repository and create them directly on servers.
