pragma circom 2.0.0;

include "../../circomlib/gates.circom";

template Main(n) {
    signal input in[n];
    signal output out;

    component multi = MultiAND(3);

    multi.in[0] <== in[0];
    multi.in[1] <== in[1];
    multi.in[2] <== in[2];

    out <== multi.out;
}

component main = Main(3);
