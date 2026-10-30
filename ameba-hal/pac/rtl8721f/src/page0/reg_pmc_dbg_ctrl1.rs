#[doc = "Register `REG_PMC_DBG_CTRL1` reader"]
pub type R = crate::R<RegPmcDbgCtrl1Spec>;
#[doc = "Field `BIT_WLPMC_ERRINT` reader - WLPMC Error interrupt(tie 0)"]
pub type BitWlpmcErrintR = crate::FieldReader;
#[doc = "Field `BIT_PMC_WR_OVF` reader - PMC Write Access Timer Overflow(no use)"]
pub type BitPmcWrOvfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - WLPMC Error interrupt(tie 0)"]
    #[inline(always)]
    pub fn bit_wlpmc_errint(&self) -> BitWlpmcErrintR {
        BitWlpmcErrintR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PMC Write Access Timer Overflow(no use)"]
    #[inline(always)]
    pub fn bit_pmc_wr_ovf(&self) -> BitPmcWrOvfR {
        BitPmcWrOvfR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_dbg_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcDbgCtrl1Spec;
impl crate::RegisterSpec for RegPmcDbgCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_dbg_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegPmcDbgCtrl1Spec {}
#[doc = "`reset()` method sets REG_PMC_DBG_CTRL1 to value 0"]
impl crate::Resettable for RegPmcDbgCtrl1Spec {}
