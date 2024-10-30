use crate::*;
use anyhow::Result;
use axum::Json;
use hex::decode;
use rand::Rng;
use relayer_utils::{
    calculate_account_salt, extract_template_vals_from_command, TemplateValue, LOG,
};
use serde::{Deserialize, Serialize};
use std::str;

/// Retrieves the status of a request.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the request ID.
///
/// # Returns
///
/// A `Result` containing a JSON `RequestStatusResponse` or an `ApiError`.
pub async fn request_status_api(
    Json(payload): Json<RequestStatusRequest>,
) -> Result<Json<RequestStatusResponse>, ApiError> {
    let row = DB.get_request(payload.request_id).await?;

    // Determine the status based on the retrieved row
    let status = if let Some(ref row) = row {
        if row.is_processed {
            RequestStatus::Processed
        } else {
            RequestStatus::Pending
        }
    } else {
        RequestStatus::NotExist
    };

    Ok(Json(RequestStatusResponse {
        request_id: payload.request_id,
        status,
        is_success: row
            .as_ref()
            .map_or(false, |r| r.is_success.unwrap_or(false)),
        email_nullifier: row.clone().and_then(|r| r.email_nullifier),
        account_salt: row.clone().and_then(|r| r.account_salt),
    }))
}

/// Handles an acceptance request for a wallet.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the acceptance request details.
///
/// # Returns
///
/// A `Result` containing a JSON `AcceptanceResponse` or an `ApiError`.
pub async fn handle_acceptance_request(
    Json(payload): Json<AcceptanceRequest>,
) -> Result<Json<AcceptanceResponse>, ApiError> {
    let command_template = CLIENT
        .get_acceptance_command_templates(&payload.controller_eth_addr, payload.template_idx)
        .await?;

    // Extract and validate command parameters
    let command_params = extract_template_vals_from_command(&payload.command, command_template)
        .map_err(|_| ApiError::Validation("Invalid command".to_string()))?;

    // Recover the account address
    let account_eth_addr = CLIENT
        .get_recovered_account_from_acceptance_command(
            &payload.controller_eth_addr,
            command_params.clone(),
            payload.template_idx,
        )
        .await?;

    let account_eth_addr = format!("0x{:x}", account_eth_addr);

    // Check if the wallet is deployed
    if !CLIENT.is_wallet_deployed(&account_eth_addr).await? {
        return Err(ApiError::Validation("Wallet not deployed".to_string()));
    }

    // Check if the account code is already used
    if let Ok(Some(creds)) = DB.get_credentials(&payload.account_code).await {
        return Err(ApiError::Validation(
            "Account code already used".to_string(),
        ));
    }

    // Generate a unique request ID
    let mut request_id = rand::thread_rng().gen::<u32>();
    while let Ok(Some(request)) = DB.get_request(request_id).await {
        request_id = rand::thread_rng().gen::<u32>();
    }

    let account_salt = calculate_account_salt(&payload.guardian_email_addr, &payload.account_code);

    DB.insert_request(&Request {
        request_id,
        account_eth_addr: account_eth_addr.clone(),
        controller_eth_addr: payload.controller_eth_addr.clone(),
        guardian_email_addr: payload.guardian_email_addr.clone(),
        is_for_recovery: false,
        template_idx: payload.template_idx,
        is_processed: false,
        is_success: None,
        email_nullifier: None,
        account_salt: Some(account_salt.clone()),
    })
    .await?;

    // Handle different scenarios based on guardian status
    if DB
        .is_guardian_set(&account_eth_addr, &payload.guardian_email_addr)
        .await?
    {
        handle_email_event(EmailAuthEvent::GuardianAlreadyExists {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
        })
        .await
        // TODO: Add custom errors for handle_email_events and map_err
        .expect("Failed to send GuardianAlreadyExists event");
    } else if DB
        .is_wallet_and_email_registered(&account_eth_addr, &payload.guardian_email_addr)
        .await?
    {
        // Update credentials and send acceptance request email
        DB.update_credentials_of_wallet_and_email(&Credentials {
            account_code: payload.account_code.clone(),
            account_eth_addr: account_eth_addr.clone(),
            guardian_email_addr: payload.guardian_email_addr.clone(),
            is_set: false,
        })
        .await?;

        handle_email_event(EmailAuthEvent::AcceptanceRequest {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
            request_id,
            command: payload.command.clone(),
            account_code: payload.account_code.clone(),
        })
        .await?;
    } else {
        // Insert new credentials and send acceptance request email
        DB.insert_credentials(&Credentials {
            account_code: payload.account_code.clone(),
            account_eth_addr: account_eth_addr.clone(),
            guardian_email_addr: payload.guardian_email_addr.clone(),
            is_set: false,
        })
        .await?;

        handle_email_event(EmailAuthEvent::AcceptanceRequest {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
            request_id,
            command: payload.command.clone(),
            account_code: payload.account_code.clone(),
        })
        .await?;
    }

    Ok(Json(AcceptanceResponse {
        request_id,
        command_params,
    }))
}

