pragma circom 2.1.6;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/email-verifier.circom";
include "@zk-email/circuits/utils/regex.circom";
include "@zk-email/circuits/utils/functions.circom";
include "./utils/constants.circom";
include "./utils/account_salt.circom";
include "./utils/hash_sign.circom";
include "./utils/email_nullifier.circom";
include "./utils/bytes2ints.circom";
include "./utils/digit2int.circom";
include "./utils/hex2int.circom";
include "./utils/regex.circom";
include "./utils/email_addr_commit.circom";
include "./regexes/invitation_code_with_prefix_regex.circom";
include "./regexes/invitation_code_regex.circom";
include "./regexes/command_regex.circom";
include "./regexes/forced_subject_regex.circom";
include "@zk-email/zk-regex-circom/circuits/common/from_addr_regex.circom";
include "@zk-email/zk-regex-circom/circuits/common/email_addr_regex.circom";
include "@zk-email/zk-regex-circom/circuits/common/email_domain_regex.circom";
include "@zk-email/zk-regex-circom/circuits/common/subject_all_regex.circom";
include "@zk-email/zk-regex-circom/circuits/common/timestamp_regex.circom";

// This template verifies email from user (sender) and extracts a command in the email body, 
// timestamp, recipient email (commitment), etc.
template EmailAuthPubKey(
    n, // The number of bits in each chunk of the RSA public key (modulus)
    k, // The number of chunks in the RSA public key (n * k > 2048)
    max_header_bytes, // The maximum number of bytes in the email header
    max_body_bytes, // The maximum number of bytes in the email body
    max_command_bytes, // The maximum number of bytes in the command
    recipient_enabled, // Whether the email address commitment of the recipient equals the email address in the subject and is exposed
    is_qp_encoded, // Whether the email body is qp encoded
    timestamp_enabled, // Whether the timestamp is enabled
    reveal_public_key, // Whether the public key is revealed (1 to reveal, 0 to hide)
    reveal_from_addr // Whether the from address is revealed (1 to reveal, 0 to hide)
) {
    signal input padded_header[max_header_bytes]; // email data (only header part)
    signal input padded_header_len; // length of in email data including the padding
    signal input public_key[k]; // RSA public key (modulus), k parts of n bits each.
    signal input signature[k]; // RSA signature, k parts of n bits each.
    signal input body_hash_idx; // index of the bodyhash in the header
    signal input precomputed_sha[32]; // precomputed sha256 of the email body
    signal input padded_body[max_body_bytes]; // email data (only body part)
    signal input padded_body_len; // length of in email data including the padding
    signal input account_code;
    signal input from_addr_idx; // Index of the from email address (= sender email address) in the email header
    signal input domain_idx; // Index of the domain name in the from email address
    signal input timestamp_idx; // Index of the timestamp in the header
    signal input code_idx; // index of the invitation code in the header
    signal input command_idx; // index of the command in the body
    /// Note: padded_cleaned_body is only used for qp encoded email body, 
    /// for non-qp encoded email body, it should be equal to padded_body
    signal input padded_cleaned_body[max_body_bytes]; // cleaned email body

    var email_max_bytes = email_max_bytes_const();
    var command_field_len = compute_ints_size(max_command_bytes);
    var domain_len = domain_len_const();
    var domain_field_len = compute_ints_size(domain_len);
    var k2_chunked_size = k >> 1;
    if(k % 2 == 1) {
        k2_chunked_size += 1;
    }
    var timestamp_len = timestamp_len_const();
    var code_len = invitation_code_len_const();

    signal output public_key_hash;
    
    // Verify Email Signature
    component email_verifier = EmailVerifier(max_header_bytes, max_body_bytes, n, k, 0, 0, 0, is_qp_encoded);
    email_verifier.emailHeader <== padded_header;
    email_verifier.emailHeaderLength <== padded_header_len;
    email_verifier.pubkey <== public_key;
    email_verifier.signature <== signature;
    email_verifier.bodyHashIndex <== body_hash_idx;
    email_verifier.precomputedSHA <== precomputed_sha;
    email_verifier.emailBody <== padded_body;
    email_verifier.emailBodyLength <== padded_body_len;
    if (is_qp_encoded == 1) {
        email_verifier.decodedEmailBodyIn <== padded_cleaned_body;
    }
    public_key_hash <== email_verifier.pubkeyHash;
    
}