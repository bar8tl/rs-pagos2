/*******************************************************************************
** fixvalues.rs: Constants defined for default values for program settings     *
** [20220406-BAR8TL]                                                           *
*******************************************************************************/
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::env;

// Default values
pub const INPDR: &str  = ".\\files\\input\\";              // INPUTS_DIR
pub const OUTDR: &str  = ".\\files\\output\\";             // OUTPUTS_DIR
pub const SAMPL: &str  = "sample.xlsx";                    // SAMPLE_FILE
pub const IFILT: &str  = "!(*processed*)";                 // INPUTS_FILTER
pub const INPNM: &str  = "dtsys'_'+inpfl'_inp_processed'"; // INPUTS_NAMING
pub const OUTNM: &str  = "dtsys'_'+inpfl'_out'";           // OUTPUTS_NAMING
pub const IMPTO: &str  = "002";                            // IMPUESTO
pub const TIPOF: &str  = "Tasa";                           // TIPOFACTOR
pub const OBJIM: &str  = "02";                             // OBJETOIMPUESTO
pub const MXN  : &str  = "MXN";                            // TOTALS_CURRENCY
pub const TAB  : &str  = "edicom";                         // TAB
pub const DEC  : &str  = "2";                              // DEC
pub const BATCH: &str  = "BATCH";                          // BATCH
pub const INDIV: &str  = "INDIV";                          // INDIV
pub const TITLE: &str  = "TITLE";                          // TITLE_LINE
pub const PAYMT: &str  = "PAYMT";                          // PAYMENT_LINE
pub const INVOI: &str  = "INVOI";                          // INVOICE_LINE

