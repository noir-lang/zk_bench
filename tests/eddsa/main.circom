pragma circom 2.0.0;

include "../../circomlib/eddsa.circom";

template Main(n) {
    signal input in[n];
    signal input A[256];
    signal input R8[256];
    signal input S[256];

    //signal output out[2];

    component eddsa = EdDSAVerifier(n);
    for (var j=0; j<n; j++) {
        eddsa.msg[j] <== in[j];
    }
    for (var j=0; j<256; j++) {
        eddsa.A[j] <== A[j];
        eddsa.R8[j] <== R8[j];
        eddsa.S[j] <== S[j];
    }

}

component main = Main(80);

