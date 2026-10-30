#[doc = "Register `CH8_MBIAS_ATH` reader"]
pub type R = crate::R<Ch8MbiasAthSpec>;
#[doc = "Register `CH8_MBIAS_ATH` writer"]
pub type W = crate::W<Ch8MbiasAthSpec>;
#[doc = "Field `ch8_mbias` reader - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch8MbiasR = crate::FieldReader<u16>;
#[doc = "Field `ch8_mbias` writer - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch8MbiasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ch8_a_touch_th` reader - Absolute threshold data of touch judgement for channel 8. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch8ATouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch8_a_touch_th` writer - Absolute threshold data of touch judgement for channel 8. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch8ATouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch8_mbias(&self) -> Ch8MbiasR {
        Ch8MbiasR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 8. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch8_a_touch_th(&self) -> Ch8ATouchThR {
        Ch8ATouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch8_mbias(&mut self) -> Ch8MbiasW<'_, Ch8MbiasAthSpec> {
        Ch8MbiasW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 8. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch8_a_touch_th(&mut self) -> Ch8ATouchThW<'_, Ch8MbiasAthSpec> {
        Ch8ATouchThW::new(self, 16)
    }
}
#[doc = "Channel 8 Mbias\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_mbias_ath::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_mbias_ath::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch8MbiasAthSpec;
impl crate::RegisterSpec for Ch8MbiasAthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch8_mbias_ath::R`](R) reader structure"]
impl crate::Readable for Ch8MbiasAthSpec {}
#[doc = "`write(|w| ..)` method takes [`ch8_mbias_ath::W`](W) writer structure"]
impl crate::Writable for Ch8MbiasAthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH8_MBIAS_ATH to value 0"]
impl crate::Resettable for Ch8MbiasAthSpec {}
