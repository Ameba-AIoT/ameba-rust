#[doc = "Register `REG_TSFT_HIOE_RFON_L` reader"]
pub type R = crate::R<RegTsftHioeRfonLSpec>;
#[doc = "Field `BIT_TSFT_AT_HIOE_RFON_L` reader - when hioe_int_chn_on asserted, latch the tsft\\[31:0\\] (select port by 0x1520\\[0\\])"]
pub type BitTsftAtHioeRfonLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - when hioe_int_chn_on asserted, latch the tsft\\[31:0\\] (select port by 0x1520\\[0\\])"]
    #[inline(always)]
    pub fn bit_tsft_at_hioe_rfon_l(&self) -> BitTsftAtHioeRfonLR {
        BitTsftAtHioeRfonLR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsft_hioe_rfon_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsftHioeRfonLSpec;
impl crate::RegisterSpec for RegTsftHioeRfonLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsft_hioe_rfon_l::R`](R) reader structure"]
impl crate::Readable for RegTsftHioeRfonLSpec {}
#[doc = "`reset()` method sets REG_TSFT_HIOE_RFON_L to value 0"]
impl crate::Resettable for RegTsftHioeRfonLSpec {}
