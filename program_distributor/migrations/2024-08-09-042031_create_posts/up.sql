-- Your SQL goes here
CREATE TABLE `specific_program_input`(
	`specific_input_id` VARCHAR(255) NOT NULL PRIMARY KEY,
	`input_group_id` VARCHAR(255) NOT NULL,
	`blob_data` VARBINARY(1024),
	`order` INTEGER NOT NULL,
	INDEX `input_group_id_index` (`input_group_id`)
);

CREATE TABLE `refresh_token`(
	`token_id` VARCHAR(255) NOT NULL PRIMARY KEY,
	`user_id` VARCHAR(255) NOT NULL
);

CREATE TABLE `program`(
	`organization_id` VARCHAR(255) NOT NULL,
	`program_id` VARCHAR(255) NOT NULL PRIMARY KEY,
	`name` VARCHAR(255) NOT NULL,
	`description` VARCHAR(700) NOT NULL,
	`input_lock_timeout` BIGINT NOT NULL,
	UNIQUE INDEX `name_index` (`name`)
);

CREATE TABLE `program_input_group`(
	`input_group_id` VARCHAR(255) NOT NULL PRIMARY KEY,
	`program_id` VARCHAR(255) NOT NULL,
	`name` VARCHAR(255) NOT NULL,
	`last_reserved` DATETIME,
	`proven_datetime` DATETIME
);


CREATE TABLE `account`(
	`organization_id` VARCHAR(255) NOT NULL PRIMARY KEY,
	`name` VARCHAR(255) NOT NULL,
	`description` VARCHAR(700) NOT NULL,
	`username` VARCHAR(255) NOT NULL,
	`password_hash` VARCHAR(255) NOT NULL,
	INDEX `name_index` (`name`)
);

