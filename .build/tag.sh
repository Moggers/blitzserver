#!/bin/bash
set-cargo-version Cargo.toml $1
git reset 
git add Cargo.toml
git commit -m "Version bump to $1"
git tag $1
