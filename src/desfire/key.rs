extern crate freefare as mod_freefare;
use mod_freefare::*;

use super::DesFire;

pub fn change_key(aid: u8, kid: u8, mut key_old: [u8; 16usize], mut key_new: [u8; 16usize], version: Option<u8>) {
  let desfire = DesFire::initialize();
  desfire.connect(); 
  desfire.authenticate(kid, key_old, version);
  let key_new = mifare::desfire_aes_key_new_with_version(&mut key_new[0], version.unwrap_or_else(|| 0) + 1);
  let key_old = mifare::desfire_aes_key_new_with_version(&mut key_old[0], version.unwrap_or_else(|| 0));
  mifare::desfire_change_key(desfire.tag, kid, key_new, key_old);
  mifare::desfire_key_free(key_old);
  mifare::desfire_key_free(key_new);
  desfire.destruct();
}

pub fn add_key(aid: u8, kid: u8, mut key_old: [u8; 16usize], mut key_new: [u8; 16usize], version: Option<u8>) {
  let desfire = DesFire::initialize();
  desfire.connect(); 
  desfire.authenticate(kid, key_old, version);
  //let mut aids;
  let mut count: usize = 0;
  //mifare::desfire_get_application_ids(desfire.tag, &mut aids, &mut count);
  //let aids = unsafe { std::slice::from_raw_parts(aids, count); }
  
  // mifare::desfire_select_application(desfire.tag, aid);
  let key_new = mifare::desfire_aes_key_new_with_version(&mut key_new[0], version.unwrap_or_else(|| 0) + 1);
  let key_old = mifare::desfire_aes_key_new_with_version(&mut key_old[0], version.unwrap_or_else(|| 0));
  mifare::desfire_change_key(desfire.tag, kid, key_new, key_old);
  mifare::desfire_key_free(key_old);
  mifare::desfire_key_free(key_new);
  desfire.destruct();
}

pub struct KeySettings {
  pub master_key_changeable: bool,
  pub list_needs_master: bool,
  pub create_needs_master: bool,
  pub settings_changeable: bool,
  pub change_needs_key_auth: bool,
  pub keys_frozen: bool
}

pub fn settings_struct_to_u8(settings: KeySettings) -> u8 {
  let mut settings_byte: u8 = 0;
  if settings.master_key_changeable { settings_byte |= 1 << 0; }
  if !settings.list_needs_master { settings_byte |= 1 << 1; }
  if !settings.create_needs_master { settings_byte |= 1 << 2; }
  if settings.settings_changeable { settings_byte |= 1 << 3; }
  if settings.change_needs_key_auth { settings_byte |= 0b00001110; }
  if settings.keys_frozen { settings_byte |= 0b00001111; }
  settings_byte
}

pub fn change_key_settings(aid: u8, kid: u8, mut key: [u8; 16usize], version: Option<u8>, settings: KeySettings) {
  let settings_byte = settings_struct_to_u8(settings);
  let desfire = DesFire::initialize();
  desfire.connect();
  desfire.authenticate(kid, key, version);
  mifare::desfire_change_key_settings(desfire.tag, settings_byte);
  desfire.destruct();
}

pub fn get_key_settings(aid: u8, kid: u8) -> KeySettings {
  let desfire = DesFire::initialize();
  desfire.connect();
  let mut settings: u8 = 0;
  let mut max_keys: u8 = 0;
  let res = mifare::desfire_get_key_settings(desfire.tag, &mut settings, &mut max_keys);
  KeySettings {
    master_key_changeable: (settings & (1 << 0)) >> 0 == 1,
    list_needs_master: (settings & (1 << 1)) >> 1 == 0,
    create_needs_master: (settings & (1 << 2)) >> 2 == 0,
    settings_changeable: (settings & (1 << 3)) >> 3 == 1,
    change_needs_key_auth: (settings & 0b00001111) >> 4 == 14,
    keys_frozen: (settings & 0b00001111) >> 4 == 15
  }
}