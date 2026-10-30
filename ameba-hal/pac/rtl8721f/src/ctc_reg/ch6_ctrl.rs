#[doc = "Register `CH6_CTRL` reader"]
pub type R = crate::R<Ch6CtrlSpec>;
#[doc = "Register `CH6_CTRL` writer"]
pub type W = crate::W<Ch6CtrlSpec>;
#[doc = "Field `ch6_en` reader - Cap sensor activity control of channel 6 0: Disable 1: Enable"]
pub type Ch6EnR = crate::BitReader;
#[doc = "Field `ch6_en` writer - Cap sensor activity control of channel 6 0: Disable 1: Enable"]
pub type Ch6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch6_baseline` reader - Digital baseline data of channel 6. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch6BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch6_baseline` writer - Digital baseline data of channel 6. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch6BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch6_d_touch_th` reader - Difference threshold data of touch judgement for channel 6. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch6DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch6_d_touch_th` writer - Difference threshold data of touch judgement for channel 6. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch6DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 6 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch6_en(&self) -> Ch6EnR {
        Ch6EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 6. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch6_baseline(&self) -> Ch6BaselineR {
        Ch6BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 6. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch6_d_touch_th(&self) -> Ch6DTouchThR {
        Ch6DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 6 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch6_en(&mut self) -> Ch6EnW<'_, Ch6CtrlSpec> {
        Ch6EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 6. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch6_baseline(&mut self) -> Ch6BaselineW<'_, Ch6CtrlSpec> {
        Ch6BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 6. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch6_d_touch_th(&mut self) -> Ch6DTouchThW<'_, Ch6CtrlSpec> {
        Ch6DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 6 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6CtrlSpec;
impl crate::RegisterSpec for Ch6CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch6CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch6CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch6CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
