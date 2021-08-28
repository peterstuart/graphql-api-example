## Setup

- Copy `.env.example` to `.env`, and adjust as needed
- Install the SQLx CLI:
```shell
cargo install sqlx-cli --no-default-features --features postgres
```
- Set up the database:
```shell
sqlx database create
sqlx migrate run
```

## Running

```shell
cargo run
```

Open http://localhost:8080/graphiql in your browser.

### Example Mutations and Queries

```graphql
mutation {
  createUser(name: "Test User") {
    id
    name
  }
}
```

```graphql
mutation {
  createTodo(userId: 1, name: "My Todo", status: INCOMPLETE) {
    id
    name
    status
  }
}
```

```graphql
query {
  user(id: 1) {
    id
    name
    todos {
      id
      name
      status
    }
  }
}
```
