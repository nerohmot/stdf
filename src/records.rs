#![allow(unused_parens)]
use byte::ctx;
use byte::{BytesExt, TryRead, TryWrite};
use std::fmt;

use crate::types::*;

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

impl<'a> TryWrite<ctx::Endian> for Header {
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct FAR {
    pub cpu_type: U1,
    pub stdf_ver: U1,
}

impl FAR {
    pub fn name() -> String {
        "FAR".to_string()
    }
}

impl fmt::Display for FAR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FAR : File Attrubute Record\n")?;
        write!(f, "   CPU_TYPE [U1] : {}\n", self.cpu_type)?;
        write!(f, "   STDF_VER [U1] : {}\n", self.stdf_ver)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct ATR<'a> {
    #[default(U4E::from(0))]
    pub mod_tim: U4E,
    #[default(Cn(b""))]
    pub cmd_line: Cn<'a>,
}

impl ATR<'_> {
    pub fn name() -> String {
        "ATR".to_string()
    }
}

impl<'a> fmt::Display for ATR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ATR : Audit Trail Record\n")?;
        write!(f, "   MOD_TIM [U4E]: {}\n", self.mod_tim)?;
        write!(f, "   CMD_LINE [Cn]: '{}'\n", self.cmd_line)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl MIR<'_> {
    pub fn name() -> String {
        "MIR".to_string()
    }
}

impl<'a> fmt::Display for MIR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MIR : Master Information Record\n")?;
        write!(f, "   SETUP_T [U4E] : {}\n", self.setup_t)?;
        write!(f, "   START_T [U4E] : {}\n", self.start_t)?;
        write!(f, "   STAT_NUM [U1] : {}\n", self.stat_num)?;
        write!(f, "   MODE_COD [C1] : '{}'\n", self.mode_cod)?;
        write!(f, "   RTST_COD [C1] : '{}'\n", self.rtst_cod)?;
        write!(f, "   PROT_COD [C1] : '{}'\n", self.prot_cod)?;
        write!(f, "   BURN_TIM [U2] : {}\n", self.burn_tim)?;
        write!(f, "   CMOD_COD [C1] : '{}'\n", self.cmod_cod)?;
        write!(f, "   LOT_ID   [Cn] : '{}'\n", self.lot_id)?;
        write!(f, "   PART_TYP [Cn] : '{}'\n", self.part_typ)?;
        write!(f, "   NODE_NAM [Cn] : '{}'\n", self.node_nam)?;
        write!(f, "   TSTR_TYP [Cn] : '{}'\n", self.tstr_typ)?;
        write!(f, "   JOB_NAM  [Cn] : '{}'\n", self.job_nam)?;
        write!(f, "   JOB_REV  [Cn] : '{}'\n", self.job_rev)?;
        write!(f, "   SBLOT_ID [Cn] : '{}'\n", self.sblot_id)?;
        write!(f, "   OPER_NAM [Cn] : '{}'\n", self.oper_nam)?;
        write!(f, "   EXEC_TYP [Cn] : '{}'\n", self.exec_typ)?;
        write!(f, "   EXEC_VER [Cn] : '{}'\n", self.exec_ver)?;
        write!(f, "   TEST_COD [Cn] : '{}'\n", self.test_cod)?;
        write!(f, "   TST_TEMP [Cn] : '{}'\n", self.tst_temp)?;
        write!(f, "   USER_TXT [Cn] : '{}'\n", self.user_txt)?;
        write!(f, "   AUX_FILE [Cn] : '{}'\n", self.aux_file)?;
        write!(f, "   PKG_TYP  [Cn] : '{}'\n", self.pkg_typ)?;
        write!(f, "   FAMLY_ID [Cn] : '{}'\n", self.famly_id)?;
        write!(f, "   DATE_COD [Cn] : '{}'\n", self.date_cod)?;
        write!(f, "   FACIL_ID [Cn] : '{}'\n", self.facil_id)?;
        write!(f, "   FLOOR_ID [Cn] : '{}'\n", self.floor_id)?;
        write!(f, "   PROC_ID  [Cn] : '{}'\n", self.proc_id)?;
        write!(f, "   OPER_FRQ [Cn] : '{}'\n", self.oper_frq)?;
        write!(f, "   SPEC_NAM [Cn] : '{}'\n", self.spec_nam)?;
        write!(f, "   SPEC_VER [Cn] : '{}'\n", self.spec_ver)?;
        write!(f, "   FLOW_ID  [Cn] : '{}'\n", self.flow_id)?;
        write!(f, "   SETUP_ID [Cn] : '{}'\n", self.setup_id)?;
        write!(f, "   DSGN_REV [Cn] : '{}'\n", self.dsgn_rev)?;
        write!(f, "   ENG_ID   [Cn] : '{}'\n", self.eng_id)?;
        write!(f, "   ROM_COD  [Cn] : '{}'\n", self.rom_cod)?;
        write!(f, "   SERL_NUM [Cn] : '{}'\n", self.serl_num)?;
        write!(f, "   SUPR_NAM [Cn] : '{}'\n", self.supr_nam)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct MRR<'a> {
    pub finish_t: U4E,
    #[default(C1::from(b' '))]
    pub disp_cod: C1,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