/// Handles a recovery request for a wallet.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the recovery request details.
///
/// # Returns
///
/// A `Result` containing a JSON `RecoveryResponse` or an `ApiError`.
pub async fn handle_recovery_request(
    Json(payload): Json<RecoveryRequest>,
) -> Result<Json<RecoveryResponse>, ApiError> {
    // Fetch the command template
    let command_template = CLIENT
        .get_recovery_command_templates(&payload.controller_eth_addr, payload.template_idx)
        .await?;

    // Extract and validate command parameters
    let command_params = extract_template_vals_from_command(&payload.command, command_template)
        .map_err(|_| ApiError::Validation("Invalid command".to_string()))?;

    // Recover the account address
    let account_eth_addr = CLIENT
        .get_recovered_account_from_recovery_command(
            &payload.controller_eth_addr,
            command_params.clone(),
            payload.template_idx,
        )
        .await?;

    let account_eth_addr = format!("0x{:x}", account_eth_addr);

    // Check if the wallet is deployed
    if !CLIENT.is_wallet_deployed(&account_eth_addr).await? {
        return Err(ApiError::Validation("Wallet not deployed".to_string()));
    }

    // Generate a unique request ID
    let mut request_id = rand::thread_rng().gen::<u32>();
    while let Ok(Some(request)) = DB.get_request(request_id).await {
        request_id = rand::thread_rng().gen::<u32>();
    }

    // Fetch account details and calculate account salt
    let account = DB
        .get_credentials_from_wallet_and_email(&account_eth_addr, &payload.guardian_email_addr)
        .await?;

    let account_salt = if let Some(account_details) = account {
        calculate_account_salt(&payload.guardian_email_addr, &account_details.account_code)
    } else {
        return Err(ApiError::Validation("Wallet not deployed".to_string()));
    };

    // Handle the case when wallet and email are not registered
    if !DB
        .is_wallet_and_email_registered(&account_eth_addr, &payload.guardian_email_addr)
        .await?
    {
        DB.insert_request(&Request {
            request_id,
            account_eth_addr: account_eth_addr.clone(),
            controller_eth_addr: payload.controller_eth_addr.clone(),
            guardian_email_addr: payload.guardian_email_addr.clone(),
            is_for_recovery: true,
            template_idx: payload.template_idx,
            is_processed: false,
            is_success: None,
            email_nullifier: None,
            account_salt: Some(account_salt.clone()),
        })
        .await?;

        handle_email_event(EmailAuthEvent::GuardianNotRegistered {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
            command: payload.command.clone(),
            request_id,
        })
        .await?;

        return Ok(Json(RecoveryResponse {
            request_id,
            command_params,
        }));
    }

    // Insert the recovery request
    DB.insert_request(&Request {
        request_id,
        account_eth_addr: account_eth_addr.clone(),
        controller_eth_addr: payload.controller_eth_addr.clone(),
        guardian_email_addr: payload.guardian_email_addr.clone(),
        is_for_recovery: true,
        template_idx: payload.template_idx,
        is_processed: false,
        is_success: None,
        email_nullifier: None,
        account_salt: Some(account_salt.clone()),
    })
    .await?;

    // Handle different scenarios based on guardian status
    if DB
        .is_guardian_set(&account_eth_addr, &payload.guardian_email_addr)
        .await?
    {
        handle_email_event(EmailAuthEvent::RecoveryRequest {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
            request_id,
            command: payload.command.clone(),
        })
        .await
        // TODO: Add custom error for handle_email_event
        .expect("Failed to send Recovery event");
    } else {
        handle_email_event(EmailAuthEvent::GuardianNotSet {
            account_eth_addr,
            guardian_email_addr: payload.guardian_email_addr.clone(),
        })
        .await
        // TODO: Add error handling
        .expect("Failed to send Recovery event");
    }

    Ok(Json(RecoveryResponse {
        request_id,
        command_params,
    }))
}

