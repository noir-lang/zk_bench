pragma circom 2.0.0;

include "../../circomlib/poseidon.circom";

template Main(n) {
    signal input in[n][2];
    signal output out[n];
    component poseidons[n];

    for (var j=0; j<30; j++) {
        poseidons[j] = Poseidon(2);

        poseidons[j].inputs[0] <== in[j][0];
        poseidons[j].inputs[1] <== in[j][1];
        out[j] <== poseidons[j].out;
    }    
}

component main = Main(30);
