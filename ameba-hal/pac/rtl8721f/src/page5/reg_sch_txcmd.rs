#[doc = "Register `REG_SCH_TXCMD` reader"]
pub type R = crate::R<RegSchTxcmdSpec>;
#[doc = "Field `BIT_SCH_TXCMD` reader - Scheduler TXCMD Content"]
pub type BitSchTxcmdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scheduler TXCMD Content"]
    #[inline(always)]
    pub fn bit_sch_txcmd(&self) -> BitSchTxcmdR {
        BitSchTxcmdR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sch_txcmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSchTxcmdSpec;
impl crate::RegisterSpec for RegSchTxcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sch_txcmd::R`](R) reader structure"]
impl crate::Readable for RegSchTxcmdSpec {}
#[doc = "`reset()` method sets REG_SCH_TXCMD to value 0"]
impl crate::Resettable for RegSchTxcmdSpec {}
