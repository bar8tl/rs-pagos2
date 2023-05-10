/*******************************************************************************
** pagos2.rs: Extend Pagos1.0 EDICOM-file with Pagos2.0 fields (Core Logic)    *
** [20220406-BAR8TL]                                                           *
*******************************************************************************/
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::fixvalues::{FixvaluesTp, *};
use crate::rblib::*;
use crate::settings::SettingsTp;
use calamine::DataType;
use calamine::{Reader, Xlsx, open_workbook};
use chrono::NaiveDate;
use std::num;
use std::fs::File;
use std::io::Write;

type LineInvoStr = Vec<String>;
type LineInvoNum = Vec<f32>;

#[derive(Debug, Clone, Default)]
pub struct Pagos2Tp {
  pub s    : SettingsTp,       // Program and Run Settings
  pub k    : FixvaluesTp,      // Hardcode
  pub t    : ItablesTp,        // Internal tables
  pub lxs  : [String; 28],     // Input excel line alpha values (array)
  pub lxf  : [f32; 28],        // Input excel line num values   (array)
  pub lis  : LineInvoStr,      // Invoice single line alpha values
  pub lif  : LineInvoNum,      // Invoice single line num values (invo curr)
  pub ljf  : LineInvoNum,      // Invoice single line num values (paym curr)
  pub cif  : LineInvoNum,      // Invoice cumulative amounts (invo curr)
  pub cjf  : LineInvoNum,      // Invoice cumulative amounts (paym curr)
  pub lps  : Vec<String>,      // Payment single line alpha values
  pub lpf  : Vec<f32>,         // Payment single line num values
  pub gis  : Vec<LineInvoStr>, // Invoice lines in paymnt alpha values
  pub gif  : Vec<LineInvoNum>, // Invoice lines in paymnt num values (invo curr)
  pub gjf  : Vec<LineInvoNum>, // Invoice lines in paymnt num values (paym curr)
  pub fiv  : [bool; 7],        // First invoice flags           (array)
  pub ipp  : usize,            // Index for invoices within a payments
  pub fline: bool,             // First line in input file
  pub recn : i32,              // Input file records number
  pub doctp: String,           // Document type
  pub oline: String,           // Output line
}

impl Pagos2Tp {
  pub fn new_pagos2() -> Pagos2Tp {
    let p = Pagos2Tp { ..Default::default() };
    p
  }

  pub fn proc_indiv_file(&mut self, stg: SettingsTp, f: &str) {
    self.s = stg.clone();
    self.k = FixvaluesTp::new_fixvalues();
    self.k.set_fixvalues();
    self.t = ItablesTp::new_itables();
    self.t.get_itables();
    self.allocate_storage(); // allocate memory for work vectors
    let mut OF = File::create(format!("{}{}", self.s.outdr, self.s.inpfl))
      .expect("creation failed");
    let mut i: usize = 0;
    let mut excel: Xlsx<_> = open_workbook(&self.s.inppt).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range(TAB) {
      for row in r.rows() {
        let s = stg.clone();
        self.get_linefields(row, i);
        self.doctp = self.t.seek_doctp(&self.lxs[CMPNY], &self.lxs[DOCTP]);
        match self.doctp.as_str() {
          TITLE => { self.print_title(&mut OF);   },
          PAYMT => { self.proc_paymline(&mut OF); },
          INVOI => { self.proc_involine();        },
              _ => {},
        };
        i += 1;
        self.lxs = Default::default();
        self.lxf = Default::default();
      }
      self.build_paymline(&mut OF).build_involines(&mut OF);
      ren_inpfile(self.s.inpdr.clone(), self.s.inpfl.clone());
      ren_outfile(self.s.outdr.clone(), self.s.inpfl.clone());
    }
  }

  // Logic for Payments
  fn proc_paymline(&mut self, mut OF: &mut File) {
    if self.fline {
      self.fline = false;
    } else {
      self.build_paymline(&mut OF).build_involines(&mut OF).reset_paymdata();
    }
    self.store_paym();
  }

