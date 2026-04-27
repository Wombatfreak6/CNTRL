use tauri::State;
use uuid::Uuid;

use crate::services::browser::{BrowserService, Tab};

#[tauri::command]
pub fn open_tab(
    url: String,
    is_background: bool,
    browser_service: State<'_, BrowserService>,
) -> Result<Uuid, String> {
    browser_service
        .open_tab(url, is_background)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_tab(id: Uuid, browser_service: State<'_, BrowserService>) -> Result<(), String> {
    browser_service.close_tab(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn navigate(
    id: Uuid,
    url: String,
    browser_service: State<'_, BrowserService>,
) -> Result<(), String> {
    browser_service.navigate(id, url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tabs(browser_service: State<'_, BrowserService>) -> Result<Vec<Tab>, String> {
    browser_service.get_tabs().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_active_tab(id: Uuid, browser_service: State<'_, BrowserService>) -> Result<(), String> {
    browser_service
        .set_active_tab(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_fallback(url: String, app: tauri::AppHandle) -> Result<String, String> {
    crate::services::fallback::fetch_fallback_html(&app, &url)
        .await
        .map_err(|e| e.to_string())
}
