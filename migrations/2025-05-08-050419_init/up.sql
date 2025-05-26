-- Your SQL goes here
CREATE TABLE `tracks`(
	`fetch_url` TEXT NOT NULL,
	`profile_slug` TEXT NOT NULL,
	`track_slug` TEXT NOT NULL,
	`title` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`file_extension` TEXT NOT NULL,
	`content_hash` TEXT NOT NULL,
	`content_length` BIGINT NOT NULL,
	PRIMARY KEY(`profile_slug`, `track_slug`)
);

