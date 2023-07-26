#[derive(Debug, Clone, PartialEq)]
pub struct ChatConfig {
    pub model: Model,
    pub temperature: f64,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            model: Default::default(),
            temperature: 0.5,
        }
    }
}

impl ChatConfig {
    pub fn set_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }
    pub fn set_temprature(mut self, temprature: f64) -> Self {
        self.temperature = temprature;
        self
    }
    pub fn build(self) -> Self {
        self
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum Model {
    #[default]
    Gpt35Turbo,
    Gpt35Turbo_0301,
    Gpt4,
    Gpt4_32k,
    Gpt4_0314,
    Gpt4_32k_0314,
    Custom(&'static str),
}

impl AsRef<str> for Model {
    fn as_ref(&self) -> &'static str {
        match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo",
            Model::Gpt35Turbo_0301 => "gpt-3.5-turbo-0301",
            Model::Gpt4 => "gpt-4",
            Model::Gpt4_32k => "gpt-4-32k",
            Model::Gpt4_0314 => "gpt-4-0314",
            Model::Gpt4_32k_0314 => "gpt-4-32k-0314",
            Model::Custom(custom) => custom,
        }
    }
}
