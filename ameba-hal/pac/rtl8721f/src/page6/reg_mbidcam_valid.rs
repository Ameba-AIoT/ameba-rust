#[doc = "Register `REG_MBIDCAM_VALID` reader"]
pub type R = crate::R<RegMbidcamValidSpec>;
#[doc = "Field `refugmbidcam_valid` reader - corresponding to 32 entries, 1 means the entry is valid. For refugee cam or mbidcam"]
pub type RefugmbidcamValidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - corresponding to 32 entries, 1 means the entry is valid. For refugee cam or mbidcam"]
    #[inline(always)]
    pub fn refugmbidcam_valid(&self) -> RefugmbidcamValidR {
        RefugmbidcamValidR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcamValidSpec;
impl crate::RegisterSpec for RegMbidcamValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam_valid::R`](R) reader structure"]
impl crate::Readable for RegMbidcamValidSpec {}
#[doc = "`reset()` method sets REG_MBIDCAM_VALID to value 0"]
impl crate::Resettable for RegMbidcamValidSpec {}
