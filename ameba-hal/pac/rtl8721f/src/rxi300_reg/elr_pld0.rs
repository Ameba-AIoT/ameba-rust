#[doc = "Register `ELR_PLD0` reader"]
pub type R = crate::R<ElrPld0Spec>;
#[doc = "Field `ELR_SRC` reader - Error source of the error request. 0x90: timeout default slave of S6 group"]
pub type ElrSrcR = crate::FieldReader;
#[doc = "Field `ELR_CMD` reader - CMD of the error request. 0x1: Write command 0x2: Read command"]
pub type ElrCmdR = crate::FieldReader;
#[doc = "Field `ELR_BSTTYPE` reader - BurstSeq of the error request."]
pub type ElrBsttypeR = crate::FieldReader;
#[doc = "Field `ELR_BSTLEN` reader - Burst Length of the error request."]
pub type ElrBstlenR = crate::FieldReader;
#[doc = "Field `ELR_BSTINDEX` reader - Burst index of the error request."]
pub type ElrBstindexR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Error source of the error request. 0x90: timeout default slave of S6 group"]
    #[inline(always)]
    pub fn elr_src(&self) -> ElrSrcR {
        ElrSrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - CMD of the error request. 0x1: Write command 0x2: Read command"]
    #[inline(always)]
    pub fn elr_cmd(&self) -> ElrCmdR {
        ElrCmdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - BurstSeq of the error request."]
    #[inline(always)]
    pub fn elr_bsttype(&self) -> ElrBsttypeR {
        ElrBsttypeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Burst Length of the error request."]
    #[inline(always)]
    pub fn elr_bstlen(&self) -> ElrBstlenR {
        ElrBstlenR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Burst index of the error request."]
    #[inline(always)]
    pub fn elr_bstindex(&self) -> ElrBstindexR {
        ElrBstindexR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Error log payload register0\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_pld0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrPld0Spec;
impl crate::RegisterSpec for ElrPld0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_pld0::R`](R) reader structure"]
impl crate::Readable for ElrPld0Spec {}
#[doc = "`reset()` method sets ELR_PLD0 to value 0"]
impl crate::Resettable for ElrPld0Spec {}
