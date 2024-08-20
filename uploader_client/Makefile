.PHONY: all run_all clear_build clear_docker clear_all

all: run_all

run_all:
	mkdir -p uploads programs_data
	docker-compose run --rm rust-app-uploader

clear_build:
	docker-compose down
	docker volume rm uploader_client_uploader_volume
	echo "Deleted build folders"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_docker clear_build