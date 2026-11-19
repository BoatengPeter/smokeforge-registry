//! OS keyring-backed credentials with a memory-only fallback.

use keyring::Entry;

pub struct TokenStore {
    entry: Option<Entry>,
    memory: Option<String>,
    warning: Option<String>,
}

impl TokenStore {
    #[must_use]
    pub fn new(service: &str, account: &str) -> Self {
        match Entry::new(service, account) {
            Ok(entry) => Self {
                entry: Some(entry),
                memory: None,
                warning: None,
            },
            Err(error) => Self {
                entry: None,
                memory: None,
                warning: Some(format!("OS keyring unavailable: {error}")),
            },
        }
    }

    #[must_use]
    pub const fn memory_only() -> Self {
        Self {
            entry: None,
            memory: None,
            warning: None,
        }
    }

    #[must_use]
    pub fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    /// Stores a token in the keyring, falling back to memory.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the keyring error is retained as a warning.
    pub fn set(&mut self, token: &str) -> Result<(), String> {
        if let Some(entry) = &self.entry
            && let Err(error) = entry.set_password(token)
        {
            self.entry = None;
            self.warning = Some(format!("OS keyring write failed: {error}"));
        }

        self.memory = Some(token.to_owned());
        Ok(())
    }

    #[must_use]
    pub fn get(&self) -> Option<String> {
        if let Some(token) = &self.memory {
            return Some(token.clone());
        }

        self.entry
            .as_ref()
            .and_then(|entry| entry.get_password().ok())
    }

    /// Deletes the stored token.
    ///
    /// # Errors
    ///
    /// Returns the keyring error if the entry cannot be deleted.
    pub fn delete(&mut self) -> Result<(), String> {
        self.memory = None;
        if let Some(entry) = &self.entry {
            entry
                .delete_credential()
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_fallback_round_trips_and_deletes() {
        let mut store = TokenStore::memory_only();
        store.set("secret-token").unwrap();
        assert_eq!(store.get().as_deref(), Some("secret-token"));
        store.delete().unwrap();
        assert!(store.get().is_none());
    }
}
