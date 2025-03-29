//! Session management utilities.
//! This module provides functionality for managing user session data, particularly for handling browser tabs.

use actix_session::Session;
use serde::{Deserialize, Serialize};

/// Represents a browser tab in the user's session
/// 
/// This struct stores information about an open tab in the user interface:
/// - `name`: Display name of the tab
/// - `id`: Optional ID of the item being viewed in the tab
/// - `type`: Type of content being displayed (e.g., "collection", "request")
/// - `collection_id`: ID of the collection the tab belongs to
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tab {
    /// Display name of the tab
    pub name: String,
    /// Optional ID of the item being viewed
    pub id: Option<i32>,
    /// Type of content being displayed
    pub r#type: String,
    /// ID of the collection the tab belongs to
    pub collection_id: i32,
}

/// Retrieves the list of open tabs from the user's session
/// 
/// This function:
/// 1. Attempts to get the tabs from the session
/// 2. Returns an empty vector if no tabs are found
/// 
/// # Arguments
/// * `session` - The user's session
/// 
/// # Returns
/// * `Vec<Tab>` - List of open tabs, or empty vector if none found
pub async fn get_session_tabs(session: Session) -> Vec<Tab> {
    if let Some(current) = session.get::<Vec<Tab>>("tabs").unwrap() {
        return current;
    } else {
        return vec![];
    };
}
