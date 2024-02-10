mod encode;
mod decode;

#[cfg(test)]
mod crypt_test;

use encode::*;
use decode::*;

pub fn encoder(input: &str) -> String {
    encode_2(&encode_1(input))
}

pub fn decoder(input: &str) -> String {
    decode_1(&decode_2(input))
}
