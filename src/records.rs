#![allow(unused_parens)]
use byte::ctx;
use byte::{BytesExt, TryRead, TryWrite};
use std::fmt;
use crate::types::*;
use serde::Serialize;
use serde_json;

pub trait IsTestRecord {
    fn is_test_record(&self) -> bool;
    fn get_name_as_string(&self) -> String;
}

pub trait Atdf {
    fn to_atdf(&self) -> String;
}

// pub struct Vec<U2>;

// impl std::fmt::Display for Vec<U2> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.  .0.iter().map(|x| x.to_string()).collect::<Vec<String>>())
//     }
// }

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    pub rec_len: U2,
    pub rec_typ: U1,
    pub rec_sub: U1,
}

impl<'a> TryRead<'a, ctx::Endian> for Header {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        Ok((
            Header {
                rec_len: bytes.read_with::<U2>(offset, endian)?,
                rec_typ: bytes.read_with::<U1>(offset, endian)?,
                rec_sub: bytes.read_with::<U1>(offset, endian)?,
            },
            *offset,
        ))
    }
}

impl TryWrite<ctx::Endian> for Header {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        bytes.write_with::<U2>(offset, self.rec_len, endian)?;
        bytes.write_with::<U1>(offset, self.rec_typ, endian)?;
        bytes.write_with::<U1>(offset, self.rec_sub, endian)?;
        Ok(*offset)
    }
}

impl Header {
    pub fn detect_endian(bytes: &[u8]) -> byte::Result<ctx::Endian> {
        byte::check_len(bytes, 2)?;
        let header = bytes.read_with::<Header>(&mut 0, byte::BE)?;
        if u8::from(header.rec_typ) != 0 || u8::from(header.rec_sub) != 10 {
            return Err(byte::Error::BadInput {
                err: "refusing to detect endian-ness with a non-FAR record",
            });
        }
        if header.rec_len == U2::from(2) {
            Ok(byte::BE)
        } else if header.rec_len == U2::from(512) {
            Ok(byte::LE)
        } else {
            Err(byte::Error::BadInput {
                err: "invalid or unrecognized FAR record header length",
            })
        }
    }
}

//TODO: move this declarative macro to the STDFRecord derive macro
macro_rules! record_id {
    ($record_name:ident, $test_type:expr, $($lt:lifetime)?) => {
        impl IsTestRecord for $record_name<$($lt)?> {
            fn is_test_record(&self) -> bool {
                $test_type
            }
            fn get_name_as_string(&self) -> String {
                stringify!($record_name).to_string()
            }
        }
    };
}

//TODO: move this declarative macro to the STDFRecord derive macro
macro_rules! atdf {
    ($record_name:ident, $($lt:lifetime)?) => {
        impl Atdf for $record_name<$($lt)?> {
            fn to_atdf(&self) -> String {
                let serialized = serde_json::to_string(self).unwrap();
                let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
                if let serde_json::Value::Object(map) = json {
                    let result = map.values()
                        .map(|value| format!("{}", value))
                        .collect::<Vec<String>>()
                        .join("|");
                    return format!("{}\n", result.replace("\"", ""));
                }
                String::new() // fallback
            }
        }
    };
}

// ========================================================
// FAR : File Attribute Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct FAR {
    pub cpu_type: U1,
    pub stdf_ver: U1,
}

record_id!(FAR, false,  );
atdf!(FAR,);

impl fmt::Display for FAR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FAR : File Attrubute Record")?;
        writeln!(f, "   CPU_TYPE [U1] : {}", self.cpu_type)?;
        writeln!(f, "   STDF_VER [U1] : {}", self.stdf_ver)
    }
}

// ========================================================
// ATR : Audit Trail Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct ATR<'a> {
    #[default(U4E::from(0))]
    pub mod_tim: U4E,
    #[default(Cn(b""))]
    pub cmd_line: Cn<'a>,
}

record_id!(ATR, false, '_);
atdf!(ATR, '_);

impl fmt::Display for ATR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ATR : Audit Trail Record")?;
        writeln!(f, "   MOD_TIM [U4E]: {}", self.mod_tim)?;
        writeln!(f, "   CMD_LINE [Cn]: '{}'", self.cmd_line)
    }
}

// ========================================================
// MIR : Master Information Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct MIR<'a> {
    #[default(U4E::from(0))]
    pub setup_t: U4E,
    #[default(U4E::from(0))]
    pub start_t: U4E,
    #[default(U1::from(0))]
    pub stat_num: U1,
    #[default(C1(b' '))]
    pub mode_cod: C1,
    #[default(C1(b' '))]
    pub rtst_cod: C1,
    #[default(C1(b' '))]
    pub prot_cod: C1,
    #[default(U2::from(0))]
    pub burn_tim: U2,
    #[default(C1(b' '))]
    pub cmod_cod: C1,
    #[default(Cn(b""))]
    pub lot_id: Cn<'a>,
    #[default(Cn(b""))]
    pub part_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub node_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub tstr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub job_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub job_rev: Cn<'a>,
    #[default(Cn(b""))]
    pub sblot_id: Cn<'a>,
    #[default(Cn(b""))]
    pub oper_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub exec_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub exec_ver: Cn<'a>,
    #[default(Cn(b""))]
    pub test_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub tst_temp: Cn<'a>,
    #[default(Cn(b""))]
    pub user_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub aux_file: Cn<'a>,
    #[default(Cn(b""))]
    pub pkg_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub famly_id: Cn<'a>,
    #[default(Cn(b""))]
    pub date_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub facil_id: Cn<'a>,
    #[default(Cn(b""))]
    pub floor_id: Cn<'a>,
    #[default(Cn(b""))]
    pub proc_id: Cn<'a>,
    #[default(Cn(b""))]
    pub oper_frq: Cn<'a>,
    #[default(Cn(b""))]
    pub spec_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub spec_ver: Cn<'a>,
    #[default(Cn(b""))]
    pub flow_id: Cn<'a>,
    #[default(Cn(b""))]
    pub setup_id: Cn<'a>,
    #[default(Cn(b""))]
    pub dsgn_rev: Cn<'a>,
    #[default(Cn(b""))]
    pub eng_id: Cn<'a>,
    #[default(Cn(b""))]
    pub rom_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub serl_num: Cn<'a>,
    #[default(Cn(b""))]
    pub supr_nam: Cn<'a>,
}

record_id!(MIR, false, '_);
atdf!(MIR, '_);

impl fmt::Display for MIR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MIR : Master Information Record")?;
        writeln!(f, "   SETUP_T [U4E] : {}", self.setup_t)?;
        writeln!(f, "   START_T [U4E] : {}", self.start_t)?;
        writeln!(f, "   STAT_NUM [U1] : {}", self.stat_num)?;
        writeln!(f, "   MODE_COD [C1] : '{}'", self.mode_cod)?;
        writeln!(f, "   RTST_COD [C1] : '{}'", self.rtst_cod)?;
        writeln!(f, "   PROT_COD [C1] : '{}'", self.prot_cod)?;
        writeln!(f, "   BURN_TIM [U2] : {}", self.burn_tim)?;
        writeln!(f, "   CMOD_COD [C1] : '{}'", self.cmod_cod)?;
        writeln!(f, "   LOT_ID   [Cn] : '{}'", self.lot_id)?;
        writeln!(f, "   PART_TYP [Cn] : '{}'", self.part_typ)?;
        writeln!(f, "   NODE_NAM [Cn] : '{}'", self.node_nam)?;
        writeln!(f, "   TSTR_TYP [Cn] : '{}'", self.tstr_typ)?;
        writeln!(f, "   JOB_NAM  [Cn] : '{}'", self.job_nam)?;
        writeln!(f, "   JOB_REV  [Cn] : '{}'", self.job_rev)?;
        writeln!(f, "   SBLOT_ID [Cn] : '{}'", self.sblot_id)?;
        writeln!(f, "   OPER_NAM [Cn] : '{}'", self.oper_nam)?;
        writeln!(f, "   EXEC_TYP [Cn] : '{}'", self.exec_typ)?;
        writeln!(f, "   EXEC_VER [Cn] : '{}'", self.exec_ver)?;
        writeln!(f, "   TEST_COD [Cn] : '{}'", self.test_cod)?;
        writeln!(f, "   TST_TEMP [Cn] : '{}'", self.tst_temp)?;
        writeln!(f, "   USER_TXT [Cn] : '{}'", self.user_txt)?;
        writeln!(f, "   AUX_FILE [Cn] : '{}'", self.aux_file)?;
        writeln!(f, "   PKG_TYP  [Cn] : '{}'", self.pkg_typ)?;
        writeln!(f, "   FAMLY_ID [Cn] : '{}'", self.famly_id)?;
        writeln!(f, "   DATE_COD [Cn] : '{}'", self.date_cod)?;
        writeln!(f, "   FACIL_ID [Cn] : '{}'", self.facil_id)?;
        writeln!(f, "   FLOOR_ID [Cn] : '{}'", self.floor_id)?;
        writeln!(f, "   PROC_ID  [Cn] : '{}'", self.proc_id)?;
        writeln!(f, "   OPER_FRQ [Cn] : '{}'", self.oper_frq)?;
        writeln!(f, "   SPEC_NAM [Cn] : '{}'", self.spec_nam)?;
        writeln!(f, "   SPEC_VER [Cn] : '{}'", self.spec_ver)?;
        writeln!(f, "   FLOW_ID  [Cn] : '{}'", self.flow_id)?;
        writeln!(f, "   SETUP_ID [Cn] : '{}'", self.setup_id)?;
        writeln!(f, "   DSGN_REV [Cn] : '{}'", self.dsgn_rev)?;
        writeln!(f, "   ENG_ID   [Cn] : '{}'", self.eng_id)?;
        writeln!(f, "   ROM_COD  [Cn] : '{}'", self.rom_cod)?;
        writeln!(f, "   SERL_NUM [Cn] : '{}'", self.serl_num)?;
        writeln!(f, "   SUPR_NAM [Cn] : '{}'", self.supr_nam)
    }
}

