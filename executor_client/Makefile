.PHONY: all run_all clear_build clear_aux_folders clear_docker clear_all

all: run_all

run_all:
	sudo rm -rf program_with_input
	# mkdir -p program_with_input
	docker-compose run --rm rust-app-executor

clear_build:
	# sudo rm -rf target src/runner/target
	docker-compose down
	docker volume rm executor_client_executor_volume
	echo "Deleted build folders"

# clear_aux_folders:
# 	sudo rm -rf program_with_input
# 	echo "Deleted aux folders"

clear_docker:
	docker-compose rm -s -f
	docker-compose down --rmi all

clear_all: clear_aux_folders clear_build clear_docker