use number::Number;
use field::Field;

#[derive(Clone, Copy)]
pub struct DiffieMeta {

    pub field: Field,
    pub modulus_p: Number,
    pub base_g: Number

}

impl DiffieMeta {

    pub fn new(modulus_p: u64, base_g: i64) -> DiffieMeta {

        let f = Field::new(modulus_p);

        DiffieMeta {
            field: f,
            modulus_p: Number::new(modulus_p as i64, f),
            base_g: Number::new(base_g, f)
        }

    }
}

pub struct Actor {

    pub diffie_meta: DiffieMeta,
    secret: Number,

}

impl Actor {

    pub fn new(diffie_meta: DiffieMeta) -> Actor {

        Actor {
            diffie_meta: diffie_meta,
            secret: Number::infinity()
        }

    }

    pub fn secret(mut self, secret: i64) -> Actor {
        self.secret = Number::new(secret, self.diffie_meta.field);

        self
    }

    pub fn public_key(&self) -> Number {

        let g = self.diffie_meta.base_g;
        let field = self.diffie_meta.field;

        let exp = Number::pow(g, self.secret).value;
        Number::new(exp % field.prime as i64, field)
    }

    pub fn shared_secret(&self, public_key: Number) -> Number {
        let exp = Number::pow(public_key, self.secret).value % self.diffie_meta.modulus_p.value;
        Number::new(exp, public_key.field)
    }

}


#[test]
pub fn diffie_hellman_test() {


    // base p: 99901
    // generator g: 19
    let diffie_meta = DiffieMeta::new(99901, 19);

    // private keys a and b
    let alice = Actor::new(diffie_meta).secret(23);
    let bob = Actor::new(diffie_meta).secret(11);

    // public keys A and B
    // 19^a mod 99901
    let alice_pub = alice.public_key();
    // 19^b mod 99901
    let bob_pub = bob.public_key();

    // shared secret keys are equal
    // bob_pub^k mod 99901
    let alice_shared = alice.shared_secret(bob_pub);
    // alice_pub^k mod 99901
    let bob_shared = bob.shared_secret(alice_pub);

    println!("{}, {}", alice_shared, bob_shared);
    assert_eq!(alice_shared.value, 92206);
    assert_eq!(alice_shared, bob_shared);
}