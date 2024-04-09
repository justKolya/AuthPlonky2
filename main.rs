use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

struct Token{
    type_req: u8,
    domain_name: String,
    token: usize,
}
struct User{
    username: String,
    password: String,
}

struct Server{
    salt: String,
}

fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let x = builder.add_virtual_target();
    let a = builder.mul(x, x);
    let b = builder.mul_const(F::from_canonical_u32(4), x);
    let c = builder.mul_const(F::NEG_ONE, b);
    let d = builder.add(a, c);
    let e = builder.add_const(d, F::from_canonical_u32(7));

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_input(x);
    builder.register_public_input(e);
    let mut pw = PartialWitness::new();
    pw.set_target(x, F::from_canonical_u32(1));
    let data = builder.build::<C>();
    let proof = data.prove(pw)?;
    println!(
        "x² - 4 *x + 7 where x = {} is {}",
        proof.public_inputs[0],
        proof.public_inputs[1]
    );
    data.verify(proof)
}


// struct User{
//     username: String,
//     password: String,
// }

// struct Server{
//     salt: String,
// }

// impl Server{
//     fn authenticate(&self, user: &User, response: &str) -> bool{
//         let challenge: &String = &self.salt;
        
//         let expected_response: String = hash(user.password.clone(), challenge);

//         response == expected_response
//     }
// }

// fn hash(input: String, salt: &str) -> String{
//     format!("HASH({}+{})", input, salt)
// }

// fn main(){
//     let user:User = User {
//         username: "Alice".to_string(),
//         password: "Password123".to_string(),
//     };
//     let server: Server = Server {
//         salt: "Salt".to_string(),
//     };
//     let response: String = hash(user.password.clone(), &server.salt);
    
//     if server.authenticate(&user, &response){
//         println!("GOOD");
//     } else {
//         println!("BAD");
//     }
// }