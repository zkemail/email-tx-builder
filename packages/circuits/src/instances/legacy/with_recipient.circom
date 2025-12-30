pragma circom 2.1.6;

include "../../templates/email_auth_legacy_template.circom";

component main = EmailAuthLegacy(
    121,  // n: number of bits in each chunk
    17,   // k: number of chunks
    1024, // max_header_bytes
    605,  // max_subject_bytes
    1     // recipient_enabled (reveal recipient)
);