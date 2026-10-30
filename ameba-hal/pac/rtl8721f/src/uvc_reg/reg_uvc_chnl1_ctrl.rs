#[doc = "Register `REG_UVC_CHNL1_CTRL` reader"]
pub type R = crate::R<RegUvcChnl1CtrlSpec>;
#[doc = "Register `REG_UVC_CHNL1_CTRL` writer"]
pub type W = crate::W<RegUvcChnl1CtrlSpec>;
#[doc = "Field `ch1_dma_addr` reader - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
pub type Ch1DmaAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch1_dma_addr` writer - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
pub type Ch1DmaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
    #[inline(always)]
    pub fn ch1_dma_addr(&self) -> Ch1DmaAddrR {
        Ch1DmaAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
    #[inline(always)]
    pub fn ch1_dma_addr(&mut self) -> Ch1DmaAddrW<'_, RegUvcChnl1CtrlSpec> {
        Ch1DmaAddrW::new(self, 0)
    }
}
#[doc = "Channel_1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_uvc_chnl1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_uvc_chnl1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUvcChnl1CtrlSpec;
impl crate::RegisterSpec for RegUvcChnl1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_uvc_chnl1_ctrl::R`](R) reader structure"]
impl crate::Readable for RegUvcChnl1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_uvc_chnl1_ctrl::W`](W) writer structure"]
impl crate::Writable for RegUvcChnl1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UVC_CHNL1_CTRL to value 0"]
impl crate::Resettable for RegUvcChnl1CtrlSpec {}
