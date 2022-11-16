# What is Zion?

Zion is a password manager being developed to be a modern, cross platform, and secure with a
backend written in Rust. currently the only frontend being developed for use is a webpage but
eventually an android version will be released when the backend is stable.

## Testing the project

First things first, if you aren't using Docker you need to setup a postgresql database

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

Then you can run it with Docker Compose

```shell
docker-compose up
```

### License

This project is dual licensed under the MIT and Apache 2.0 license
