pragma circom 2.1.6;

include "./email_auth_template.circom";

// Reveals both the public key and from address
component main = EmailAuth(121, 17, 1024, 1024, 605, 0, 1, 0, 1, 1);