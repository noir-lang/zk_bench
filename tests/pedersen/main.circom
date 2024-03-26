pragma circom 2.0.0;

include "../../circomlib/pedersen.circom";

template Main(n) {
    signal input in[n];
    signal output out[2];

    component pedersen = Pedersen(n);
    for (var j=0; j<n; j++) {
        pedersen.in[j] <== in[j];
    }
    out[0] <== pedersen.out[0];
    out[1] <== pedersen.out[1];
}

component main = Main(2);

