pragma circom 2.1.6;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
include "./constants.circom";


// `in` is a big-endtian digit string of `out`.
// `check_valid` is a boolean value that indicates whether to check the validity of the input.
template Digit2Int(n) {
    signal input in[n];
    signal input check_valid;
    signal output out;

    signal is_g[n];
    signal is_l[n];
    signal is_valid[n];
    for(var i=0; i<n; i++) {
        is_g[i] <== GreaterEqThan(8)([in[i], 48]);
        is_l[i] <== LessEqThan(8)([in[i], 57]);
        is_valid[i] <== is_g[i] * is_l[i];
        (1 - check_valid) + check_valid * is_valid[i] === 1;

    }
    out <== DigitBytesToInt(n)(in);
}

