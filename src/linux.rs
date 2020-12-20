use speech_dispatcher::*;

pub struct Linux {
    connection: Connection,
}

impl Linux {
    pub fn new() -> Self {
        let connection = Connection::open("tts", "tts", "tts", Mode::Single);
        Self { connection }
    }

    pub fn id(&self) -> Option<u64> {
        Some(self.connection.client_id())
    }

    pub fn speak(&self, text: &str) {
        self.connection.say(Priority::Text, text);
    }

    pub fn stop(&self) {
        self.connection.stop();
    }

    // Getters
    pub fn get_language(&self) -> &str {
        self.connection.get_language()
    }

    pub fn get_rate(&self) -> i32 {
        self.connection.get_voice_rate()
    }

    pub fn get_volume(&self) -> i32 {
        self.connection.get_volume()
    }

    // Setters
    pub fn set_language(&self, language: &str) {
        self.connection.set_language(language);
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.connection.set_voice_rate(rate as i32);
    }

    pub fn set_volume(&mut self, volume: i32) {
        self.connection.set_volume(volume);
    }
}
