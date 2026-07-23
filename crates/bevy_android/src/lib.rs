//! Provides Android functionality for Bevy Engine.

#![cfg(target_os = "android")]

pub use android_activity;

use std::{
    ops::Deref,
    sync::{RwLock, RwLockReadGuard},
};

use android_activity::AndroidApp;

/// Global storage for the current Android application handle.
pub static ANDROID_APP: AndroidAppStorage = AndroidAppStorage::new();

//==================================================================================================
// AndroidAppStorage
//==================================================================================================

/// Thread-safe storage for the Android application handle.
///
/// The handle can change when Android recreates the activity.
/// Calling [`set`] replaces the stored handle with the current instance.
pub struct AndroidAppStorage {
    app: RwLock<Option<AndroidApp>>,
}

impl AndroidAppStorage {
    /// Creates an empty Android app storage.
    pub const fn new() -> Self {
        Self {
            app: RwLock::new(None),
        }
    }

    /// Updates/Replaces the stored Android application handle.
    pub fn set(&self, app: AndroidApp) {
        *self.app.write().unwrap() = Some(app);
    }

    /// Returns a read guard for the current Android application handle.
    ///
    /// Returns `None` if the handle has not been initialized yet.
    pub fn get(&self) -> Option<AndroidAppGuard<'_>> {
        let guard = self.app.read().unwrap();

        if guard.is_some() {
            Some(AndroidAppGuard { guard })
        } else {
            None
        }
    }
}

//==================================================================================================
// AndroidAppGuard
//==================================================================================================

/// Read-only wrapper providing access to the current [`AndroidApp`].
///
/// Keeps the [`RwLock`] guard alive while preserving the existing API used by bevy code.
pub struct AndroidAppGuard<'a> {
    guard: RwLockReadGuard<'a, Option<AndroidApp>>,
}

impl<'a> Deref for AndroidAppGuard<'a> {
    type Target = AndroidApp;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().expect("AndroidApp not initialized")
    }
}
