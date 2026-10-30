#[doc = "Register `REG_TSFT_HIOE_RFON_H` reader"]
pub type R = crate::R<RegTsftHioeRfonHSpec>;
#[doc = "Field `BIT_TSFT_AT_HIOE_RFON_H` reader - when hioe_int_chn_on asserted, latch the tsft\\[63:32\\] (select port by 0x1520\\[0\\])"]
pub type BitTsftAtHioeRfonHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - when hioe_int_chn_on asserted, latch the tsft\\[63:32\\] (select port by 0x1520\\[0\\])"]
    #[inline(always)]
    pub fn bit_tsft_at_hioe_rfon_h(&self) -> BitTsftAtHioeRfonHR {
        BitTsftAtHioeRfonHR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsft_hioe_rfon_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsftHioeRfonHSpec;
impl crate::RegisterSpec for RegTsftHioeRfonHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsft_hioe_rfon_h::R`](R) reader structure"]
impl crate::Readable for RegTsftHioeRfonHSpec {}
#[doc = "`reset()` method sets REG_TSFT_HIOE_RFON_H to value 0"]
impl crate::Resettable for RegTsftHioeRfonHSpec {}
