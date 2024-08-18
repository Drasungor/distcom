.PHONY: all run_all clear_build clear_docker clear_all clear_db

all: run_all

run_all:
	docker-compose up

clear_build:
	docker-compose down
	docker volume rm program_distributor_program_distributor_volume
	echo "Deleted build files"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_db:
	docker-compose down
	docker volume rm program_distributor_persisted_database

clear_all: clear_build clear_docker