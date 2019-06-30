extern crate freefare as mod_freefare;
extern crate freefare_sys;
use mod_freefare::*;
use nfc;
use nfc_sys;

pub struct DesFire {
  device: *mut nfc_sys::nfc_device,
  context: *mut nfc::ffi::nfc_context,
  pub tag: freefare_sys::FreefareTag
}

impl DesFire {
  
  pub fn initialize () -> DesFire {
    let mut context = nfc::context::new();
    
    if context.is_null() {
        panic!("Unable to initialize new NFC context!");
    }

    // Initialize libnfc
    nfc::init(&mut context);

    let mut devices = Vec::with_capacity(8);

    let device_count = nfc::list_devices(context, devices.as_mut_ptr() , 4);

    if device_count <= 0 {
      panic!("No NFC Devices found!")
    }

    for i in 0..device_count {
      let device = unsafe { nfc_sys::nfc_open(&mut to_sys_context(*context), devices.as_mut_ptr()) };
      let tags = unsafe { std::slice::from_raw_parts(freefare::get_tags(device), 1) };
      return DesFire { device: device, context: context, tag: tags[0] }
    }
    
    panic!("Could not initialize NFC Library!")
  }

  fn get_tag(&self) -> freefare_sys::FreefareTag {
    // if let freefare_sys::Enum_freefare_tag_type::MIFARE_DESFIRE = freefare::get_tag_type(self.tag) { return self.tag }
    return self.tag
    // panic!("No MIFARE DesFire Tag found!")
  }

  pub fn connect(&self) {
    let tag = self.get_tag();
    let res = mifare::desfire_connect(tag);
    if res < 0 { panic!("cannot connect to tag!") }
    let mut info = freefare_sys::Struct_mifare_desfire_version_info { ..Default::default() };
    let res = mifare::desfire_get_version(tag, &mut info);
    if res < 0 { panic!("cannot read tag version!") }
    if info.hardware.version_major < 1 { panic!("Tag older than EV1!")}
    unsafe { println!("Found tag {} with UID {}", *freefare::get_tag_friendly_name(tag), *freefare::get_tag_uid(tag)); }
  }

  pub fn authenticate(&self, key_id: u8, mut key: [u8; 16usize], version: Option<u8>) {
    let key = mifare::desfire_aes_key_new_with_version(&mut key[0], version.unwrap_or_else(|| 0));
    let res = mifare::desfire_authenticate_aes(self.tag, key_id, key);
    mifare::desfire_key_free(key);
  }

  pub fn destruct(&self) {
    mifare::desfire_disconnect(self.tag);
    // freefare::free_tags(&self.tag);
    unsafe { nfc_sys::nfc_close(self.device); }
    nfc::exit(self.context)
  }

}

fn to_sys_context(context: nfc::ffi::nfc_context) -> nfc_sys::nfc_context {
  nfc_sys::nfc_context {
    allow_autoscan: context.allow_autoscan,
    allow_intrusive_scan: context.allow_intrusive_scan,
    log_level: context.log_level,
    ..Default::default()
  }
}