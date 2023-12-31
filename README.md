# tide-api

Rust based web API, listening at localhost:8080.
Used RustRover EAP.

```rust
 app.listen("0.0.0.0:8080").await?;
```

## Commands

```rust
  // return wizards in json format
  app.at("/wizards").get(get);
```

```rust
  // post json { "name": "foo", "level": bar }
  app.at("/wizards").post(create);
```

## Deps
```rust
  [dependencies]
  tide = { version = "0.16.0" }
  serde = { version = "1.0", fetures = ["derive"] }
  tokio = { version = "1.0", features = ["full"] }
  femme = "2.2.1"
  async-std = { version = "1.12.0", features = ["attributes"] }
```
