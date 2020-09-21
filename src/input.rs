use serde::Deserialize;
use std::error::Error;
use std::fs;
use toml;
pub struct Config {
    pub filename: String,
}

pub struct Rawinput {
    pub contents: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub duration: u64,
    pub profile: Vec<u64>,
}

// get config from command line arguments
impl Config {
    pub fn new(args: &[String], default: &'static str) -> Config {
        let filename = if args.len() > 1 {
            args[1].clone()
        } else {
            default.to_string()
        };
        Config { filename }
    }
}

// read file with input data
impl Rawinput {
    pub fn new(config: Config) -> Result<Rawinput, Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        Ok(Rawinput { contents })
    }
}

// deserialize raw input data
impl Data {
    pub fn new(rawinput: Rawinput) -> Result<Data, Box<dyn Error>> {
        let parsed: Data = toml::from_str(&rawinput.contents)?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_argument() {
        let b = "example.yaml";
        let received = Config::new(&[], b).filename;
        let expected = b;
        assert_eq!(received, expected);
    }

    #[test]
    fn valid_argument() {
        let a: &[String] = &["binary".to_string(), "foo.yaml".to_string()];
        let b = "example.yaml";
        let received = Config::new(a, b).filename;
        let expected = "foo.yaml";
        assert_eq!(received, expected);
    }

    #[test]
    fn parse_toml() {
        let a = Rawinput {
            contents: r#"
                duration = 5
                profile = [ 3, 4, 0 ]
                "#
            .to_string(),
        };
        let expected = Data::new(a).unwrap();
        assert_eq!(expected.duration, 5);
        assert_eq!(expected.profile, [3, 4, 0]);
    }
}
