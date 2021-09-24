#!/bin/bash

DIR=$(dirname $(readlink -f $0))
APP="api"
if [ $1 ]; then
	APP=$1
fi

export RUST_BACKTRACE=1
cargo watch -x "run --bin $APP"
