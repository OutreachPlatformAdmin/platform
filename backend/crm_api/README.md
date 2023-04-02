# CRM for the Educational Platform 

* uses Axum as a web application framework (API Server)



# Rust / Axum Setup

crates:
```
cargo add axum 
cargo add tokio
cargo add tokio -F macros -F rt-multi-thread
cargo add sqlx -F runtime-tokio-rustls postgres
cargo add serde
cargo add serde -F derive
```

To see documentation for the Rust crates used in this project: 
```
> cargo doc --open
```

To start the Axum server such that it will refresh with each saved code change:
```
cargo watch -x run
```


### Environment Variables 

all environment variables are stored in `crm_api/.env`

```
POSTGRES_PASSWORD=<password>
DB_URI="postgresql://<username>:<password>@localhost:<port>/<database_name>"
```

# Docker / Postgres Setup

within the `database/` directory there is an `init.sql` file which contains the SQL to create  
the `platform` db and schema. It also contains some insert statements to insert sample data.

There's also a `.env` file in `backend/crm_api` that can be used to set environment variables for the docker container that is not checked in to git. It's recommended to set the `POSTGRES_PASSWORD` environment variable there.

Start up the docker container which will create a docker container running a postgres instance
```
cd backend/crm_api
docker compose up
```
get container name
```
docker ps
```
exec into the container 
```
docker exec -it <container_name> bash
```
get into the db via `psql`
```
psql -U postgres
```
commands within psql
```
\c platform;
select * from platform.topics;
```
shutting down the docker container 
```
docker stop <container_name>
```


If you want to make updates to `init.sql` file after having started the docker container once,
you'll need to run this command within `backend/crm_api` to remove the cached version of the 
postgres DB. 
```
docker-compose down --volumes
```
