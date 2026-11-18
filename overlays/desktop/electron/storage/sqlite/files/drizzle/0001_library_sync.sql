ALTER TABLE `library_items` ADD COLUMN `version` integer DEFAULT 0 NOT NULL;
--> statement-breakpoint
ALTER TABLE `library_items` ADD COLUMN `sync_status` text DEFAULT 'local' NOT NULL;
--> statement-breakpoint
ALTER TABLE `library_items` ADD COLUMN `owner_id` text;
--> statement-breakpoint
CREATE TABLE `sync_outbox` (
  `id` text PRIMARY KEY NOT NULL,
  `owner_id` text NOT NULL,
  `entity` text NOT NULL,
  `entity_id` text NOT NULL,
  `operation` text NOT NULL,
  `payload` text NOT NULL,
  `base_version` integer NOT NULL,
  `idempotency_key` text NOT NULL,
  `status` text DEFAULT 'pending' NOT NULL,
  `attempts` integer DEFAULT 0 NOT NULL,
  `next_attempt_at` integer DEFAULT 0 NOT NULL,
  `last_error` text,
  `created_at` integer NOT NULL,
  `updated_at` integer NOT NULL
);
--> statement-breakpoint
CREATE INDEX `sync_outbox_owner_status_idx` ON `sync_outbox` (`owner_id`, `status`);
--> statement-breakpoint
CREATE TABLE `sync_cursor` (
  `owner_id` text PRIMARY KEY NOT NULL,
  `cursor` integer DEFAULT 0 NOT NULL,
  `updated_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `sync_conflicts` (
  `id` text PRIMARY KEY NOT NULL,
  `owner_id` text NOT NULL,
  `entity_id` text NOT NULL,
  `local_payload` text NOT NULL,
  `remote_payload` text NOT NULL,
  `created_at` integer NOT NULL,
  `resolved_at` integer
);
--> statement-breakpoint
CREATE INDEX `sync_conflicts_owner_idx` ON `sync_conflicts` (`owner_id`);