impl MRR<'_> {
    pub fn name() -> String {
        "MRR".to_string()
    }
}

impl <'a> fmt::Display for MRR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MRR : Master Result Record\n")?;
        write!(f, "   FINISH_T [U4E] : {}\n", self.finish_t)?;
        write!(f, "   DISP_COD  [C1] : '{}'\n", self.disp_cod)?;
        write!(f, "   USR_DESC  [Cn] : '{}'\n", self.usr_desc)?;
        write!(f, "   EXC_DESC  [Cn] : '{}'\n", self.exc_desc)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl PCR {
    pub fn name() -> String {
        "PCR".to_string()
    }
}

impl fmt::Display for PCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PCR : Part Count Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)?;
        write!(f, "   PART_CNT [U4] : {}\n", self.part_cnt)?;
        write!(f, "   RTST_CNT [U4] : {}\n", self.rtst_cnt)?;
        write!(f, "   ABRT_CNT [U4] : {}\n", self.abrt_cnt)?;
        write!(f, "   GOOD_CNT [U4] : {}\n", self.good_cnt)?;
        write!(f, "   FUNC_CNT [U4] : {}\n", self.func_cnt)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl HBR<'_> {
    pub fn name() -> String {
        "HBR".to_string()
    }
}

impl <'a> fmt::Display for HBR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HBR : Hard Bin Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)?;
        write!(f, "   HBIN_NUM [U2] : {}\n", self.hbin_num)?;
        write!(f, "   HBIN_CNT [U4] : {}\n", self.hbin_cnt)?;
        write!(f, "   HBIN_PF  [C1] : '{}'\n", self.hbin_pf)?;
        write!(f, "   HBIN_NAM [Cn] : '{}'\n", self.hbin_nam)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl SBR<'_> {
    pub fn name() -> String {
        "SBR".to_string()
    }
}

impl <'a> fmt::Display for SBR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SBR : Soft Bin Record\n")?;
        write!(f, "   HEAD_NUM [U1]: {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1]: {}\n", self.site_num)?;
        write!(f, "   SBIN_NUM [U2]: {}\n", self.sbin_num)?;
        write!(f, "   SBIN_CNT [U4]: {}\n", self.sbin_cnt)?;
        write!(f, "   SBIN_PF  [C1]: '{}'\n", self.sbin_pf)?;
        write!(f, "   SBIN_NAM [Cn]: '{}'\n", self.sbin_nam)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl PMR<'_> {
    pub fn name() -> String {
        "PMR".to_string()
    }
}

impl <'a> fmt::Display for PMR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PMR : Pin Map Record\n")?;
        write!(f, "   PMR_INDEX [U2] : {}\n", self.pmr_index)?;
        write!(f, "   CHAN_TYP  [U2] : {}\n", self.chan_typ)?;
        write!(f, "   CHAN_NAM  [Cn] : '{}'\n", self.chan_nam)?;
        write!(f, "   PHY_NAM   [Cn] : '{}'\n", self.phy_nam)?;
        write!(f, "   LOG_NAM   [Cn] : {}\n", self.log_nam)?;
        write!(f, "   HEAD_NUM  [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM  [U1] : {}\n", self.site_num)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl PGR<'_> {
    pub fn name() -> String {
        "PGR".to_string()
    }
}

impl <'a> fmt::Display for PGR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PGR : Pin Group Record\n")?;
        write!(f, "   GRP_INDX   [U2] : {}\n", self.grp_indx)?;
        write!(f, "   GRP_NAM    [Cn] : {}\n", self.grp_nam)?;
        write!(f, "   INDX_CNT k [U2] : {}\n", self.indx_cnt)?;
        write!(f, "   PMR_INDX [kxU2] : {:?}\n", self.pmr_indx)  //TODO: implement std::fmt::Display for Vec<U2>
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl PLR<'_> {
    pub fn name() -> String {
        "PLR".to_string()
    }
}

