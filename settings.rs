/*******************************************************************************
** settings.rs: Defines Pagos 2.0 Excel File pgm-level & run-level settings    *
** [20220406-BAR8TL]                                                           *
*******************************************************************************/
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::fixvalues::*;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::{Datelike, Duration, NaiveDate};
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::env;

/*******************************************************************************
** settings.rs: Establishes program and run level settings                     *
*******************************************************************************/
#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfd  : ConfigTp,
  pub IMPTO: String,       // IMPUESTO
  pub TIPOF: String,       // TIPOFACTOR
  pub OBJIM: String,       // OBJETOIMPUESTO
  pub TAB  : String,       // TAB
  pub DEC  : String,       // DEC
  pub DECPS: u32,          // DECIMAL_POS
  pub inpdr: String,       // INPUTS_DIR
  pub outdr: String,       // OUTPUTS_DIR
  pub ifilt: String,       // INPUTS_FILTER
  pub inpnm: String,       // INPUTS_NAMING
  pub outnm: String,       // OUTPUTS_NAMING
  pub optin: String,       // OPTION
  pub objnm: String,       // OBJECT_NAME
  pub modep: String,       // MODE
  pub inpfl: String,       // INPUT_FILE
  pub inppt: String,       // INPUT_PATH
  pub found: bool,
  pub dtsys: NaiveDateTime
}

impl SettingsTp {
  pub fn new_settings() -> SettingsTp {
    let mut stg = SettingsTp { ..Default::default() };
    stg.prm = ParamsTp::new_params();
    stg.cfd = ConfigTp::new_config();
    stg
  }

  pub fn set_settings(&mut self, cfnam: &str) {
    self.prm.scan_params();
    self.cfd.get_config(cfnam);
    let c = &self.cfd;
    self.IMPTO = if c.konst.IMPTO.len() > 0
      { c.konst.IMPTO.clone() } else { IMPTO.to_string() };
    self.TIPOF = if c.konst.TIPOF.len() > 0
      { c.konst.TIPOF.clone() } else { TIPOF.to_string() };
    self.OBJIM = if c.konst.OBJIM.len() > 0
      { c.konst.OBJIM.clone() } else { OBJIM.to_string() };
    self.TAB   = if c.konst.TAB.len()   > 0
      { c.konst.TAB.clone()   } else { TAB.to_string()   };
    self.DEC   = if c.konst.DEC.len()   > 0
      { c.konst.DEC.clone()   } else { DEC.to_string()   };
    self.inpdr = if c.progm.inpdr.len() > 0
      { c.progm.inpdr.clone() } else { INPDR.to_string() };
    self.outdr = if c.progm.outdr.len() > 0
      { c.progm.outdr.clone() } else { OUTDR.to_string() };
    self.ifilt = if c.progm.ifilt.len() > 0
      { c.progm.ifilt.clone() } else { IFILT.to_string() };
    self.inpnm = if c.progm.inpnm.len() > 0
      { c.progm.inpnm.clone() } else { INPNM.to_string() };
    self.outnm = if c.progm.outnm.len() > 0
      { c.progm.outnm.clone() } else { OUTNM.to_string() };
    self.DECPS = DEC.trim().parse().unwrap();
    self.dtsys = Local::now().naive_local();
  }

  pub fn set_runvars(&mut self, p: ParameTp) {
    if p.prm1.len() > 0 {
      self.objnm = p.prm1;
    } else {
      panic!("Error: Not possible to determine EDICOM Type name");
    }
    self.found = false;
    for run in &self.cfd.run {
      if p.optn == run.optin {
        if p.optn == "txc" {
          self.optin = p.optn.clone();
          self.objnm = if run.objnm.len() > 0
            { run.objnm.clone() } else { panic!("Object name is mandatory") };
          self.modep = if run.modep.len() > 0
            { run.modep.clone() } else { INDIV.to_string() };
          self.inpfl = if run.inpfl.len() > 0
            { run.inpfl.clone() } else { SAMPL.to_string() };
          if run.inpdr.len() > 0 {
            self.inpdr = run.inpdr.clone();
          }
          if run.outdr.len() > 0 {
            self.outdr = run.outdr.clone();
          }
          self.inppt = format!("{}{}", self.inpdr, self.inpfl);
        }
        self.found = true;
        break;
      }
    }
  }
}

