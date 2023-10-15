#!/bin/bash

cd crates/server || exit
cargo shuttle deploy --name bramletts-games
