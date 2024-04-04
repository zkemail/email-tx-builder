pub mod circuit;
pub mod converters;
pub mod cryptos;
pub mod parse_email;
pub mod regex;
pub mod statics;
pub use circuit::*;
pub(crate) use converters::*;
pub(crate) use cryptos::*;
pub(crate) use neon::prelude::*;
pub(crate) use parse_email::*;
pub use poseidon_rs::*;
pub(crate) use regex::*;
pub(crate) use statics::*;
pub use zk_regex_apis::extract_substrs::*;
pub use zk_regex_apis::padding::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("genEmailAuthInput", generate_email_auth_input_node)?;
    cx.export_function("parseEmail", parse_email_node)?;
    cx.export_function("padString", pad_string_node)?;
    cx.export_function("bytes2Fields", bytes2fields_node)?;
    cx.export_function("extractSubstrIdxes", extract_substr_idxes_node)?;
    cx.export_function("extractEmailAddrIdxes", extract_email_addr_idxes_node)?;
    cx.export_function("extractEmailDomainIdxes", extract_email_domain_idxes_node)?;
    cx.export_function(
        "extractEmailAddrWithNameIdxes",
        extract_email_addr_with_name_idxes_node,
    )?;
    cx.export_function("extractFromAllIdxes", extract_from_all_idxes_node)?;
    cx.export_function("extractFromAddrIdxes", extract_from_addr_idxes_node)?;
    cx.export_function("extractSubjectAllIdxes", extract_subject_all_idxes_node)?;
    cx.export_function("extractBodyHashIdxes", extract_body_hash_idxes_node)?;
    cx.export_function("extractTimestampIdxes", extract_timestamp_idxes_node)?;
    cx.export_function("extractTimestampInt", extract_timestamp_int_node)?;
    cx.export_function("extractMessageIdIdxes", extract_message_id_idxes_node)?;
    cx.export_function(
        "extractInvitationCodeIdxes",
        extract_invitation_code_idxes_node,
    )?;
    cx.export_function(
        "extractInvitationCodeWithPrefixIdxes",
        extract_invitation_code_with_prefix_idxes_node,
    )?;
    cx.export_function("padEmailAddr", pad_email_addr_node)?;
    cx.export_function("genAccountCode", gen_account_code_node)?;
    cx.export_function("accountSalt", account_salt_node)?;
    cx.export_function("publicKeyHash", public_key_hash_node)?;
    cx.export_function("emailNullifier", email_nullifier_node)?;
    cx.export_function("emailAddrCommit", email_addr_commit_node)?;
    cx.export_function(
        "emailAddrCommitWithSignature",
        email_addr_commit_with_signature_node,
    )?;
    cx.export_function("extractRandFromSignature", extract_rand_from_signature_node)?;
    Ok(())
}