impl <'a> fmt::Display for PLR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PLR : Pin List Record\n")?;
        write!(f, "   GRP_CNT  k [U2]: {}\n", self.grp_cnt)?;
        write!(f, "   GRP_INDX [kxU2]: {:?}\n", self.grp_indx)?; //TODO: implement std::fmt::Display for Vec<U2>
        write!(f, "   GRP_MODE [kxU2]: {:?}\n", self.grp_mode)?; //TODO: implement std::fmt::Display for Vec<U2>
        write!(f, "   GRP_RADX [kxU1]: {:?}\n", self.grp_radx)?; //TODO: implement std::fmt::Display for Vec<U1>
        write!(f, "   PGM_CHAR [kxCn]: {:?}\n", self.pgm_char)?; //TODO: implement std::fmt::Display for Vec<Cn>
        write!(f, "   RTN_CHAR [kxCn]: {:?}\n", self.rtn_char)?; //TODO: implement std::fmt::Display for Vec<Cn>
        write!(f, "   PGM_CHAL [kxCn]: {:?}\n", self.pgm_chal)?; //TODO: implement std::fmt::Display for Vec<Cn>
        write!(f, "   RTN_CHAL [kxCn]: {:?}\n", self.rtn_chal)   //TODO: implement std::fmt::Display for Vec<Cn>
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct RDR {
    pub num_bins: U2,
    #[array_length(num_bins)]
    #[array_type(U2)]
    pub rtst_bin: Vec<U2>,
}

impl RDR {
    pub fn name() -> String {
        "RDR".to_string()
    }
}

impl fmt::Display for RDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RDR : Retest Data Record\n")?;
        write!(f, "   NUM_BINS k [U2] : {}\n", self.num_bins)?;
        write!(f, "   RTST_BIN [kxU2] : {:?}\n", self.rtst_bin) //TODO: implement std::fmt::Display for Vec<U2>
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl SDR<'_> {
    pub fn name() -> String {
        "SDR".to_string()
    }
}

impl <'a> fmt::Display for SDR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SDR : Site Description Record\n")?;
        write!(f, "   HEAD_NUM   [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_GRP   [U1] : {}\n", self.site_grp)?;
        write!(f, "   SITE_CNT k [U1] : {}\n", self.site_cnt)?;
        write!(f, "   SITE_NUM [kxU1] : {:?}\n", self.site_num)?; //TODO: implement std::fmt::Display for Vec<U1>
        write!(f, "   HAND_TYP   [Cn] : '{}'\n", self.hand_typ)?;
        write!(f, "   HAND_ID    [Cn] : '{}'\n", self.hand_id)?;
        write!(f, "   CARD_TYP   [Cn] : '{}'\n", self.card_typ)?;
        write!(f, "   CARD_ID    [Cn] : '{}'\n", self.card_id)?;
        write!(f, "   LOAD_TYP   [Cn] : '{}'\n", self.load_typ)?;
        write!(f, "   LOAD_ID    [Cn] : '{}'\n", self.load_id)?;
        write!(f, "   DIB_TYP    [Cn] : '{}'\n", self.dib_typ)?;
        write!(f, "   DIB_ID     [Cn] : '{}'\n", self.dib_id)?;
        write!(f, "   CABL_TYP   [Cn] : '{}'\n", self.cabl_typ)?;
        write!(f, "   CABL_ID    [Cn] : '{}'\n", self.cabl_id)?;
        write!(f, "   CONT_TYP   [Cn] : '{}'\n", self.cont_typ)?;
        write!(f, "   CONT_ID    [Cn] : '{}'\n", self.cont_id)?;
        write!(f, "   LASR_TYP   [Cn] : '{}'\n", self.lasr_typ)?;
        write!(f, "   LASR_ID    [Cn] : '{}'\n", self.lasr_id)?;
        write!(f, "   EXTR_TYP   [Cn] : '{}'\n", self.extr_typ)?;
        write!(f, "   EXTR_ID    [Cn] : '{}'\n", self.extr_id)
    }
} 

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct WIR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub start_t: U4E,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
}

impl WIR<'_> {
    pub fn name() -> String {
        "WIR".to_string()
    }
}

