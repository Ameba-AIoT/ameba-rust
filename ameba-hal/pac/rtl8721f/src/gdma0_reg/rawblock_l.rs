#[doc = "Register `RAWBLOCK_L` reader"]
pub type R = crate::R<RawblockLSpec>;
#[doc = "Field `RAWBLOCK_L_Raw_Block` reader - Raw interrupt status for chn0-chn15"]
pub type RawblockLRawBlockR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for chn0-chn15"]
    #[inline(always)]
    pub fn rawblock_l_raw_block(&self) -> RawblockLRawBlockR {
        RawblockLRawBlockR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Raw Status for IntBlock Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`rawblock_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawblockLSpec;
impl crate::RegisterSpec for RawblockLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawblock_l::R`](R) reader structure"]
impl crate::Readable for RawblockLSpec {}
#[doc = "`reset()` method sets RAWBLOCK_L to value 0"]
impl crate::Resettable for RawblockLSpec {}
