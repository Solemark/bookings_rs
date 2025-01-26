#!/bin/sh

if [ ! -d build ]
  then mkdir build
  else echo "build folder already exists"
fi

# build file
cargo build --release

if [ -f target/release/bookings_rs ]
  then mv target/release/bookings_rs build/bookings
    rm -rf target/release
  else echo "unable to find exe: bookings_rs in target/release"
fi
