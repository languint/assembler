/// The version of the mod consisting of a major, middle, and minor part (0.0.0)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModVersion {
    pub major: u16,
    pub middle: u16,
    pub minor: u16,
}

impl From<(u16, u16, u16)> for ModVersion {
    fn from(value: (u16, u16, u16)) -> Self {
        ModVersion {
            major: value.0,
            middle: value.1,
            minor: value.2,
        }
    }
}

impl std::fmt::Display for ModVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.middle, self.minor)
    }
}

impl serde::Serialize for ModVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for ModVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let mut parts = s.split('.');

        let major = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing major version"))?
            .parse::<u16>()
            .map_err(serde::de::Error::custom)?;

        let middle = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing middle version"))?
            .parse::<u16>()
            .map_err(serde::de::Error::custom)?;

        let minor = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing minor version"))?
            .parse::<u16>()
            .map_err(serde::de::Error::custom)?;

        if parts.next().is_some() {
            return Err(serde::de::Error::custom("too many version components"));
        }

        Ok(ModVersion {
            major,
            middle,
            minor,
        })
    }
}
