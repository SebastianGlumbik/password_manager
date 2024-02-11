use std::collections::HashMap;
use std::sync::Mutex;
use totp_rs::{Rfc6238, TOTP};

/// TOTP manager for tauri state. Used for managing TOTP secrets and generating codes.
pub struct TOTPManager {
    hash_map: Mutex<HashMap<u64, TOTP>>,
}

impl TOTPManager {
    /// Creates a new TOTP manager with the given size. Size is used for pre-allocating the hashmap to avoid re-allocations.
    pub fn new(size: usize) -> Self {
        TOTPManager {
            hash_map: Mutex::new(HashMap::with_capacity(size)),
        }
    }
    /// Adds a new TOTP secret to the manager. It takes a constant id and a totp secret
    /// # Errors
    /// Returns an error if the secret is invalid or if the manager mutex is poisoned.
    pub fn add_secret(&self, id: u64, secret: String) -> Result<(), &'static str> {
        let mut guard = self
            .hash_map
            .lock()
            .map_err(|_| "Failed to access manager lock")?;

        if guard.capacity() == guard.len() {
            return Err("TOTP Manager is full");
        }

        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes() else {
            return Err("Invalid OTP Secret");
        };
        let Ok(rfc6238) = Rfc6238::with_defaults(secret) else {
            return Err("Invalid OTP Secret");
        };
        let Ok(totp) = TOTP::from_rfc6238(rfc6238) else {
            return Err("Invalid OTP Secret");
        };

        guard.insert(id, totp);
        Ok(())
    }

    /// Generates a TOTP code for the given secret.
    /// # Return
    /// Returns the current TOTP code and the time to live in seconds or None if the secret does not exist or if the manager mutex is poisoned.
    pub fn get_code(&self, id: &u64) -> Option<(String, u64)> {
        let mut guard = self.hash_map.lock().ok()?;
        let totp = guard.get_mut(id)?;
        let current = totp.generate_current().ok()?;
        let ttl = totp.ttl().ok()?;
        Some((current, ttl))
    }

    /// Removes a TOTP secrets from the manager.
    pub fn reset(&self) {
        if let Ok(mut guard) = self.hash_map.lock() {
            guard.clear();
        }
    }
}
