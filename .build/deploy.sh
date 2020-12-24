#!/bin/bash
cd /home/dom5/blitzserver
git pull
~/.cargo/bin/diesel migration run
cargo build
systemctl --user restart blitzserver