impl <'a> fmt::Display for WIR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WIR : Wafer Information Record\n")?;
        write!(f, "   HEAD_NUM  [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_GRP  [U1] : {}\n", self.site_grp)?;
        write!(f, "   START_T  [U4E] : {}\n", self.start_t)?;
        write!(f, "   WAFER_ID  [Cn] : '{}'\n", self.wafer_id)
    }
}	

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl WRR<'_> {
    pub fn name() -> String {
        "WRR".to_string()
    }
}

impl <'a> fmt::Display for WRR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WRR : Wafer Result Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_GRP [U1] : {}\n", self.site_grp)?;
        write!(f, "   FINISH_T [U4] : {}\n", self.finish_t)?;
        write!(f, "   PART_CNT [U4] : {}\n", self.part_cnt)?;
        write!(f, "   RTST_CNT [U4] : {}\n", self.rtst_cnt)?;
        write!(f, "   ABRT_CNT [U4] : {}\n", self.abrt_cnt)?;
        write!(f, "   GOOD_CNT [U4] : {}\n", self.good_cnt)?;
        write!(f, "   FUNC_CNT [U4] : {}\n", self.func_cnt)?;
        write!(f, "   WAFER_ID [Cn] : '{}'\n", self.wafer_id)?;
        write!(f, "   FABWF_ID [Cn] : '{}'\n", self.fabwf_id)?;
        write!(f, "   FRAME_ID [Cn] : '{}'\n", self.frame_id)?;
        write!(f, "   MASK_ID  [Cn] : '{}'\n", self.mask_id)?;
        write!(f, "   USR_DESC [Cn] : '{}'\n", self.usr_desc)?;
        write!(f, "   EXC_DESC [Cn] : '{}'\n", self.exc_desc)
    }
}

#[derive(Debug, PartialEq, STDFRecord)]
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
    #[default(I2::from(std::i16::MIN))]
    pub center_x: I2,
    #[default(I2::from(std::i16::MIN))]
    pub center_y: I2,
    #[default(C1::from(0x20))]
    pub pos_x: C1,
    #[default(C1::from(0x20))]
    pub pos_y: C1,
}

impl WCR {
    pub fn name() -> String {
        "WCR".to_string()
    }
}

impl fmt::Display for WCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WCR : Wafer Configuration Record\n")?;
        write!(f, "   WAFR_SIZ [R4] : {}\n", self.wafr_siz)?;
        write!(f, "   DIE_HT   [R4] : {}\n", self.die_ht)?;
        write!(f, "   DIE_WID  [R4] : {}\n", self.die_wid)?;
        write!(f, "   WF_UNITS [U1] : {}\n", self.wf_units)?;
        write!(f, "   WF_FLAT  [C1] : {}\n", self.wf_flat)?;
        write!(f, "   CENTER_X [I2] : {}\n", self.center_x)?;
        write!(f, "   CENTER_Y [I2] : {}\n", self.center_y)?;
        write!(f, "   POS_X    [C1] : '{}'\n", self.pos_x)?;
        write!(f, "   POS_Y    [C1] : '{}'\n", self.pos_y)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct PIR {
    pub head_num: U1,
    pub site_num: U1,
}

impl PIR {
    pub fn name() -> String {
        "PIR".to_string()
    }
}

impl fmt::Display for PIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PIR : Part Information Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct PRR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub part_flg: B1,
    pub num_test: U2,
    pub hard_bin: U2,
    #[default(U2::from(0xffff))]
    pub soft_bin: U2,
    #[default(I2::from(std::i16::MIN))]
    pub x_coord: I2,
    #[default(I2::from(std::i16::MIN))]
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

impl PRR<'_> {
    pub fn name() -> String {
        "PRR".to_string()
    }
}

impl <'a> fmt::Display for PRR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PRR : Part Results Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)?;
        write!(f, "   PART_FLG [B1] : {} {}\n", self.part_flg, prr_part_flg(self.part_flg))?;
        write!(f, "   NUM_TEST [U2] : {}\n", self.num_test)?;
        write!(f, "   HARD_BIN [U2] : {}\n", self.hard_bin)?;
        write!(f, "   SOFT_BIN [U2] : {}\n", self.soft_bin)?;
        write!(f, "   X_COORD  [I2] : {}\n", if i16::from(self.x_coord) == -32768_i16 {"".to_string()} else {format!("{}", i16::from(self.x_coord))})?;
        write!(f, "   Y_COORD  [I2] : {}\n", if i16::from(self.y_coord) == -32768_i16 {"".to_string()} else {format!("{}", i16::from(self.y_coord))})?;
        write!(f, "   TEST_T   [U4] : {}\n", self.test_t)?;
        write!(f, "   PART_ID  [Cn] : '{}'\n", self.part_id.to_string().replace("\n", "").replace("\r", ""))?;
        write!(f, "   PART_TXT [Cn] : '{}'\n", self.part_txt.to_string().replace("\n", "").replace("\r", ""))?;
        write!(f, "   PART_FIX [Bn] : {}\n", self.part_fix)
    }
}

