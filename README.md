# Rust Full Stack Workshop

An implementation of the Barcelona Rust [Rust Full Stack Workshop](https://bcnrust.github.io/devbcn-workshop/index.html) project
without the use of Shuttle.

## Database Deployment

The Postgres database is deployed as a Docker container. For simplicity in development, use

```bash
docker run --name postgres -e POSTGRES_PASSWORD=password -p 127.0.0.1:5432:5432 -d postgres
```

which creates the container with the default `postgres` user.