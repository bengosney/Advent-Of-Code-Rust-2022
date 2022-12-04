.PHONY: help clean test go production
.DEFAULT_GOAL := go

CURRENT_RS:=src/day_$(shell date +%d).rs
CURRENT_INPUT:=inputs/day_$(shell date +%d).txt
COOKIEFILE:=cookies.txt

SOURCE_FILES:=$(wildcard src/*.rs)


help: ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

inputs/day_%.txt: $(COOKIEFILE)
	cp ../Advent-Of-Code-2022/$@ $@
	## curl -H 'User-Agent: Makefile - curl : bengosney@googlemail.com' --cookie "$(shell cat $^)" -s -L -o $@ https://adventofcode.com/$(shell date +%Y)/day/$(shell echo "$@" | egrep -o "[0-9]+" | sed 's/^0*//')/input

src/day_%.rs:
	cp template.rs $@

test:
	cargo test -- --nocapture && notify-send "Passed" || notify-send "Failed"

go: $(CURRENT_RS) $(CURRENT_INPUT) ## Setup current day and start runing test monitor
	while inotifywait -e close_write src/*.rs; do make test; done

target/release/rust-aoc-2022: $(SOURCE_FILES)
	cargo build --release

production: target/release/rust-aoc-2022