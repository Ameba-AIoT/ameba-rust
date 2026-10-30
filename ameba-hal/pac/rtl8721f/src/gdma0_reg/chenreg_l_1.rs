#[doc = "Register `CHENREG_L_1` reader"]
pub type R = crate::R<ChenregL1Spec>;
#[doc = "Register `CHENREG_L_1` writer"]
pub type W = crate::W<ChenregL1Spec>;
#[doc = "Field `CHENREG_L_1_CH_EN` reader - Enables/Disables the channel. Setting this bit enables a channel; clearing this bit disables the channel. 0 = Disable the Channel 1 = Enable the Channel The CHENREG_L.CH_EN bit is automatically cleared by hardware to disable the channel after the last AXI transfer of the DMA transfer to the destination has completed. Software can therefore poll this bit to determine when this channel is free for a new DMA transfer."]
pub type ChenregL1ChEnR = crate::FieldReader;
#[doc = "Field `CHENREG_L_1_CH_EN` writer - Enables/Disables the channel. Setting this bit enables a channel; clearing this bit disables the channel. 0 = Disable the Channel 1 = Enable the Channel The CHENREG_L.CH_EN bit is automatically cleared by hardware to disable the channel after the last AXI transfer of the DMA transfer to the destination has completed. Software can therefore poll this bit to determine when this channel is free for a new DMA transfer."]
pub type ChenregL1ChEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHENREG_L_1_CH_EN_WE` writer - Channel enable write enable."]
pub type ChenregL1ChEnWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enables/Disables the channel. Setting this bit enables a channel; clearing this bit disables the channel. 0 = Disable the Channel 1 = Enable the Channel The CHENREG_L.CH_EN bit is automatically cleared by hardware to disable the channel after the last AXI transfer of the DMA transfer to the destination has completed. Software can therefore poll this bit to determine when this channel is free for a new DMA transfer."]
    #[inline(always)]
    pub fn chenreg_l_1_ch_en(&self) -> ChenregL1ChEnR {
        ChenregL1ChEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables/Disables the channel. Setting this bit enables a channel; clearing this bit disables the channel. 0 = Disable the Channel 1 = Enable the Channel The CHENREG_L.CH_EN bit is automatically cleared by hardware to disable the channel after the last AXI transfer of the DMA transfer to the destination has completed. Software can therefore poll this bit to determine when this channel is free for a new DMA transfer."]
    #[inline(always)]
    pub fn chenreg_l_1_ch_en(&mut self) -> ChenregL1ChEnW<'_, ChenregL1Spec> {
        ChenregL1ChEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Channel enable write enable."]
    #[inline(always)]
    pub fn chenreg_l_1_ch_en_we(&mut self) -> ChenregL1ChEnWeW<'_, ChenregL1Spec> {
        ChenregL1ChEnWeW::new(self, 8)
    }
}
#[doc = "DMA low-32 bits Channel Enable Register if configed chn number is less than or equal to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg_l_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg_l_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenregL1Spec;
impl crate::RegisterSpec for ChenregL1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenreg_l_1::R`](R) reader structure"]
impl crate::Readable for ChenregL1Spec {}
#[doc = "`write(|w| ..)` method takes [`chenreg_l_1::W`](W) writer structure"]
impl crate::Writable for ChenregL1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHENREG_L_1 to value 0"]
impl crate::Resettable for ChenregL1Spec {}
