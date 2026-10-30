#[doc = "Register `REG_UVC_CHNL0_CTRL` reader"]
pub type R = crate::R<RegUvcChnl0CtrlSpec>;
#[doc = "Register `REG_UVC_CHNL0_CTRL` writer"]
pub type W = crate::W<RegUvcChnl0CtrlSpec>;
#[doc = "Field `ch0_dma_addr` reader - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
pub type Ch0DmaAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch0_dma_addr` writer - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
pub type Ch0DmaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
    #[inline(always)]
    pub fn ch0_dma_addr(&self) -> Ch0DmaAddrR {
        Ch0DmaAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For checking ahb xfer with header, this value should be the same as HCDMAi in otg_core"]
    #[inline(always)]
    pub fn ch0_dma_addr(&mut self) -> Ch0DmaAddrW<'_, RegUvcChnl0CtrlSpec> {
        Ch0DmaAddrW::new(self, 0)
    }
}
#[doc = "Channel_0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_uvc_chnl0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_uvc_chnl0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUvcChnl0CtrlSpec;
impl crate::RegisterSpec for RegUvcChnl0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_uvc_chnl0_ctrl::R`](R) reader structure"]
impl crate::Readable for RegUvcChnl0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_uvc_chnl0_ctrl::W`](W) writer structure"]
impl crate::Writable for RegUvcChnl0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UVC_CHNL0_CTRL to value 0"]
impl crate::Resettable for RegUvcChnl0CtrlSpec {}
