use super::super::KeyCommand;
use super::super::desfire::*;
use colored::*;

pub fn key(command: KeyCommand, aid: u8) {
  match command {
    KeyCommand::Add { kid } =>        add(aid, kid),
    KeyCommand::Change { kid } =>     change(aid, kid),
    KeyCommand::Configure { kid } =>  configure(aid, kid),
    KeyCommand::Remove { kid } =>     remove(aid, kid),
    KeyCommand::Info { kid } =>       info(aid, kid),
    KeyCommand::List =>               list()
  }
}

fn add (aid: u8, kid: u8) {}

fn change (aid: u8, kid: u8) {}

fn configure (aid: u8, kid: u8) {}

fn remove (aid: u8, kid: u8) {}

fn info (aid: u8, kid: u8) {
  let key_settings = get_key_settings(aid, kid);
  println!("{}", "Reading settings for the specified Key ...".green());
  println!("Master Key is changeable:                 {}", key_settings.master_key_changeable);
  println!("Directory List needs Master key:          {}", key_settings.list_needs_master);
  println!("Creating Applications needs Master key:   {}", key_settings.create_needs_master);
  println!("Key Settings are changeable:              {}", key_settings.settings_changeable);
}

fn list () {}