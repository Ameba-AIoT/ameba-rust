#[doc = "Register `REG_CHIPEN_ISR` reader"]
pub type R = crate::R<RegChipenIsrSpec>;
#[doc = "Field `CHIPEN_SP_ISR` reader - chipen short press interrupt status 0:no interrupt 1:interrupt occurs"]
pub type ChipenSpIsrR = crate::BitReader;
#[doc = "Field `CHIPEN_LP_ISR` reader - chipen long press interrupt status 0:no interrupt 1:interrupt occurs"]
pub type ChipenLpIsrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - chipen short press interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn chipen_sp_isr(&self) -> ChipenSpIsrR {
        ChipenSpIsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - chipen long press interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn chipen_lp_isr(&self) -> ChipenLpIsrR {
        ChipenLpIsrR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chipen_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChipenIsrSpec;
impl crate::RegisterSpec for RegChipenIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chipen_isr::R`](R) reader structure"]
impl crate::Readable for RegChipenIsrSpec {}
#[doc = "`reset()` method sets REG_CHIPEN_ISR to value 0"]
impl crate::Resettable for RegChipenIsrSpec {}
