
## How to test it?
### If you want to test the webserver with the database...

start a postgresql server with
```js
// from .env: DATABASE_URL=postgres://postgres:password@localhost/postgres
user = "postgres"
password = "password"
database_name = "postgres"
host = "localhost"
port = 5432
```
this will let Diesel connect to it

```shell
cargo run 
```
or if you want to reload on changes
```shell
cargo install cargo-watch
```
then
```shell
cargo watch -x run
```

### Database Config
create an exported variable with access to a MongoDB database called users.
The database should have a collection titled Users, and it will populate when
the app is run with the variable set.