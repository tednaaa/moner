use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use rand::{rngs::OsRng, Rng};

#[derive(Clone)]
pub struct VerificationService {
	codes: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
}

impl VerificationService {
	pub fn new() -> Self {
		let codes = Arc::new(Mutex::new(HashMap::new()));

		Self { codes }
	}

	pub fn generate_code(&mut self, user_id: &str) -> String {
		let mut rng = OsRng;
		let verification_code: u32 = rng.gen_range(100_000..1_000_000);

		let mut codes = self.codes.lock().unwrap();
		codes.insert(user_id.to_string(), (verification_code.to_string(), SystemTime::now()));

		verification_code.to_string()
	}

	pub fn verify_code(&mut self, user_id: &str, code: &str) -> bool {
		let mut codes = self.codes.lock().unwrap();
		if let Some((stored_code, _)) = codes.get(user_id) {
			if stored_code == code {
				codes.remove(user_id);
				return true;
			}
		}
		false
	}

	pub fn cleanup_expired_codes(&mut self) {
		let mut codes = self.codes.lock().unwrap().clone();

		let now = SystemTime::now();
		codes.retain(|_, (_, timestamp)| {
			now.duration_since(*timestamp).unwrap_or(Duration::from_secs(0)) < Duration::from_secs(300)
		});
	}
}
