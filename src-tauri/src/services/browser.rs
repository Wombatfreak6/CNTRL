use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::error::VibError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub is_background: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Default)]
pub struct BrowserState {
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<Uuid>,
}

#[derive(Clone)]
pub struct BrowserService {
    state: Arc<RwLock<BrowserState>>,
}

impl Default for BrowserService {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(BrowserState::default())),
        }
    }

    pub fn open_tab(&self, url: String, is_background: bool) -> Result<Uuid, VibError> {
        let id = Uuid::new_v4();
        let tab = Tab {
            id,
            url,
            title: "New Tab".to_string(),
            is_background,
            created_at: Utc::now(),
        };

        let mut state = self.state.write();
        state.tabs.push(tab);

        if !is_background {
            state.active_tab_id = Some(id);
        }

        Ok(id)
    }

    pub fn close_tab(&self, id: Uuid) -> Result<(), VibError> {
        let mut state = self.state.write();
        state.tabs.retain(|t| t.id != id);

        if state.active_tab_id == Some(id) {
            state.active_tab_id = state.tabs.last().map(|t| t.id);
        }

        Ok(())
    }

    pub fn navigate(&self, id: Uuid, url: String) -> Result<(), VibError> {
        let mut state = self.state.write();
        if let Some(tab) = state.tabs.iter_mut().find(|t| t.id == id) {
            tab.url = url;
            Ok(())
        } else {
            Err(VibError::Browser(format!("Tab {} not found", id)))
        }
    }

    pub fn get_tabs(&self) -> Result<Vec<Tab>, VibError> {
        let state = self.state.read();
        Ok(state.tabs.clone())
    }

    pub fn set_active_tab(&self, id: Uuid) -> Result<(), VibError> {
        let mut state = self.state.write();
        if state.tabs.iter().any(|t| t.id == id) {
            state.active_tab_id = Some(id);
            Ok(())
        } else {
            Err(VibError::Browser(format!("Tab {} not found", id)))
        }
    }
}
