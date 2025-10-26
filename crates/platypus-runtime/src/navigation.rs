//! Multi-page app navigation support
//! Provides st.navigation() and page routing

use std::collections::HashMap;

/// Page definition for multi-page apps
#[derive(Clone, Debug)]
pub struct Page {
    /// Page name/identifier
    pub name: String,
    /// Page title for display
    pub title: String,
    /// Page icon (optional)
    pub icon: Option<String>,
    /// Page description
    pub description: Option<String>,
}

impl Page {
    /// Create a new page
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Page {
            name: name.into(),
            title: title.into(),
            icon: None,
            description: None,
        }
    }

    /// Set page icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set page description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Navigation configuration for multi-page apps
#[derive(Clone, Debug)]
pub struct Navigation {
    pages: HashMap<String, Page>,
    current_page: String,
}

impl Navigation {
    /// Create a new navigation
    pub fn new() -> Self {
        Navigation {
            pages: HashMap::new(),
            current_page: String::new(),
        }
    }

    /// Add a page to navigation
    pub fn add_page(&mut self, page: Page) {
        if self.current_page.is_empty() {
            self.current_page = page.name.clone();
        }
        self.pages.insert(page.name.clone(), page);
    }

    /// Get all pages
    pub fn pages(&self) -> Vec<Page> {
        self.pages.values().cloned().collect()
    }

    /// Get current page
    pub fn current_page(&self) -> Option<Page> {
        self.pages.get(&self.current_page).cloned()
    }

    /// Navigate to a page
    pub fn navigate_to(&mut self, page_name: &str) -> bool {
        if self.pages.contains_key(page_name) {
            self.current_page = page_name.to_string();
            true
        } else {
            false
        }
    }

    /// Get page by name
    pub fn get_page(&self, name: &str) -> Option<Page> {
        self.pages.get(name).cloned()
    }

    /// Get page count
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

impl Default for Navigation {
    fn default() -> Self {
        Self::new()
    }
}

/// Page link for navigation
#[derive(Clone, Debug)]
pub struct PageLink {
    pub label: String,
    pub page: String,
    pub icon: Option<String>,
}

impl PageLink {
    /// Create a new page link
    pub fn new(label: impl Into<String>, page: impl Into<String>) -> Self {
        PageLink {
            label: label.into(),
            page: page.into(),
            icon: None,
        }
    }

    /// Set link icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Multi-page app builder
pub struct MultiPageApp {
    navigation: Navigation,
    pages: HashMap<String, String>, // page_name -> page_content
}

impl MultiPageApp {
    /// Create a new multi-page app
    pub fn new() -> Self {
        MultiPageApp {
            navigation: Navigation::new(),
            pages: HashMap::new(),
        }
    }

    /// Add a page
    pub fn add_page(&mut self, page: Page, content: String) {
        self.navigation.add_page(page.clone());
        self.pages.insert(page.name, content);
    }

    /// Get navigation
    pub fn navigation(&self) -> &Navigation {
        &self.navigation
    }

    /// Get mutable navigation
    pub fn navigation_mut(&mut self) -> &mut Navigation {
        &mut self.navigation
    }

    /// Get page content
    pub fn get_page_content(&self, page_name: &str) -> Option<String> {
        self.pages.get(page_name).cloned()
    }

    /// Get current page content
    pub fn current_page_content(&self) -> Option<String> {
        let current = self.navigation.current_page()?;
        self.pages.get(&current.name).cloned()
    }

    /// Navigate to page
    pub fn navigate(&mut self, page_name: &str) -> bool {
        self.navigation.navigate_to(page_name)
    }
}

impl Default for MultiPageApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new("home", "Home Page")
            .with_icon("🏠")
            .with_description("Welcome to home");
        
        assert_eq!(page.name, "home");
        assert_eq!(page.title, "Home Page");
        assert_eq!(page.icon, Some("🏠".to_string()));
    }

    #[test]
    fn test_navigation() {
        let mut nav = Navigation::new();
        
        nav.add_page(Page::new("home", "Home"));
        nav.add_page(Page::new("about", "About"));
        
        assert_eq!(nav.page_count(), 2);
        assert_eq!(nav.current_page().unwrap().name, "home");
        
        nav.navigate_to("about");
        assert_eq!(nav.current_page().unwrap().name, "about");
    }

    #[test]
    fn test_page_link() {
        let link = PageLink::new("Go Home", "home").with_icon("🏠");
        
        assert_eq!(link.label, "Go Home");
        assert_eq!(link.page, "home");
        assert_eq!(link.icon, Some("🏠".to_string()));
    }

    #[test]
    fn test_multi_page_app() {
        let mut app = MultiPageApp::new();
        
        app.add_page(
            Page::new("home", "Home"),
            "Home content".to_string(),
        );
        app.add_page(
            Page::new("about", "About"),
            "About content".to_string(),
        );
        
        assert_eq!(app.current_page_content(), Some("Home content".to_string()));
        
        app.navigate("about");
        assert_eq!(app.current_page_content(), Some("About content".to_string()));
    }
}
