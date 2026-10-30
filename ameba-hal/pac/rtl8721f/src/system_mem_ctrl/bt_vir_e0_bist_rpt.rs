#[doc = "Register `BT_VIR_E0_BIST_RPT` reader"]
pub type R = crate::R<BtVirE0BistRptSpec>;
#[doc = "Field `bt_vir_e0_bist_rpt_dout` reader - "]
pub type BtVirE0BistRptDoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bt_vir_e0_bist_rpt_dout(&self) -> BtVirE0BistRptDoutR {
        BtVirE0BistRptDoutR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_vir_e0_bist_rpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtVirE0BistRptSpec;
impl crate::RegisterSpec for BtVirE0BistRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_vir_e0_bist_rpt::R`](R) reader structure"]
impl crate::Readable for BtVirE0BistRptSpec {}
#[doc = "`reset()` method sets BT_VIR_E0_BIST_RPT to value 0"]
impl crate::Resettable for BtVirE0BistRptSpec {}
