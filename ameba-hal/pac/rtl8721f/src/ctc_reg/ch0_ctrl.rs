#[doc = "Register `CH0_CTRL` reader"]
pub type R = crate::R<Ch0CtrlSpec>;
#[doc = "Register `CH0_CTRL` writer"]
pub type W = crate::W<Ch0CtrlSpec>;
#[doc = "Field `ch0_en` reader - Cap sensor activity control of channel 0 0: Disable 1: Enable"]
pub type Ch0EnR = crate::BitReader;
#[doc = "Field `ch0_en` writer - Cap sensor activity control of channel 0 0: Disable 1: Enable"]
pub type Ch0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_baseline` reader - Digital baseline data of channel 0. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch0BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch0_baseline` writer - Digital baseline data of channel 0. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch0BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_d_touch_th` reader - Difference threshold data of touch judgement for channel 0. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch0DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch0_d_touch_th` writer - Difference threshold data of touch judgement for channel 0. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch0DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 0 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch0_en(&self) -> Ch0EnR {
        Ch0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 0. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch0_baseline(&self) -> Ch0BaselineR {
        Ch0BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 0. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch0_d_touch_th(&self) -> Ch0DTouchThR {
        Ch0DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 0 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch0_en(&mut self) -> Ch0EnW<'_, Ch0CtrlSpec> {
        Ch0EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 0. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch0_baseline(&mut self) -> Ch0BaselineW<'_, Ch0CtrlSpec> {
        Ch0BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 0. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch0_d_touch_th(&mut self) -> Ch0DTouchThW<'_, Ch0CtrlSpec> {
        Ch0DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0CtrlSpec;
impl crate::RegisterSpec for Ch0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch0CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
