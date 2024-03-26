pragma circom 2.0.0;

include "../../circomlib/mimc.circom";

template Main(n) {
    signal input in[n];
    signal output out;

    component mimc = MultiMiMC7(n, 91);
    mimc.k <== 0;
    for (var j=0; j<n; j++) {
        mimc.in[j] <== in[j];
    }
    out <== mimc.out;
}

component main = Main(2);