// Indexes for readibility - Position Excel columns and Indexes for arrays
// Source fields: Common to Payment lines (DZ) and Invoice lines (RV)
pub const CMPNY: usize =  0; // company_code
pub const CUSTO: usize =  1; // customer
pub const DOCNM: usize =  2; // document_number
pub const DOCTP: usize =  3; // document_type
pub const PYDTE: usize =  4; // payment_datetime
pub const CLRDC: usize =  5; // clearing_document
pub const AMOUN: usize =  6; // amount_doc_curr
pub const CURCY: usize =  7; // document_currency
pub const EXCHG: usize =  8; // eff_exchange_rate
pub const ASGMT: usize =  9; // assignment
pub const PYFRM: usize = 10; // forma_pago
pub const PARTL: usize = 11; // no_parcialidad
pub const PRVAM: usize = 12; // importe_saldo_anterior
pub const PYAMT: usize = 13; // importe_pago
pub const CURAM: usize = 14; // importe_saldo_insoluto
pub const TPREL: usize = 15; // tipo_relacion
pub const CANCL: usize = 16; // pago_cancelado_doc_number
pub const NUMOP: usize = 17; // num_operacion
pub const RFCOR: usize = 18; // rfc_banco_ordenente
pub const BNKOR: usize = 19; // nombre_banco_ordenante
pub const CTAOR: usize = 20; // cuenta_ordenante
pub const RFCBF: usize = 21; // rfc_banco_beneficiario
pub const CTABF: usize = 22; // cuenta_beneficiario
pub const PYTIP: usize = 23; // tipo_cadena_pago
pub const PYCER: usize = 24; // certificado_pago
pub const PYCAD: usize = 25; // cadena_pago
pub const PYSEL: usize = 26; // sello_pago
pub const TAXCD: usize = 27; // tax_code
// Total new fields, amounts in MXN currency
pub const TRETN: usize = 28; // retnc_iva
pub const TTB16: usize = 29; // trasl_basei_iva16
pub const TTA16: usize = 30; // trasl_impto_iva16
pub const TTB08: usize = 31; // trasl_basei_iva8
pub const TTA08: usize = 32; // trasl_impto_iva8
pub const TTB00: usize = 33; // trasl_basei_iva0
pub const TTA00: usize = 34; // trasl_impto_iva0
pub const TPAYM: usize = 35; // monto_total_pagos
// Invoice new fields (amounts in invoice currency)
pub const ITBAS: usize = 36; // trasl_basei_dr
pub const ITIMP: usize = 37; // trasl_impto_dr
pub const ITFAC: usize = 38; // trasl_tipof_dr
pub const ITRTE: usize = 39; // trasl_tasac_dr
pub const ITAMT: usize = 40; // trasl_impor_dr
pub const IRBAS: usize = 41; // retnc_basei_dr
pub const IRIMP: usize = 42; // retnc_impto_dr
pub const IRFAC: usize = 43; // retnc_tipof_dr
pub const IRRTE: usize = 44; // retnc_tasac_dr
pub const IRAMT: usize = 45; // retnc_impor_dr
pub const IOBJI: usize = 46; // objeto_impto_dr
// Payment new fields (amounts in payment currency)
pub const PTB16: usize = 47; // trasl_basei_iva16_p
pub const PTI16: usize = 48; // trasl_impto_iva16_p
pub const PTF16: usize = 49; // trasl_tipof_iva16_p
pub const PTR16: usize = 50; // trasl_tasac_iva16_p
pub const PTA16: usize = 51; // trasl_impor_iva16_p
pub const PRB16: usize = 52; // retnc_basei_iva16_p
pub const PRI16: usize = 53; // retnc_impto_iva16_p
pub const PRF16: usize = 54; // retnc_tipof_iva16_p
pub const PRR16: usize = 55; // retnc_tasac_iva16_p
pub const PRA16: usize = 56; // retnc_impor_iva16_p
pub const PTB08: usize = 57; // trasl_basei_iva8_p
pub const PTI08: usize = 58; // trasl_impto_iva8_p
pub const PTF08: usize = 59; // trasl_tipof_iva8_p
pub const PTR08: usize = 60; // trasl_tasac_iva8_p
pub const PTA08: usize = 61; // trasl_impor_iva8_p
pub const PRB08: usize = 62; // retnc_basei_iva8_p
pub const PRI08: usize = 63; // retnc_impto_iva8_p
pub const PRF08: usize = 64; // retnc_tipof_iva8_p
pub const PRR08: usize = 65; // retnc_tasac_iva8_p
pub const PRA08: usize = 66; // retnc_impor_iva8_p
pub const PTB00: usize = 67; // trasl_basei_iva0_p
pub const PTI00: usize = 68; // trasl_impto_iva0_p
pub const PTF00: usize = 69; // trasl_tipof_iva0_p
pub const PTR00: usize = 70; // trasl_tasac_iva0_p
pub const PTA00: usize = 71; // trasl_impor_iva0_p
pub const PRB00: usize = 72; // retnc_basei_iva0_p
pub const PRI00: usize = 73; // retnc_impto_iva0_p
pub const PRF00: usize = 74; // retnc_tipof_iva0_p
pub const PRR00: usize = 75; // retnc_tasac_iva0_p
pub const PRA00: usize = 76; // retnc_impor_iva0_p
// Differences
pub const DIFTL: usize = 77; // dif_monto_total_pagos
pub const DIFPY: usize = 78; // dif_impor_pago
// First invoice indicators
pub const FIT16: usize =  0; // first_invo_trasl_iva16
pub const FIR16: usize =  1; // first_invo_retnc_iva16
pub const FIT08: usize =  2; // first_invo_trasl_iva8
pub const FIR08: usize =  3; // first_invo_retnc_iva8
pub const FIT00: usize =  4; // first_invo_trasl_iva0
pub const FIR00: usize =  5; // first_invo_retnc_iva0
// Internal tables - DocumentType, TaxCode
pub const ITABLES: &str = r#"
{
  "comco": [
    { "code":"*", "desc":"ALL",
      "doctp": [
        { "code":"Document Type", "dtype":"TITLE" },
        { "code":"DZ",            "dtype":"PAYMT" },
        { "code":"PK",            "dtype":"PAYMT" },
        { "code":"RV",            "dtype":"INVOI" }
      ],
      "taxcd": [
        { "code":"A0", "trate":0.00, "wrate":0.00 },
        { "code":"A2", "trate":0.16, "wrate":0.00 },
        { "code":"A5", "trate":0.16, "wrate":0.16 },
        { "code":"AA", "trate":0.08, "wrate":0.00 },
        { "code":"AB", "trate":0.08, "wrate":0.08 },
        { "code":"AE", "trate":0.16, "wrate":0.08 },
        { "code":"AF", "trate":0.08, "wrate":0.03 },
        { "code":"B0", "trate":0.00, "wrate":0.00 },
        { "code":"B2", "trate":0.16, "wrate":0.00 },
        { "code":"B5", "trate":0.16, "wrate":0.16 },
        { "code":"BA", "trate":0.08, "wrate":0.00 },
        { "code":"BB", "trate":0.08, "wrate":0.08 },
        { "code":"BE", "trate":0.16, "wrate":0.08 },
        { "code":"BF", "trate":0.08, "wrate":0.03 },
        { "code":"CG", "trate":0.00, "wrate":0.00 },
        { "code":"CI", "trate":0.16, "wrate":0.00 },
        { "code":"CF", "trate":0.16, "wrate":0.00 },
        { "code":"V0", "trate":0.00, "wrate":0.00 },
        { "code":"VA", "trate":0.08, "wrate":0.00 }
      ]
    }
  ]
}
"#;

