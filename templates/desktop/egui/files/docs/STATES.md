# App states

Every screen must handle more than the happy path. This app has a fixed set of
state patterns; use them instead of inventing new ones, and add a new pattern
here before using it in a page.

## Which pattern to use

| Situation | Pattern | Component | Example in the app |
| --- | --- | --- | --- |
| The app cannot start at all | Startup failure screen | `startup::StartupFailure` | settings, migrations, or the database fail in `DesktopApp::new` |
| An area cannot show its content | Status page | `components::StatusPage` | database unreadable (`views::database_error`), record not found |
| A list or area is legitimately empty | Empty state | `components::EmptyState` | no records yet, search has no matches, nothing selected |
| The page works but something is degraded | Notice | `components::Notice` | sign-in failed, last sync failed, keyring unavailable |
| Work is in progress | Inline spinner + text | `ui.spinner()` + label | sign-in, sync |
| A destructive, irreversible action | Confirmation dialog | `components::ConfirmDialog` | clear local data |
| First launch, or the user asks for an introduction | Welcome tour | `components::WizardWindow` | `views/welcome.rs` |
| A reversible action finished | Toast with Undo | `toasts` + status bar link | delete record (soft delete + Undo) |
| A field is invalid | Inline error above the form | `palette.danger` label | Library editor validation |

Decision rules:

- **Status page vs notice.** If the user can still do the page's main task, use
  a notice. If not, replace the area with a status page.
- **Empty vs error.** "No records" is empty; "could not read records" is an
  error. Never show an empty state when a read failed — that hides data loss.
- **Confirm vs undo.** Reversible actions do not ask for confirmation; offer
  Undo. Only irreversible actions get a `ConfirmDialog`.
- **Copy names the object and the next step.** "Can't read the local database"
  + "Try again" / "Open logs folder", not "Error" + "OK". Confirmations name
  the verb: title "Clear all local records?", button "Clear local data".
- **Details are for reports.** Put the raw error chain in
  `StatusPage::details`, never in the title or body.

## The states in this app

### Startup failure

`crates/desktop/src/main.rs` starts `DesktopApp::new`; on error it shows
`StartupFailure` instead of exiting. It explains that nothing was deleted,
shows the error chain, and offers **Try again** (re-runs startup in place),
**Open logs folder**, and **Quit**. Test: `startup::tests`.

### Database unreadable

Pages read records through `DesktopApp::records(search)`, which returns
`Result`. On `Err`, call `views::database_error(ui, &error, &self.paths.logs)`;
it draws the status page and returns `Action::ReopenDatabase` for **Try
again**. Dashboard and Library both use it.

### Record not found

Selecting a record that no longer exists sets `Editor::Missing(id)`. The
Library editor shows a neutral status page with **Back to records**.

### Sign-in failed, sync failed, keyring unavailable

`Notice` banners: danger for sign-in failure (Account), warning for the last
sync's error (`sync_error`, Account and Activity), warning when the OS keyring
is unavailable (Settings). They disappear when the condition clears.

### Syncing

`DesktopApp::is_syncing()` is true while a sync thread runs; Account and
Activity show a spinner and "Syncing…". The worker calls `request_repaint` when
it finishes.

### Clear local data

Settings dispatches `Action::RequestConfirmation(Confirmation::ClearLocalData)`.
The shell draws a `ConfirmDialog` while `confirmation` is `Some`, and dispatches
`Action::Confirm` or `Action::CancelConfirmation`.

### First launch

`DesktopApp.welcome` starts as `Some(WelcomePage::Intro)` when
`settings.onboarded` is false. The tour is non-blocking: the app behind it
works. Closing it any way (finish, ×, Esc) sets `onboarded`, so it never
reappears uninvited; Settings › Help reopens it. See `docs/UI.md` › Welcome
tour for adding pages.

## Adding a new state

1. Pick the pattern from the table. If none fits, add a row here first and get
   it reviewed; do not draw ad-hoc frames.
2. Model the state in data, not in widgets: an enum variant (`Editor::Missing`,
   `Confirmation::...`) or a field (`sync_error`). Avoid boolean pairs that can
   contradict each other.
3. Make the read path return `Result` so the page can tell empty from failed.
4. Add strings to **both** `locales/en.yml` and `locales/es.yml` under
   `state.*` (or the page's section). Titles say what happened; bodies say what
   it means for the user; actions say what happens next.
5. Draw it with the component from the table; dispatch an `Action` for the
   recovery step.
6. Add an `egui_kittest` test that sets up the state and asserts the title and
   the recovery action by accessible label (see `startup::tests`).

### Adding a new confirmation

1. Add a variant to `Confirmation` in `lib.rs`.
2. Add its title, body, and confirm label in `shell.rs`'s
   `confirmation_dialog` match and in both locale files.
3. Handle it in the `Action::Confirm` arm of `dispatch`.
4. Request it with `Action::RequestConfirmation(...)` from the page.
