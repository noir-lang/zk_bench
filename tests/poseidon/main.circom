pragma circom 2.0.0;

include "../../circomlib/poseidon.circom";

template Main(n) {
    signal input in[n];
    signal output out;

    component poseidon = Poseidon(n);

    for (var j=0; j<n; j++) {
        poseidon.inputs[j] <== in[j];
    }
    out <== poseidon.out;
}

component main = Main(2);
