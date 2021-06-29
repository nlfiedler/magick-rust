FROM rust:latest

RUN apt-get update \
 && apt-get -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1

RUN curl https://download.imagemagick.org/ImageMagick/download/ImageMagick.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION}* \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}*

RUN adduser --disabled-password --gecos '' magick-rust

USER magick-rust

ENV USER=magick-rust LD_LIBRARY_PATH=/usr/local/lib

WORKDIR /src
