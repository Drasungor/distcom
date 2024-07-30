.PHONY: all run_all

all: run_all

run_all:
	sudo rm -r aux_files downloads
	mkdir -p uploads programs_data aux_files downloads
	docker-compose run --rm rust-app-uploader

clear_build:
	sudo rm -r target src/runner/target