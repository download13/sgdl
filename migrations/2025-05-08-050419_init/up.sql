-- SQLite
CREATE TABLE `tracks`(
	`profile_slug` TEXT NOT NULL,
	`track_slug` TEXT NOT NULL,
	`title` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`sound_id` TEXT,
	`file_extension` TEXT,
	`content_hash` TEXT,
	`content_length` BIGINT,
	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
	`deleted_at` DATETIME,
	PRIMARY KEY(`profile_slug`, `track_slug`)
);

