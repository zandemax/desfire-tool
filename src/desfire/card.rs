extern crate freefare as mod_freefare;
use mod_freefare::*;
use std::error::Error;

use super::DesFire;

pub struct CardInfo {
  pub free_memory: u32,
  pub card_version: u8,
  pub size: u8,
  pub uid: [u8; 7usize]
}

pub fn get_card_info() -> CardInfo {
  let desfire = DesFire::initialize();
  desfire.connect();
  let mut size: u32 = 0;
  let res = mifare::desfire_free_mem(desfire.tag, &mut size);
  if res < 0 { size = 0; }
  let mut info = freefare_sys::Struct_mifare_desfire_version_info { ..Default::default() };
  let res = mifare::desfire_get_version(desfire.tag, &mut info);
  desfire.destruct(); 
  CardInfo {
    free_memory: size,
    card_version: info.hardware.version_major,
    size: match info.hardware.storage_size {
      26 => 8,
      18 => 4,
      16 => 2,
      _ => 0
    },
    uid: info.uid
  }
}

pub fn format_card(mut key: [u8; 16usize], key_id: u8, version: Option<u8>) {
  let desfire = DesFire::initialize();
  desfire.connect();
  desfire.authenticate(key_id, key, version);
  let res = mifare::desfire_format_picc(desfire.tag);
  desfire.destruct();
}