// ========================================================
// MRR : Master Result Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct MRR<'a> {
    pub finish_t: U4E,
    #[default(C1::from(b' '))]
    pub disp_cod: C1,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

record_id!(MRR, false, '_);
atdf!(MRR, '_);

impl fmt::Display for MRR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MRR : Master Result Record")?;
        writeln!(f, "   FINISH_T [U4] : {}", self.finish_t)?;
        writeln!(f, "   DISP_COD [C1] : {}", if self.disp_cod.to_string() == " " {
                                                 "∕".to_string()
                                             } else {
                                                 format!("'{}'", self.disp_cod).to_string()
                                             })?;
        writeln!(f, "   USR_DESC [Cn] : {}", if self.usr_desc.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                 "∕".to_string()
                                              } else {
                                                 format!("'{}'", self.usr_desc).to_string().replace("\n", "").replace("\r", "")
                                              })?;
        writeln!(f, "   EXC_DESC [Cn] : {}", if self.exc_desc.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                 "∕".to_string()
                                             } else {
                                                 format!("'{}'", self.exc_desc).to_string().replace("\n", "").replace("\r", "")
                                             })
    }
}

// ========================================================
// PCR : Part Count Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PCR {
    pub head_num: U1,
    pub site_num: U1,
    pub part_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub rtst_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub abrt_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub good_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub func_cnt: U4,
}

record_id!(PCR, false,);
atdf!(PCR,);

impl fmt::Display for PCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PCR : Part Count Record\n")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", if self.head_num == U1::MAX {
                                                 format!("{} → Summary", self.head_num).to_string()
                                             } else {
                                                self.head_num.to_string()
                                             })?;
        writeln!(f, "   SITE_NUM [U1] : {}", if self.head_num == U1::MAX {
                                                 "∕".to_string()
                                             } else {
                                                 self.site_num.to_string()
                                             })?;
        writeln!(f, "   PART_CNT [U4] : {}", self.part_cnt)?;
        writeln!(f, "   RTST_CNT [U4] : {}", if self.rtst_cnt == U4::MAX {
                                                 "∕".to_string()
                                             } else {
                                                self.rtst_cnt.to_string()
                                             })?;
        writeln!(f, "   ABRT_CNT [U4] : {}", if self.abrt_cnt == U4::MAX {
                                                 "∕".to_string()
                                             } else {
                                                self.abrt_cnt.to_string()
                                             })?;
        writeln!(f, "   GOOD_CNT [U4] : {}", if self.good_cnt == U4::MAX {
                                                 "∕".to_string()
                                             } else {
                                                self.good_cnt.to_string()
                                             })?;
        writeln!(f, "   FUNC_CNT [U4] : {}", if self.func_cnt == U4::MAX {
                                                 "∕".to_string()
                                             } else {
                                                 self.func_cnt.to_string()	                                                 
                                             })
    }
}

// ========================================================
// HBR : Hard Bin Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct HBR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub hbin_num: U2,
    pub hbin_cnt: U4,
    #[default(C1::from(0x20))]
    pub hbin_pf: C1,
    #[default(Cn(b""))]
    pub hbin_nam: Cn<'a>,
}

record_id!(HBR, false, '_);
atdf!(HBR, '_);

impl fmt::Display for HBR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HBR : Hard Bin Record")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", if self.head_num == U1::MAX {
                                                 "→ Summary".to_string()
                                             } else {
                                                 self.head_num.to_string()
                                             })?;
        writeln!(f, "   SITE_NUM [U1] : {}", if self.head_num == U1::MAX {
                                                 "∕".to_string()
                                             } else {
                                                 self.site_num.to_string()
                                             })?;
        writeln!(f, "   HBIN_NUM [U2] : {}", self.hbin_num)?;
        writeln!(f, "   HBIN_CNT [U4] : {}", self.hbin_cnt)?;
        writeln!(f, "   HBIN_PF  [C1] : '{}'", if self.hbin_pf.to_string() == " " {
                                                   "?".to_string()
                                               } else {
                                                   self.hbin_pf.to_string()
                                               })?;
        writeln!(f, "   HBIN_NAM [Cn] : '{}'", if self.hbin_nam.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                   "∕".to_string()
                                               } else {
                                                   format!("'{}'", self.hbin_nam).to_string().replace("\n", "").replace("\r", "")
                                               })
    }
}

// ========================================================
// SBR : Soft Bin Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct SBR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub sbin_num: U2,
    pub sbin_cnt: U4,
    #[default(C1::from(0x20))]
    pub sbin_pf: C1,
    #[default(Cn(b""))]
    pub sbin_nam: Cn<'a>,
}

record_id!(SBR, false, '_);
atdf!(SBR, '_);

impl fmt::Display for SBR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SBR : Soft Bin Record")?;
        writeln!(f, "   HEAD_NUM [U1]: {}", if self.head_num == U1::MAX {
                                                "→ Summary".to_string()
                                            } else {
                                                self.head_num.to_string()
                                            })?;
        writeln!(f, "   SITE_NUM [U1]: {}", if self.head_num == U1::MAX {
                                                "∕".to_string()
                                            } else {
                                                self.site_num.to_string()
                                            })?;
        writeln!(f, "   SBIN_NUM [U2]: {}", self.sbin_num)?;
        writeln!(f, "   SBIN_CNT [U4]: {}", self.sbin_cnt)?;
        writeln!(f, "   SBIN_PF  [C1]: '{}'", if self.sbin_pf.to_string() == " " {
                                                  "?".to_string()
                                              } else {
                                                  self.sbin_pf.to_string()
                                              })?;
        writeln!(f, "   SBIN_NAM [Cn]: {}", if self.sbin_nam.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                "∕".to_string()
                                            } else {
                                                format!("'{}'", self.sbin_nam).to_string().replace("\n", "").replace("\r", "")
                                            })
    }
}

// ========================================================
// PMR : Pin Map Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PMR<'a> {
    pub pmr_index: U2,
    #[default(U2::from(0))]
    pub chan_typ: U2,
    #[default(Cn(b""))]
    pub chan_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub phy_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub log_nam: Cn<'a>,
    #[default(U1::from(1))]
    pub head_num: U1,
    #[default(U1::from(1))]
    pub site_num: U1,
}

record_id!(PMR, false, '_);
atdf!(PMR, '_);

impl fmt::Display for PMR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PMR : Pin Map Record")?;
        writeln!(f, "   PMR_INDEX [U2] : {}", self.pmr_index)?;
        writeln!(f, "   CHAN_TYP  [U2] : {}", self.chan_typ)?;       
        writeln!(f, "   CHAN_NAM  [Cn] : '{}'", if self.chan_nam.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                    "∕".to_string()
                                                } else {
                                                    format!("'{}'", self.chan_nam).to_string().replace("\n", "").replace("\r", "")
                                                })?;
        writeln!(f, "   PHY_NAM   [Cn] : '{}'", if self.phy_nam.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                    "∕".to_string()
                                                } else {
                                                    format!("'{}'", self.phy_nam).to_string().replace("\n", "").replace("\r", "")
                                                })?;
        writeln!(f, "   LOG_NAM   [Cn] : {}", if self.log_nam.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                    "∕".to_string()
                                                } else {
                                                    format!("'{}'", self.log_nam).to_string().replace("\n", "").replace("\r", "")
                                                })?;        
        writeln!(f, "   HEAD_NUM  [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM  [U1] : {}", self.site_num)
    }
}

// ========================================================
// PGR : Pin Group Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PGR<'a> {
    pub grp_indx: U2,
    #[default(Cn(b""))]
    pub grp_nam: Cn<'a>,
    #[default(U2::from(0))]
    pub indx_cnt: U2,
    #[array_length(indx_cnt)]
    #[array_type(U2)]
    pub pmr_indx: Vec<U2>,
}

record_id!(PGR, false, '_);
atdf!(PGR, '_);

impl fmt::Display for PGR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PGR : Pin Group Record")?;
        writeln!(f, "   GRP_INDX   [U2] : {}", self.grp_indx)?;
        writeln!(f, "   GRP_NAM    [Cn] : {}", self.grp_nam)?;
        writeln!(f, "   INDX_CNT k [U2] : {}", self.indx_cnt)?;
        writeln!(f, "   PMR_INDX [kxU2] : {:?}", self.pmr_indx)  //TODO: implement std::fmt::Display for Vec<U2>
    }
}

// ========================================================
// PLR : Pin List Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PLR<'a> {
    pub grp_cnt: U2,
    #[array_length(grp_cnt)]
    #[array_type(U2)]
    pub grp_indx: Vec<U2>,
    #[array_length(grp_cnt)]
    #[array_type(U2)]
    pub grp_mode: Vec<U2>,
    #[array_length(grp_cnt)]
    #[array_type(U1)]
    pub grp_radx: Vec<U1>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub pgm_char: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub rtn_char: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub pgm_chal: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub rtn_chal: Vec<Cn<'a>>,
}

