-- SQLite
CREATE TABLE `soundgasm_tracks`(
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
CREATE INDEX `idx_soundgasm_tracks_profile_slug` ON `soundgasm_tracks`(`profile_slug`);
CREATE INDEX `idx_soundgasm_tracks_sound_id` ON `soundgasm_tracks`(`sound_id`);

CREATE TABLE `soundgasm_profiles`(
	`slug` TEXT NOT NULL,
	`name` TEXT NOT NULL,
	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`slug`)
);
CREATE INDEX `idx_soundgasm_profiles_slug` ON `soundgasm_profiles`(`slug`);

CREATE TABLE `kemono_posts`(
	`profile_slug` TEXT NOT NULL,
	`post_id` TEXT NOT NULL,
	`title` TEXT NOT NULL,
	`content` TEXT NOT NULL,
	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`profile_slug`, `post_id`)
);
CREATE INDEX `idx_kemono_posts_profile_slug` ON `kemono_posts`(`profile_slug`);

CREATE TABLE `kemono_creators`(
	`slug` TEXT NOT NULL,
	`name` TEXT NOT NULL,
	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`slug`)
);
CREATE INDEX `idx_kemono_creators_slug` ON `kemono_creators`(`slug`);
