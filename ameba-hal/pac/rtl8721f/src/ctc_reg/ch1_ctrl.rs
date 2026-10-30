#[doc = "Register `CH1_CTRL` reader"]
pub type R = crate::R<Ch1CtrlSpec>;
#[doc = "Register `CH1_CTRL` writer"]
pub type W = crate::W<Ch1CtrlSpec>;
#[doc = "Field `ch1_en` reader - Cap sensor activity control of channel 1 0: Disable 1: Enable"]
pub type Ch1EnR = crate::BitReader;
#[doc = "Field `ch1_en` writer - Cap sensor activity control of channel 1 0: Disable 1: Enable"]
pub type Ch1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_baseline` reader - Digital baseline data of channel 1. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch1BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch1_baseline` writer - Digital baseline data of channel 1. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch1BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch1_d_touch_th` reader - Difference threshold data of touch judgement for channel 1. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch1DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch1_d_touch_th` writer - Difference threshold data of touch judgement for channel 1. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch1DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 1 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch1_en(&self) -> Ch1EnR {
        Ch1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 1. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch1_baseline(&self) -> Ch1BaselineR {
        Ch1BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 1. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch1_d_touch_th(&self) -> Ch1DTouchThR {
        Ch1DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 1 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch1_en(&mut self) -> Ch1EnW<'_, Ch1CtrlSpec> {
        Ch1EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 1. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch1_baseline(&mut self) -> Ch1BaselineW<'_, Ch1CtrlSpec> {
        Ch1BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 1. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch1_d_touch_th(&mut self) -> Ch1DTouchThW<'_, Ch1CtrlSpec> {
        Ch1DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1CtrlSpec;
impl crate::RegisterSpec for Ch1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch1CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
