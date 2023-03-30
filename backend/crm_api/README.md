# CRM for the Educational Platform 

* uses Axum as a web application framework (API Server)



# Rust / Axum Setup

crates:
```
cargo add axum 
cargo add tokio
cargo add tokio -F macros -F rt-multi-thread
cargo add sqlx -F runtime-tokio-rustls
cargo add serde
cargo add serde -F derive
```

To see documentation for the Rust crates used in this project: 
```
> cargo doc --open
```



# Docker / Postgres Setup

within the `database/` directory there is an `init.sql` file which contains the SQL to create  
the `platform` db and schema. It also contains some insert statements to insert sample data. 

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