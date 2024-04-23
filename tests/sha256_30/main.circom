pragma circom 2.0.0;

include "../../circomlib/sha256/sha256_2.circom";

template Main(n) {
    signal input in[n][2];
    signal output out[n];
    component sha256s[n];

    for (var j=0; j<n; j++) {
        sha256s[j] = Sha256_2();

        sha256s[j].a <== in[j][0];
        sha256s[j].b <== in[j][1];
        out[j] <== sha256s[j].out;
    }    
}

component main = Main(30);
