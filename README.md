## How to test it?
If you want to test the webserver you can run
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

if you want to run the test module then config the database(have the mongodb env variable set) and then run
```shell
cargo test
```

### Database Config
create an exported variable with access to a MongoDB database called users.
The database should have a collection titled Users, and it will populate when
the app is run with the variable set(it will look something like 
"export MONGODB_URI=mongodb+srv://user:<password>@users.somedata.mongodb.net/").
