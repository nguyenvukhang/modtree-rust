#!/usr/bin/env sh

if [ -z $1 ] || [ -z $2 ]; then
  echo "Set [release|debug] as first arg."
  echo "Set <PACKAGE> as second arg."
  echo "Example: sh debug.sh debug fetcher"
  exit 1
fi

TARGET=$1
PACKAGE=$2

if [[ "$TARGET" == "release" ]]; then
  cargo build --release --workspace --package $PACKAGE || exit 1
  ./target/release/$PACKAGE
elif [[ "$TARGET" == "debug" ]]; then
  cargo build --workspace --package $PACKAGE || exit 1
  ./target/debug/$PACKAGE
fi
