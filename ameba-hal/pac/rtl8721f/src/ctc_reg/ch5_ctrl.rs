#[doc = "Register `CH5_CTRL` reader"]
pub type R = crate::R<Ch5CtrlSpec>;
#[doc = "Register `CH5_CTRL` writer"]
pub type W = crate::W<Ch5CtrlSpec>;
#[doc = "Field `ch5_en` reader - Cap sensor activity control of channel 5 0: Disable 1: Enable"]
pub type Ch5EnR = crate::BitReader;
#[doc = "Field `ch5_en` writer - Cap sensor activity control of channel 5 0: Disable 1: Enable"]
pub type Ch5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch5_baseline` reader - Digital baseline data of channel 5. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch5BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch5_baseline` writer - Digital baseline data of channel 5. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch5BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch5_d_touch_th` reader - Difference threshold data of touch judgement for channel 5. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch5DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch5_d_touch_th` writer - Difference threshold data of touch judgement for channel 5. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch5DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 5 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch5_en(&self) -> Ch5EnR {
        Ch5EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 5. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch5_baseline(&self) -> Ch5BaselineR {
        Ch5BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 5. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch5_d_touch_th(&self) -> Ch5DTouchThR {
        Ch5DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 5 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch5_en(&mut self) -> Ch5EnW<'_, Ch5CtrlSpec> {
        Ch5EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 5. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch5_baseline(&mut self) -> Ch5BaselineW<'_, Ch5CtrlSpec> {
        Ch5BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 5. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch5_d_touch_th(&mut self) -> Ch5DTouchThW<'_, Ch5CtrlSpec> {
        Ch5DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 5 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5CtrlSpec;
impl crate::RegisterSpec for Ch5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch5CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch5CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
