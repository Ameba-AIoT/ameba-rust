#[doc = "Register `CH4_CTRL` reader"]
pub type R = crate::R<Ch4CtrlSpec>;
#[doc = "Register `CH4_CTRL` writer"]
pub type W = crate::W<Ch4CtrlSpec>;
#[doc = "Field `ch4_en` reader - Cap sensor activity control of channel 4 0: Disable 1: Enable"]
pub type Ch4EnR = crate::BitReader;
#[doc = "Field `ch4_en` writer - Cap sensor activity control of channel 4 0: Disable 1: Enable"]
pub type Ch4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch4_baseline` reader - Digital baseline data of channel 4. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch4BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch4_baseline` writer - Digital baseline data of channel 4. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch4BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch4_d_touch_th` reader - Difference threshold data of touch judgement for channel 4. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch4DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch4_d_touch_th` writer - Difference threshold data of touch judgement for channel 4. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch4DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 4 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch4_en(&self) -> Ch4EnR {
        Ch4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 4. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch4_baseline(&self) -> Ch4BaselineR {
        Ch4BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 4. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch4_d_touch_th(&self) -> Ch4DTouchThR {
        Ch4DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 4 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch4_en(&mut self) -> Ch4EnW<'_, Ch4CtrlSpec> {
        Ch4EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 4. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch4_baseline(&mut self) -> Ch4BaselineW<'_, Ch4CtrlSpec> {
        Ch4BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 4. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch4_d_touch_th(&mut self) -> Ch4DTouchThW<'_, Ch4CtrlSpec> {
        Ch4DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 4 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4CtrlSpec;
impl crate::RegisterSpec for Ch4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch4CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
