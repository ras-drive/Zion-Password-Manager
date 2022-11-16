# CURRENTLY THIS PROJECT IS BEING REDONE

First things first, you need to setup a postgresql database

Any of the links below explain how to setup a postgresql database so
pick your distributions (if yours isn't listed I prefer the Arch Docs)

* [Debian](https://wiki.debian.org/PostgreSql)
* [Fedora](https://docs.fedoraproject.org/en-US/quick-docs/postgresql/)
* [Arch](https://wiki.archlinux.org/title/PostgreSQL)

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
docker-compose up
```
