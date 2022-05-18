use rand::{distributions::Alphanumeric, Rng};

pub struct TestNewPunch {
    pub setup: String,
    pub punchline: String,
}

impl Default for TestNewPunch {
    fn default() -> Self {
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        Self {
            setup: format!("Как каннибал называет Пашу? {}", salt),
            punchline: "Паштет".to_string(),
        }
    }
}
