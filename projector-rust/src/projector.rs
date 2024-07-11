use core::str;
use std::{collections::HashMap, path::PathBuf};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    pwd: PathBuf,
    config: PathBuf,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut cur = Some(self.pwd.as_path());
        let mut out = None;

        while let Some(p) = cur {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            cur = p.parent();
        }
        return out;
    }

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut paths = vec![];
        let mut cur = Some(self.pwd.as_path());

        while let Some(p) = cur {
            paths.push(p);
            cur = p.parent();
        }

        let mut out = HashMap::new();

        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }
        return out;
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector.get_mut(&self.pwd).map(|map| {
            map.remove(key);
        });
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(&p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }

        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;

        return Ok(());
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or("{\"projector\" : {}}".to_string());

            let data = serde_json::from_str(&contents);

            let data = data.unwrap_or(default_data());
            return Projector { config, pwd, data };
        }
        return Projector {
            config,
            pwd,
            data: default_data(),
        };
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use crate::config::{Config, Operation};

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
        PathBuf::from("/") => hashmap! {
                "foo" .into() => "bar1".into(),
                "fem".into() => "is_great".into(),
            },
        PathBuf::from("/foo") => hashmap! {
                "foo".into() => "bar2".into(),
                "fem".into() => "is_great".into(),
            },
        PathBuf::from("/foo/bar") => hashmap! {
                "foo".into() => "bar3".into(),
                "fem".into() => "is_great".into(),
            }
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: PathBuf::from("/"),
            pwd,
            data: Data {
                projector: get_data(),
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")));
    }

    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        proj.set_value("foo", "bar4");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar4")));
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        proj.remove_value("foo");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
    }
}
