use anyhow::Result;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub enum Version {
    #[default]
    V1,
}
impl AsRef<str> for Version {
    fn as_ref(&self) -> &'static str {
        match self {
            Version::V1 => "https://api.openai.com/v1/",
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub enum Function {
    #[default]
    CreateChatCompletion,
    CreateImage,
}

impl AsRef<str> for Function {
    fn as_ref(&self) -> &'static str {
        match self {
            Function::CreateChatCompletion => "chat/completions",
            Function::CreateImage => "images/generations",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct EntryPoint {
    pub function: Function,
    pub version: Version,
}
impl Default for EntryPoint {
    fn default() -> Self {
        Self {
            function: Default::default(),
            version: Default::default(),
        }
    }
}
impl EntryPoint {
    pub fn set_function(mut self, function: Function) -> Self {
        self.function = function;
        self
    }
    pub fn set_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }
    pub fn path(&self) -> String {
        let base_path = self.version.as_ref().to_string();
        let function_path = self.function.as_ref().to_string();
        format!("{base_path}{function_path}").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_point() -> Result<()> {
        let entry_point = EntryPoint::default();
        assert_eq!(
            entry_point.path().as_str(),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            entry_point
                .set_function(Function::CreateImage)
                .path()
                .as_str(),
            "https://api.openai.com/v1/images/generations"
        );

        Ok(())
    }
}