record_id!(PLR, false, '_);
atdf!(PLR, '_);

impl fmt::Display for PLR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PLR : Pin List Record")?;
        writeln!(f, "   GRP_CNT  k [U2]: {}", self.grp_cnt)?;
        writeln!(f, "   GRP_INDX [kxU2]: {:?}", self.grp_indx)?; //TODO: implement std::fmt::Display for Vec<U2>
        writeln!(f, "   GRP_MODE [kxU2]: {:?}", self.grp_mode)?; //TODO: implement std::fmt::Display for Vec<U2>
        writeln!(f, "   GRP_RADX [kxU1]: {:?}", self.grp_radx)?; //TODO: implement std::fmt::Display for Vec<U1>
        writeln!(f, "   PGM_CHAR [kxCn]: {:?}", self.pgm_char)?; //TODO: implement std::fmt::Display for Vec<Cn>
        writeln!(f, "   RTN_CHAR [kxCn]: {:?}", self.rtn_char)?; //TODO: implement std::fmt::Display for Vec<Cn>
        writeln!(f, "   PGM_CHAL [kxCn]: {:?}", self.pgm_chal)?; //TODO: implement std::fmt::Display for Vec<Cn>
        writeln!(f, "   RTN_CHAL [kxCn]: {:?}", self.rtn_chal)   //TODO: implement std::fmt::Display for Vec<Cn>
    }
}

// ========================================================
// RDR : Retest Data Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct RDR {
    pub num_bins: U2,
    #[array_length(num_bins)]
    #[array_type(U2)]
    pub rtst_bin: Vec<U2>,
}

record_id!(RDR, false,);
atdf!(RDR,);

impl fmt::Display for RDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RDR : Retest Data Record")?;
        writeln!(f, "   NUM_BINS k [U2] : {}", self.num_bins)?;
        writeln!(f, "   RTST_BIN [kxU2] : {:?}", self.rtst_bin) //TODO: implement std::fmt::Display for Vec<U2>
    }
}

// ========================================================
// SDR : Site Description Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct SDR<'a> {
    pub head_num: U1,
    pub site_grp: U1,
    pub site_cnt: U1,
    #[array_length(site_cnt)]
    #[array_type(U1)]
    pub site_num: Vec<U1>,
    #[default(Cn(b""))]
    pub hand_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub hand_id: Cn<'a>,
    #[default(Cn(b""))]
    pub card_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub card_id: Cn<'a>,
    #[default(Cn(b""))]
    pub load_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub load_id: Cn<'a>,
    #[default(Cn(b""))]
    pub dib_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub dib_id: Cn<'a>,
    #[default(Cn(b""))]
    pub cabl_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub cabl_id: Cn<'a>,
    #[default(Cn(b""))]
    pub cont_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub cont_id: Cn<'a>,
    #[default(Cn(b""))]
    pub lasr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub lasr_id: Cn<'a>,
    #[default(Cn(b""))]
    pub extr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub extr_id: Cn<'a>,
}

record_id!(SDR, false, '_);
atdf!(SDR, '_);

impl fmt::Display for SDR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SDR : Site Description Record")?;
        writeln!(f, "   HEAD_NUM   [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_GRP   [U1] : {}", self.site_grp)?;
        writeln!(f, "   SITE_CNT k [U1] : {}", self.site_cnt)?;
        writeln!(f, "   SITE_NUM [kxU1] : {:?}", self.site_num)?; //TODO: implement std::fmt::Display for Vec<U1>
        writeln!(f, "   HAND_TYP   [Cn] : '{}'", self.hand_typ)?;
        writeln!(f, "   HAND_ID    [Cn] : '{}'", self.hand_id)?;
        writeln!(f, "   CARD_TYP   [Cn] : '{}'", self.card_typ)?;
        writeln!(f, "   CARD_ID    [Cn] : '{}'", self.card_id)?;
        writeln!(f, "   LOAD_TYP   [Cn] : '{}'", self.load_typ)?;
        writeln!(f, "   LOAD_ID    [Cn] : '{}'", self.load_id)?;
        writeln!(f, "   DIB_TYP    [Cn] : '{}'", self.dib_typ)?;
        writeln!(f, "   DIB_ID     [Cn] : '{}'", self.dib_id)?;
        writeln!(f, "   CABL_TYP   [Cn] : '{}'", self.cabl_typ)?;
        writeln!(f, "   CABL_ID    [Cn] : '{}'", self.cabl_id)?;
        writeln!(f, "   CONT_TYP   [Cn] : '{}'", self.cont_typ)?;
        writeln!(f, "   CONT_ID    [Cn] : '{}'", self.cont_id)?;
        writeln!(f, "   LASR_TYP   [Cn] : '{}'", self.lasr_typ)?;
        writeln!(f, "   LASR_ID    [Cn] : '{}'", self.lasr_id)?;
        writeln!(f, "   EXTR_TYP   [Cn] : '{}'", self.extr_typ)?;
        writeln!(f, "   EXTR_ID    [Cn] : '{}'", self.extr_id)
    }
} 

// ========================================================
// WIR : Wafer Information Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct WIR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub start_t: U4E,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
}

record_id!(WIR, false, '_);
atdf!(WIR, '_);

impl fmt::Display for WIR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WIR : Wafer Information Record")?;
        writeln!(f, "   HEAD_NUM  [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_GRP  [U1] : {}", self.site_grp)?;
        writeln!(f, "   START_T  [U4E] : {}", self.start_t)?;
        writeln!(f, "   WAFER_ID  [Cn] : '{}'", self.wafer_id)
    }
}	

// ========================================================
// WRR : Wafer Result Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct WRR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub finish_t: U4E,
    pub part_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub rtst_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub abrt_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub good_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub func_cnt: U4,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
    #[default(Cn(b""))]
    pub fabwf_id: Cn<'a>,
    #[default(Cn(b""))]
    pub frame_id: Cn<'a>,
    #[default(Cn(b""))]
    pub mask_id: Cn<'a>,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

record_id!(WRR, false, '_);
atdf!(WRR, '_);

impl fmt::Display for WRR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WRR : Wafer Result Record")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_GRP [U1] : {}", self.site_grp)?;
        writeln!(f, "   FINISH_T [U4] : {}", self.finish_t)?;
        writeln!(f, "   PART_CNT [U4] : {}", self.part_cnt)?;
        writeln!(f, "   RTST_CNT [U4] : {}", self.rtst_cnt)?;
        writeln!(f, "   ABRT_CNT [U4] : {}", self.abrt_cnt)?;
        writeln!(f, "   GOOD_CNT [U4] : {}", self.good_cnt)?;
        writeln!(f, "   FUNC_CNT [U4] : {}", self.func_cnt)?;
        writeln!(f, "   WAFER_ID [Cn] : '{}'", self.wafer_id)?;
        writeln!(f, "   FABWF_ID [Cn] : '{}'", self.fabwf_id)?;
        writeln!(f, "   FRAME_ID [Cn] : '{}'", self.frame_id)?;
        writeln!(f, "   MASK_ID  [Cn] : '{}'", self.mask_id)?;
        writeln!(f, "   USR_DESC [Cn] : '{}'", self.usr_desc)?;
        writeln!(f, "   EXC_DESC [Cn] : '{}'", self.exc_desc)
    }
}

// ========================================================
// WCS : Wafer Configuration Record
// ========================================================
#[derive(Debug, PartialEq, Serialize, STDFRecord)]
pub struct WCR {
    #[default(R4::from(0.0))]
    pub wafr_siz: R4,
    #[default(R4::from(0.0))]
    pub die_ht: R4,
    #[default(R4::from(0.0))]
    pub die_wid: R4,
    #[default(U1::from(0))]
    pub wf_units: U1,
    #[default(C1::from(0x20))]
    pub wf_flat: C1,
    #[default(I2::from(i16::MIN))]
    pub center_x: I2,
    #[default(I2::from(i16::MIN))]
    pub center_y: I2,
    #[default(C1::from(0x20))]
    pub pos_x: C1,
    #[default(C1::from(0x20))]
    pub pos_y: C1,
}

record_id!(WCR, false,);
atdf!(WCR,);

impl fmt::Display for WCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WCR : Wafer Configuration Record")?;
        writeln!(f, "   WAFR_SIZ [R4] : {}", self.wafr_siz)?;
        writeln!(f, "   DIE_HT   [R4] : {}", self.die_ht)?;
        writeln!(f, "   DIE_WID  [R4] : {}", self.die_wid)?;
        writeln!(f, "   WF_UNITS [U1] : {}", self.wf_units)?;
        writeln!(f, "   WF_FLAT  [C1] : {}", self.wf_flat)?;
        writeln!(f, "   CENTER_X [I2] : {}", self.center_x)?;
        writeln!(f, "   CENTER_Y [I2] : {}", self.center_y)?;
        writeln!(f, "   POS_X    [C1] : '{}'", self.pos_x)?;
        writeln!(f, "   POS_Y    [C1] : '{}'", self.pos_y)
    }
}

// ========================================================
// PIR : Part Information Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PIR {
    pub head_num: U1,
    pub site_num: U1,
}

