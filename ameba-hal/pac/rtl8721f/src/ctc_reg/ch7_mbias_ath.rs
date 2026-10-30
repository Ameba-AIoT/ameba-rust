#[doc = "Register `CH7_MBIAS_ATH` reader"]
pub type R = crate::R<Ch7MbiasAthSpec>;
#[doc = "Register `CH7_MBIAS_ATH` writer"]
pub type W = crate::W<Ch7MbiasAthSpec>;
#[doc = "Field `ch7_mbias` reader - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch7MbiasR = crate::FieldReader<u16>;
#[doc = "Field `ch7_mbias` writer - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch7MbiasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ch7_a_touch_th` reader - Absolute threshold data of touch judgement for channel 7. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch7ATouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch7_a_touch_th` writer - Absolute threshold data of touch judgement for channel 7. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch7ATouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch7_mbias(&self) -> Ch7MbiasR {
        Ch7MbiasR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 7. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch7_a_touch_th(&self) -> Ch7ATouchThR {
        Ch7ATouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch7_mbias(&mut self) -> Ch7MbiasW<'_, Ch7MbiasAthSpec> {
        Ch7MbiasW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 7. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch7_a_touch_th(&mut self) -> Ch7ATouchThW<'_, Ch7MbiasAthSpec> {
        Ch7ATouchThW::new(self, 16)
    }
}
#[doc = "Channel 7 Mbias\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_mbias_ath::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_mbias_ath::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7MbiasAthSpec;
impl crate::RegisterSpec for Ch7MbiasAthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_mbias_ath::R`](R) reader structure"]
impl crate::Readable for Ch7MbiasAthSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_mbias_ath::W`](W) writer structure"]
impl crate::Writable for Ch7MbiasAthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_MBIAS_ATH to value 0"]
impl crate::Resettable for Ch7MbiasAthSpec {}
