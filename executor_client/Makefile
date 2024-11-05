.PHONY: all run_all clear_build clear_docker clear_all

all: run_all

run_all:
	docker-compose run --rm rust-app-executor

clear_build:
	docker-compose down
	docker volume rm executor_client_executor_volume

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_build clear_docker