/*******************************************************************************
** Fixvalues - Fixed values structures                                         *
*******************************************************************************/
#[derive(Debug, Clone, Default)]
pub struct FixvaluesTp {
  pub TT   : Vec<String>,
  pub NUMER: Vec<usize>,
  pub ALPHA: Vec<usize>
}

impl FixvaluesTp {
  pub fn new_fixvalues() -> FixvaluesTp {
    let k = FixvaluesTp { ..Default::default() };
    k
  }

  pub fn set_fixvalues(&mut self) {
    // Load Titles table
    // Source fields titles    ....+....1....+....2....+...
    self.TT.push(String::from("Company Code"                )); //  0
    self.TT.push(String::from("Customer"                    )); //  1
    self.TT.push(String::from("Document Number"             )); //  2
    self.TT.push(String::from("Document Type"               )); //  3
    self.TT.push(String::from("Payment Date - Time"         )); //  4
    self.TT.push(String::from("Clearing Document"           )); //  5
    self.TT.push(String::from("Amount in Doc. Curr"         )); //  6
    self.TT.push(String::from("Document Currency"           )); //  7
    self.TT.push(String::from("Eff.exchange rate"           )); //  8
    self.TT.push(String::from("Assignment"                  )); //  9
    self.TT.push(String::from("Forma de Pago"               )); // 10
    self.TT.push(String::from("No. de Parcialidad"          )); // 11
    self.TT.push(String::from("Importe Saldo Anterior"      )); // 12
    self.TT.push(String::from("Importe Pago"                )); // 13
    self.TT.push(String::from("Importe Saldo Insoluto"      )); // 14
    self.TT.push(String::from("Tipo Relacion (04)"          )); // 15
    self.TT.push(String::from("Pago Cancelado (Doc Number)" )); // 16
    self.TT.push(String::from("Num Operacion"               )); // 17
    self.TT.push(String::from("RFC Banco Ordenente"         )); // 18
    self.TT.push(String::from("Nombre Banco Ordenante"      )); // 19
    self.TT.push(String::from("Cuenta Ordenante"            )); // 20
    self.TT.push(String::from("RFC Banco Beneficiario"      )); // 21
    self.TT.push(String::from("Cuenta Beneficiario"         )); // 22
    self.TT.push(String::from("Tipo Cadena Pago (01)"       )); // 23
    self.TT.push(String::from("Certificado Pago"            )); // 24
    self.TT.push(String::from("Cadena Pago"                 )); // 25
    self.TT.push(String::from("Sello Pago"                  )); // 26
    self.TT.push(String::from("Tax Code"                    )); // 27
    // Totals new fields titles
    self.TT.push(String::from("Retenciones IVA"             )); // 28
    self.TT.push(String::from("Traslados Base IVA16"        )); // 29
    self.TT.push(String::from("Traslados Impuesto IVA16"    )); // 30
    self.TT.push(String::from("Traslados Base IVA8"         )); // 31
    self.TT.push(String::from("Traslados Impuesto IVA8"     )); // 32
    self.TT.push(String::from("Traslados Base IVA0"         )); // 33
    self.TT.push(String::from("Traslados Impuesto IVA0"     )); // 34
    self.TT.push(String::from("Monto Total Pagos"           )); // 35
    // Invoice new fields titles
    self.TT.push(String::from("DR Traslado Base"            )); // 36
    self.TT.push(String::from("DR Traslado Impuesto"        )); // 37
    self.TT.push(String::from("DR Traslado TipoFactor"      )); // 38
    self.TT.push(String::from("DR Traslado TasaOCuota"      )); // 39
    self.TT.push(String::from("DR Traslado Importe"         )); // 40
    self.TT.push(String::from("DR Retencion Base"           )); // 41
    self.TT.push(String::from("DR Retencion Impuesto"       )); // 42
    self.TT.push(String::from("DR Retencion TipoFactor"     )); // 43
    self.TT.push(String::from("DR Retencion TasaOCuota"     )); // 44
    self.TT.push(String::from("DR Retencion Importe"        )); // 45
    self.TT.push(String::from("DR Objeto Impuesto"          )); // 46
    // Payment new fields titles
    self.TT.push(String::from("P Traslado Base IVA16"       )); // 47
    self.TT.push(String::from("P Traslado Impuesto IVA16"   )); // 48
    self.TT.push(String::from("P Traslado TipoFactor IVA16" )); // 49
    self.TT.push(String::from("P Traslado TasaOCuota IVA16" )); // 50
    self.TT.push(String::from("P Traslado Importe IVA16"    )); // 51
    self.TT.push(String::from("P Retencion Base IVA16"      )); // 52
    self.TT.push(String::from("P Retencion Impuesto IVA16"  )); // 53
    self.TT.push(String::from("P Retencion TipoFactor IVA16")); // 54
    self.TT.push(String::from("P Retencion TasaOCuota IVA16")); // 55
    self.TT.push(String::from("P Retencion Importe IVA16"   )); // 56
    self.TT.push(String::from("P Traslado Base IVA8"        )); // 57
    self.TT.push(String::from("P Traslado Impuesto IVA8"    )); // 58
    self.TT.push(String::from("P Traslado TipoFactor IVA8"  )); // 59
    self.TT.push(String::from("P Traslado TasaOCuota IVA8"  )); // 60
    self.TT.push(String::from("P Traslado Importe IVA8"     )); // 61
    self.TT.push(String::from("P Retencion Base IVA8"       )); // 62
    self.TT.push(String::from("P Retencion Impuesto IVA8"   )); // 63
    self.TT.push(String::from("P Retencion TipoFactor IVA8" )); // 64
    self.TT.push(String::from("P Retencion TasaOCuota IVA8" )); // 65
    self.TT.push(String::from("P Retencion Importe IVA8"    )); // 66
    self.TT.push(String::from("P Traslado Base IVA0"        )); // 67
    self.TT.push(String::from("P Traslado Impuesto IVA0"    )); // 68
    self.TT.push(String::from("P Traslado TipoFactor IVA0"  )); // 69
    self.TT.push(String::from("P Traslado TasaOCuota IVA0"  )); // 70
    self.TT.push(String::from("P Traslado Importe IVA0"     )); // 71
    self.TT.push(String::from("P Retencion Base IVA0"       )); // 72
    self.TT.push(String::from("P Retencion Impuesto IVA0"   )); // 73
    self.TT.push(String::from("P Retencion TipoFactor IVA0" )); // 74
    self.TT.push(String::from("P Retencion TasaOCuota IVA0" )); // 75
    self.TT.push(String::from("P Retencion Importe IVA0"    )); // 76
    // Differences
    self.TT.push(String::from("Diff Monto Total Pagos"      )); // 77
    self.TT.push(String::from("Diff Importe Pago"           )); // 78
    // Indexes for common numeric fields
    self.NUMER.push(AMOUN); //  6 = amount_doc_curr
    self.NUMER.push(PRVAM); // 12 = importe_saldo_anterior
    self.NUMER.push(PYAMT); // 13 = importe_pago
    self.NUMER.push(CURAM); // 14 = importe_saldo_insoluto
    // Indexes for new invoice alphanumeric fields
    self.ALPHA.push(ITIMP); // 37 = trasl_impto_dr
    self.ALPHA.push(ITFAC); // 38 = trasl_tipof_dr
    self.ALPHA.push(IRIMP); // 42 = retnc_impto_dr
    self.ALPHA.push(IRFAC); // 43 = retnc_tipof_dr
    self.ALPHA.push(IOBJI); // 46 = objeto_impto_dr
    // Indexes for new payment alphanumeric fields
    self.ALPHA.push(PTI16); // 48 = trasl_impto_iva16_p
    self.ALPHA.push(PTF16); // 49 = trasl_tipof_iva16_p
    self.ALPHA.push(PRI16); // 53 = retnc_impto_iva16_p
    self.ALPHA.push(PRF16); // 54 = retnc_tipof_iva16_p
    self.ALPHA.push(PTI08); // 58 = trasl_impto_iva8_p
    self.ALPHA.push(PTF08); // 59 = trasl_tipof_iva8_p
    self.ALPHA.push(PRI08); // 63 = retnc_impto_iva8_p
    self.ALPHA.push(PRF08); // 64 = retnc_tipof_iva8_p
    self.ALPHA.push(PTI00); // 68 = trasl_impto_iva0_p
    self.ALPHA.push(PTF00); // 69 = trasl_tipof_iva0_p
    self.ALPHA.push(PRI00); // 73 = retnc_impto_iva0_p
    self.ALPHA.push(PRF00); // 74 = retnc_tipof_iva0_p
  }
}

