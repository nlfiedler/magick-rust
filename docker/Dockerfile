FROM rust:1.19.0-stretch

RUN apt-get update \
 && apt-get -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.0.7-14

RUN curl https://www.imagemagick.org/download/ImageMagick-${MAGICK_VERSION}.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION} \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}

RUN adduser --disabled-password --gecos '' magick-rust

USER magick-rust

ENV USER=magick-rust LD_LIBRARY_PATH=/usr/local/lib

WORKDIR /src
