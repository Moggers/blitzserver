#!/bin/bash
TAG=$(git tag --points-at HEAD)
if [[ ${#TAG} > 0 ]]; then
	cd /home/dom5/blitzserver
	git pull
	~/.cargo/bin/diesel migration run
	cargo build
	systemctl --user restart blitzserver
fi
