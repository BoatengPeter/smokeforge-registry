# App states

Every screen must handle more than the happy path. Use these patterns instead
of inventing new ones; add a new pattern here before using it.

## Which pattern to use

| Situation | Pattern | Component | Example in the app |
| --- | --- | --- | --- |
| An area cannot show its content | Status page | `crates/ui` `StatusPage` | database cannot be opened (`pages/states.rs`) |
| A list or area is legitimately empty | Empty state | `crates/ui` `EmptyState` (wraps GPUI Kit `Empty`) | no records yet, no background jobs |
| The page works but something is degraded | Alert | GPUI Kit `alert::Alert` (`warning`, `error`, `info`) | keyring unavailable (Account) |
| Work is in progress | Status text or `spinner::Spinner` | GPUI Kit | sign-in, sync |
| A destructive action without undo | Alert dialog | `window.open_alert_dialog` | delete record (Library) |
| A field is invalid | Inline error near the field | `Input` validation / muted text | sign-in form |

Decision rules:

- **Status page vs alert.** If the user can still do the page's main task, use
  an `Alert`. If not, replace the area with a `StatusPage`.
- **Empty vs error.** "No records" is empty; "could not read records" is an
  error. Never show an empty state when a read failed.
- **Confirm vs undo.** Reversible actions do not ask; irreversible ones use an
  alert dialog with `ok_variant(ButtonVariant::Danger)`.
- **Copy names the object and the verb.** Title `Delete "Roadmap"?`, button
  `Delete record`, never `Are you sure?` / `OK`.
- **Details are for reports.** Put the raw error in `StatusPage::details`, not
  in the title or body.

## The states in this app

- **Database unavailable.** `AppView::new` records `database_error` when the
  database cannot open. Dashboard, Library, and Activity render
  `database_error_page` with **Open logs folder** and **Try again**
  (`reopen_database`).
- **Empty library / no jobs.** `EmptyState` with the next useful action.
- **Keyring unavailable.** `Alert::warning` on the Account page.
- **Delete record.** The Library row's delete button opens an alert dialog;
  the record is deleted only from `on_ok`.

## Adding a new state

1. Pick the pattern above; if none fits, add a row here first.
2. Model it in data on `AppView` (an `Option<String>` error, an enum variant),
   not in render-local variables.
3. Make the read path return `Result` so the page can tell empty from failed.
4. Add strings to `crates/app/locales/en.yml.tmpl` and `es.yml.tmpl` under
   `state.*`.
5. Render it from `pages/states.rs` (shared states) or the page itself.
6. Add a UI integration test (`#[gpui_kit::test]`) that sets the state and
   checks the recovery action, as described in the gpui-kit skill's testing
   reference.
