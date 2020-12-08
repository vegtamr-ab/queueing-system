use crate::queueing_system::types::ConfidenceLevel;

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::{GString, clone};
use gtk::prelude::*;

use gtk::*;

pub fn get_confidence(str: &str) -> Result<ConfidenceLevel, &str> {
    match str {
        "Standard" => Ok(ConfidenceLevel::Standard),
        "High" => Ok(ConfidenceLevel::High),
        "Very High" => Ok(ConfidenceLevel::VeryHigh),
        _ => Err("Invalid confidence level"),
    }
}
