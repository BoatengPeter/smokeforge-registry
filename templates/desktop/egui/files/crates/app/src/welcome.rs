//! First-run welcome tour: which page is showing and how it ended.
//!
//! This module is state only. The window is drawn by `views/welcome.rs` with
//! the reusable [`crate::components::WizardWindow`]; [`crate::DesktopApp`]
//! holds `welcome: Option<WelcomePage>` and changes it through the
//! `ShowWelcome`, `WelcomeGo`, and `CloseWelcome` actions.
//!
//! To add a page: add a variant here (in display order) and to [`WelcomePage::ALL`],
//! add its `welcome.*` locale keys, and draw it in `views/welcome.rs`.

/// The pages of the welcome tour, in order.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WelcomePage {
    /// What the app is, plus appearance and size.
    #[default]
    Intro,
    /// The shortcuts worth knowing.
    Shortcuts,
    /// Where data lives and how to reopen the tour.
    Ready,
}

impl WelcomePage {
    pub const ALL: [Self; 3] = [Self::Intro, Self::Shortcuts, Self::Ready];

    /// Zero-based position in the tour.
    #[must_use]
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|page| *page == self)
            .unwrap_or_default()
    }

    #[must_use]
    pub fn next(self) -> Option<Self> {
        Self::ALL.get(self.index() + 1).copied()
    }

    #[must_use]
    pub fn previous(self) -> Option<Self> {
        self.index()
            .checked_sub(1)
            .and_then(|index| Self::ALL.get(index).copied())
    }
}

/// How the welcome tour was closed; logged so first-run drop-off is visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WelcomeOutcome {
    /// The user reached the last page and pressed its button.
    Finished,
    /// The user closed it early (×, Escape).
    Skipped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_step_in_order() {
        assert_eq!(WelcomePage::Intro.previous(), None);
        assert_eq!(WelcomePage::Intro.next(), Some(WelcomePage::Shortcuts));
        assert_eq!(WelcomePage::Shortcuts.previous(), Some(WelcomePage::Intro));
        assert_eq!(WelcomePage::Ready.next(), None);
        assert_eq!(WelcomePage::Ready.index(), WelcomePage::ALL.len() - 1);
    }

    #[test]
    fn all_lists_every_page_once() {
        for (index, page) in WelcomePage::ALL.iter().enumerate() {
            assert_eq!(page.index(), index);
        }
    }
}
