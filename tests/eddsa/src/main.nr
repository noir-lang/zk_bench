use dep::std::compat;
use dep::std::ec::consts::te::baby_jubjub;
use dep::std::ec::tecurve::affine::Point as TEPoint;
use dep::std::hash;
use dep::std::eddsa::{eddsa_poseidon_verify, eddsa_verify_with_hasher};

fn main(
    msg: pub field,
    pub_key_x: field,
    pub_key_y: field,
    r8_x: field,
    r8_y: field,
    s: field
) -> pub bool {
    eddsa_poseidon_verify(pub_key_x, pub_key_y, s, r8_x, r8_y, msg)
}

