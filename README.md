# Roctokit

|          |status|
|----------|------|
| CI build |[![Build Status](https://travis-ci.com/jburger/roctokit.svg?branch=master)](https://travis-ci.com/jburger/roctokit)|

Roctokit is a Github v3 API Client, built entirely in rust, using `reqwest` and `serde_json` under the hood.

> Roctokit is currently under development and is not yet complete
## Examples
### The basics
#### Add a dependency
`Cargo.toml`
```toml
[dependencies]
"roctokit" = "0.1.0"
```
#### Creating an anonymous client

```rust
let client = 
    GitHubClientBuilder::new()
        .for_user_agent("this is a drill, this is a drill...")
        .build();
```
#### Creating an authenticated client
```rust
let client = 
    GitHubClientBuilder::new()
        .for_user_agent("this is a drill, this is a drill...")
        .with_oauth_token(env::var("my_github_token"))
        .build();
```
#### Finding an organization
```rust
let tanz_industries = 
    client
        .organizations
        .get_by_name("Tanz industries international")
        .unwrap();
println!("{}", tanz_industries.name);
```