/// Handles the completion of a recovery request.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the complete recovery request details.
///
/// # Returns
///
/// A `Result` containing a `String` message or an `ApiError`.
pub async fn handle_complete_recovery_request(
    Json(payload): Json<CompleteRecoveryRequest>,
) -> Result<String, ApiError> {
    // Check if the wallet is deployed
    if !CLIENT.is_wallet_deployed(&payload.account_eth_addr).await? {
        return Err(ApiError::Validation("Wallet not deployed".to_string()));
    }

    // Attempt to complete the recovery
    match CLIENT
        .complete_recovery(
            &payload.controller_eth_addr,
            &payload.account_eth_addr,
            &payload.complete_calldata,
        )
        .await
    {
        Ok(true) => Ok("Recovery completed".to_string()),
        Ok(false) => Err(ApiError::Validation("Recovery failed".to_string())),
        Err(e) => {
            // Parse the error message if it follows the known format
            let error_message = if e
                .to_string()
                .starts_with("Contract call reverted with data:")
            {
                parse_error_message(e.to_string())
            } else {
                "Internal server error".to_string()
            };
            // Remove all non printable characters
            let error_message = error_message
                .chars()
                .filter(|c| c.is_ascii())
                .collect::<String>();
            Err(ApiError::Internal(error_message))
        }
    }
}

/// Retrieves the account salt for a given email address and account code.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the email address and account code.
///
/// # Returns
///
/// A `Result` containing the account salt as a `String` or an `ApiError`.
pub async fn get_account_salt(
    Json(payload): Json<GetAccountSaltRequest>,
) -> Result<String, ApiError> {
    let account_salt = calculate_account_salt(&payload.email_addr, &payload.account_code);
    Ok(account_salt)
}

/// Marks a guardian as inactive for a given wallet.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the account and controller Ethereum addresses.
///
/// # Returns
///
/// A `Result` containing a `String` message or an `ApiError`.
pub async fn inactive_guardian(
    Json(payload): Json<InactiveGuardianRequest>,
) -> Result<String, ApiError> {
    // Check if the wallet is activated
    let is_activated = CLIENT
        .get_is_activated(&payload.controller_eth_addr, &payload.account_eth_addr)
        .await?;

    if is_activated {
        return Ok("Wallet is activated".to_string());
    }

    trace!(LOG, "Inactive guardian"; "is_activated" => is_activated);

    // Parse and format the account Ethereum address
    let account_eth_addr: Address = payload
        .account_eth_addr
        .parse()
        .map_err(|e| ApiError::Validation(format!("Failed to parse account_eth_addr: {}", e)))?;
    let account_eth_addr = format!("0x{:x}", &account_eth_addr);
    trace!(LOG, "Inactive guardian"; "account_eth_addr" => &account_eth_addr);

    // Update the credentials of the inactive guardian
    DB.update_credentials_of_inactive_guardian(false, &account_eth_addr)
        .await?;

    Ok("Guardian inactivated".to_string())
}

/// Parses an error message from contract call data.
///
/// # Arguments
///
/// * `error_data` - The error data as a `String`.
///
/// # Returns
///
/// A `String` containing the parsed error message or a default error message.
fn parse_error_message(error_data: String) -> String {
    // Attempt to extract and decode the error message
    if let Some(hex_error) = error_data.split(" ").last() {
        if hex_error.len() > 138 {
            // Check if the length is sufficient for a message
            let error_bytes = decode(&hex_error[138..]).unwrap_or_else(|_| vec![]);
            if let Ok(message) = str::from_utf8(&error_bytes) {
                return message.to_string();
            }
        }
    }
    format!("Failed to parse contract error: {}", error_data)
}

