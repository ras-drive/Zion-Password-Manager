# CURRENTLY THIS PROJECT IS BEING REDONE

in order to test the project right now just run

```shell
make configure
make build
make run
```

## Runing in Docker

To run with docker first check that the docker service has been started

```shell
sudo systemctl start docker.service
```

In order to build and run docker easily you can just run
the provided docker shell script

```shell
./build_and_run_docker.sh
```

## How to test it?

If you want to test the webserver you can run

```shell
make configure 
```

then

```shell
make dev
```

if you want to run the test module run

```shell
make test
```
