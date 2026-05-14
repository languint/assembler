use crate::info::ModVersion;
use std::str::FromStr;

/// Exactly 99 `-` chars
pub const CHANGELOG_LINE_SEPARATOR: &str = "---------------------------------------------------------------------------------------------------";
/// 4 spaces, then `-`, then 1 space
pub const ENTRY_PREFIX: &str = "    - ";
/// 6 spaces
pub const MULTILINE_ENTRY_PREFIX: &str = "      ";

/// 2 spaces
pub const CATEGORY_PREFIX: &str = "  ";
pub const CATEGORY_SUFFIX: &str = ":";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ChangelogCategory {
    Named(KnownChangelogCategory),
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KnownChangelogCategory {
    #[serde(rename = "Major Features")]
    MajorFeatures,
    #[serde(rename = "Features")]
    Features,
    #[serde(rename = "Minor Features")]
    MinorFeatures,
    #[serde(rename = "Graphics")]
    Graphics,
    #[serde(rename = "Sounds")]
    Sounds,
    #[serde(rename = "Optimizations")]
    Optimizations,
    #[serde(rename = "Balancing")]
    Balancing,
    #[serde(rename = "Combat Balancing")]
    CombatBalancing,
    #[serde(rename = "Circuit Network")]
    CircuitNetwork,
    #[serde(rename = "Changes")]
    Changes,
    #[serde(rename = "Bugfixes")]
    Bugfixes,
    #[serde(rename = "Modding")]
    Modding,
    #[serde(rename = "Scripting")]
    Scripting,
    #[serde(rename = "Gui")]
    Gui,
    #[serde(rename = "Control")]
    Control,
    #[serde(rename = "Translation")]
    Translation,
    #[serde(rename = "Debug")]
    Debug,
    #[serde(rename = "Ease of use")]
    EaseOfUse,
    #[serde(rename = "Info")]
    Info,
    #[serde(rename = "Locale")]
    Locale,
    #[serde(rename = "Compatibility")]
    Compatibility,
}

impl std::fmt::Display for ChangelogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogCategory::Named(category) => {
                let text = serde_json::to_string(category)
                    .expect("Known categories should always serialize");

                write!(f, "{}", text.trim_matches('"'))
            }
            ChangelogCategory::Custom(value) => write!(f, "{value}"),
        }
    }
}

impl FromStr for ChangelogCategory {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category = match s {
            "Major Features" => Self::Named(KnownChangelogCategory::MajorFeatures),
            "Features" => Self::Named(KnownChangelogCategory::Features),
            "Minor Features" => Self::Named(KnownChangelogCategory::MinorFeatures),
            "Graphics" => Self::Named(KnownChangelogCategory::Graphics),
            "Sounds" => Self::Named(KnownChangelogCategory::Sounds),
            "Optimizations" => Self::Named(KnownChangelogCategory::Optimizations),
            "Balancing" => Self::Named(KnownChangelogCategory::Balancing),
            "Combat Balancing" => Self::Named(KnownChangelogCategory::CombatBalancing),
            "Circuit Network" => Self::Named(KnownChangelogCategory::CircuitNetwork),
            "Changes" => Self::Named(KnownChangelogCategory::Changes),
            "Bugfixes" => Self::Named(KnownChangelogCategory::Bugfixes),
            "Modding" => Self::Named(KnownChangelogCategory::Modding),
            "Scripting" => Self::Named(KnownChangelogCategory::Scripting),
            "Gui" => Self::Named(KnownChangelogCategory::Gui),
            "Control" => Self::Named(KnownChangelogCategory::Control),
            "Translation" => Self::Named(KnownChangelogCategory::Translation),
            "Debug" => Self::Named(KnownChangelogCategory::Debug),
            "Ease of use" => Self::Named(KnownChangelogCategory::EaseOfUse),
            "Info" => Self::Named(KnownChangelogCategory::Info),
            "Locale" => Self::Named(KnownChangelogCategory::Locale),
            "Compatibility" => Self::Named(KnownChangelogCategory::Compatibility),
            other => Self::Custom(other.to_string()),
        };

