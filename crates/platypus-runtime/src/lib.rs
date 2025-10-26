//! Webag Runtime - App execution and state management.
//!
//! This crate provides the runtime engine for executing Webag applications,
//! managing state, handling events, and generating UI deltas.

pub mod cache;
pub mod components;
pub mod context;
pub mod error;
pub mod event;
pub mod navigation;
pub mod secrets;
pub mod session_store;

pub use cache::{CacheManager, DataCache, ResourceCache};
pub use components::{ComponentInstance, ComponentMetadata, ComponentProperty, ComponentRegistry, CustomComponent};
pub use context::St;
pub use error::{Error, Result};
pub use event::Event;
pub use navigation::{MultiPageApp, Navigation, Page, PageLink};
pub use secrets::{Secret, SecretSource, SecretsManager, Secrets};
pub use session_store::SessionStore;

pub mod prelude {
    pub use crate::{
        cache::{CacheManager, DataCache, ResourceCache},
        components::{ComponentInstance, ComponentMetadata, ComponentProperty, ComponentRegistry, CustomComponent},
        context::St,
        error::Result,
        navigation::{MultiPageApp, Navigation, Page, PageLink},
        secrets::{Secret, SecretSource, SecretsManager, Secrets},
        session_store::SessionStore,
    };
}
