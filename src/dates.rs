use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

pub mod datetime_iso8601 {
    use super::*;

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            return serializer.serialize_str(&date.to_rfc3339());
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .map(Some)
                .map_err(serde::de::Error::custom);
        }
        Ok(None)
    }
}
