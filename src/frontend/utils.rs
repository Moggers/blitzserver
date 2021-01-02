use serde::de::Error;
use actix_web::dev::Payload;
use serde_qs::actix::{QsQueryConfig};
use serde::de;
use futures::future::{ready, Ready};
use std::fmt;
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Display};

pub fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
{
    let s = <&str as serde::Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| D::Error::custom("could not parse string"))
}
pub fn from_str_seq<'de, D, S>(deserializer: D) -> Result<Vec<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
{
    let s = <Vec<&str> as serde::Deserialize>::deserialize(deserializer)?;
    s.iter()
        .map(|s| S::from_str(&s).map_err(|_| D::Error::custom("could not parse string")))
        .collect()
}

