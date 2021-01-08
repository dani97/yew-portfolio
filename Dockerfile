FROM rust:1.48-slim-buster
MAINTAINER Christopher Daniel <chris.daniel@hey.com>

RUN apt-get update -y && apt-get install curl -y

RUN curl -sL https://deb.nodesource.com/setup_14.x -o nodesource_setup.sh

RUN sh nodesource_setup.sh

RUN apt install nodejs -y

RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main"
RUN apt-get update &&  apt-get install --no-install-recommends yarn -y

COPY . .
RUN yarn install && yarn build
