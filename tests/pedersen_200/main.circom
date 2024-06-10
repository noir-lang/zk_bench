pragma circom 2.0.0;

include "../../circomlib/pedersen.circom";

template Main(n) {
    signal input in[2*n];
    signal output out[2*n];
    component pedersens[n];

    for (var j=0; j<200; j++) {
        pedersens[j] = Pedersen(2);

        pedersens[j].in[0] <== in[2*j];
        pedersens[j].in[1] <== in[2*j+1];
        out[2*j] <== pedersens[j].out[0];
        out[2*j+1] <== pedersens[j].out[1];
    }    
}

component main = Main(200);
