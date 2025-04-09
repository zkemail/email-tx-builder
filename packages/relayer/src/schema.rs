use ethers::types::{Address, U256};
use relayer_utils::AccountCode;
use serde::{Deserialize, Serialize};

/// Represents the schema for email transaction authentication.
///
/// This struct is used to deserialize and serialize email transaction authentication data,
/// which includes contract details, command templates, and email metadata.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmailTxAuthSchema {
    /// The account code associated with the transaction.
    pub account_code: AccountCode,
    /// Indicates whether the code exists in the email.
    pub code_exists_in_email: bool,
    pub command_template: String,
    /// The parameters for the command template.
    pub command_params: Vec<String>,
    /// The ID of the template used in the transaction.
    pub template_id: U256,
    pub email_address: String,
    /// The subject of the email.
    pub subject: String,
    /// The body content of the email.
    pub body: String,
    /// The blockchain chain on which the transaction is to be executed.
    pub chain: Option<String>,
    /// The address of the DKIM contract.
    pub dkim_contract_address: Option<Address>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSaltSchema {
    pub account_code: String,
    pub email_address: String,
}
