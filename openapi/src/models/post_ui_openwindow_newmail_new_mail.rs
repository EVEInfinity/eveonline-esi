/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.33
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PostUiOpenwindowNewmailNewMail : new_mail object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUiOpenwindowNewmailNewMail {
    /// body string
    #[serde(rename = "body")]
    pub body: String,
    /// recipients array
    #[serde(rename = "recipients")]
    pub recipients: Vec<i32>,
    /// subject string
    #[serde(rename = "subject")]
    pub subject: String,
    /// to_corp_or_alliance_id integer
    #[serde(rename = "to_corp_or_alliance_id", skip_serializing_if = "Option::is_none")]
    pub to_corp_or_alliance_id: Option<i32>,
    /// Corporations, alliances and mailing lists are all types of mailing groups. You may only send to one mailing group, at a time, so you may fill out either this field or the to_corp_or_alliance_ids field
    #[serde(rename = "to_mailing_list_id", skip_serializing_if = "Option::is_none")]
    pub to_mailing_list_id: Option<i32>,
}

impl PostUiOpenwindowNewmailNewMail {
    /// new_mail object
    pub fn new(body: String, recipients: Vec<i32>, subject: String) -> PostUiOpenwindowNewmailNewMail {
        PostUiOpenwindowNewmailNewMail {
            body,
            recipients,
            subject,
            to_corp_or_alliance_id: None,
            to_mailing_list_id: None,
        }
    }
}

