.PHONY: all run_all

all: run_all

run_all:
	sudo rm -r aux_files uploads downloads
	mkdir -p aux_files uploads downloads
	docker-compose up

clear_build:
	sudo rm -r target src/runner/target
	echo "Deleted build folders"

clear_aux_folders:
	sudo rm -r aux_files uploads downloads
	echo "Deleted aux folders"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_aux_folders clear_build clear_docker