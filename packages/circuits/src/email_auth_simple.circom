pragma circom 2.1.6;

template EmailAuthSimple() {
    signal input i;
    signal output o;

    o <== i;
}

component main = EmailAuthSimple();