record_id!(PIR, false,);
atdf!(PIR,);

impl fmt::Display for PIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PIR : Part Information Record")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM [U1] : {}", self.site_num)
    }
}

// ========================================================
// PRR : Part Results Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct PRR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub part_flg: B1,
    pub num_test: U2,
    pub hard_bin: U2,
    #[default(U2::from(0xffff))]
    pub soft_bin: U2,
    #[default(I2::from(i16::MIN))]
    pub x_coord: I2,
    #[default(I2::from(i16::MIN))]
    pub y_coord: I2,
    #[default(U4::from(0))]
    pub test_t: U4,
    #[default(Cn(b""))]
    pub part_id: Cn<'a>,
    #[default(Cn(b""))]
    pub part_txt: Cn<'a>,
    #[default(Bn(b""))]
    pub part_fix: Bn<'a>,
}

record_id!(PRR, false, '_);
atdf!(PRR, '_);

impl fmt::Display for PRR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PRR : Part Results Record")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM [U1] : {}", self.site_num)?;
        writeln!(f, "   PART_FLG [B1] : {} {}", self.part_flg, prr_part_flg(self.part_flg))?;
        writeln!(f, "   NUM_TEST [U2] : {}", self.num_test)?;
        writeln!(f, "   HARD_BIN [U2] : {}", self.hard_bin)?;
        writeln!(f, "   SOFT_BIN [U2] : {}", if u16::from(self.soft_bin) == 65535_u16 {
                                                 "∕".to_string()
                                             } else {
                                                 format!("{}", u16::from(self.soft_bin))
                                             })?;
        writeln!(f, "   X_COORD  [I2] : {}", if self.x_coord == I2::MIN {
                                               "∕".to_string()
                                           } else {
                                               self.x_coord.to_string()
                                           })?;
        writeln!(f, "   Y_COORD  [I2] : {}", if self.y_coord == I2::MIN {
                                               "∕".to_string()
                                           } else {
                                               self.y_coord.to_string()
                                           })?;
        writeln!(f, "   TEST_T   [U4] : {}", if u32::from(self.test_t) == 0_u32 {
                                               "∕".to_string()
                                           } else {
                                               format!("{} → {:.3} sec", u32::from(self.test_t), u32::from(self.test_t) as f32 / 1_000.0)
                                           })?;
        writeln!(f, "   PART_ID  [Cn] : {}", if self.part_id.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                 "∕".to_string()
                                             } else {
                                                 format!("'{}'", self.part_id.to_string().replace("\n", "").replace("\r", ""))
                                             })?;
        writeln!(f, "   PART_TXT [Cn] : {}", if self.part_txt.to_string().replace("\n", "").replace("\r", "").is_empty() {
                                                 "∕".to_string()
                                             } else {
                                                 format!("'{}'", self.part_txt.to_string().replace("\n", "").replace("\r", ""))
                                             })?;
        writeln!(f, "   PART_FIX [Bn] : {}", if self.part_fix.to_string().is_empty() {
                                                 "∕".to_string()
                                             } else {
                                                 format!("{}",self.part_fix)
                                             })
    }
}

fn prr_part_flg(part_flg: B1) -> String {
    let mut msg = String::new();

    msg.push_str("→ ");

    if u8::from(part_flg) & 0b0001_0000 == 0b0001_0000  { 
        msg.push('?');
    } else if u8::from(part_flg) & 0b0000_1000 == 0b0000_1000 {
        msg.push_str("FAIL");
    } else {
        msg.push_str("PASS");
    }

    if u8::from(part_flg) & 0b0000_0100 == 0b0000_0100  { 
        msg.push_str(" (");
        msg.push_str("Abnormal end of testing");
        msg.push(')');
    }

    msg
}

// ========================================================
// TSR : Test Synopsis Record
// ========================================================
#[derive(Debug, PartialEq, Serialize, STDFRecord)]
pub struct TSR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub test_typ: C1,
    pub test_num: U4,
    pub exec_cnt: U4,
    pub fail_cnt: U4,
    pub alrm_cnt: U4,
    #[default(Cn(b""))]
    pub test_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
    #[default(Cn(b""))]
    pub test_lbl: Cn<'a>,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(R4::from(f32::NAN))]
    pub test_tim: R4,
    #[default(R4::from(f32::NAN))]
    pub test_min: R4,
    #[default(R4::from(f32::NAN))]
    pub test_max: R4,
    #[default(R4::from(f32::NAN))]
    pub tst_sums: R4,
    #[default(R4::from(f32::NAN))]
    pub tst_sqrs: R4,
}

record_id!(TSR, false, '_);
atdf!(TSR, '_);

impl fmt::Display for TSR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TSR : Test Synopsis Record")?;
        writeln!(f, "   HEAD_NUM [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM [U1] : {}", self.site_num)?;
        writeln!(f, "   TEST_TYP [C1] : '{}'", self.test_typ)?;
        writeln!(f, "   TEST_NUM [U4] : {}", self.test_num)?;
        writeln!(f, "   EXEC_CNT [U4] : {}", self.exec_cnt)?;
        writeln!(f, "   FAIL_CNT [U4] : {}", self.fail_cnt)?;
        writeln!(f, "   ALRM_CNT [U4] : {}", self.alrm_cnt)?;
        writeln!(f, "   TEST_NAM [Cn] : '{}'", self.test_nam)?;
        writeln!(f, "   SEQ_NAME [Cn] : '{}'", self.seq_name)?;
        writeln!(f, "   TEST_LBL [Cn] : '{}'", self.test_lbl)?;
        writeln!(f, "   OPT_FLAG [B1] : {} (see check marks below)", self.opt_flag)?;
        writeln!(f, "   TEST_TIM [R4] : {}", if u8::from(self.opt_flag) & 0b00000100 == 0b00000100 { String::from("∕") } else { self.test_tim.to_string() })?;
        writeln!(f, "   TEST_MIN [R4] : {}", if u8::from(self.opt_flag) & 0b00000001 == 0b00000001 { String::from("∕") } else { self.test_min.to_string() })?;
        writeln!(f, "   TEST_MAX [R4] : {}", if u8::from(self.opt_flag) & 0b00000010 == 0b00000010 { String::from("∕") } else { self.test_max.to_string() })?;
        writeln!(f, "   TST_SUMS [R4] : {}", if u8::from(self.opt_flag) & 0b00010000 == 0b00010000 { String::from("∕") } else { self.tst_sums.to_string() })?;
        writeln!(f, "   TST_SQRS [R4] : {}", if u8::from(self.opt_flag) & 0b00100000 == 0b00100000 { String::from("∕") } else { self.tst_sqrs.to_string() })
    }
}

// ========================================================
// PTR : Parametric Test Record
// ========================================================
#[derive(Debug, PartialEq, Serialize, STDFRecord)]
pub struct PTR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    #[default(R4::from(f32::NAN))]
    pub result: R4,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(B1::from(0x00))]
    pub opt_flag: B1,
    #[default(I1::from(i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(f32::NAN))]
    pub hi_limit: R4,
    #[default(Cn(b""))]
    pub units: Cn<'a>,
    #[default(Cn(b""))]
    pub c_resfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_llmfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_hlmfmt: Cn<'a>,
    #[default(R4::from(f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(f32::NAN))]
    pub hi_spec: R4,
}

record_id!(PTR, true, '_);
atdf!(PTR, '_);

impl fmt::Display for PTR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PTR : Parametric Test Record")?;
        writeln!(f, "   TEST_NUM [U4] : {}", self.test_num)?;
        writeln!(f, "   HEAD_NUM [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM [U1] : {}", self.site_num)?;
        writeln!(f, "   TEST_FLG [B1] : {} {}", self.test_flg, ptr_test_flg(self.test_flg))?;
        writeln!(f, "   PARM_FLG [B1] : {} {}", self.parm_flg, ptr_parm_flg(self.parm_flg))?;
        writeln!(f, "   RESULT   [R4] : {}", self.result)?;
        writeln!(f, "   TEST_TXT [Cn] : '{}'", self.test_txt)?;
        if self.opt_flag == B1::from(0x00) {
            writeln!(f, "   ALARM_ID [Cn] : {}", self.alarm_id)
        } else {
            writeln!(f, "   ALARM_ID [Cn] : {}", self.alarm_id)?;
            writeln!(f, "   --------------")?;
            writeln!(f, "   OPT_FLAG [B1] : {} {}", self.opt_flag, ptr_opt_flag(self.opt_flag))?;
            writeln!(f, "   RES_SCAL [I1] : {}", self.res_scal)?;
            writeln!(f, "   LLM_SCAL [I1] : {}", self.llm_scal)?;
            writeln!(f, "   HLM_SCAL [I1] : {}", self.hlm_scal)?;
            writeln!(f, "   LO_LIMIT [R4] : {}", self.lo_limit)?;
            writeln!(f, "   HI_LIMIT [R4] : {}", self.hi_limit)?;
            writeln!(f, "   UNITS    [Cn] : '{}'", self.units)?;
            writeln!(f, "   C_RESFMT [Cn] : '{}'", self.c_resfmt)?;
            writeln!(f, "   C_LLMFMT [Cn] : '{}'", self.c_llmfmt)?;
            writeln!(f, "   C_HLMFMT [Cn] : '{}'", self.c_hlmfmt)?;
            writeln!(f, "   LO_SPEC  [R4] : {}", self.lo_spec)?;
            writeln!(f, "   HI_SPEC  [R4] : {}", self.hi_spec)
        }
    }
}

