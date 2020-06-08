use crate::models::{ContractType, Property, PropertyType};
use crate::observers::Observer;
use crate::ApplicationConfig;
use std::collections::HashMap;

use num_format::{Locale, ToFormattedString};

pub struct Telegram {}

impl Observer for Telegram {
  fn observation(&self, app_config: &ApplicationConfig, property: &Property) -> () {
    if app_config.telegram.enabled {
      match property.data {
        Some(ref property_data) => {
          let url = get_url(&property.source, property_data.externalid.to_owned());
          let property_type = match property_data.property_type {
            PropertyType::Flat => "flat",
            PropertyType::House => "house",
          };
          let contract_type = match property_data.contract_type {
            ContractType::Buy => "Buying",
            ContractType::Rent => "Renting",
          };
          let mut msg = String::from(format!(
            "Hey guys, found *a new {} on {}*!\n",
            property_type, property.source,
          ));
          msg.push_str(&format!("{}\n", property_data.address));
          msg.push_str(&format!("[{}]({})\n", property_data.title, url));
          msg.push_str(&format!(
            "{} the {} costs *{} €*.\n",
            contract_type,
            property_type,
            (property_data.price as i32).to_formatted_string(&Locale::en)
          ));
          msg.push_str(&format!(
            "It has *{} rooms* and *{} sqm*.\n",
            property_data.rooms,
            (property_data.squaremeters as i32).to_formatted_string(&Locale::en)
          ));
          if property_data.plot_squaremeters.is_some() {
            msg.push_str(&format!(
              "Plot of land has a size of *{} sqm*.",
              (property_data.plot_squaremeters.unwrap() as i32).to_formatted_string(&Locale::en),
            ));
          }
          send_telegram_message(app_config, msg);
        }
        None => (),
      }
    }
  }
}

fn send_telegram_message(app_config: &ApplicationConfig, msg: String) -> () {
  let chat_id = &app_config.telegram.chat_id;
  let api_key = &app_config.telegram.api_key;

  let client = reqwest::blocking::Client::new();
  let mut map = HashMap::new();
  map.insert("chat_id", format!("{}", chat_id));
  map.insert("text", msg);
  map.insert("parse_mode", String::from("Markdown"));

  let result = client
    .post(&format!(
      "https://api.telegram.org/bot{}/sendMessage",
      api_key
    ))
    .json(&map)
    .send();

  match result {
    Ok(response) => {
      if response.status() != 200 {
        println!(
          "Error while sending message: {:?}",
          response.text().unwrap()
        )
      }
    }
    Err(e) => println!("{}", e),
  }
}

fn get_url(source: &String, external_id: String) -> String {
  match &source[..] {
    "immoscout" => format!("http://www.immobilienscout24.de/expose/{}", external_id),
    "immowelt" => format!("https://www.immowelt.de/expose/{}", external_id),
    "sueddeutsche" => format!(
      "https://immobilienmarkt.sueddeutsche.de/Wohnungen/mieten/Muenchen/Wohnung/{}?comeFromTL=1",
      external_id
    ),
    "wggesucht" => format!("https://www.wg-gesucht.de/{}", external_id),
    "wohnungsboerse" => format!("https://www.wohnungsboerse.net/immodetail/{}", external_id),
    _ => String::from(""),
  }
}
