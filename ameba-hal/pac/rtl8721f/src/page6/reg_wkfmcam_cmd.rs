#[doc = "Register `REG_WKFMCAM_CMD` reader"]
pub type R = crate::R<RegWkfmcamCmdSpec>;
#[doc = "Register `REG_WKFMCAM_CMD` writer"]
pub type W = crate::W<RegWkfmcamCmdSpec>;
#[doc = "Field `BIT_CAM_NUM` reader - CAM Entry Number"]
pub type BitCamNumR = crate::FieldReader;
#[doc = "Field `BIT_CAM_NUM` writer - CAM Entry Number"]
pub type BitCamNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WKFMCAM_AD` reader - Entry address"]
pub type BitWkfmcamAdR = crate::FieldReader;
#[doc = "Field `BIT_WKFMCAM_AD` writer - Entry address"]
pub type BitWkfmcamAdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WKFMCAM_WE` reader - CAM Write Enable"]
pub type BitWkfmcamWeR = crate::BitReader;
#[doc = "Field `BIT_WKFMCAM_WE` writer - CAM Write Enable"]
pub type BitWkfmcamWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - CAM Entry Number"]
    #[inline(always)]
    pub fn bit_cam_num(&self) -> BitCamNumR {
        BitCamNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Entry address"]
    #[inline(always)]
    pub fn bit_wkfmcam_ad(&self) -> BitWkfmcamAdR {
        BitWkfmcamAdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - CAM Write Enable"]
    #[inline(always)]
    pub fn bit_wkfmcam_we(&self) -> BitWkfmcamWeR {
        BitWkfmcamWeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAM Entry Number"]
    #[inline(always)]
    pub fn bit_cam_num(&mut self) -> BitCamNumW<'_, RegWkfmcamCmdSpec> {
        BitCamNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Entry address"]
    #[inline(always)]
    pub fn bit_wkfmcam_ad(&mut self) -> BitWkfmcamAdW<'_, RegWkfmcamCmdSpec> {
        BitWkfmcamAdW::new(self, 8)
    }
    #[doc = "Bit 16 - CAM Write Enable"]
    #[inline(always)]
    pub fn bit_wkfmcam_we(&mut self) -> BitWkfmcamWeW<'_, RegWkfmcamCmdSpec> {
        BitWkfmcamWeW::new(self, 16)
    }
}
#[doc = "Wakeup Frame CAM Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wkfmcam_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wkfmcam_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWkfmcamCmdSpec;
impl crate::RegisterSpec for RegWkfmcamCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wkfmcam_cmd::R`](R) reader structure"]
impl crate::Readable for RegWkfmcamCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wkfmcam_cmd::W`](W) writer structure"]
impl crate::Writable for RegWkfmcamCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WKFMCAM_CMD to value 0"]
impl crate::Resettable for RegWkfmcamCmdSpec {}