fn ptr_test_flg(test_flg: B1) -> String {
    let mut msg = String::new();
    let mut info: Vec<String> = Vec::new(); 

    if u8::from(test_flg) & 0b0000_0001 == 0b0000_0001  { info.push(String::from("Alarm")); }
    if u8::from(test_flg) & 0b0000_0010 == 0b0000_0010  { info.push(String::from("RESULT not valid")); }
    if u8::from(test_flg) & 0b0000_0100 == 0b0000_0100  { info.push(String::from("RESULT is unreliable")); }
    if u8::from(test_flg) & 0b0000_1000 == 0b0000_1000  { info.push(String::from("Timeout")); }
    if u8::from(test_flg) & 0b0001_0000 == 0b0001_0000  { info.push(String::from("Test not executed")); }
    if u8::from(test_flg) & 0b0010_0000 == 0b0010_0000  { info.push(String::from("Test aborted")); }

    msg.push('(');
    msg.push_str(&info.join(", "));
    msg.push_str(") → ");

    if u8::from(test_flg) & 0b0100_0000 == 0b0100_0000  { 
        msg.push('?');
    } else if u8::from(test_flg) & 0b1000_0000 == 0b1000_0000 {
        msg.push_str("FAIL");
    } else {
        msg.push_str("PASS");
    }
    msg
}

fn ptr_parm_flg(parm_flg: B1) -> String {
    let mut msg = String::new();
    let mut info: Vec<String> = Vec::new();

    if u8::from(parm_flg) & 0b0000_0001 == 0b0000_0001  { info.push(String::from("Scale error")); }
    if u8::from(parm_flg) & 0b0000_0010 == 0b0000_0010  { info.push(String::from("Drift error")); }
    if u8::from(parm_flg) & 0b0000_0100 == 0b0000_0100  { info.push(String::from("Oscillation detected")); }
    if u8::from(parm_flg) & 0b0000_1000 == 0b0000_1000  { info.push(String::from("RESULT > HI_LIMIT")); }
    if u8::from(parm_flg) & 0b0001_0000 == 0b0001_0000  { info.push(String::from("RESULT < LO_LIMIT")); }

    msg.push('(');
    msg.push_str(&info.join(", "));
    msg.push(')');

    msg
}

fn ptr_opt_flag(opt_flag:B1) -> String {
    let mut msg = String::new();
    let mut info: Vec<String> = Vec::new();

    if u8::from(opt_flag) & 0b0000_0001 == 0b0000_0001  { info.push(String::from("RES_SCAL is invalid")); }
    if u8::from(opt_flag) & 0b0000_0100 == 0b0000_0100  { info.push(String::from("No low spec limit")); }
    if u8::from(opt_flag) & 0b0000_1000 == 0b0000_1000  { info.push(String::from("No high spec limit")); }
    if u8::from(opt_flag) & 0b0001_0000 == 0b0001_0000  { info.push(String::from("LO_LIMIT and LLM_SCAL are invalid")); }
    if u8::from(opt_flag) & 0b0010_0000 == 0b0010_0000  { info.push(String::from("HI_LIMIT and HLM_SCAL are invalid")); }  
    if u8::from(opt_flag) & 0b0100_0000 == 0b0100_0000  { info.push(String::from("no LO_LIMIT")); } 
    if u8::from(opt_flag) & 0b1000_0000 == 0b1000_0000  { info.push(String::from("no HI_LIMIT")); }

    msg.push('(');
    msg.push_str(&info.join(", "));
    msg.push(')');

    msg
}

// ========================================================
// MRR : Multiple-Result Record
// ========================================================
#[derive(Debug, PartialEq, Serialize, STDFRecord)]
pub struct MPR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    #[default(U2::from(0))]
    pub rtn_icnt: U2,      // j
    #[default(U2::from(0))]
    pub rslt_cnt: U2,      // k
    #[array_length(rtn_icnt)]
    #[array_type(N1)] 
    pub rtn_stat: Vec<N1>, // jxN1
    #[array_length(rslt_cnt)]
    #[array_type(R4)]
    pub rtn_rslt: Vec<R4>, // kxR4
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(B1::from(0x00))]
    pub opt_flag: B1,
    #[default(I1::from(i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(f32::NAN))]
    pub hi_limit: R4,
    #[default(R4::from(f32::NAN))]
    pub start_in: R4,
    #[default(R4::from(f32::NAN))]
    pub incr_in: R4,
    #[array_length(rtn_icnt)]
    #[array_type(U2)]
    pub rtn_indx: Vec<U2>, // jxU2
    #[default(Cn(b""))]
    pub units: Cn<'a>,
    #[default(Cn(b""))]
    pub units_in: Cn<'a>,
    #[default(Cn(b""))]
    pub c_resfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_llmfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_hlmfmt: Cn<'a>,
    #[default(R4::from(f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(f32::NAN))]
    pub hi_spec: R4,
}

record_id!(MPR, true, '_);
atdf!(MPR, '_);

impl fmt::Display for MPR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MPR : Multiple-Result Parametric Record")?;
        writeln!(f, "   TEST_NUM   [U4] : {}", self.test_num)?;
        writeln!(f, "   HEAD_NUM   [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM   [U1] : {}", self.site_num)?;
        writeln!(f, "   TEST_FLG   [B1] : {}", self.test_flg)?;
        writeln!(f, "   PARM_FLG   [B1] : {}", self.parm_flg)?;
        writeln!(f, "   RTN_ICNT j [U2] : {}", self.rtn_icnt)?;
        writeln!(f, "   RSLT_CNT k [U2] : {}", self.rslt_cnt)?;
        writeln!(f, "   RTN_STAT [jxN1] : {:?}", self.rtn_stat)?; //TODO: implement std:fmt::Display for Vec<N1>
        writeln!(f, "   RTN_RSLT [kxR4] : {:?}", self.rtn_rslt)?;  //TODO: implement std:fmt::Display for Vec<R4>
        writeln!(f, "   TEST_TXT   [Cn] : '{}'", self.test_txt)?;
        if self.opt_flag == B1::from(0x00) {
            writeln!(f, "   ALARM_ID   [Cn] : '{}'", self.alarm_id)
        } else {
            writeln!(f, "   ALARM_ID   [Cn] : '{}'", self.alarm_id)?;
            writeln!(f, "   OPT_FLAG   [B1] : {}", self.opt_flag)?;
            writeln!(f, "   RES_SCAL   [I1] : {}", self.res_scal)?;
            writeln!(f, "   LLM_SCAL   [I1] : {}", self.llm_scal)?;
            writeln!(f, "   HLM_SCAL   [I1] : {}", self.hlm_scal)?;
            writeln!(f, "   LO_LIMIT   [R4] : {}", self.lo_limit)?;
            writeln!(f, "   HI_LIMIT   [R4] : {}", self.hi_limit)?;
            writeln!(f, "   START_IN   [R4] : {}", self.start_in)?;
            writeln!(f, "   INCR_IN    [R4] : {}", self.incr_in)?;
            writeln!(f, "   RTN_INDX [jxU2] : {:?}", self.rtn_indx)?; //TODO: implement std:fmt::Display for Vec<U2>
            writeln!(f, "   UNITS      [Cn] : '{}'", self.units)?;
            writeln!(f, "   UNITS_IN   [Cn] : '{}'", self.units_in)?;
            writeln!(f, "   C_RESFMT   [Cn] : '{}'", self.c_resfmt)?;
            writeln!(f, "   C_LLMFMT   [Cn] : '{}'", self.c_llmfmt)?;
            writeln!(f, "   C_HLMFMT   [Cn] : '{}'", self.c_hlmfmt)?;
            writeln!(f, "   LO_SPEC    [R4] : {}", self.lo_spec)?;
            writeln!(f, "   HI_SPEC    [R4] : {}", self.hi_spec) 
        }
    }
}

// ========================================================
// FTR : Functional Test Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct FTR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(U4::from(0))]
    pub cycl_cnt: U4,
    #[default(U4::from(0))]
    pub rel_vadr: U4,
    #[default(U4::from(0))]
    pub rept_cnt: U4,
    #[default(U4::from(0))]
    pub num_fail: U4,
    #[default(I4::from(0))]
    pub xfail_ad: I4,
    #[default(I4::from(0))]
    pub yfail_ad: I4,
    #[default(I2::from(0))]
    pub vect_off: I2,
    #[default(U2::from(0))]
    pub rtn_icnt: U2,
    #[default(U2::from(0))]
    pub pgm_icnt: U2,
    #[array_length(rtn_icnt)]
    #[array_type(U2)]
    pub rtn_indx: Vec<U2>,
    #[array_length(rtn_icnt)]
    #[array_type(N1)]
    pub rtn_stat: Vec<N1>,
    #[array_length(pgm_icnt)]
    #[array_type(U2)]
    pub pgm_indx: Vec<U2>,
    #[array_length(pgm_icnt)]
    #[array_type(N1)]
    pub pgm_stat: Vec<N1>,
    #[default(Dn(0, b""))]
    pub fail_pin: Dn<'a>,
    #[default(Cn(b""))]
    pub vect_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub time_set: Cn<'a>,
    #[default(Cn(b""))]
    pub op_code: Cn<'a>,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(Cn(b""))]
    pub prog_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub rslt_txt: Cn<'a>,
    #[default(U1::from(0xff))]
    pub patg_num: U1,
    #[default(Dn(0, b""))]
    pub spin_map: Dn<'a>,
}

