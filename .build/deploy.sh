#!/bin/bash
cd /home/dom5/blitzserver
TAG=$(git tag --points-at HEAD)
if [[ ${#TAG} > 0 ]]; then
	git pull
	~/.cargo/bin/diesel migration run
	cargo build
	systemctl --user restart blitzserver
fi