fn prr_part_flg(part_flg: B1) -> String {
    let mut msg = String::new();
    let mut info: Vec<String> = Vec::new(); 

    if u8::from(part_flg) & 0b0000_0100 == 0b0000_0100  { info.push(String::from("Abnormal end of testing")); }

    msg.push_str("(");
    msg.push_str(&info.join(", "));
    msg.push_str(") → ");

    if u8::from(part_flg) & 0b0001_0000 == 0b0001_0000  { 
        msg.push_str("?");
    } else {
        if u8::from(part_flg) & 0b0000_1000 == 0b0000_1000 {
            msg.push_str("FAIL");
        } else {
            msg.push_str("PASS");
        }
    }
    msg
}
#[derive(Debug, PartialEq, STDFRecord)]
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
    #[default(R4::from(std::f32::NAN))]
    pub test_tim: R4,
    #[default(R4::from(std::f32::NAN))]
    pub test_min: R4,
    #[default(R4::from(std::f32::NAN))]
    pub test_max: R4,
    #[default(R4::from(std::f32::NAN))]
    pub tst_sums: R4,
    #[default(R4::from(std::f32::NAN))]
    pub tst_sqrs: R4,
}

impl TSR<'_> {
    pub fn name() -> String {
        "TSR".to_string()
    }
}

impl <'a> fmt::Display for TSR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TSR : Test Synopsis Record\n")?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)?;
        write!(f, "   TEST_TYP [C1] : '{}'\n", self.test_typ)?;
        write!(f, "   TEST_NUM [U4] : {}\n", self.test_num)?;
        write!(f, "   EXEC_CNT [U4] : {}\n", self.exec_cnt)?;
        write!(f, "   FAIL_CNT [U4] : {}\n", self.fail_cnt)?;
        write!(f, "   ALRM_CNT [U4] : {}\n", self.alrm_cnt)?;
        write!(f, "   TEST_NAM [Cn] : '{}'\n", self.test_nam)?;
        write!(f, "   SEQ_NAME [Cn] : '{}'\n", self.seq_name)?;
        write!(f, "   TEST_LBL [Cn] : '{}'\n", self.test_lbl)?;
        write!(f, "   OPT_FLAG [B1] : {} (see check marks below)\n", self.opt_flag)?;
        write!(f, "   TEST_TIM [R4] : {} {}\n", self.test_tim, if u8::from(self.opt_flag) & 0b00000100 == 0b00000100 { "✗" } else { "✓" })?;
        write!(f, "   TEST_MIN [R4] : {} {}\n", self.test_min, if u8::from(self.opt_flag) & 0b00000001 == 0b00000001 { "✗" } else { "✓" })?;
        write!(f, "   TEST_MAX [R4] : {} {}\n", self.test_max, if u8::from(self.opt_flag) & 0b00000010 == 0b00000010 { "✗" } else { "✓" })?;
        write!(f, "   TST_SUMS [R4] : {} {}\n", self.tst_sums, if u8::from(self.opt_flag) & 0b00010000 == 0b00010000 { "✗" } else { "✓" })?;
        write!(f, "   TST_SQRS [R4] : {} {}\n", self.tst_sqrs, if u8::from(self.opt_flag) & 0b00100000 == 0b00100000 { "✗" } else { "✓" })
    }
}

#[derive(Debug, PartialEq, STDFRecord)]
pub struct PTR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    #[default(R4::from(std::f32::NAN))]
    pub result: R4,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(B1::from(0x00))]
    pub opt_flag: B1,
    #[default(I1::from(std::i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(std::f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_limit: R4,
    #[default(Cn(b""))]
    pub units: Cn<'a>,
    #[default(Cn(b""))]
    pub c_resfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_llmfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_hlmfmt: Cn<'a>,
    #[default(R4::from(std::f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_spec: R4,
}

impl PTR<'_> {
    pub fn name() -> String {
        "PTR".to_string()
    }
}