  // Logic for Invoices
  fn proc_involine(&mut self) {
    let txdta = self.t.seek_taxcd(&self.lxs[CMPNY], &self.lxs[TAXCD]);
    let trate: f32 = txdta.trate;
    let wrate: f32 = txdta.wrate;
    let amtic: f32 = self.lxf[PYAMT];
    let mut convf: f32 = 1.0;
    if self.lps[CURCY] == MXN.to_string() {
      if self.lxs[CURCY] == MXN.to_string() {
        convf = 1.0;
      } else {
        convf = 1.0 / self.lxf[EXCHG];
      }
    } else {
      convf = self.lpf[EXCHG];
    }
    self.lis[IOBJI]  = self.s.OBJIM.clone();
    self.lif[ITBAS]  = amtic / (1.0 + trate - wrate);
    self.ljf[ITBAS]  = amtic / (1.0 + trate - wrate) * convf;
    self.lis[ITIMP]  = self.s.IMPTO.clone();
    self.lis[ITFAC]  = self.s.TIPOF.clone();
    self.lif[ITRTE]  = trate;
    self.lif[ITAMT]  = self.lif[ITBAS] * trate;
    self.ljf[ITAMT]  = self.lif[ITBAS] * trate * convf;
    self.cif[TPAYM] += amtic;
    self.cjf[TPAYM] += amtic * convf;
    if trate == 0.16 {
      self.cif[PTB16] += self.lif[ITBAS];
      self.cjf[PTB16] += self.ljf[ITBAS];
      self.cif[PTA16] += self.lif[ITAMT];
      self.cjf[PTA16] += self.ljf[ITAMT];
    } else if trate == 0.08 {
      self.cif[PTB08] += self.lif[ITBAS];
      self.cjf[PTB08] += self.ljf[ITBAS];
      self.cif[PTA08] += self.lif[ITAMT];
      self.cjf[PTA08] += self.ljf[ITAMT];
    } else if trate == 0.00 {
      self.cif[PTB00] += self.lif[ITBAS];
      self.cjf[PTB00] += self.ljf[ITBAS];
      self.cif[PTA00] += self.lif[ITAMT];
      self.cjf[PTA00] += self.ljf[ITAMT];
    }
    self.lif[IRBAS] = 0.0;
    self.ljf[IRBAS] = 0.0;
    self.lis[IRIMP] = "".to_string();
    self.lis[IRFAC] = "".to_string();
    self.lif[IRRTE] = 0.0;
    self.lif[IRAMT] = 0.0;
    self.ljf[IRAMT] = 0.0;
    if wrate != 0.00 {
      self.lif[IRBAS] = self.lif[ITBAS];
      self.ljf[IRBAS] = self.lif[ITBAS] + convf;
      self.lis[IRIMP] = self.s.IMPTO.clone();
      self.lis[IRFAC] = self.s.TIPOF.clone();
      self.lif[IRRTE] = wrate;
      self.lif[IRAMT] = self.lif[IRBAS] * wrate;
      self.ljf[IRAMT] = self.lif[IRBAS] * wrate * convf;
      if wrate == 0.16 {
        self.cif[PRB16] += self.lif[IRBAS];
        self.cjf[PRB16] += self.ljf[IRBAS];
        self.cif[PRA16] += self.lif[IRAMT];
        self.cjf[PRA16] += self.ljf[IRAMT];
      } else if wrate == 0.08 {
        self.cif[PRB08] += self.lif[IRBAS];
        self.cjf[PRB08] += self.ljf[IRBAS];
        self.cif[PRA08] += self.lif[IRAMT];
        self.cjf[PRA08] += self.ljf[IRAMT];
      }
    }
    self.store_invo();
  }

  fn get_linefields (&mut self, row: &[calamine::DataType], k: usize) {
    let mut temp: String = Default::default();
    for (j, c) in row.iter().enumerate() {
      match *c {
        DataType::Empty           => { temp = "".to_string();   },
        DataType::String  (ref s) => { temp =  s.to_string();   },
        DataType::Float   (ref f) |
        DataType::DateTime(ref f) => { temp = format!("{}", f); },
        DataType::Int     (ref i) => { temp = format!("{}", i); },
        DataType::Error   (ref e) => { temp = format!("{}", e); },
        DataType::Bool    (ref b) => { temp = format!("{}", b); },
      };
      if k == 0 {
        self.lxs[j] = temp.clone(); // first row in excel should be the title
      } else {
        if contains(&self.k.NUMER, &j) {
          match temp.parse::<f32>() {
            Ok(rslt)   => {
              self.lxf[j] = rslt;
              self.lxs[j] = format!("{:.2}", rb_round(rslt, 2));
            },
            Err(error) => {
              self.lxf[j] = 0.0;
              self.lxs[j] = "0.00".to_string();
            }
          }
        } else if j == EXCHG {
          match temp.parse::<f32>() {
            Ok(rslt) => {
              self.lxf[j] = rslt;
              self.lxs[j] = format!("{:.6}", rb_round(rslt, 6));
            },
            Err(error) => {
              if temp == "" || temp == MXN {
                self.lxf[j] = 1.0;
              }
            }
          }
          if self.lxf[j] == 0.0 {
            self.lxs[j] = "".to_string();
          }
        } else {
          self.lxs[j] = temp.clone();
        }
      }
    }
    if self.doctp.as_str() == PAYMT {
      if self.lxf[PYAMT] == 0.0 {
        self.lxs[PYAMT] = "".to_string();
      }
      if self.lxf[PRVAM] == 0.0 {
        self.lxs[PRVAM] = "".to_string();
      }
      if self.lxf[CURAM] == 0.0 {
        self.lxs[CURAM] = "".to_string();
      }
    }
  }

