#[doc = "Register `CH3_CTRL` reader"]
pub type R = crate::R<Ch3CtrlSpec>;
#[doc = "Register `CH3_CTRL` writer"]
pub type W = crate::W<Ch3CtrlSpec>;
#[doc = "Field `ch3_en` reader - Cap sensor activity control of channel 3 0: Disable 1: Enable"]
pub type Ch3EnR = crate::BitReader;
#[doc = "Field `ch3_en` writer - Cap sensor activity control of channel 3 0: Disable 1: Enable"]
pub type Ch3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch3_baseline` reader - Digital baseline data of channel 3. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch3BaselineR = crate::FieldReader<u16>;
#[doc = "Field `ch3_baseline` writer - Digital baseline data of channel 3. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
pub type Ch3BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch3_d_touch_th` reader - Difference threshold data of touch judgement for channel 3. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch3DTouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch3_d_touch_th` writer - Difference threshold data of touch judgement for channel 3. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
pub type Ch3DTouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cap sensor activity control of channel 3 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch3_en(&self) -> Ch3EnR {
        Ch3EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 3. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch3_baseline(&self) -> Ch3BaselineR {
        Ch3BaselineR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 3. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch3_d_touch_th(&self) -> Ch3DTouchThR {
        Ch3DTouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cap sensor activity control of channel 3 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ch3_en(&mut self) -> Ch3EnW<'_, Ch3CtrlSpec> {
        Ch3EnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Digital baseline data of channel 3. Init number=0x0 It can be initialed by HW when baseline initial function enabled, and can be updated automatically by HW when ETC function enabled. It could be configured by manual tuning."]
    #[inline(always)]
    pub fn ch3_baseline(&mut self) -> Ch3BaselineW<'_, Ch3CtrlSpec> {
        Ch3BaselineW::new(self, 4)
    }
    #[doc = "Bits 16:27 - Difference threshold data of touch judgement for channel 3. It needs to be configured during development, (0x0~0xFFF) (0~4095). Init number=0x1E Recommend data=80%*(signal-baseline)"]
    #[inline(always)]
    pub fn ch3_d_touch_th(&mut self) -> Ch3DTouchThW<'_, Ch3CtrlSpec> {
        Ch3DTouchThW::new(self, 16)
    }
}
#[doc = "Channel 3 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3CtrlSpec;
impl crate::RegisterSpec for Ch3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_CTRL to value 0x0064_0000"]
impl crate::Resettable for Ch3CtrlSpec {
    const RESET_VALUE: u32 = 0x0064_0000;
}
