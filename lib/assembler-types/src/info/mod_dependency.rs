use crate::info::ModVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModDependencyPrefix {
    Incompatible,
    Optional { hidden: bool },
    NoLoadOrderModification,
}

impl std::fmt::Display for ModDependencyPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModDependencyPrefix::Incompatible => "!",
                ModDependencyPrefix::Optional { hidden } => {
                    if *hidden { "(?)" } else { "?" }
                }
                ModDependencyPrefix::NoLoadOrderModification => "~",
            }
        )
    }
}

impl<'de> serde::Deserialize<'de> for ModDependencyPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "!" => Ok(ModDependencyPrefix::Incompatible),
            "~" => Ok(ModDependencyPrefix::NoLoadOrderModification),
            "?" => Ok(ModDependencyPrefix::Optional { hidden: false }),
            "(?)" => Ok(ModDependencyPrefix::Optional { hidden: true }),
            v => Err(serde::de::Error::custom(format!(
                "Invalid mod dependency prefix `{v}`!"
            ))),
        }
    }
}

impl serde::Serialize for ModDependencyPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModDependencyEqualityOperator {
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "<=")]
    LessEq,
    #[serde(rename = "=")]
    Eq,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    GreaterEq,
}

impl std::fmt::Display for ModDependencyEqualityOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Less => "<",
            Self::LessEq => "<=",
            Self::Eq => "=",
            Self::Greater => ">",
            Self::GreaterEq => ">=",
        };

        write!(f, "{s}")
    }
}

/// <prefix> mod-name <equality-operator> <version>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModDependency {
    pub prefix: Option<ModDependencyPrefix>,
    pub name: String,
    pub equality_operator: Option<ModDependencyEqualityOperator>,
    pub version: Option<ModVersion>,
}

impl std::fmt::Display for ModDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        if let Some(prefix) = &self.prefix {
            out.push_str(&prefix.to_string());
            out.push(' ');
        }

        out.push_str(&self.name);

        if let Some(op) = &self.equality_operator {
            out.push(' ');
            out.push_str(&op.to_string());
            out.push(' ');
        }

        if let Some(version) = &self.version {
            out.push_str(&version.to_string());
        }

        write!(f, "{out}")
    }
}

impl serde::Serialize for ModDependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for ModDependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim();

        let (prefix, rest) = if let Some(rest) = s.strip_prefix("(?)") {
            (Some(ModDependencyPrefix::Optional { hidden: true }), rest)
        } else if let Some(rest) = s.strip_prefix('?') {
            (Some(ModDependencyPrefix::Optional { hidden: false }), rest)
        } else if let Some(rest) = s.strip_prefix('!') {
            (Some(ModDependencyPrefix::Incompatible), rest)
        } else if let Some(rest) = s.strip_prefix('~') {
            (Some(ModDependencyPrefix::NoLoadOrderModification), rest)
        } else {
            (None, s)
        };

        let parts: Vec<_> = rest.split_whitespace().collect();

        match parts.as_slice() {
            [name] => Ok(Self {
                prefix,
                name: (*name).to_string(),
                equality_operator: None,
                version: None,
            }),

            [name, op, version] => {
                let equality_operator =
                    serde_json::from_str::<ModDependencyEqualityOperator>(&format!("\"{op}\""))
                        .map_err(serde::de::Error::custom)?;

                let version_parts: Vec<_> = version.split('.').collect();

                if version_parts.len() != 3 {
                    return Err(serde::de::Error::custom(
                        "version must be in major.middle.minor format",
                    ));
                }

                let major = version_parts[0].parse().map_err(serde::de::Error::custom)?;
                let middle = version_parts[1].parse().map_err(serde::de::Error::custom)?;
                let minor = version_parts[2].parse().map_err(serde::de::Error::custom)?;

                Ok(Self {
                    prefix,
                    name: (*name).to_string(),
                    equality_operator: Some(equality_operator),
                    version: Some(ModVersion {
                        major,
                        middle,
                        minor,
                    }),
                })
            }

            _ => Err(serde::de::Error::custom("invalid dependency format")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::info::{
        ModDependency, ModVersion,
        mod_dependency::{ModDependencyEqualityOperator, ModDependencyPrefix},
    };

    pub const PREFIXES: [(ModDependencyPrefix, &str); 4] = [
        (ModDependencyPrefix::Incompatible, "!"),
        (ModDependencyPrefix::NoLoadOrderModification, "~"),
        (ModDependencyPrefix::Optional { hidden: true }, "(?)"),
        (ModDependencyPrefix::Optional { hidden: false }, "?"),
    ];

    #[test]
    fn prefix_serialization() {
        for (prefix, expected) in PREFIXES {
            assert_eq!(prefix.to_string(), expected.to_string());
        }
    }

    #[test]
    fn prefix_deserialization() {
        for (prefix, prefix_str) in PREFIXES {
            assert_eq!(
                serde_json::from_str::<ModDependencyPrefix>(&format!("\"{prefix_str}\"")).unwrap(),
                prefix
            );
        }
    }

    const EQUALITY_OPS: [(ModDependencyEqualityOperator, &str); 5] = [
        (ModDependencyEqualityOperator::Eq, "="),
        (ModDependencyEqualityOperator::Less, "<"),
        (ModDependencyEqualityOperator::LessEq, "<="),
        (ModDependencyEqualityOperator::Greater, ">"),
        (ModDependencyEqualityOperator::GreaterEq, ">="),
    ];

    #[test]
    fn equality_serialization() {
        for (op, expected) in EQUALITY_OPS {
            assert_eq!(op.to_string(), expected);
        }
    }

    #[test]
    fn equality_deserialization() {
        for (eq_op, equality_str) in EQUALITY_OPS {
            assert_eq!(
                serde_json::from_str::<ModDependencyEqualityOperator>(&format!(
                    "\"{equality_str}\""
                ))
                .unwrap(),
                eq_op
            );
        }
    }

    #[test]
    fn full_deserialization() {
        let pairs = [
            (
                ModDependency {
                    prefix: None,
                    name: "mod-name".to_string(),
                    equality_operator: None,
                    version: None,
                },
                "mod-name",
            ),
            (
                ModDependency {
                    prefix: Some(ModDependencyPrefix::Incompatible),
                    name: "mod-name".to_string(),
                    equality_operator: Some(ModDependencyEqualityOperator::GreaterEq),
                    version: Some(ModVersion::from((2, 0, 0))),
                },
                "! mod-name >= 2.0.0",
            ),
            (
                ModDependency {
                    prefix: Some(ModDependencyPrefix::Optional { hidden: true }),
                    name: "hidden-opt-dep".to_string(),
                    equality_operator: Some(ModDependencyEqualityOperator::Eq),
                    version: Some(ModVersion::from((2, 0, 0))),
                },
                "(?) hidden-opt-dep = 2.0.0",
            ),
        ];

        for (dependency, dependency_str) in pairs {
            assert_eq!(dependency.to_string(), dependency_str);

            assert_eq!(
                serde_json::from_str::<ModDependency>(&format!("\"{dependency_str}\"")).unwrap(),
                dependency
            );
        }
    }
}
