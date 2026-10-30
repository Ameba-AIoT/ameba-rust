#[doc = "Register `REG_WKFMCAM_WD` reader"]
pub type R = crate::R<RegWkfmcamWdSpec>;
#[doc = "Register `REG_WKFMCAM_WD` writer"]
pub type W = crate::W<RegWkfmcamWdSpec>;
#[doc = "Field `BIT_WKFMCAM_WD` reader - CAM Write Data"]
pub type BitWkfmcamWdR = crate::FieldReader<u32>;
#[doc = "Field `BIT_WKFMCAM_WD` writer - CAM Write Data"]
pub type BitWkfmcamWdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CAM Write Data"]
    #[inline(always)]
    pub fn bit_wkfmcam_wd(&self) -> BitWkfmcamWdR {
        BitWkfmcamWdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CAM Write Data"]
    #[inline(always)]
    pub fn bit_wkfmcam_wd(&mut self) -> BitWkfmcamWdW<'_, RegWkfmcamWdSpec> {
        BitWkfmcamWdW::new(self, 0)
    }
}
#[doc = "Wakeup Frame CAM CONTENT Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wkfmcam_wd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wkfmcam_wd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWkfmcamWdSpec;
impl crate::RegisterSpec for RegWkfmcamWdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wkfmcam_wd::R`](R) reader structure"]
impl crate::Readable for RegWkfmcamWdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wkfmcam_wd::W`](W) writer structure"]
impl crate::Writable for RegWkfmcamWdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WKFMCAM_WD to value 0"]
impl crate::Resettable for RegWkfmcamWdSpec {}
