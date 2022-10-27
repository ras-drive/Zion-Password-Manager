# How to test it?

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

## How to run with Docker

To run with docker first check that the docker service has been started

```shell
sudo systemctl start docker.service
```

After you are sure the docker daemon is active you can run the following to make a build

```shell
sudo docker build .
```

Then run this to get a build id

```shell
sudo docker images
```

The output should look like this

REPOSITORY        TAG       IMAGE ID       CREATED          SIZE
zion              0.1.0     cc0c5467c9f0   3 minutes ago    2.81GB
ubuntu/postgres   latest    d185f4582c7b   7 days ago       393MB

Copy the image id of zion.

Run this to start the docker server and you're all set

```shell
sudo docker run <ZION_IMAGE_ID>
```
