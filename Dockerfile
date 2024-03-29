# gets base image
FROM rust as builder

# get tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential npm nodejs libpq-dev

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash 

RUN ["/bin/bash", "-c", "source ~/.nvm/nvm.sh; nvm install 16; nvm alias default 16"]

WORKDIR /usr/src/app

COPY . .

RUN ["/bin/bash", "-c", "cd frontend; source ~/.nvm/nvm.sh; nvm use 16; npm i"]

RUN echo "DATABASE_URL=postgres://postgres:password@database:5432/zion\nTEST_DATABASE_URL=postgres://postgres:password@database:5432/zion" > .env

RUN ["/bin/bash", "-c", "source ~/.nvm/nvm.sh; nvm use 16; make build"]

EXPOSE 8080
