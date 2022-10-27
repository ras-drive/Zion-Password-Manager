# gets base image
FROM ubuntu/postgres


# get tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential

# get cargo
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

WORKDIR /usr/src/app

# clone project source "$HOME/.cargo/env"
RUN git clone https://github.com/ras-drive/Zion-Password-Manager.git

# TEMPORARY FOR THIS BRANCH ***PLEASE REMOVE AFTER THIS FILE IS PUSHED TO MAIN!***
RUN ["/bin/bash", "-c", "cd Zion-Password-Manager/; git checkout db-rewrite; cd .."]

RUN ["/bin/bash", "-c", "cd Zion-Password-Manager; source $HOME/.cargo/env; make build"]

EXPOSE 8080

CMD ["/bin/bash", "-c", "cd Zion-Password-Manager; source $HOME/.cargo/env; make run"]
#ENTRYPOINT ["/bin/bash", "-c", "ls; make run"]

