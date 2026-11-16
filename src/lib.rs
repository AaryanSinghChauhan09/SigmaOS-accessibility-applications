// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-accessibility-applications: accessibility applications component model.
//!
//! Real implementation: applicationss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applications {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityApplicationsError {
    EmptyApplicationsName,
    DuplicateApplications(String),
    EmptyEventSubject,
}

impl fmt::Display for AccessibilityApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyApplicationsName => write!(f, "applications name must not be empty"),
            Self::DuplicateApplications(id) => write!(f, "duplicate applications {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AccessibilityApplicationsError {}

pub struct AccessibilityApplications {
    pub name: String,
    applicationss: BTreeMap<String, Applications>,
    events: BTreeMap<String, Event>,
}

impl AccessibilityApplications {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            applicationss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_applications(&mut self, item: Applications) -> Result<(), AccessibilityApplicationsError> {
        if item.name.trim().is_empty() {
            return Err(AccessibilityApplicationsError::EmptyApplicationsName);
        }
        let id = item.id.clone();
        if self.applicationss.contains_key(&id) {
            return Err(AccessibilityApplicationsError::DuplicateApplications(id.clone()));
        }
        self.applicationss.insert(id, item);
        Ok(())
    }

    pub fn applications_count(&self) -> usize {
        self.applicationss.len()
    }

    pub fn get_applications(&self, id: &str) -> Option<&Applications> {
        self.applicationss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AccessibilityApplicationsError> {
        if event.subject.trim().is_empty() {
            return Err(AccessibilityApplicationsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_applications(id: &str, name: &str) -> Applications {
        Applications {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AccessibilityApplications::new("AccessibilityApplications");
        assert_eq!(m.name, "AccessibilityApplications");
        assert_eq!(m.applications_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_applications() {
        let mut m = AccessibilityApplications::new("AccessibilityApplications");
        m.add_applications(sample_applications("a1", "Alpha")).unwrap();
        assert_eq!(m.applications_count(), 1);
        assert!(m.get_applications("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AccessibilityApplications::new("AccessibilityApplications");
        assert!(m.add_applications(sample_applications("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AccessibilityApplications::new("AccessibilityApplications");
        m.add_applications(sample_applications("x", "One")).unwrap();
        assert!(m.add_applications(sample_applications("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AccessibilityApplications::new("AccessibilityApplications");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
