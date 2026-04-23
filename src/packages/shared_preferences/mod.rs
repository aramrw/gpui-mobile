//! Key-value persistent storage (NSUserDefaults / SharedPreferences).

#[cfg(target_os = "android")]
mod android;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod ios;

/// Cross-platform key-value persistent storage.
///
/// On iOS this wraps `NSUserDefaults.standardUserDefaults`.
/// On Android this wraps `PreferenceManager.getDefaultSharedPreferences()`.
pub struct SharedPreferences {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    inner: ios::IosSharedPreferences,
    #[cfg(target_os = "android")]
    inner: android::AndroidSharedPreferences,
    #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
    #[allow(dead_code)]
    inner: (),
}

impl SharedPreferences {
    /// Get the default shared preferences instance.
    pub fn instance() -> Self {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            Self {
                inner: ios::IosSharedPreferences::new(),
            }
        }
        #[cfg(target_os = "android")]
        {
            Self {
                inner: android::AndroidSharedPreferences::new(),
            }
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            Self { inner: () }
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.get_string(key)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.get_string(key)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = key;
            None
        }
    }

    pub fn set_string(&self, key: &str, value: &str) -> Result<(), String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.set_string(key, value)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.set_string(key, value)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = (key, value);
            Err("Not supported".into())
        }
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.get_int(key)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.get_int(key)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = key;
            None
        }
    }

    pub fn set_int(&self, key: &str, value: i64) -> Result<(), String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.set_int(key, value)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.set_int(key, value)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = (key, value);
            Err("Not supported".into())
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.get_bool(key)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.get_bool(key)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = key;
            None
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) -> Result<(), String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.set_bool(key, value)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.set_bool(key, value)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = (key, value);
            Err("Not supported".into())
        }
    }

    pub fn remove(&self, key: &str) -> Result<(), String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.remove(key)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.remove(key)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = key;
            Err("Not supported".into())
        }
    }

    pub fn clear(&self) -> Result<(), String> {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.clear()
        }
        #[cfg(target_os = "android")]
        {
            self.inner.clear()
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            Err("Not supported".into())
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            self.inner.contains_key(key)
        }
        #[cfg(target_os = "android")]
        {
            self.inner.contains_key(key)
        }
        #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "macos")))]
        {
            let _ = key;
            false
        }
    }
}