        Ok(category)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogToken {
    Version(ModVersion),
    Date(String),
    Entry(Vec<String>),
    Category(ChangelogCategory),
    Separator,
}

pub type Changelog = Vec<ChangelogToken>;

pub struct ChangelogRenderer {
    changelog: Changelog,
}

impl ChangelogRenderer {
    #[must_use]
    pub fn new(changelog: Changelog) -> Self {
        Self { changelog }
    }
}

impl ChangelogRenderer {
    #[must_use]
    pub fn render_token(token: &ChangelogToken) -> String {
        match token {
            ChangelogToken::Version(v) => Self::render_version(v),
            ChangelogToken::Date(d) => Self::render_date(d),
            ChangelogToken::Category(c) => Self::render_category(c),
            ChangelogToken::Entry(l) => Self::render_entry(l),
            ChangelogToken::Separator => Self::render_line_separator(),
        }
    }

    fn render_version(version: &ModVersion) -> String {
        format!("Version: {version}\n")
    }

    fn render_date(date: &String) -> String {
        format!("Date: {date}\n")
    }

    fn render_category(category: &ChangelogCategory) -> String {
        format!("{CATEGORY_PREFIX}{category}{CATEGORY_SUFFIX}\n")
    }

    fn render_entry(lines: &[String]) -> String {
        let mut output = String::new();

        for (index, line) in lines.iter().enumerate() {
            let prefix_str = if index == 0 {
                ENTRY_PREFIX
            } else {
                MULTILINE_ENTRY_PREFIX
            };

            output = format!("{output}{prefix_str}{line}\n");
        }

        output
    }

    fn render_line_separator() -> String {
        format!("{CHANGELOG_LINE_SEPARATOR}\n")
    }

    #[must_use]
    pub fn render(&self) -> String {
        let mut output = String::new();

        for token in &self.changelog {
            output.push_str(&Self::render_token(token));
        }

        output
    }
}

#[derive(Debug)]
pub enum ChangelogParseError {
    InvalidVersion(String),
    InvalidLine(String),
    MultilineEntryWithoutEntry,
}

pub struct ChangelogParser;

impl ChangelogParser {
    /// Parse a changelog
    ///
    /// # Errors
    /// Returns `Err` if the parsing of the changelog fails
    ///
    /// # Panics
    /// This function *can* panic, but never should
    pub fn parse(input: &str) -> Result<Changelog, ChangelogParseError> {
        let mut tokens = Vec::new();
        let mut current_entry: Option<Vec<String>> = None;

        for raw_line in input.lines() {
            let line = raw_line.trim_end();

            if line.trim().is_empty() {
                continue;
            }

            if line == CHANGELOG_LINE_SEPARATOR {
                Self::flush_entry(&mut current_entry, &mut tokens);
                tokens.push(ChangelogToken::Separator);
                continue;
            }

            if let Some(version) = line.strip_prefix("Version: ") {
                Self::flush_entry(&mut current_entry, &mut tokens);

                let version = Self::parse_version(version)
                    .map_err(|()| ChangelogParseError::InvalidVersion(version.to_string()))?;

                tokens.push(ChangelogToken::Version(version));
                continue;
            }

            if let Some(date) = line.strip_prefix("Date: ") {
                Self::flush_entry(&mut current_entry, &mut tokens);
                tokens.push(ChangelogToken::Date(date.to_string()));
                continue;
            }

            if line.starts_with(CATEGORY_PREFIX)
                && line.ends_with(CATEGORY_SUFFIX)
                && !line.starts_with(ENTRY_PREFIX)
            {
                Self::flush_entry(&mut current_entry, &mut tokens);

                let category = line
                    .trim_start_matches(CATEGORY_PREFIX)
                    .trim_end_matches(CATEGORY_SUFFIX)
                    .trim();

                tokens.push(ChangelogToken::Category(
                    category.parse().expect("This should not fail here"),
                ));

                continue;
            }

            if let Some(entry) = line.strip_prefix(ENTRY_PREFIX) {
                Self::flush_entry(&mut current_entry, &mut tokens);
                current_entry = Some(vec![entry.to_string()]);
                continue;
            }

            if let Some(extra_line) = line.strip_prefix(MULTILINE_ENTRY_PREFIX) {
                match current_entry.as_mut() {
                    Some(entry) => entry.push(extra_line.to_string()),
                    None => return Err(ChangelogParseError::MultilineEntryWithoutEntry),
                }

                continue;
            }

            return Err(ChangelogParseError::InvalidLine(line.to_string()));
        }

        Self::flush_entry(&mut current_entry, &mut tokens);

        Ok(tokens)
    }