impl <'a> fmt::Display for PTR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PTR : Parametric Test Record\n")?;
        write!(f, "   TEST_NUM [U4] : {}\n", self.test_num)?;
        write!(f, "   HEAD_NUM [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM [U1] : {}\n", self.site_num)?;
        write!(f, "   TEST_FLG [B1] : {} {}\n", self.test_flg, ptr_test_flg(self.test_flg))?;
        write!(f, "   PARM_FLG [B1] : {} {}\n", self.parm_flg, ptr_parm_flg(self.parm_flg))?;
        write!(f, "   RESULT   [R4] : {}\n", self.result)?;
        write!(f, "   TEST_TXT [Cn] : '{}'\n", self.test_txt)?;
        if self.opt_flag == B1::from(0x00) {
            write!(f, "   ALARM_ID [Cn] : {}\n", self.alarm_id)
        } else {
            write!(f, "   ALARM_ID [Cn] : {}\n", self.alarm_id)?;
            write!(f, "   --------------\n")?;
            write!(f, "   OPT_FLAG [B1] : {} {}\n", self.opt_flag, ptr_opt_flag(self.opt_flag))?;
            write!(f, "   RES_SCAL [I1] : {}\n", self.res_scal)?;
            write!(f, "   LLM_SCAL [I1] : {}\n", self.llm_scal)?;
            write!(f, "   HLM_SCAL [I1] : {}\n", self.hlm_scal)?;
            write!(f, "   LO_LIMIT [R4] : {}\n", self.lo_limit)?;
            write!(f, "   HI_LIMIT [R4] : {}\n", self.hi_limit)?;
            write!(f, "   UNITS    [Cn] : '{}'\n", self.units)?;
            write!(f, "   C_RESFMT [Cn] : '{}'\n", self.c_resfmt)?;
            write!(f, "   C_LLMFMT [Cn] : '{}'\n", self.c_llmfmt)?;
            write!(f, "   C_HLMFMT [Cn] : '{}'\n", self.c_hlmfmt)?;
            write!(f, "   LO_SPEC  [R4] : {}\n", self.lo_spec)?;
            write!(f, "   HI_SPEC  [R4] : {}\n", self.hi_spec)
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

    msg.push_str("(");
    msg.push_str(&info.join(", "));
    msg.push_str(") → ");

    if u8::from(test_flg) & 0b0100_0000 == 0b0100_0000  { 
        msg.push_str("?");
    } else {
        if u8::from(test_flg) & 0b1000_0000 == 0b1000_0000 {
            msg.push_str("FAIL");
        } else {
            msg.push_str("PASS");
        }
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

    msg.push_str("(");
    msg.push_str(&info.join(", "));
    msg.push_str(")");

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

    msg.push_str("(");
    msg.push_str(&info.join(", "));
    msg.push_str(")");

    msg
}


#[derive(Debug, PartialEq, STDFRecord)]
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
    #[default(I1::from(std::i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(std::f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub start_in: R4,
    #[default(R4::from(std::f32::NAN))]
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
    #[default(R4::from(std::f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_spec: R4,
}

impl MPR<'_> {
    pub fn name() -> String {
        "MPR".to_string()
    }
}

impl <'a> fmt::Display for MPR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MPR : Multiple-Result Parametric Record\n")?;
        write!(f, "   TEST_NUM   [U4] : {}\n", self.test_num)?;
        write!(f, "   HEAD_NUM   [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM   [U1] : {}\n", self.site_num)?;
        write!(f, "   TEST_FLG   [B1] : {}\n", self.test_flg)?;
        write!(f, "   PARM_FLG   [B1] : {}\n", self.parm_flg)?;
        write!(f, "   RTN_ICNT j [U2] : {}\n", self.rtn_icnt)?;
        write!(f, "   RSLT_CNT k [U2] : {}\n", self.rslt_cnt)?;
        write!(f, "   RTN_STAT [jxN1] : {:?}\n", self.rtn_stat)?; //TODO: implement std:fmt::Display for Vec<N1>
        write!(f, "   RTN_RSLT [kxR4] : {:?}\n", self.rtn_rslt)?;  //TODO: implement std:fmt::Display for Vec<R4>
        write!(f, "   TEST_TXT   [Cn] : '{}'\n", self.test_txt)?;
        if self.opt_flag == B1::from(0x00) {
            write!(f, "   ALARM_ID   [Cn] : '{}'\n", self.alarm_id)
        } else {
            write!(f, "   ALARM_ID   [Cn] : '{}'\n", self.alarm_id)?;
            write!(f, "   OPT_FLAG   [B1] : {}\n", self.opt_flag)?;
            write!(f, "   RES_SCAL   [I1] : {}\n", self.res_scal)?;
            write!(f, "   LLM_SCAL   [I1] : {}\n", self.llm_scal)?;
            write!(f, "   HLM_SCAL   [I1] : {}\n", self.hlm_scal)?;
            write!(f, "   LO_LIMIT   [R4] : {}\n", self.lo_limit)?;
            write!(f, "   HI_LIMIT   [R4] : {}\n", self.hi_limit)?;
            write!(f, "   START_IN   [R4] : {}\n", self.start_in)?;
            write!(f, "   INCR_IN    [R4] : {}\n", self.incr_in)?;
            write!(f, "   RTN_INDX [jxU2] : {:?}\n", self.rtn_indx)?; //TODO: implement std:fmt::Display for Vec<U2>
            write!(f, "   UNITS      [Cn] : '{}'\n", self.units)?;
            write!(f, "   UNITS_IN   [Cn] : '{}'\n", self.units_in)?;
            write!(f, "   C_RESFMT   [Cn] : '{}'\n", self.c_resfmt)?;
            write!(f, "   C_LLMFMT   [Cn] : '{}'\n", self.c_llmfmt)?;
            write!(f, "   C_HLMFMT   [Cn] : '{}'\n", self.c_hlmfmt)?;
            write!(f, "   LO_SPEC    [R4] : {}\n", self.lo_spec)?;
            write!(f, "   HI_SPEC    [R4] : {}\n", self.hi_spec) 
        }
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

