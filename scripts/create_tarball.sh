#!/bin/bash

name=marcador-v0.6.0-x86_64-unknown-linux-gnu

mkdir $name

cargo build --release
cp target/release/marcador $name
cp target/release/marcador_server $name
cp README.md $name
cp LICENSE $name

tar -cvf $name.tar.gz $name/*

rm -rf $name
