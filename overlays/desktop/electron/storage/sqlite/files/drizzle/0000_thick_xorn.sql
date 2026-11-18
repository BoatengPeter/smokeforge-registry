CREATE TABLE `library_items` (
	`id` text PRIMARY KEY NOT NULL,
	`title` text NOT NULL,
	`description` text DEFAULT '' NOT NULL,
	`status` text DEFAULT 'draft' NOT NULL,
	`tags` text NOT NULL,
	`created_at` integer NOT NULL,
	`updated_at` integer NOT NULL,
	`deleted_at` integer
);
--> statement-breakpoint
CREATE INDEX `library_items_status_idx` ON `library_items` (`status`);--> statement-breakpoint
CREATE INDEX `library_items_updated_idx` ON `library_items` (`updated_at`);