/*******************************************************************************
** itables - Upload internal tables                                            *
*******************************************************************************/
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DoctypesTp {
  pub code : String,
  pub dtype: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaxcodesTp {
  pub code : String,
  pub trate: f32,
  pub wrate: f32
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CompanycodesTp {
  pub code : String,
  pub desc : String,
  pub doctp: Vec<DoctypesTp>,
  pub taxcd: Vec<TaxcodesTp>
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ItablesTp {
  pub comco: Vec<CompanycodesTp>
}

impl ItablesTp {
  pub fn new_itables() -> ItablesTp {
    let it = ItablesTp{ ..Default::default() };
    it
  }

  pub fn get_itables(&mut self) {
    let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
    self.comco = it.comco;
  }

  pub fn seek_doctp(&mut self, cmpny: &String, doctp: &String) -> String {
    let mut dtype: &str = "";
    let mut found: bool = false;
    for cc in &self.comco {
      if cc.code.as_str() == "*" {
        for tp in &cc.doctp {
          if tp.code == *doctp {
            dtype = tp.dtype.as_str();
            break;
          }
        }
      }
      if cc.code == *cmpny {
        for tp in &cc.doctp {
          if tp.code.as_str() == *doctp {
            dtype = tp.dtype.as_str();
            found = true;
            break;
          }
        }
      }
      if found {
        break;
      }
    }
    return dtype.to_string();
  }

  pub fn seek_taxcd(&mut self, cmpny: &String, taxcd: &String) -> TaxcodesTp {
    let mut txdta: TaxcodesTp = Default::default();
    let mut found: bool = false;
    for cc in &self.comco {
      if cc.code.as_str() == "*" {
        for tc in &cc.taxcd {
          if tc.code == *taxcd {
            let txc = (*tc).clone();
            txdta = txc;
            break;
          }
        }
      }
      if cc.code == *cmpny {
        for tc in &cc.taxcd {
          if tc.code.as_str() == taxcd {
            let txc = (*tc).clone();
            txdta = txc;
            found = true;
            break;
          }
        }
      }
      if found {
        break;
      }
    }
    return txdta;
  }
}
