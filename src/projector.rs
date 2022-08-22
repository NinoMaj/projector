use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    pub config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.pwd.as_path());
        let mut paths = vec![];

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            self.data
                .projector
                .get(path)
                .map(|map| out.extend(map.iter()));
        }

        out
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let mut curr = Some(self.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent();
        }

        return out.map(|s| s.to_string());
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        self.data
            .projector
            .entry(self.pwd.to_owned())
            .or_default()
            .insert(String::from(key), String::from(value));
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector.get_mut(&self.pwd).map(|x| {
            x.remove(key);
        });
        // .entry(self.config.pwd.to_owned())
        // .or_default()
        // .remove(key);
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(&p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }

        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, &contents)?;
        Ok(())
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        let mut data = Data {
            projector: HashMap::new(),
        };

        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"projector\": {}}"));
            let curr_data = serde_json::from_str(&contents);
            let curr_data = curr_data.unwrap_or(data);

            data = curr_data;
        }

        Projector { config, pwd, data }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use collection_macros::hashmap;

    use super::{Data, Projector};

    fn get_data() -> Data {
        return Data {
            projector: hashmap! {
                PathBuf::from("/") => hashmap! {
                    "foo".into() => "bar1".into(),
                    "bar".into() => "bazz".into(),
                },
                PathBuf::from("/foo") => hashmap! {
                    "foo".into() => "bar2".into()
                },
                PathBuf::from("/foo/bar") => hashmap! {
                    "foo".into() => "bar3".into()
                },
                PathBuf::from("/foo/bar/baz") => hashmap! {
                    "foo".into() => "bar3".into()
                },
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = Projector {
            data: get_data(),
            config: PathBuf::from("/foo/bar"),
            pwd: PathBuf::from("/foo/bar"),
        };

        assert_eq!(proj.get_value("foo"), Some(String::from("bar3")));
        assert_eq!(proj.get_value("bar"), Some(String::from("bazz")));
        assert_eq!(proj.get_value("notehu"), None);
    }

    #[test]
    fn set_value() {
        let mut proj = Projector {
            data: get_data(),
            config: PathBuf::from("/foo/bar"),
            pwd: PathBuf::from("/foo/bar"),
        };

        assert_eq!(proj.get_value("foo"), Some(String::from("bar3")));
        proj.set_value("foo", "hello, fem".into());
        assert_eq!(proj.get_value("foo"), Some(String::from("hello, fem")));
    }

    #[test]
    fn delete_value() {
        let mut proj = Projector {
            data: get_data(),
            config: PathBuf::from("/foo/bar"),
            pwd: PathBuf::from("/foo/bar"),
        };

        assert_eq!(proj.get_value("foo"), Some(String::from("bar3")));
        proj.remove_value("foo");
        assert_eq!(proj.get_value("foo"), Some(String::from("bar2")));
    }
}
