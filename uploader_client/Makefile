.PHONY: all run_all clear_build clear_docker clear_all

all: run_all

run_all:
	# sudo rm -rf aux_files downloads
	# mkdir -p uploads programs_data aux_files downloads
	mkdir -p uploads programs_data
	docker-compose run --rm rust-app-uploader

clear_build:
	# sudo rm -rf target src/runner/target
	docker-compose down
	docker volume rm uploader_client_uploader_volume
	echo "Deleted build folders"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_docker clear_build