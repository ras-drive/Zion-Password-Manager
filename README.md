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

### Database Config
create an exported variable with access to a MongoDB database called users.
The database should have a collection titled Users, and it will populate when
the app is run with the variable set.