#[cfg(test)]
mod tests;

mod alphanum;
mod default;
mod helpers;
mod polynomials;
mod vecl;

/// Still useless, only test purposes for now.
fn main() {
    const VERSION: usize = 1;
    const QUALITY: vecl::ECL = vecl::ECL::M;
    const STRING_TO_ENCODE: &[u8] = b"HELLO WORLD";

    let res = alphanum::encode_alphanum(STRING_TO_ENCODE, VERSION, QUALITY);
    println!("{:?}", alphanum::alphanum_to_binary(&res));

    println!("{:?}", polynomials::generator(10));
}
