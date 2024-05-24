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