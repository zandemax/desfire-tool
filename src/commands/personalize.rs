use super::helper::*;
use colored::*;
use std::io::{self, Write};

pub fn personalize() {
  println!("{}", "Personalizing your card!".green());
  let generate_master_key: bool = ask_bool("Should I generate a new Master key for you? [yes/no]:  ");
  let random_uid: bool = ask_bool("Do you want your card to respond with a random UID? (CAUTION: Once turned on, this cannot be turned off again!) [yes|no]: ");
  let format_lock: bool = ask_bool("Do you want to lock your card from being formatted? (CAUTION: Once turned on, this cannot be turned off again!) [yes|no]: ");
  let create_needs_master: bool = ask_bool("Should you need the Master Key to create Applications? (Delete always requires master) [yes|no]: ");
  let list_needs_master: bool = ask_bool("Should you need the Master Key to list Applications? [yes|no]: ");

}

struct PersonalizationSettings {
  generate_master_key: bool,
  app_create_needs_master: bool,
  
}