impl FTR<'_> {
    pub fn name() -> String {
        "FTR".to_string()
    }
}

impl <'a> fmt::Display for FTR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FTR : Functional Test Record\n")?;
        write!(f, "   TEST_NUM   [U4] : {}\n", self.test_num)?;
        write!(f, "   HEAD_NUM   [U1] : {}\n", self.head_num)?;
        write!(f, "   SITE_NUM   [U1] : {}\n", self.site_num)?;
        if self.opt_flag == B1::from(0x00) {
            write!(f, "   TEST_FLG   [B1] : {}\n", self.test_flg)
        } else {
            write!(f, "   TEST_FLG   [B1] : {}\n", self.test_flg)?;
            write!(f, "   OPT_FLAG   [B1] : {}\n", self.opt_flag)?;
            write!(f, "   CYCL_CNT   [U4] : {}\n", self.cycl_cnt)?;
            write!(f, "   REL_VADR   [U4] : {}\n", self.rel_vadr)?;
            write!(f, "   REPT_CNT   [U4] : {}\n", self.rept_cnt)?;
            write!(f, "   NUM_FAIL   [U4] : {}\n", self.num_fail)?;
            write!(f, "   XFAIL_AD   [I4] : {}\n", self.xfail_ad)?;
            write!(f, "   YFAIL_AD   [I4] : {}\n", self.yfail_ad)?;
            write!(f, "   VECT_OFF   [I2] : {}\n", self.vect_off)?;
            write!(f, "   RTN_ICNT j [U2] : {}\n", self.rtn_icnt)?;
            write!(f, "   PGM_ICNT k [U2] : {}\n", self.pgm_icnt)?;
            write!(f, "   RTN_INDX [jxU2] : {:?}\n", self.rtn_indx)?;
            write!(f, "   RTN_STAT [jxN1] : {:?}\n", self.rtn_stat)?;
            write!(f, "   PGM_INDX [kxU2] : {:?}\n", self.pgm_indx)?;
            write!(f, "   PGM_STAT [kxN1] : {:?}\n", self.pgm_stat)?;
            write!(f, "   FAIL_PIN   [Dn] : {}\n", self.fail_pin)?;
            write!(f, "   VECT_NAM   [Cn] : '{}'\n", self.vect_nam)?;
            write!(f, "   TIME_SET   [Cn] : '{}'\n", self.time_set)?;
            write!(f, "   OP_CODE    [Cn] : '{}'\n", self.op_code)?;
            write!(f, "   TEST_TXT   [Cn] : '{}'\n", self.test_txt)?;
            write!(f, "   ALARM_ID   [Cn] : '{}'\n", self.alarm_id)?;
            write!(f, "   PROG_TXT   [Cn] : '{}'\n", self.prog_txt)?;
            write!(f, "   RSLT_TXT   [Cn] : '{}'\n", self.rslt_txt)?;
            write!(f, "   PATG_NUM   [U1] : {}\n", self.patg_num)?;
            write!(f, "   SPIN_MAP   [Dn] : {}\n", self.spin_map)
        }
    }
}