    fn flush_entry(current_entry: &mut Option<Vec<String>>, tokens: &mut Changelog) {
        if let Some(entry) = current_entry.take() {
            tokens.push(ChangelogToken::Entry(entry));
        }
    }

    fn parse_version(version: &str) -> Result<ModVersion, ()> {
        let mut split = version.split('.');

        let major = split.next().ok_or(())?.trim().parse().map_err(|_| ())?;
        let middle = split.next().ok_or(())?.trim().parse().map_err(|_| ())?;
        let minor = split.next().ok_or(())?.trim().parse().map_err(|_| ())?;

        Ok(ModVersion {
            major,
            middle,
            minor,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::changelog::{
        ChangelogCategory, ChangelogParser, ChangelogRenderer, ChangelogToken,
        KnownChangelogCategory,
    };

    #[test]
    pub fn render_entry() {
        let entry = ChangelogToken::Entry(vec![
            "Fixed bugs".to_string(),
            "Fixed more bugs".to_string(),
        ]);

        let renderer = ChangelogRenderer::new(vec![entry]);

        assert_eq!(
            renderer.render(),
            "    - Fixed bugs\n      Fixed more bugs\n".to_string()
        );
    }

    #[test]
    pub fn parse_custom_category() {
        let text = r#"
---------------------------------------------------------------------------------------------------
Version: 1.0.0
Date: 01. 01. 2025
  My Custom Category:
    - Hello world
"#;

        let parsed = ChangelogParser::parse(text).unwrap();

        assert_eq!(
            parsed[3],
            ChangelogToken::Category(ChangelogCategory::Custom("My Custom Category".to_string()))
        );
    }

    #[test]
    pub fn full_text() {
        let text = r#"---------------------------------------------------------------------------------------------------
Version: 1.1.60
Date: 06. 06. 2022
  Features:
    - This is an entry in the "Features" category.
    - This is another entry in the "Features" category.
    - This general section is the 1.1.60 version section.
  Balancing:
    - This is a multiline entry in the "Balancing" category.
      There is some extra text here because it is needed for the example.
      Lorem ipsum dolor sit amet, consectetur adipiscing elit.
  Bugfixes:
    - Fixed that canceling syncing mods with a save would exit the GUI.
    - Fixed a desync when fast-replacing burner generators.
---------------------------------------------------------------------------------------------------
Version: 1.1.59
Date: 06. 05. 2022
  Bugfixes:
    - This general section is the 1.1.59 version section.
    - This is an entry in the "Bugfixes" category.
    - Fixed grenade shadows.
"#;

        let parsed = ChangelogParser::parse(text).unwrap();

        assert!(!parsed.is_empty());

        assert!(matches!(
            parsed[3],
            ChangelogToken::Category(ChangelogCategory::Named(KnownChangelogCategory::Features))
        ));

        let rendered = ChangelogRenderer::new(parsed).render();

        assert_eq!(rendered, text);

        // assert!(rendered.contains("Version: 1.1.60"));
        // assert!(rendered.contains("Fixed grenade shadows."));
    }
}
