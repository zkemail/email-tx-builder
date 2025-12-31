pragma circom 2.1.6;

include "../../../src/templates/email_auth_template.circom";

component main = EmailAuth(
    121,  // n: number of bits in each chunk
    17,   // k: number of chunks
    1024, // max_header_bytes
    1024, // max_body_bytes
    605,  // max_command_bytes
    1,    // recipient_enabled (hide recipient)
    1,    // is_qp_encoded
    1,    // timestamp_enabled
    0,    // reveal_from_addr
    0,    // reveal_to_addr
    0     // reveal_subject 
);