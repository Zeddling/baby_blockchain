mod keysig;

use keysig::KeySig;

fn main() {
    let keysig = KeySig::new();

    println!("Keypair\n{}", keysig.to_string());

    let data = b"Hello World";

    let signature = keysig.sign(b"Hello World");

    println!("Is it ours? {}", keysig.verify(data, &signature));
}