#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct BPS<'a> {
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
}

impl BPS<'_> {
    pub fn name() -> String {
        "BPS".to_string()
    }
}

impl <'a> fmt::Display for BPS<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BPS : Begin Program Section record\n")?;
        write!(f, "   SEQ_NAME [Cn] : '{}'\n", self.seq_name)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct EPS;

impl EPS {
    pub fn name() -> String {
        "EPS".to_string()
    }
}

impl fmt::Display for EPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EPS : End Program Section record\n")
    }
}

#[derive(Debug, PartialEq, STDFRecord)]
pub struct GDR<'a> {
    #[default(U2::from(0))]
    pub fld_cnt: U2,
    #[array_length(fld_cnt)]
    #[array_type(Vn<'a>)]
    pub gen_data: Vec<Vn<'a>>,
}

impl GDR<'_> {
    pub fn name() -> String {
        "GDR".to_string()
    }
}

impl <'a> fmt::Display for GDR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GDR\n")?;
        write!(f, "   FLD_CNT  [U2]: {}\n", self.fld_cnt)?;
        write!(f, "   GEN_DATA [Vn]: {:?}\n", self.gen_data)
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct DTR<'a> {
    #[default(Cn(b""))]
    pub text_dat: Cn<'a>,
}

impl DTR<'_> {
    pub fn name() -> String {
        "DTR".to_string()
    }
}

impl <'a> fmt::Display for DTR<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DTR : Datalog Text Record\n")?;
        write!(f, "   TEXT_DAT [Cn] : '{}'\n", self.text_dat)
    }
}

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
            V4::FAR(_) => FAR::name(),
            V4::ATR(_) => ATR::name(),
            V4::MIR(_) => MIR::name(),
            V4::MRR(_) => MRR::name(),
            V4::PCR(_) => PCR::name(),
            V4::HBR(_) => HBR::name(),
            V4::SBR(_) => SBR::name(),
            V4::PMR(_) => PMR::name(),
            V4::PGR(_) => PGR::name(),
            V4::PLR(_) => PLR::name(),
            V4::RDR(_) => RDR::name(),
            V4::SDR(_) => SDR::name(),
            V4::WIR(_) => WIR::name(),
            V4::WRR(_) => WRR::name(),
            V4::WCR(_) => WCR::name(),
            V4::PIR(_) => PIR::name(),
            V4::PRR(_) => PRR::name(),
            V4::TSR(_) => TSR::name(),
            V4::PTR(_) => PTR::name(),
            V4::MPR(_) => MPR::name(),
            V4::FTR(_) => FTR::name(),
            V4::BPS(_) => BPS::name(),
            V4::EPS(_) => EPS::name(),
            V4::GDR(_) => GDR::name(),
            V4::DTR(_) => DTR::name(),
            V4::Unknown(_) => "???".to_string(),
            V4::Invalid(_) => "???".to_string()
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

impl<'a> TryWrite<ctx::Endian> for V4<'a> {
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

impl<'a> V4<'a> {
    fn rec_typ_sub(&self) -> (u8, u8) {
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

//TODO: Implement std::fmt::Display for V4
// impl <'a> fmt::Display for V4<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             V4::MRR(rec) => write!(f, "{}", rec),
//             _ => todo!(),
//         }
//     }
// }

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
    match typ_sub {
        (0, 10) => true,
        (0, 20) => true,
        (1, 10) => true,
        (1, 20) => true,
        (1, 30) => true,
        (1, 40) => true,
        (1, 50) => true,
        (1, 60) => true,
        (1, 62) => true,
        (1, 63) => true,
        (1, 70) => true,
        (1, 80) => true,
        (2, 10) => true,
        (2, 20) => true,
        (2, 30) => true,
        (5, 10) => true,
        (5, 20) => true,
        (10, 30) => true,
        (15, 10) => true,
        (15, 15) => true,
        (15, 20) => true,
        (20, 10) => true,
        (20, 20) => true,
        (50, 10) => true,
        (50, 30) => true,
        _ => false,
    }
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
/// assert_eq!(unknown_name, "Unknown");
/// ```
pub fn typ_sub_to_name(typ: u8, sub: u8) -> String {
    match (typ, sub) {
        (0, 10) => String::from("FAR"),
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



pub fn is_test(typ:u8) -> bool {
    if typ == 15 {
        true
    } else {
        false
    }
}

pub fn test_needs_columns(rec: V4) -> u16 {

}

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
