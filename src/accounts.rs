// Yeahbut January 2024

#![allow(non_snake_case)]

use reqwest;
use serde::{Serialize, Deserialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize)]
pub struct UsernameToUuid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorMessage: Option<String>,
}

pub async fn username_to_uuid(username: &str)
    -> Result<UsernameToUuid, Box<dyn std::error::Error>>
{
    let url = format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        username
    );

    let resp = reqwest::get(url)
        .await?
        .json::<UsernameToUuid>()
        .await?;

    Ok(resp)
}

#[derive(Serialize, Deserialize)]
pub struct ProfileTextureMetadata {
    pub model: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileTexture {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ProfileTextureMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileTextures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SKIN: Option<ProfileTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CAPE: Option<ProfileTexture>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileValue {
    pub timestamp: i64,
    pub profileId: String,
    pub profileName: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatureRequired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<ProfileTextures>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: ProfileValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfilePropertyPrivate {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UUIDToProfile {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
    pub profileActions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorMessage: Option<String>,
}


#[derive(Serialize, Deserialize)]
struct UUIDToProfilePrivate {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfilePropertyPrivate>,
    pub profileActions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorMessage: Option<String>,
}

pub(crate) fn get_profile_value(properties: Vec<ProfilePropertyPrivate>)
    -> Result<Vec<ProfileProperty>, Box<dyn std::error::Error>>
{
    let mut output: Vec<ProfileProperty> = Vec::new();

    for property in properties {
        output.push(ProfileProperty {
            name: property.name,
            value: serde_json::from_slice(
                &general_purpose::STANDARD_NO_PAD.decode(property.value)?
            )?,
            signature: property.signature,
        })
    }

    Ok(output)
}

pub async fn uuid_to_profile(uuid: &str)
    -> Result<UUIDToProfile, Box<dyn std::error::Error>>
{
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );

    let resp = reqwest::get(url)
        .await?
        .json::<UUIDToProfilePrivate>()
        .await?;

    let output = UUIDToProfile {
        id: resp.id,
        name: resp.name,
        properties: get_profile_value(resp.properties)?,
        profileActions: resp.profileActions,
        legacy: resp.legacy,
        demo: resp.demo,
        path: resp.path,
        error: resp.error,
        errorMessage: resp.errorMessage,
    };

    Ok(output)
}
