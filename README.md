# Rust Full Stack Workshop

An implementation of the Barcelona Rust [Rust Full Stack Workshop](https://bcnrust.github.io/devbcn-workshop/index.html) project
without the use of Shuttle, with Actix 4 and Dioxus 0.5. Accordingly, there are several implementation differences between
this implementation and the reference version, particularly when it comes to deployment and the implementation of the frontend,
as Dioxus 0.5 differs significantly from previous versions due to the new signal-based hooks.

## Database Deployment

The Postgres database is deployed as a Docker container. For simplicity in development, use

```shell
docker run --name postgres -e POSTGRES_PASSWORD=password -p 127.0.0.1:5432:5432 -d postgres
```

which creates the container with the default `postgres` user. Alternatively, you can start
the database by using `cargo make` with the following command and the existing Makefile.toml
in the workspace directory:
```shell
cargo make db-start
```

To shut down the database, use either docker directly on the command line or use 
```shell
cargo make db-stop
```

## Building and Deploying

Use the following cargo command to build the front project:

```shell
cargo make front-build
```

and then use this command to deploy the application to localhost:8080

```shell
cargo make app-run
```

## Implementation Details

Due to [this issue](https://github.com/DioxusLabs/dioxus/issues/2307) as of 05/23/24 it is necessary to pull from GitHub 
instead of from crates.io, since there's currently a fix available on GitHub. I found that this was needed because
calls to the `delete_film` closure in app.rs in the front crate did not trigger `use_effect` when `spawn` was invoked on
an `async` block that updated a `signal` that was subscribed to in `use_effect`, which made it necessary to trigger a full
rerender of the `App` component in order to make the deleted film disappear from the UI.