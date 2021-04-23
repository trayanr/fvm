use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    base_url: String,
    current_release: HashMap<String, String>,
    releases: Vec<Release>,
}

pub async fn get_list_of_versions() -> Result<Vec<Release>, reqwest::Error> {
    let resp: Response = reqwest::get(get_release_url()).await?.json().await?;
    let mut root_url_str = resp.base_url.clone();
    root_url_str.push('/');
    let root = Url::parse(&root_url_str).unwrap();
    let releases = resp.releases.to_vec();
    let fixed_releases: Vec<Release> = releases
        .iter()
        .map(|r| {
            let root_copy = root.clone();
            let full_root_archive = root_copy.join(&r.archive).unwrap();
            let mut release = r.clone();
            release.archive = String::from(full_root_archive.as_str());
            release
        })
        .collect();
    Ok(fixed_releases)
}

#[cfg(target_os = "linux")]
fn get_release_url() -> String {
    return String::from(
        "https://storage.googleapis.com/flutter_infra_release/releases/releases_linux.json",
    );
}

#[cfg(target_os = "windows")]
fn get_release_url() -> String {
    return String::from(
        "https://storage.googleapis.com/flutter_infra_release/releases/releases_windows.json",
    );
}

#[cfg(target_os = "macos")]
fn get_release_url() -> String {
    return String::from(
        "https://storage.googleapis.com/flutter_infra_release/releases/releases_macos.json",
    );
}

#[derive(Debug, Clone, PartialEq)]
pub enum Channel {
    Stable,
    Beta,
    Dev,
}

impl Channel {
    fn to_string(&self) -> String {
        match self {
            &Channel::Stable => String::from("stable"),
            &Channel::Beta => String::from("beta"),
            &Channel::Dev => String::from("dev"),
        }
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        if s == Channel::Stable.to_string().to_ascii_lowercase() {
            Ok(Channel::Stable)
        } else if s == Channel::Beta.to_string().to_ascii_lowercase() {
            Ok(Channel::Beta)
        } else {
            Ok(Channel::Dev)
        }
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string().to_lowercase())
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let time: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&time).map_err(de::Error::custom)
}

fn serialize_time<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub hash: String,
    pub channel: Channel,
    pub version: String,
    #[serde(
        deserialize_with = "deserialize_time",
        serialize_with = "serialize_time"
    )]
    pub release_date: DateTime<FixedOffset>,
    pub archive: String, //maybe path
    pub sha256: String,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.version, self.channel.to_string())
    }
}
