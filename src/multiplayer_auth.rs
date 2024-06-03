// Yeahbut June 2024

#![allow(non_snake_case)]

use std::fmt;
use reqwest;
use serde::{Serialize, Deserialize};

use crate::accounts::{
    ProfileProperty,
    ProfilePropertyPrivate,
    get_profile_value,
};

#[derive(Debug)]
pub enum AuthError {
    AuthError,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AuthError => write!(f, "AuthError"),
        }
    }
}
impl std::error::Error for AuthError {}

#[derive(Serialize, Deserialize)]
struct PlayerJoining {
    pub accessToken: String,
    pub selectedProfile: String,
    pub serverId: String,
}

pub async fn join(
    accessToken: String,
    selectedProfile: String,
    serverId: String,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let url = "https://sessionserver.mojang.com/session/minecraft/join";

    let resp = reqwest::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .json(&PlayerJoining {accessToken,selectedProfile,serverId})
        .send()
        .await?;

    Ok(resp)
}

#[derive(Serialize, Deserialize)]
pub struct JoinedPlayer {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
}

#[derive(Serialize, Deserialize)]
struct JoinedPlayerPrivate {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfilePropertyPrivate>,
}

pub async fn joined(
    username: &str,
    server_id: &str,
    ip: Option<&str>,
) -> Result<JoinedPlayer, Box<dyn std::error::Error>> {
    let url = match ip {
        Some(ip) => format!(
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?\
                username={}&serverId={}&ip={}",
            username,
            server_id,
            ip,
        ),
        None => format!(
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?\
                username={}&serverId={}",
            username,
            server_id,
        )
    };

    let resp = reqwest::get(url)
        .await?;

    if resp.status() == 200 {
        let data = resp.json::<JoinedPlayerPrivate>().await?;
        Ok(JoinedPlayer {
            id: data.id,
            name: data.name,
            properties: get_profile_value(data.properties)?
        })
    } else {
        Err(Box::new(AuthError::AuthError))
    }

}
