// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `fm.teal.alpha.feed.getPlay` namespace.
//!This lexicon is in a not officially released state. It is subject to change. | Retrieves a play given an author DID and record key.
pub const NSID: &str = "fm.teal.alpha.feed.getPlay";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    ///The author's DID for the play
    pub author_did: crate::types::string::AtIdentifier,
    ///The record key of the play
    pub rkey: String,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub play: crate::fm::teal::alpha::feed::defs::PlayView,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
