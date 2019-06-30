extern crate freefare as mod_freefare;
use mod_freefare::*;
use std::error::Error;

use super::DesFire;
use super::key::*;


pub struct Application {
  pub encryption: Encryption,
  pub id: *mut freefare_sys::Struct_mifare_desfire_aid,
  pub settings: KeySettings,
  pub kid: u8
}

pub enum Encryption {
  NONE,
  AES,
  DES3K3
}
pub fn add_application(application: Application, key: Option<[u8; 16usize]>, version: Option<u8>) {
  let desfire = DesFire::initialize();
  desfire.connect();
  let settings = get_key_settings(0, 0);
  if settings.create_needs_master {
    match key {
      Some(key) => desfire.authenticate(0, key, version),
      None      => panic!("Your card needs master key auth to create applications! Please supply the master key!")
    }
  }
  let res;
  match application.encryption {
    Encryption::NONE => res = mifare::desfire_create_application(desfire.tag, application.id, settings_struct_to_u8(application.settings), application.kid),
    Encryption::DES3K3 => res = mifare::desfire_create_application_3k3des(desfire.tag, application.id, settings_struct_to_u8(application.settings), application.kid),
    Encryption::AES => res = mifare::desfire_create_application_aes(desfire.tag, application.id, settings_struct_to_u8(application.settings), application.kid)
  }
}