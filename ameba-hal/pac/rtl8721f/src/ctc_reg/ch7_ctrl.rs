#[doc = "Register `CH7_CTRL` reader"]
pub type R = crate::R<Ch7CtrlSpec>;
#[doc = "Register `CH7_CTRL` writer"]
pub type W = crate::W<Ch7CtrlSpec>;
#[doc = "Field `ch7_en` reader - Cap sensor activity control of channel 7 0: Disable 1: Enable"]
pub type Ch7EnR = crate::BitReader;
#[doc = "Field `ch7_en` writer - Cap sensor activity control of channel 7 0: Disable 1: Enable"]
pub type Ch7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch7_baseline` reader - Digital baseline data of channel 7. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch7BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch7_baseline` writer - Digital baseline data of channel 7. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch7BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch7_d_touch_th` reader - Difference threshold data of touch judgement for channel 7. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch7DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch7_d_touch_th` writer - Difference threshold data of touch judgement for channel 7. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch7DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 7 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch7_en(&self) -> Ch7EnR {
        Ch7EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 7. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch7_baseline(&self) -> Ch7BaselineR {
        Ch7BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 7. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch7_d_touch_th(&self) -> Ch7DTouchThR {
        Ch7DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 7 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch7_en(&mut self) -> Ch7EnW<'_, Ch7CtrlSpec> {
        Ch7EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 7. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch7_baseline(&mut self) -> Ch7BaselineW<'_, Ch7CtrlSpec> {
        Ch7BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 7. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch7_d_touch_th(&mut self) -> Ch7DTouchThW<'_, Ch7CtrlSpec> {
        Ch7DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 7 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7CtrlSpec;
impl crate::RegisterSpec for Ch7CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch7CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch7CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch7CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