record_id!(FTR, true, '_);

impl fmt::Display for FTR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FTR : Functional Test Record")?;
        writeln!(f, "   TEST_NUM   [U4] : {}", self.test_num)?;
        writeln!(f, "   HEAD_NUM   [U1] : {}", self.head_num)?;
        writeln!(f, "   SITE_NUM   [U1] : {}", self.site_num)?;
        if self.opt_flag == B1::from(0x00) {
            writeln!(f, "   TEST_FLG   [B1] : {}", self.test_flg)
        } else {
            writeln!(f, "   TEST_FLG   [B1] : {}", self.test_flg)?;
            writeln!(f, "   OPT_FLAG   [B1] : {}", self.opt_flag)?;
            writeln!(f, "   CYCL_CNT   [U4] : {}", self.cycl_cnt)?;
            writeln!(f, "   REL_VADR   [U4] : {}", self.rel_vadr)?;
            writeln!(f, "   REPT_CNT   [U4] : {}", self.rept_cnt)?;
            writeln!(f, "   NUM_FAIL   [U4] : {}", self.num_fail)?;
            writeln!(f, "   XFAIL_AD   [I4] : {}", self.xfail_ad)?;
            writeln!(f, "   YFAIL_AD   [I4] : {}", self.yfail_ad)?;
            writeln!(f, "   VECT_OFF   [I2] : {}", self.vect_off)?;
            writeln!(f, "   RTN_ICNT j [U2] : {}", self.rtn_icnt)?;
            writeln!(f, "   PGM_ICNT k [U2] : {}", self.pgm_icnt)?;
            writeln!(f, "   RTN_INDX [jxU2] : {:?}", self.rtn_indx)?;
            writeln!(f, "   RTN_STAT [jxN1] : {:?}", self.rtn_stat)?;
            writeln!(f, "   PGM_INDX [kxU2] : {:?}", self.pgm_indx)?;
            writeln!(f, "   PGM_STAT [kxN1] : {:?}", self.pgm_stat)?;
            writeln!(f, "   FAIL_PIN   [Dn] : {}", self.fail_pin)?;
            writeln!(f, "   VECT_NAM   [Cn] : '{}'", self.vect_nam)?;
            writeln!(f, "   TIME_SET   [Cn] : '{}'", self.time_set)?;
            writeln!(f, "   OP_CODE    [Cn] : '{}'", self.op_code)?;
            writeln!(f, "   TEST_TXT   [Cn] : '{}'", self.test_txt)?;
            writeln!(f, "   ALARM_ID   [Cn] : '{}'", self.alarm_id)?;
            writeln!(f, "   PROG_TXT   [Cn] : '{}'", self.prog_txt)?;
            writeln!(f, "   RSLT_TXT   [Cn] : '{}'", self.rslt_txt)?;
            writeln!(f, "   PATG_NUM   [U1] : {}", self.patg_num)?;
            writeln!(f, "   SPIN_MAP   [Dn] : {}", self.spin_map)
        }
    }
}

// ========================================================
// BPS : Begin Program Section
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct BPS<'a> {
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
}

record_id!(BPS, false, '_);
atdf!(BPS, '_);

impl fmt::Display for BPS<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BPS : Begin Program Section record")?;
        writeln!(f, "   SEQ_NAME [Cn] : '{}'", self.seq_name)
    }
}

// ========================================================
// EPS : End Program Section
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct EPS;

record_id!(EPS, false,);
atdf!(EPS,);

impl fmt::Display for EPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EPS : End Program Section record")
    }
}

// ========================================================
// GDR : Generic Data Record
// ========================================================
#[derive(Debug, PartialEq, Serialize, STDFRecord)]
pub struct GDR<'a> {
    #[default(U2::from(0))]
    pub fld_cnt: U2,
    #[array_length(fld_cnt)]
    #[array_type(Vn<'a>)]
    pub gen_data: Vec<Vn<'a>>,
}

record_id!(GDR, false, '_);
atdf!(GDR, '_);

impl fmt::Display for GDR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GDR")?;
        writeln!(f, "   FLD_CNT  [U2]: {}", self.fld_cnt)?;
        writeln!(f, "   GEN_DATA [Vn]: {:?}", self.gen_data)
    }
}

// ========================================================
// DTR : Datalog Text Record
// ========================================================
#[derive(Debug, Eq, PartialEq, Serialize, STDFRecord)]
pub struct DTR<'a> {
    #[default(Cn(b""))]
    pub text_dat: Cn<'a>,
}

record_id!(DTR, false, '_);
atdf!(DTR, '_);

impl fmt::Display for DTR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DTR : Datalog Text Record")?;
        writeln!(f, "   TEXT_DAT [Cn] : '{}'", self.text_dat)
    }
}

// ========================================================
// Raw record
// ========================================================
#[derive(Debug, Eq, PartialEq)]
pub struct Raw<'a> {
    pub rec_typ: U1,
    pub rec_sub: U1,
    pub contents: &'a [u8],
}

//TODO: Implement TryRead for Raw
//TODO: Implement TryWrite for Raw
//TODO: Implement Display for Raw

