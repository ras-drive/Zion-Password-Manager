# gets base image
FROM ubuntu

# get tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential npm nodejs

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash 

RUN ["/bin/bash", "-c", "source ~/.nvm/nvm.sh; nvm install 16; nvm alias default 16"]

# get cargo
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env; rustup override set nightly"]

WORKDIR /usr/src/app

# clone project source "$HOME/.cargo/env"
RUN git clone https://github.com/ras-drive/Zion-Password-Manager.git

RUN ["/bin/bash", "-c", "cd Zion-Password-Manager; source $HOME/.cargo/env; source ~/.nvm/nvm.sh; nvm use 16; make configure"]

RUN ["/bin/bash", "-c", "cd Zion-Password-Manager; source $HOME/.cargo/env; source ~/.nvm/nvm.sh; nvm use 16; make build"]

EXPOSE 8000

ENTRYPOINT ["/bin/bash", "-c", "cd Zion-Password-Manager; source $HOME/.cargo/env; make run"]
