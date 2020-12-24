#!/bin/bash
cd /home/dom5/blitzserver
git checkout .
git pull
~/.cargo/bin/diesel migration run
cargo build
systemctl --user restart blitzserver
