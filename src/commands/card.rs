use super::super::CardCommand;
use super::super::desfire::*;
use colored::*;

pub fn card(command: CardCommand) {
  match command {
    CardCommand::Configure  => configure(),
    CardCommand::Format     => format(),
    CardCommand::Info       => info(),
  }
}

fn configure() {

}

fn format() {

}

fn info() {
  let card_version = get_card_info();
  println!("{}", "Reading your Card ...".green());
  println!("Card Version: {}", card_version.card_version);
  println!("Free Space:   {} b", card_version.free_memory);
  println!("Size:         {} kb", card_version.size);
  println!("UID:          {:X?}", card_version.uid);
}