  fn store_paym(&mut self) {
    for i in 0..28 {
      self.lps[i] = self.lxs[i].clone();
      self.lpf[i] = self.lxf[i].clone();
    }
    if self.lxs[CURCY] == MXN.to_string() {
      self.lpf[EXCHG] = 1.0;
    } else {
      self.lpf[EXCHG] = self.lxf[EXCHG];
    }
  }

  fn store_invo(&mut self) {
    for i in 0..28 {
      self.lis[i] = self.lxs[i].clone();
      self.lif[i] = self.lxf[i].clone();
    }
    if self.ipp < self.gis.len() {
      self.gis[self.ipp] = self.lis.clone();
      self.gif[self.ipp] = self.lif.clone();
      self.gjf[self.ipp] = self.ljf.clone();
    } else {
      self.gis.push(self.lis.clone());
      self.gif.push(self.lif.clone());
      self.gjf.push(self.ljf.clone());
    }
    self.ipp += 1;
    // Reset cumulative amounts of Multiple-taxcode payments
    if self.lif[ITRTE] == 0.16 {
      if self.fiv[FIT16] {
        self.lps[PTI16] = self.lis[ITIMP].clone();
        self.lps[PTF16] = self.lis[ITFAC].clone();
        self.lpf[PTR16] = self.lif[ITRTE].clone();
        self.fiv[FIT16] = false;
      }
    }
    if self.lif[IRRTE] == 0.16 {
      if self.fiv[FIR16] {
        self.lps[PRI16] = self.lis[IRIMP].clone();
        self.lps[PRF16] = self.lis[IRFAC].clone();
        self.lpf[PRR16] = self.lif[IRRTE].clone();
        self.fiv[FIR16] = false;
      }
    }
    if self.lif[ITRTE] == 0.08 {
      if self.fiv[FIT08] {
        self.lps[PTI08] = self.lis[ITIMP].clone();
        self.lps[PTF08] = self.lis[ITFAC].clone();
        self.lpf[PTR08] = self.lif[ITRTE].clone();
        self.fiv[FIT08] = false;
      }
    }
    if self.lif[IRRTE] == 0.08 {
      if self.fiv[FIR08] {
        self.lps[PRI08] = self.lis[IRIMP].clone();
        self.lps[PRF08] = self.lis[IRFAC].clone();
        self.lpf[PRR08] = self.lif[IRRTE].clone();
        self.fiv[FIR08] = false;
      }
    }
    if self.lif[ITRTE] == 0.0 {
      if self.fiv[FIT00] {
        self.lps[PTI00] = self.lis[ITIMP].clone();
        self.lps[PTF00] = self.lis[ITFAC].clone();
        self.lpf[PTR00] = self.lif[ITRTE].clone();
        self.fiv[FIT00] = false;
      }
    }
    if self.lif[IRRTE] == 0.0 {
      if self.fiv[FIR00] {
        self.lps[PRI00] = self.lis[IRIMP].clone();
        self.lps[PRF00] = self.lis[IRFAC].clone();
        self.lpf[PRR00] = self.lif[IRRTE].clone();
        self.fiv[FIR00] = false;
      }
    }
  }

  fn reset_paymdata(&mut self) -> &mut Pagos2Tp {
    self.ipp = 0;
    for i in 0..self.fiv.len()-1 {
      self.fiv[i] = true;
    }
    for i in 0..self.gis.len() {
      self.gis[i] = Default::default();
      self.gif[i] = Default::default();
      self.gjf[i] = Default::default();
    }
    for i in 0..=79 {
      self.lps[i] = "".to_string();
      self.lpf[i] = 0.0;
      self.lis[i] = "".to_string();
      self.lif[i] = 0.0;
      self.ljf[i] = 0.0;
      self.cif[i] = 0.0;
      self.cjf[i] = 0.0;
    }
    self
  }

