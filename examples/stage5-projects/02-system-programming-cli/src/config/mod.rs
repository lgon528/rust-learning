#[derive(Debug, Clone)]
pub struct Config {
    pub general: GeneralConfig,
}

#[derive(Debug, Clone)]
pub struct GeneralConfig {
    pub log_level: String,
    pub parallel_workers: usize,
    pub show_progress: bool,
    pub color_output: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                log_level: "info".to_string(),
                parallel_workers: 4,
                show_progress: true,
                color_output: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Simplified: just return default for now
        Ok(Config::default())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Simplified: just print for now
        println!("Saving configuration (simplified implementation)");
        Ok(())
    }

    pub fn update_log_level(&mut self, level: &str) {
        self.general.log_level = level.to_string();
    }
}
