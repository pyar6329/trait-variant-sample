ifeq ($(shell uname -s), Darwin)
	CPU_CORES = $(shell sysctl -n hw.ncpu)
else
	CPU_CORES = $(shell grep -c processor /proc/cpuinfo)
endif

.PHONY:	help
help: ## show help message.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY:	check
check: ## check compile is succeed
	@cargo check -j $(CPU_CORES) --lib --bins --tests

.PHONY:	build
build: ## build application (This command cannot update Cargo.toml)
	@cargo test -j $(CPU_CORES) --no-run --locked

.PHONY:	update_cargo
update_cargo: ## build application
	@cargo test -j $(CPU_CORES) --no-run
