.PHONY: all run_all

all: run_all

run_all:
	sudo rm -r program_with_input
	mkdir -p program_with_input
	docker-compose run --rm rust-app-executor

clear_build:
	sudo rm -r target src/runner/target
	echo "Deleted build folders"

clear_aux_folders:
	sudo rm -r program_with_input
	echo "Deleted aux folders"

clear_build_aux: clear_aux_folders clear_build