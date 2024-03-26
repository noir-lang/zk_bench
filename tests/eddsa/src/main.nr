
use dep::std::compat;
use dep::std::ec::consts::te::baby_jubjub;
use dep::std::ec::tecurve::affine::Point as TEPoint;
use dep::std::hash;
use dep::std::eddsa::{eddsa_poseidon_verify, eddsa_verify_with_hasher};
use dep::std::hash::pedersen::PedersenHasher;

fn main(msg: pub [Field;2], pub_key_x: Field, pub_key_y: Field, r8_x: Field, r8_y: Field, s: Field) -> pub bool {
    let mut hasher = PedersenHasher::default();
    eddsa_verify_with_hasher(pub_key_x, pub_key_y, s, r8_x, r8_y, msg[0], &mut hasher)
    
}