/// Receives and processes an email.
///
/// # Arguments
///
/// * `email` - The raw email as a `String`.
///
/// # Returns
///
/// A `Result` containing `()` or an `ApiError`.
pub async fn receive_email_api_fn(email: String) -> Result<(), ApiError> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;
    let from_addr = parsed_email.get_from_addr()?;
    let original_subject = parsed_email.get_subject_all()?;
    tokio::spawn(async move {
        if !check_is_valid_request(&parsed_email).await.unwrap() {
            trace!(LOG, "Got a non valid email request. Ignoring.");
            return;
        }

        // Send acknowledgment email
        match handle_email_event(EmailAuthEvent::Ack {
            email_addr: from_addr.clone(),
            command: parsed_email.get_command(false).unwrap_or_default(),
            original_message_id: parsed_email.get_message_id().ok(),
            original_subject,
        })
        .await
        {
            Ok(_) => {
                trace!(LOG, "Ack email event sent");
            }
            Err(e) => {
                error!(LOG, "Error handling email event: {:?}", e);
            }
        }

        // Process the email
        match handle_email(email.clone()).await {
            Ok(event) => match handle_email_event(event).await {
                Ok(_) => {}
                Err(e) => {
                    error!(LOG, "Error handling email event: {:?}", e);
                }
            },
            Err(e) => {
                error!(LOG, "Error handling email: {:?}", e);
                let original_subject = parsed_email
                    .get_subject_all()
                    .unwrap_or("Unknown Error".to_string());
                match handle_email_event(EmailAuthEvent::Error {
                    email_addr: from_addr,
                    error: e.to_string(),
                    original_subject,
                    original_message_id: parsed_email.get_message_id().ok(),
                    email_request_context: *Box::new(None),
                    command: None,
                })
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!(LOG, "Error handling email event: {:?}", e);
                    }
                }
            }
        }
    });
    Ok(())
}

/// Request status request structure.
#[derive(Serialize, Deserialize)]
pub struct RequestStatusRequest {
    /// The unique identifier for the request.
    pub request_id: u32,
}

/// Enum representing the possible statuses of a request.
#[derive(Serialize, Deserialize)]
pub enum RequestStatus {
    /// The request does not exist.
    NotExist = 0,
    /// The request is pending processing.
    Pending = 1,
    /// The request has been processed.
    Processed = 2,
}

/// Response structure for a request status query.
#[derive(Serialize, Deserialize)]
pub struct RequestStatusResponse {
    /// The unique identifier for the request.
    pub request_id: u32,
    /// The current status of the request.
    pub status: RequestStatus,
    /// Indicates whether the request was successful.
    pub is_success: bool,
    /// The email nullifier, if available.
    pub email_nullifier: Option<String>,
    /// The account salt, if available.
    pub account_salt: Option<String>,
}

/// Request structure for an acceptance request.
#[derive(Serialize, Deserialize)]
pub struct AcceptanceRequest {
    /// The Ethereum address of the controller.
    pub controller_eth_addr: String,
    /// The email address of the guardian.
    pub guardian_email_addr: String,
    /// The unique account code.
    pub account_code: String,
    /// The index of the template to use.
    pub template_idx: u64,
    /// The command to execute.
    pub command: String,
}

/// Response structure for an acceptance request.
#[derive(Serialize, Deserialize)]
pub struct AcceptanceResponse {
    /// The unique identifier for the request.
    pub request_id: u32,
    /// The parameters extracted from the command.
    pub command_params: Vec<TemplateValue>,
}

/// Request structure for a recovery request.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecoveryRequest {
    /// The Ethereum address of the controller.
    pub controller_eth_addr: String,
    /// The email address of the guardian.
    pub guardian_email_addr: String,
    /// The index of the template to use.
    pub template_idx: u64,
    /// The command to execute.
    pub command: String,
}

/// Response structure for a recovery request.
#[derive(Serialize, Deserialize)]
pub struct RecoveryResponse {
    /// The unique identifier for the request.
    pub request_id: u32,
    /// The parameters extracted from the command.
    pub command_params: Vec<TemplateValue>,
}

/// Request structure for completing a recovery.
#[derive(Serialize, Deserialize)]
pub struct CompleteRecoveryRequest {
    /// The Ethereum address of the account to recover.
    pub account_eth_addr: String,
    /// The Ethereum address of the controller.
    pub controller_eth_addr: String,
    /// The calldata to complete the recovery.
    pub complete_calldata: String,
}

/// Request structure for retrieving an account salt.
#[derive(Serialize, Deserialize)]
pub struct GetAccountSaltRequest {
    /// The unique account code.
    pub account_code: String,
    /// The email address associated with the account.
    pub email_addr: String,
}

/// Structure representing a permitted wallet.
#[derive(Deserialize)]
struct PermittedWallet {
    /// The name of the wallet.
    wallet_name: String,
    /// The Ethereum address of the controller.
    controller_eth_addr: String,
    /// The hash of the bytecode of the proxy contract.
    hash_of_bytecode_of_proxy: String,
    /// The address of the implementation contract.
    impl_contract_address: String,
    /// The slot location in storage.
    slot_location: String,
}

/// Request structure for marking a guardian as inactive.
#[derive(Serialize, Deserialize)]
pub struct InactiveGuardianRequest {
    /// The Ethereum address of the account.
    pub account_eth_addr: String,
    /// The Ethereum address of the controller.
    pub controller_eth_addr: String,
}
