CREATE TABLE `file_downloads` (
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`url` TEXT NOT NULL,
	`file_path` TEXT NOT NULL,
	`created_at` DATETIME NOT NULL,
	UNIQUE(`url`, `file_path`)
);

CREATE TABLE `downloaded_segments` (
	`download_id` INTEGER NOT NULL,
	`start_index` INTEGER NOT NULL,
	`end_index` INTEGER NOT NULL,
	FOREIGN KEY(`download_id`) REFERENCES file_downloads(`id`)
);

CREATE TABLE `soundgasm_tracks` (
	`profile_slug` TEXT NOT NULL,
	`track_slug` TEXT NOT NULL,
	`title` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`sound_id` TEXT,
	`file_extension` TEXT,
	`content_hash` TEXT,
	`content_length` BIGINT,
	`created_at` DATETIME NOT NULL,
	`updated_at` DATETIME NOT NULL,
	`deleted_at` DATETIME,
	PRIMARY KEY(`profile_slug`, `track_slug`)
);
CREATE INDEX `idx_soundgasm_tracks_profile_slug` ON `soundgasm_tracks`(`profile_slug`);
CREATE INDEX `idx_soundgasm_tracks_track_slug` ON `soundgasm_tracks`(`track_slug`);

-- CREATE TABLE `kemono_creators`(
-- 	`provider_domain` TEXT NOT NULL,
-- 	`service` TEXT NOT NULL,
-- 	`slug` TEXT NOT NULL,
-- 	`name` TEXT NOT NULL,
-- 	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`deleted_at` DATETIME,
-- 	PRIMARY KEY(`slug`)
-- );
-- CREATE INDEX `idx_kemono_creators_slug` ON `kemono_creators`(`slug`);

-- CREATE TABLE `kemono_posts`(
--   `provider_domain` TEXT NOT NULL,
-- 	`service` TEXT NOT NULL,
-- 	`profile_slug` TEXT NOT NULL,
-- 	`post_id` TEXT NOT NULL,
-- 	`revision_id` TEXT NOT NULL
-- 	`title` TEXT NOT NULL,
-- 	`content` TEXT NOT NULL,
-- 	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`deleted_at` DATETIME,
-- 	PRIMARY KEY(`profile_slug`, `post_id`)
-- );
-- CREATE INDEX `idx_kemono_posts_profile_slug` ON `kemono_posts`(`profile_slug`);

-- CREATE TABLE `kemono_images`(
-- 	`profile_slug` TEXT NOT NULL,
-- 	`post_id` TEXT NOT NULL,
-- 	`title` TEXT NOT NULL,
-- 	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`deleted_at` DATETIME,
-- 	PRIMARY KEY(`profile_slug`, `post_id`)
-- );
-- CREATE INDEX `idx_kemono_posts_profile_slug` ON `kemono_posts`(`profile_slug`);

-- CREATE TABLE `kemono_videos`(
-- 	`profile_slug` TEXT NOT NULL,
-- 	`post_id` TEXT NOT NULL,
-- 	`title` TEXT NOT NULL,
-- 	`content` TEXT NOT NULL,
-- 	`created_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`updated_at` DATETIME NOT NULL DEFAULT current_timestamp,
-- 	`deleted_at` DATETIME,
-- 	PRIMARY KEY(`profile_slug`, `post_id`)
-- );
-- CREATE INDEX `idx_kemono_posts_profile_slug` ON `kemono_posts`(`profile_slug`);

