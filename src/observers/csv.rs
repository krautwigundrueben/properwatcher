use crate::models::Property;
use crate::observers::{Observer, ObserverError};
use crate::ApplicationConfig;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct CSVProperty {
  pub source: String,
  pub source_id: String,
  pub title: String,
  pub url: String,
  pub date: i64,
  pub city: String,
  pub price: f32,
  pub squaremeters: f32,
  pub plot_squaremeters: f32,
  pub address: String,
  pub rooms: f32,
  pub tags: String,
  pub latitude: f32,
  pub longitude: f32,
}

pub struct CSV {}

#[async_trait]
impl Observer for CSV {
  fn name(&self) -> String {
    String::from("csv")
  }

  fn init(&mut self, _: &ApplicationConfig) -> Result<(), String> {
    Ok(())
  }

  async fn observation(
    &self,
    app_config: &ApplicationConfig,
    property: &Property,
  ) -> Result<(), ObserverError> {
    if property.data.is_some() {
      let file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&app_config.csv.filename)
        .expect(
          format!(
            "Could not open file {} for reading.",
            &app_config.csv.filename
          )
          .as_str(),
        );

      let property_data = property.data.as_ref().unwrap();
      let csv_property = CSVProperty {
        source: property.source.clone(),
        source_id: property_data.externalid.clone(),
        title: property_data.title.clone(),
        url: property_data.url.clone(),
        date: property.date,
        city: property.city.clone(),
        price: property_data.price,
        squaremeters: property_data.squaremeters,
        plot_squaremeters: property_data.plot_squaremeters.unwrap_or(0.0),
        address: property_data.address.clone(),
        rooms: property_data.rooms,
        tags: property_data.tags.join(","),
        latitude: property
          .enrichments
          .get("latitude")
          .unwrap_or(&String::from("0"))
          .parse()
          .unwrap_or(0.0),
        longitude: property
          .enrichments
          .get("longitude")
          .unwrap_or(&String::from("0"))
          .parse()
          .unwrap_or(0.0),
      };

      // write csv entry temporarily into memory buffer
      let cursor_on_vec = std::io::Cursor::new(Vec::new());
      let buf_writer = std::io::BufWriter::new(cursor_on_vec);
      let mut csv_writer = csv::Writer::from_writer(buf_writer);
      csv_writer.serialize(csv_property).expect("Write CSV");
      let cursor_on_vec = csv_writer.into_inner().unwrap().into_inner().unwrap();
      let mut buf_reader = std::io::BufReader::new(cursor_on_vec);
      buf_reader.seek(std::io::SeekFrom::Start(0))?;
      let mut header_row = String::new();
      let mut data_row = String::new();
      buf_reader.read_line(&mut header_row)?;
      buf_reader.read_line(&mut data_row)?;

      // read first line of csv file
      let mut file_reader = std::io::BufReader::new(file);
      file_reader.seek(std::io::SeekFrom::Start(0))?;
      let mut first_line = String::new();
      file_reader.read_line(&mut first_line)?;
      let file = file_reader.into_inner();

      let mut file_writer = std::io::BufWriter::new(file);
      file_writer.seek(std::io::SeekFrom::End(0))?;

      // this is a new file
      if first_line.len() == 0 {
        file_writer.write(header_row.as_bytes())?;

      // file already present
      } else if header_row != first_line {
        eprintln!(
          "CSV file {} already present, but columns are not compatible!",
          app_config.csv.filename
        );
        eprintln!("Expected: '{}', but was: '{}'", &header_row, &first_line);
        std::process::exit(1);
      }

      // add new row
      file_writer.write(data_row.as_bytes())?;
    }
    Ok(())
  }
}
