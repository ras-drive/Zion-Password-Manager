# gets base image
FROM ubuntu

# get tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential npm nodejs libpq-dev

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash 

RUN ["/bin/bash", "-c", "source ~/.nvm/nvm.sh; nvm install 16; nvm alias default 16"]

# get cargo
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env; rustup override set nightly"]

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env; cargo install diesel_cli --no-default-features --features postgres"]

WORKDIR /usr/src/app

# clone project source
RUN git clone https://github.com/ras-drive/Zion-Password-Manager.git

WORKDIR /usr/src/app/Zion-Password-Manager

RUN ["/bin/bash", "-c", "cd frontend; npm i"]

RUN echo "DATABASE_URL=postgres://postgres:password@database:5432/zion" > .env

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env; source ~/.nvm/nvm.sh; nvm use 16; make build"]

EXPOSE 8000

RUN echo "cd backend/migrations; source $HOME/.cargo/env; diesel migration run" > start.sh
RUN chmod 777 start.sh

ENTRYPOINT ["/bin/bash", "-c", "source $HOME/.cargo/env; ./start.sh; make test"]