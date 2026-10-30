#[doc = "Register `STATUSBLOCK_L` reader"]
pub type R = crate::R<StatusblockLSpec>;
#[doc = "Field `STATUSBLOCK_L_Status_Block` reader - Interrupt status for chn0-chn15"]
pub type StatusblockLStatusBlockR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status for chn0-chn15"]
    #[inline(always)]
    pub fn statusblock_l_status_block(&self) -> StatusblockLStatusBlockR {
        StatusblockLStatusBlockR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Status for IntBlock Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`statusblock_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusblockLSpec;
impl crate::RegisterSpec for StatusblockLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusblock_l::R`](R) reader structure"]
impl crate::Readable for StatusblockLSpec {}
#[doc = "`reset()` method sets STATUSBLOCK_L to value 0"]
impl crate::Resettable for StatusblockLSpec {}
