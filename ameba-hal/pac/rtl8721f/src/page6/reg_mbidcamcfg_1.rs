#[doc = "Register `REG_MBIDCAMCFG_1` reader"]
pub type R = crate::R<RegMbidcamcfg1Spec>;
#[doc = "Register `REG_MBIDCAMCFG_1` writer"]
pub type W = crate::W<RegMbidcamcfg1Spec>;
#[doc = "Field `BIT_MBIDCAM_WDATA_L` reader - BSSID\\[31:0\\]"]
pub type BitMbidcamWdataLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MBIDCAM_WDATA_L` writer - BSSID\\[31:0\\]"]
pub type BitMbidcamWdataLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BSSID\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_mbidcam_wdata_l(&self) -> BitMbidcamWdataLR {
        BitMbidcamWdataLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BSSID\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_mbidcam_wdata_l(&mut self) -> BitMbidcamWdataLW<'_, RegMbidcamcfg1Spec> {
        BitMbidcamWdataLW::new(self, 0)
    }
}
#[doc = "MBSSID CAM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcamcfg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcamcfg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcamcfg1Spec;
impl crate::RegisterSpec for RegMbidcamcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcamcfg_1::R`](R) reader structure"]
impl crate::Readable for RegMbidcamcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcamcfg_1::W`](W) writer structure"]
impl crate::Writable for RegMbidcamcfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAMCFG_1 to value 0"]
impl crate::Resettable for RegMbidcamcfg1Spec {}
