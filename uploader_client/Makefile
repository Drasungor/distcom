.PHONY: all run_all clear_build clear_aux_folders clear_docker clear_all

all: run_all

run_all:
	sudo rm -rf aux_files downloads
	mkdir -p uploads programs_data aux_files downloads
	docker-compose run --rm rust-app-uploader

clear_build:
	sudo rm -rf target src/runner/target
	echo "Deleted build folders"

clear_aux_folders:
	sudo rm -rf aux_files downloads
	echo "Deleted aux folders"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_aux_folders clear_build clear_docker