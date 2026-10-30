#[doc = "Register `REG_TSFT_BCN_CCAUP_H` reader"]
pub type R = crate::R<RegTsftBcnCcaupHSpec>;
#[doc = "Field `BIT_TSFT_AT_BCN_CCA_UP_H` reader - when CCA asserted, latch the tsft\\[63:32\\] (select port by 0x1520\\[0\\]). updated to this register after judging that it is a beacon frame"]
pub type BitTsftAtBcnCcaUpHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - when CCA asserted, latch the tsft\\[63:32\\] (select port by 0x1520\\[0\\]). updated to this register after judging that it is a beacon frame"]
    #[inline(always)]
    pub fn bit_tsft_at_bcn_cca_up_h(&self) -> BitTsftAtBcnCcaUpHR {
        BitTsftAtBcnCcaUpHR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsft_bcn_ccaup_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsftBcnCcaupHSpec;
impl crate::RegisterSpec for RegTsftBcnCcaupHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsft_bcn_ccaup_h::R`](R) reader structure"]
impl crate::Readable for RegTsftBcnCcaupHSpec {}
#[doc = "`reset()` method sets REG_TSFT_BCN_CCAUP_H to value 0"]
impl crate::Resettable for RegTsftBcnCcaupHSpec {}