  fn build_paymline(&mut self, mut OF: &mut File) -> &mut Pagos2Tp {
    self.lpf[TRETN] = self.cjf[PRA16] + self.cjf[PRA08] + self.cjf[PRA00];
    self.lpf[TTB16] = self.cjf[PTB16].clone();
    self.lpf[TTA16] = self.cjf[PTA16].clone();
    self.lpf[TTB08] = self.cjf[PTB08].clone();
    self.lpf[TTA08] = self.cjf[PTA08].clone();
    self.lpf[TTB00] = self.cjf[PTB00].clone();
    self.lpf[TTA00] = self.cjf[PTA00].clone();
    self.lpf[TPAYM] = self.cjf[TPAYM].clone();
    self.lpf[PTB16] = self.cjf[PTB16].clone();
    self.lpf[PTA16] = self.cjf[PTA16].clone();
    self.lpf[PRB16] = self.cjf[PRB16].clone();
    self.lpf[PRA16] = self.cjf[PRA16].clone();
    self.lpf[PTB08] = self.cjf[PTB08].clone();
    self.lpf[PTA08] = self.cjf[PTA08].clone();
    self.lpf[PRB08] = self.cjf[PRB08].clone();
    self.lpf[PRA08] = self.cjf[PRA08].clone();
    self.lpf[PTB00] = self.cjf[PTB00].clone();
    self.lpf[PTA00] = self.cjf[PTA00].clone();
    self.lpf[PRB00] = self.cjf[PRB00].clone();
    self.lpf[PRA00] = self.cjf[PRA00].clone();
    self.print_paymline(OF);
    self
  }

  fn build_involines(&mut self, mut OF: &mut File) -> &mut Pagos2Tp {
    for i in 0..self.ipp {
      self.print_involine(OF, self.gis[i].clone(), self.gif[i].clone());
    }
    self
  }

  fn print_title(&mut self, mut OF: &mut File) {
    self.recn += 1;
    for i in 0..=79 {
      self.app_strline(self.k.TT[i].clone());
    }
    OF.write_all(self.oline.as_bytes()).expect("write failed");
    self.oline = "\r\n".to_string();
  }

  fn print_paymline(&mut self, mut OF: &mut File) {
    self.recn += 1;
    for i in 0..=79 { // Print original columns
      if i <= 27 {
        self.app_strline(self.lps[i].clone());
      }
      if i >= 28 && i <= 79 {
        if contains(&self.k.ALPHA, &i) {
          self.app_strline(self.lps[i].clone());
        } else {
          self.app_numline(self.lpf[i].clone());
        }
      }
    }
    OF.write_all(self.oline.as_bytes()).expect("write failed");
    self.oline = "\r\n".to_string();
  }

  fn print_involine(&mut self, mut OF: &mut File, lis: LineInvoStr,
    lif: LineInvoNum) {
    self.recn += 1;
    for i in 0..=79 {
      if i <= 27 {
        self.app_strline(lis[i].clone());
      }
      if i >= 28 && i <= 79 {
        if contains(&self.k.ALPHA, &i) {
          self.app_strline(lis[i].clone());
        } else {
          self.app_numline(lif[i].clone());
        }
      }
    }
    OF.write_all(self.oline.as_bytes()).expect("write failed");
    self.oline = "\r\n".to_string();
  }

  fn app_strline(&mut self, val: String) {
    self.oline.push_str(format!("{}|", &val).as_str());
  }

  fn app_numline(&mut self, val: f32) {
    if val == 0.0 {
      self.oline.push_str(format!("|").as_str());
    } else {
      self.oline.push_str(format!("{}|", &val).as_str());
    }
  }

  fn allocate_storage(&mut self) {
    self.ipp   = 0;
    self.fline = true;
    for i in 0..self.fiv.len()-1 {
      self.fiv[i] = true;
    }
    for i in 0..=79 {
      self.lps.push("".to_string());
      self.lpf.push(0.0);
      self.lis.push("".to_string());
      self.lif.push(0.0);
      self.ljf.push(0.0);
      self.cif.push(0.0);
      self.cjf.push(0.0);
    }
  }
}
