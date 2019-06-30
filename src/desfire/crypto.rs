extern crate rand;

use rand::{RngCore};
use rand::rngs::{OsRng};

fn generate_new_aes_key () -> Vec<u8> {

  let mut r = OsRng::new().unwrap();

  // Random bytes.
  let mut key = vec![0u8; 16];
  r.fill_bytes(&mut key);

  return key
} 