/*******************************************************************************
** config.rs: Reads config file and gets run parameter                         *
*******************************************************************************/
#[derive(Debug, Clone, Default, Deserialize)]
pub struct KonstTp { // konst
  #[serde(default)]
  pub IMPTO: String, // IMPUESTO
  #[serde(default)]
  pub TIPOF: String, // TIPOFACTOR
  #[serde(default)]
  pub OBJIM: String, // OBJETOIMPUESTO
  #[serde(default)]
  pub TAB  : String, // TAB
  #[serde(default)]
  pub DEC  : String  // DEC
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgmTp { // progm
  #[serde(default)]
  pub inpdr: String, // inputs_dir
  #[serde(default)]
  pub outdr: String, // outputs_dir
  #[serde(default)]
  pub ifilt: String, // inputs_filter
  #[serde(default)]
  pub inpnm: String, // inputs_naming
  #[serde(default)]
  pub outnm: String  // outputs_naming
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RunTp {   // run
  pub optin: String, // option
  pub objnm: String, // object_name
  #[serde(default)]
  pub modep: String, // mode
  #[serde(default)]
  pub inpdr: String, // inputs_dir
  #[serde(default)]
  pub outdr: String, // outputs_dir
  #[serde(default)]
  pub inpfl: String, // input_file
  #[serde(default)]
  pub ifilt: String, // inputs_filter
  #[serde(default)]
  pub inpnm: String, // inputs_naming
  #[serde(default)]
  pub outnm: String  // outputs_naming
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigTp {
  pub konst: KonstTp,
  pub progm: ProgmTp,
  pub run  : Vec<RunTp>
}

impl ConfigTp {
  pub fn new_config() -> ConfigTp {
    let cfg = ConfigTp{ ..Default::default() };
    cfg
  }

  pub fn get_config(&mut self, fname: &str) {
    let f = File::open(fname).unwrap();
    let cfg: ConfigTp = serde_json::from_reader(f)
      .expect("JSON not well-formed");
    self.konst = cfg.konst;
    self.progm = cfg.progm;
    self.run   = cfg.run;
  }
}

/*******************************************************************************
** params.rs: Gets a list of command-line parameters                           *
*******************************************************************************/
#[derive(Debug, Clone, Default)]
pub struct ParameTp {
  pub optn: String,
  pub prm1: String,
  pub prm2: String
}

#[derive(Debug, Clone, Default)]
pub struct ParamsTp {
  pub cmdpr: Vec<ParameTp>,
  pub messg: String
}

impl ParamsTp {
  pub fn new_params() -> ParamsTp {
    let prm = ParamsTp { cmdpr: Vec::new(), messg: String::from("") };
    prm
  }

  pub fn scan_params(&mut self) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
      self.messg = String::from("Run option missing");
      return;
    }
    let mut argn: i32 = 0;
    for curarg in args {
      if argn > 0 {
        if curarg[0..1] == "-".to_string() || curarg[0..1] == "/".to_string() {
          let mut optn: String = curarg[1..].trim().to_lowercase();
          let mut prm1: String = "".to_string();
          let mut prm2: String = "".to_string();
          if optn != "".to_string() {
            let idx = optn.find(":");
            if idx != None {
              let i = idx.unwrap();
              prm1 = optn[i + 1..].trim().to_string();
              optn = optn[..i].trim().to_string();
              let idx = prm1.find(":");
              if idx != None {
                let j = idx.unwrap();
                prm2 = prm1[j + 1..].trim().to_string();
                prm1 = prm1[..j].trim().to_string();
              }
            }
            self.cmdpr.push(ParameTp { optn, prm1, prm2 });
          }
        } else {
          self.messg = String::from("Run option missing");
        }
      }
      argn += 1;
    }
  }
}
