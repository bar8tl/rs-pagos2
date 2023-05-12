// main.rs: Starts extension of Pagos1.0 EDICOM-file with Pagos2.0 fields ------
// [20220406-BAR8TL]
#![allow(unused)]

mod fixvalues;
mod pagos2;
mod rblib;
mod settings;

use crate::fixvalues::*;
use crate::pagos2::Pagos2Tp;
use crate::rblib::*;
use crate::settings::SettingsTp;
use std::fs;
use std::path::Path;

// Starts processes for command line options -----------------------------------
fn main() {
  let mut stg = SettingsTp::new_settings();
  stg.set_settings("_config.json");
  let s = stg.clone();
  for parm in s.prm.cmdpr {
    let s = stg.clone();
    if parm.optn == "txc" { // Perform processes for taxes calculation
      tax_calc(parm, s);
    } else {
      println!("Run option not valid");
    }
  }
}

// Calculation of taxes for new additional columns in Pagos2 file --------------
fn tax_calc(parm: settings::ParameTp, mut stg: SettingsTp) {
  stg.set_runvars(parm);
  let mut p = Pagos2Tp::new_pagos2();
  // For batch process: browse inputs directory
  if stg.modep == BATCH {
    for entry in fs::read_dir(&stg.inpdr).unwrap() {
      let s = stg.clone();
      let entry = entry.unwrap().path();
      if entry.is_dir() {
        continue;
      }
      let filid = Path::new(&entry).file_name().unwrap();
      let filnm = Path::new(&filid).file_stem().unwrap();
      let extsn = Path::new(&filid).extension().unwrap();
      let flide = filid.to_str().unwrap();
      let flnam = filnm.to_str().unwrap();
      let flext = extsn.to_str().unwrap();
      if s.ifilt.len() == 0 || (s.ifilt.len() > 0 &&
        pass_filter(&s.ifilt, flnam) && extsn == "xlsx") {
        p.proc_indiv_file(s, flnam, flext);
      }
    }
  // For individual process: use specified file
  } else {
    let s = stg.clone();
    let filid = stg.inpfl.clone();
    let filnm = Path::new(&filid).file_stem().expect("File not found");
    let extsn = Path::new(&filid).extension().unwrap();
    let flide = filid.as_str();
    let flnam = filnm.to_str().unwrap();
    let flext = extsn.to_str().unwrap();
    if s.ifilt.len() == 0 || (s.ifilt.len() > 0 &&
      pass_filter(&s.ifilt, flnam) && extsn == "xlsx") {
      p.proc_indiv_file(s, flnam, flext);
    }
  }
}
