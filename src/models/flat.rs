use chrono::prelude::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PropertyType {
  House,
  Flat,
}

impl std::str::FromStr for PropertyType {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, ()> {
    match s {
      "house" => Ok(PropertyType::House),
      "flat" => Ok(PropertyType::Flat),
      _ => Err(()),
    }
  }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum ContractType {
  Rent,
  Buy,
}

impl std::str::FromStr for ContractType {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, ()> {
    match s {
      "rent" => Ok(ContractType::Rent),
      "buy" => Ok(ContractType::Buy),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
  pub latitude: f32,
  pub longitude: f32,
  pub uncertainty: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
  pub source: String,
  pub date: i64,
  pub city: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<PropertyData>,
  #[serde(skip_serializing)]
  pub enrichments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyData {
  pub price: f32,
  pub contract_type: ContractType,
  pub property_type: PropertyType,
  pub squaremeters: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plot_squaremeters: Option<f32>,
  pub address: String,
  pub title: String,
  pub externalid: String,
  pub rooms: f32,
  #[serde(skip_serializing)]
  pub tags: Vec<String>,
  pub url: String,
}

impl PartialEq for Property {
  fn eq(&self, other: &Self) -> bool {
    (self.city == other.city
      && self.source == other.source
      && self.data.is_some()
      && other.data.is_some()
      && self.data.as_ref().unwrap().externalid == other.data.as_ref().unwrap().externalid)
      || self.is_equal_to(other)
  }
}

impl Property {
  pub fn id(&self) -> String {
    match &self.data {
      Some(d) => format!("{}-{}", self.source, d.externalid),
      None => format!("{}-{}", self.source, self.date),
    }
  }

  fn is_equal_to(&self, other: &Self) -> bool {
    let special_characters_regex = Regex::new("[^0-9a-zA-Z]+").unwrap();
    self.city == other.city
      && match (&self.data, &other.data) {
        (None, None) => false,
        (Some(ref d1), Some(ref d2)) => {
          special_characters_regex.replace_all(&d1.title.to_lowercase(), "")
            == special_characters_regex.replace_all(&d2.title.to_lowercase(), "")
        }
        _ => false,
      }
  }

  #[cfg(test)]
  pub fn dummy(price: f32, squaremeters: f32, rooms: f32) -> Property {
    Property {
      date: Utc::now().timestamp(),
      enrichments: HashMap::new(),
      source: String::new(),
      city: String::new(),
      data: Some(PropertyData {
        price,
        squaremeters,
        address: String::new(),
        title: String::new(),
        externalid: String::new(),
        plot_squaremeters: None,
        rooms,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::new(),
      }),
    }
  }

  pub fn new(source: String, city: String) -> Property {
    Property {
      date: Utc::now().timestamp(),
      source,
      data: None,
      city,
      enrichments: HashMap::new(),
    }
  }

  pub fn fill(&self, data: &PropertyData) -> Property {
    Property {
      city: self.city.clone(),
      source: self.source.to_owned(),
      date: self.date,
      data: Some(data.clone()),
      enrichments: self.enrichments.clone(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::models::ContractType;
  use crate::models::Property;
  use crate::models::PropertyData;
  use crate::models::PropertyType;
  use std::collections::HashMap;

  #[test]
  fn compare_flat_too_simple() {
    // GIVEN
    let flat_a = Property::new(String::from("some source A"), String::from("Munich"));
    let flat_b = Property::new(String::from("some source A"), String::from("Munich"));

    // THEN
    assert_ne!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_simple() {
    // GIVEN
    let flat_a = Property {
      source: String::from("some source A"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is some title"),
        externalid: String::from("1"),
        plot_squaremeters: None,
        rooms: 3.,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };
    let flat_b = Property {
      source: String::from("some source A"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("This is some other address"),
        title: String::from("This is some other title"),
        externalid: String::from("1"),
        plot_squaremeters: None,
        rooms: 1.,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };

    // THEN
    assert_eq!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_complex() {
    // GIVEN
    let flat_a = Property {
      source: String::from("some source A"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is some title"),
        externalid: String::from("1a"),
        plot_squaremeters: None,
        rooms: 3.,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };
    let flat_b = Property {
      source: String::from("some source B"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is some title"),
        externalid: String::from("1b"),
        plot_squaremeters: None,
        rooms: 3.,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };

    // THEN
    assert_eq!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_complex_special_chars() {
    // GIVEN
    let flat_a = Property {
      source: String::from("some source A"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is% some title!"),
        externalid: String::from("1a"),
        plot_squaremeters: None,
        rooms: 3.,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };
    let flat_b = Property {
      source: String::from("some source B"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 101.,
        squaremeters: 101.,
        address: String::from("Some other address"),
        title: String::from("This is some title"),
        externalid: String::from("1b"),
        plot_squaremeters: None,
        rooms: 3.5,
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };

    // THEN
    assert_eq!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_not_equal() {
    // GIVEN
    let flat_a = Property::new(String::from("some source A"), String::from("Munich"));
    let flat_b = Property::new(String::from("some source B"), String::from("Munich"));

    // THEN
    assert_ne!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_complex_not_equal() {
    // GIVEN
    let flat_a = Property {
      source: String::from("some source A"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is% some title!"),
        externalid: String::from("1a"),
        plot_squaremeters: Some(100.0),
        rooms: 3.,
        contract_type: ContractType::Buy,
        property_type: PropertyType::House,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };
    let flat_b = Property {
      source: String::from("some source B"),
      city: String::from("Munich"),
      date: 0,
      data: Some(PropertyData {
        price: 101.,
        squaremeters: 101.,
        address: String::from("Some other address"),
        title: String::from("This is some other title"),
        externalid: String::from("1b"),
        rooms: 3.5,
        plot_squaremeters: Some(200.0),
        contract_type: ContractType::Buy,
        property_type: PropertyType::House,
        tags: vec![],
        url: String::from(""),
      }),
      enrichments: HashMap::new(),
    };

    // THEN
    assert_ne!(flat_a, flat_b);
  }
}
