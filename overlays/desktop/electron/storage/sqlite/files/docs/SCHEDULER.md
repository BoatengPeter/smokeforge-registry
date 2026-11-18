# Scheduled maintenance

The optional `scheduledMaintenance` setting starts a daily `node-cron` job at
03:00 local time. It runs a SQLite WAL checkpoint and logs the result through
electron-log. It does not modify user records.

The scheduler is off by default. Enable it in Settings under Advanced. The
scheduler is owned by the main process and is stopped when the setting is
disabled or the app exits.