#[derive(Debug)]
pub enum V4<'a> {
    FAR(FAR),
    ATR(ATR<'a>),
    MIR(MIR<'a>),
    MRR(MRR<'a>),
    PCR(PCR),
    HBR(HBR<'a>),
    SBR(SBR<'a>),
    PMR(PMR<'a>),
    PGR(PGR<'a>),
    PLR(PLR<'a>),
    RDR(RDR),
    SDR(SDR<'a>),
    WIR(WIR<'a>),
    WRR(WRR<'a>),
    WCR(WCR),
    PIR(PIR),
    PRR(PRR<'a>),
    TSR(TSR<'a>),
    PTR(PTR<'a>),
    MPR(MPR<'a>),
    FTR(FTR<'a>),
    BPS(BPS<'a>),
    EPS(EPS),
    GDR(GDR<'a>),
    DTR(DTR<'a>),
    Unknown(Raw<'a>),
    Invalid(Raw<'a>),
}

impl V4<'_> {
    pub fn name(&self) -> String {
        match self {
            V4::FAR(rec) => rec.get_name_as_string(),
            V4::ATR(rec) => rec.get_name_as_string(),
            V4::MIR(rec) => rec.get_name_as_string(),
            V4::MRR(rec) => rec.get_name_as_string(),
            V4::PCR(rec) => rec.get_name_as_string(),
            V4::HBR(rec) => rec.get_name_as_string(),
            V4::SBR(rec) => rec.get_name_as_string(),
            V4::PMR(rec) => rec.get_name_as_string(),
            V4::PGR(rec) => rec.get_name_as_string(),
            V4::PLR(rec) => rec.get_name_as_string(),
            V4::RDR(rec) => rec.get_name_as_string(),
            V4::SDR(rec) => rec.get_name_as_string(),
            V4::WIR(rec) => rec.get_name_as_string(),
            V4::WRR(rec) => rec.get_name_as_string(),
            V4::WCR(rec) => rec.get_name_as_string(),
            V4::PIR(rec) => rec.get_name_as_string(),
            V4::PRR(rec) => rec.get_name_as_string(),
            V4::TSR(rec) => rec.get_name_as_string(),
            V4::PTR(rec) => rec.get_name_as_string(),
            V4::MPR(rec) => rec.get_name_as_string(),
            V4::FTR(rec) => rec.get_name_as_string(),
            V4::BPS(rec) => rec.get_name_as_string(),
            V4::EPS(rec) => rec.get_name_as_string(),
            V4::GDR(rec) => rec.get_name_as_string(),
            V4::DTR(rec) => rec.get_name_as_string(),
            V4::Unknown(_) => "???".to_string(),
            V4::Invalid(_) => "???".to_string()
        }
    }

    pub fn rec_typ_sub(&self) -> (u8, u8) {
        match self {
            V4::FAR(_) => (0, 10),
            V4::ATR(_) => (0, 20),
            V4::MIR(_) => (1, 10),
            V4::MRR(_) => (1, 20),
            V4::PCR(_) => (1, 30),
            V4::HBR(_) => (1, 40),
            V4::SBR(_) => (1, 50),
            V4::PMR(_) => (1, 60),
            V4::PGR(_) => (1, 62),
            V4::PLR(_) => (1, 63),
            V4::RDR(_) => (1, 70),
            V4::SDR(_) => (1, 80),
            V4::WIR(_) => (2, 10),
            V4::WRR(_) => (2, 20),
            V4::WCR(_) => (2, 30),
            V4::PIR(_) => (5, 10),
            V4::PRR(_) => (5, 20),
            V4::TSR(_) => (10, 30),
            V4::PTR(_) => (15, 10),
            V4::MPR(_) => (15, 15),
            V4::FTR(_) => (15, 20),
            V4::BPS(_) => (20, 10),
            V4::EPS(_) => (20, 20),
            V4::GDR(_) => (50, 10),
            V4::DTR(_) => (50, 30),
            V4::Unknown(ref r) => (u8::from(&r.rec_typ), u8::from(&r.rec_sub)),
            V4::Invalid(ref r) => (u8::from(&r.rec_typ), u8::from(&r.rec_sub)),
        }
    }
}

impl<'a> TryRead<'a, ctx::Endian> for V4<'a> {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let header = bytes.read_with::<Header>(offset, endian)?;
        let typ_sub = (u8::from(&header.rec_typ), u8::from(&header.rec_sub));
        let reclen = u16::from(&header.rec_len) as usize;
        let rec_bytes = &bytes[*offset..*offset + reclen];
        let rec_offset = &mut 0;
        let mut parse_rec = || {
            let rec = match typ_sub {
                (0, 10) => V4::FAR(rec_bytes.read_with::<FAR>(rec_offset, endian)?),
                (0, 20) => V4::ATR(rec_bytes.read_with::<ATR>(rec_offset, endian)?),
                (1, 10) => V4::MIR(rec_bytes.read_with::<MIR>(rec_offset, endian)?),
                (1, 20) => V4::MRR(rec_bytes.read_with::<MRR>(rec_offset, endian)?),
                (1, 30) => V4::PCR(rec_bytes.read_with::<PCR>(rec_offset, endian)?),
                (1, 40) => V4::HBR(rec_bytes.read_with::<HBR>(rec_offset, endian)?),
                (1, 50) => V4::SBR(rec_bytes.read_with::<SBR>(rec_offset, endian)?),
                (1, 60) => V4::PMR(rec_bytes.read_with::<PMR>(rec_offset, endian)?),
                (1, 62) => V4::PGR(rec_bytes.read_with::<PGR>(rec_offset, endian)?),
                (1, 63) => V4::PLR(rec_bytes.read_with::<PLR>(rec_offset, endian)?),
                (1, 70) => V4::RDR(rec_bytes.read_with::<RDR>(rec_offset, endian)?),
                (1, 80) => V4::SDR(rec_bytes.read_with::<SDR>(rec_offset, endian)?),
                (2, 10) => V4::WIR(rec_bytes.read_with::<WIR>(rec_offset, endian)?),
                (2, 20) => V4::WRR(rec_bytes.read_with::<WRR>(rec_offset, endian)?),
                (2, 30) => V4::WCR(rec_bytes.read_with::<WCR>(rec_offset, endian)?),
                (5, 10) => V4::PIR(rec_bytes.read_with::<PIR>(rec_offset, endian)?),
                (5, 20) => V4::PRR(rec_bytes.read_with::<PRR>(rec_offset, endian)?),
                (10, 30) => V4::TSR(rec_bytes.read_with::<TSR>(rec_offset, endian)?),
                (15, 10) => V4::PTR(rec_bytes.read_with::<PTR>(rec_offset, endian)?),
                (15, 15) => V4::MPR(rec_bytes.read_with::<MPR>(rec_offset, endian)?),
                (15, 20) => V4::FTR(rec_bytes.read_with::<FTR>(rec_offset, endian)?),
                (20, 10) => V4::BPS(rec_bytes.read_with::<BPS>(rec_offset, endian)?),
                (20, 20) => V4::EPS(EPS),
                (50, 10) => V4::GDR(rec_bytes.read_with::<GDR>(rec_offset, endian)?),
                (50, 30) => V4::DTR(rec_bytes.read_with::<DTR>(rec_offset, endian)?),
                (typ, sub) => V4::Unknown(Raw {
                    rec_typ: U1::from(typ),
                    rec_sub: U1::from(sub),
                    contents: rec_bytes,
                }),
            };
            Ok(rec)
        };
        let rec = match parse_rec() {
            Ok(rec) => rec,
            Err(byte::Error::BadInput { err }) => return Err(byte::Error::BadInput { err }),
            Err(_) => V4::Invalid(Raw {
                rec_typ: U1::from(typ_sub.0),
                rec_sub: U1::from(typ_sub.1),
                contents: rec_bytes,
            }),
        };
        *offset += reclen;
        Ok((rec, *offset))
    }
}

impl TryWrite<ctx::Endian> for V4<'_> {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        let (typ, sub) = self.rec_typ_sub();
        let mut rec_bytes: Vec<u8> = vec![];
        let rec_offset = &mut 0;
        match self {
            V4::FAR(r) => rec_bytes.write_with::<FAR>(rec_offset, r, endian),
            V4::ATR(r) => rec_bytes.write_with::<ATR>(rec_offset, r, endian),
            V4::MIR(r) => rec_bytes.write_with::<MIR>(rec_offset, r, endian),
            V4::MRR(r) => rec_bytes.write_with::<MRR>(rec_offset, r, endian),
            V4::PCR(r) => rec_bytes.write_with::<PCR>(rec_offset, r, endian),
            V4::HBR(r) => rec_bytes.write_with::<HBR>(rec_offset, r, endian),
            V4::SBR(r) => rec_bytes.write_with::<SBR>(rec_offset, r, endian),
            V4::PMR(r) => rec_bytes.write_with::<PMR>(rec_offset, r, endian),
            V4::PGR(r) => rec_bytes.write_with::<PGR>(rec_offset, r, endian),
            V4::PLR(r) => rec_bytes.write_with::<PLR>(rec_offset, r, endian),
            V4::RDR(r) => rec_bytes.write_with::<RDR>(rec_offset, r, endian),
            V4::SDR(r) => rec_bytes.write_with::<SDR>(rec_offset, r, endian),
            V4::WIR(r) => rec_bytes.write_with::<WIR>(rec_offset, r, endian),
            V4::WRR(r) => rec_bytes.write_with::<WRR>(rec_offset, r, endian),
            V4::WCR(r) => rec_bytes.write_with::<WCR>(rec_offset, r, endian),
            V4::PIR(r) => rec_bytes.write_with::<PIR>(rec_offset, r, endian),
            V4::PRR(r) => rec_bytes.write_with::<PRR>(rec_offset, r, endian),
            V4::TSR(r) => rec_bytes.write_with::<TSR>(rec_offset, r, endian),
            V4::PTR(r) => rec_bytes.write_with::<PTR>(rec_offset, r, endian),
            V4::MPR(r) => rec_bytes.write_with::<MPR>(rec_offset, r, endian),
            V4::FTR(r) => rec_bytes.write_with::<FTR>(rec_offset, r, endian),
            V4::BPS(r) => rec_bytes.write_with::<BPS>(rec_offset, r, endian),
            V4::EPS(_) => Ok(()),
            V4::GDR(r) => rec_bytes.write_with::<GDR>(rec_offset, r, endian),
            V4::DTR(r) => rec_bytes.write_with::<DTR>(rec_offset, r, endian),
            V4::Unknown(_) => return Ok(0), // TODO: write unknown records
            V4::Invalid(_) => return Ok(0),
        }?;
        let header = Header {
            rec_len: U2::from(*rec_offset as u16),
            rec_typ: U1::from(typ),
            rec_sub: U1::from(sub),
        };
        bytes.write_with::<Header>(offset, header, endian)?;
        bytes.write::<&[u8]>(offset, &rec_bytes)?;
        Ok(*offset)
    }
}


//TODO: Implement std::fmt::Display for V4
// impl <'a> fmt::Display for V4<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             V4::MRR(rec) => write!(f, "{}", rec),
//             _ => todo!(),
//         }
//     }
// }
//TODO: Implement to_atdf for V4

pub fn is_supported_records() -> Vec<String> {
    vec![
        "FAR".to_string(),
        "ATR".to_string(),
        "MIR".to_string(),
        "MRR".to_string(),
        "PCR".to_string(),
        "HBR".to_string(),
        "SBR".to_string(),
        "PMR".to_string(),
        "PGR".to_string(),
        "PLR".to_string(),
        "RDR".to_string(),
        "SDR".to_string(),
        "WIR".to_string(),
        "WRR".to_string(),
        "WCR".to_string(),
        "PIR".to_string(),
        "PRR".to_string(),
        "TSR".to_string(),
        "PTR".to_string(),
        "MPR".to_string(),
        "FTR".to_string(),
        "BPS".to_string(),
        "EPS".to_string(),
        "GDR".to_string(),
        "DTR".to_string(),
    ]
    
}


//TODO: Document this function ... do we really need this ?!?
pub fn is_supported_typ_sub(typ_sub: (u8, u8)) -> bool {
    matches!(typ_sub, (0, 10) | (0, 20) | (1, 10) | (1, 20) | (1, 30) | (1, 40) | (1, 50) | (1, 60) | (1, 62) | (1, 63) | (1, 70) | (1, 80) | (2, 10) | (2, 20) | (2, 30) | (5, 10) | (5, 20) | (10, 30) | (15, 10) | (15, 15) | (15, 20) | (20, 10) | (20, 20) | (50, 10) | (50, 30)) 
}

/// Returns the name of the STDF record type and subtype.
///
/// This function takes a record type and subtype as input and returns the
/// corresponding name of the STDF (Standard Test Data Format) record. If the
/// type and subtype combination is not recognized, it returns "Unknown".
///
/// # Arguments
///
/// * `typ` - The record type as a `u8`.
/// * `sub` - The record subtype as a `u8`.
///
/// # Returns
///
/// A `&'static str` representing the name of the STDF record type and subtype.
///
/// # Examples
///
/// ```
/// use stdf::records::typ_sub_to_name;
///
/// let name = typ_sub_to_name(1, 10);
/// assert_eq!(name, "MIR");
///
/// let unknown_name = typ_sub_to_name(99, 99);
/// assert_eq!(unknown_name, "???");
/// ```
pub fn typ_sub_to_name(typ: u8, sub: u8) -> String {
    match (typ, sub) {
        (0, 10) => "FAR".to_string(),
        (0, 20) => "ATR".to_string(),
        (1, 10) => "MIR".to_string(),
        (1, 20) => "MRR".to_string(),
        (1, 30) => "PCR".to_string(),
        (1, 40) => "HBR".to_string(),
        (1, 50) => "SBR".to_string(),
        (1, 60) => "PMR".to_string(),
        (1, 62) => "PGR".to_string(),
        (1, 63) => "PLR".to_string(),
        (1, 70) => "RDR".to_string(),
        (1, 80) => "SDR".to_string(),
        (2, 10) => "WIR".to_string(),
        (2, 20) => "WRR".to_string(),
        (2, 30) => "WCR".to_string(),
        (5, 10) => "PIR".to_string(),
        (5, 20) => "PRR".to_string(),
        (10, 30) => "TSR".to_string(),
        (15, 10) => "PTR".to_string(),
        (15, 15) => "MPR".to_string(),
        (15, 20) => "FTR".to_string(),
        (20, 10) => "BPS".to_string(),
        (20, 20) => "EPS".to_string(),
        (50, 10) => "GDR".to_string(),
        (50, 30) => "DTR".to_string(),
        _ => "???".to_string(),
    }
}

/// Returns the type and subtype of the STDF record given its name.
///
/// This function takes the name of an STDF (Standard Test Data Format) record
/// and returns a tuple containing the record type and subtype as `u8` values.
/// If the name is not recognized, it returns `(0, 0)`.
///
/// # Arguments
///
/// * `name` - The name of the STDF record as a `&str`.
///
/// # Returns
///
/// A `(u8, u8)` tuple representing the type and subtype of the STDF record.
///
/// # Examples
///
/// ```
/// use stdf::records::name_to_typ_sub;
///
/// let typ_sub = name_to_typ_sub("MIR");
/// assert_eq!(typ_sub, (1, 10));
///
/// let unknown_typ_sub = name_to_typ_sub("UNKNOWN");
/// assert_eq!(unknown_typ_sub, (0, 0));
/// ```
pub fn name_to_typ_sub(name: &str) -> (u8, u8) {
    match name {
        "FAR" => (0, 10),
        "ATR" => (0, 20),
        "MIR" => (1, 10),
        "MRR" => (1, 20),
        "PCR" => (1, 30),
        "HBR" => (1, 40),
        "SBR" => (1, 50),
        "PMR" => (1, 60),
        "PGR" => (1, 62),
        "PLR" => (1, 63),
        "RDR" => (1, 70),
        "SDR" => (1, 80),
        "WIR" => (2, 10),
        "WRR" => (2, 20),
        "WCR" => (2, 30),
        "PIR" => (5, 10),
        "PRR" => (5, 20),
        "TSR" => (10, 30),
        "PTR" => (15, 10),
        "MPR" => (15, 15),
        "FTR" => (15, 20),
        "BPS" => (20, 10),
        "EPS" => (20, 20),
        "GDR" => (50, 10),
        "DTR" => (50, 30),
        _ => (0, 0),
    }
}

// pub fn is_test_record(typ:u8) -> bool {
//     if typ == 15 {
//         true
//     } else {
//         false
//     }
// }

#[cfg(test)]
mod tests {

    macro_rules! assert_float {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    use super::*;
    use byte::{BytesExt, BE, LE};

    #[test]
    fn test_header() {
        let b: &[u8] = &[0x00, 0x01, 0xa5, 0x5a];
        let offset = &mut 0;
        let header = b.read_with::<Header>(offset, BE).unwrap();
        assert_eq!(
            header,
            Header {
                rec_len: U2::from(1),
                rec_typ: U1::from(0xa5),
                rec_sub: U1::from(0x5a)
            }
        );
        let mut out = vec![0; b.len()];
        out.write_with(&mut 0, header, BE).unwrap();
        assert_eq!(b, out.as_slice());

        *offset = 0;
        let header = b.read_with::<Header>(offset, LE).unwrap();
        assert_eq!(
            header,
            Header {
                rec_len: U2::from(256),
                rec_typ: U1::from(0xa5),
                rec_sub: U1::from(0x5a)
            }
        );
        let mut out = vec![0; b.len()];
        out.write_with(&mut 0, header, LE).unwrap();
        assert_eq!(b, out.as_slice());
    }

    #[test]
    fn test_far() {
        let b: &[u8] = &[0x02, 0x00, 0u8, 10u8, 2u8, 4u8];
        let offset = &mut 0;
        let endian = Header::detect_endian(b).unwrap();
        assert_eq!(endian, LE);
        let header = b.read_with::<Header>(offset, endian).unwrap();
        let far = b.read_with::<FAR>(offset, endian).unwrap();
        assert_eq!(
            header,
            Header {
                rec_len: U2::from(2),
                rec_typ: U1::from(0),
                rec_sub: U1::from(10),
            }
        );
        assert_eq!(
            far,
            FAR {
                cpu_type: U1::from(2),
                stdf_ver: U1::from(4),
            }
        );
        let mut out = vec![0; b.len()];
        *offset = 0;
        out.write_with(offset, header, endian).unwrap();
        out.write_with(offset, far, endian).unwrap();
        assert_eq!(b, out.as_slice());

        let b: &[u8] = &[0x00, 0x02, 0u8, 10u8, 1u8, 4u8];
        assert_eq!(Header::detect_endian(b).unwrap(), BE);
    }

    #[test]
    fn test_mpr_single_nibble() {
        // BE representation
        let b: &[u8] = &[
            0u8, 23u8, 0x0fu8, 0x0fu8, // MPR
            0u8, 0u8, 0u8, 103u8, // 4: test_num 103
            1u8, 2u8, // 6: head_num 0, site_num 0
            0u8, // 7: test_flg 0
            0u8, // 8: parm_flg 0
            0u8, 1u8, // 10: rtn_icnt 1
            0u8, 2u8,  // 12: rslt_cnt 2
            0xa5, // 13: rtn_stat 5 (a should be ignored)
            0x3d, 0xcc, 0xcc, 0xcd, // 17: 0.1
            0x3e, 0x4c, 0xcc, 0xcd, // 23: 0.2
        ];
        let offset = &mut 0;
        let header = b.read_with::<Header>(offset, BE).unwrap();
        assert_eq!(
            header,
            Header {
                rec_len: U2::from(23),
                rec_typ: U1::from(15),
                rec_sub: U1::from(15),
            }
        );
        let mpr = b.read_with::<MPR>(offset, BE).unwrap();
        assert_eq!(mpr.test_num, U4::from(103));
        assert_eq!(mpr.head_num, U1::from(1));
        assert_eq!(mpr.site_num, U1::from(2));
        assert_eq!(mpr.rtn_icnt, U2::from(1));
        assert_eq!(mpr.rslt_cnt, U2::from(2));
        assert_eq!(mpr.rtn_stat, vec![N1::from(0x5)]); // only low nibble
        assert_eq!(mpr.rtn_rslt.len(), 2);
        assert_float!(mpr.rtn_rslt[0].0, 0.1, f32::EPSILON);
        assert_float!(mpr.rtn_rslt[1].0, 0.2, f32::EPSILON);

        // note: this record won't round-trip because the upper nibble of rtn_stat is not parsed
    }

    #[test]
    fn test_mpr_round_trip() {
        // BE representation
        let b: &[u8] = &[
            0u8, 29u8, 0x0fu8, 0x0fu8, // MPR
            0u8, 0u8, 0u8, 103u8, // 4: test_num 103
            1u8, 2u8, // 6: head_num 0, site_num 0
            0u8, // 7: test_flg 0
            0u8, // 8: parm_flg 0
            0u8, 2u8, // 10: rtn_icnt 1
            0u8, 2u8,  // 12: rslt_cnt 2
            0xa5, // 13: rtn_stat a, 5
            0x3d, 0xcc, 0xcc, 0xcd, // 17: 0.1
            0x3e, 0x4c, 0xcc, 0xcd, // 23: 0.2
            5u8, b'h', b'e', b'l', b'l', b'o',
        ];
        let offset = &mut 0;
        let header = b.read_with::<Header>(offset, BE).unwrap();
        assert_eq!(
            header,
            Header {
                rec_len: U2::from(29),
                rec_typ: U1::from(15),
                rec_sub: U1::from(15),
            }
        );
        let mpr = b.read_with::<MPR>(offset, BE).unwrap();
        assert_eq!(mpr.test_num, U4::from(103));
        assert_eq!(mpr.head_num, U1::from(1));
        assert_eq!(mpr.site_num, U1::from(2));
        assert_eq!(mpr.rtn_icnt, U2::from(2));
        assert_eq!(mpr.rslt_cnt, U2::from(2));
        assert_eq!(mpr.rtn_stat, vec![N1::from(0x5), N1::from(0xa)]); // only low nibble
        assert_eq!(mpr.rtn_rslt.len(), 2);
        assert_float!(mpr.rtn_rslt[0].0, 0.1, f32::EPSILON);
        assert_float!(mpr.rtn_rslt[1].0, 0.2, f32::EPSILON);
        assert_eq!(mpr.test_txt, Cn(b"hello"));

        let mut out = vec![0; *offset];
        *offset = 0;
        out.write_with(offset, header, BE).unwrap();
        out.write_with(offset, mpr, BE).unwrap();
        assert_eq!(b, out.as_slice());
    }
}
