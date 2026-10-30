#[doc = "Register `CH2_CTRL` reader"]
pub type R = crate::R<Ch2CtrlSpec>;
#[doc = "Register `CH2_CTRL` writer"]
pub type W = crate::W<Ch2CtrlSpec>;
#[doc = "Field `ch2_en` reader - Cap sensor activity control of channel 2 0: Disable 1: Enable"]
pub type Ch2EnR = crate::BitReader;
#[doc = "Field `ch2_en` writer - Cap sensor activity control of channel 2 0: Disable 1: Enable"]
pub type Ch2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch2_baseline` reader - Digital baseline data of channel 2. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch2BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch2_baseline` writer - Digital baseline data of channel 2. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch2BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch2_d_touch_th` reader - Difference threshold data of touch judgement for channel 2. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch2DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch2_d_touch_th` writer - Difference threshold data of touch judgement for channel 2. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch2DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 2 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch2_en(&self) -> Ch2EnR {
        Ch2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 2. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch2_baseline(&self) -> Ch2BaselineR {
        Ch2BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 2. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch2_d_touch_th(&self) -> Ch2DTouchThR {
        Ch2DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 2 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch2_en(&mut self) -> Ch2EnW<'_, Ch2CtrlSpec> {
        Ch2EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 2. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch2_baseline(&mut self) -> Ch2BaselineW<'_, Ch2CtrlSpec> {
        Ch2BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 2. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch2_d_touch_th(&mut self) -> Ch2DTouchThW<'_, Ch2CtrlSpec> {
        Ch2DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2CtrlSpec;
impl crate::RegisterSpec for Ch2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch2CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
