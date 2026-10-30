#[doc = "Register `CH3_MBIAS_ATH` reader"]
pub type R = crate::R<Ch3MbiasAthSpec>;
#[doc = "Register `CH3_MBIAS_ATH` writer"]
pub type W = crate::W<Ch3MbiasAthSpec>;
#[doc = "Field `ch3_mbias` reader - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch3MbiasR = crate::FieldReader<u16>;
#[doc = "Field `ch3_mbias` writer - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
pub type Ch3MbiasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ch3_a_touch_th` reader - Absolute threshold data of touch judgement for channel 3. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch3ATouchThR = crate::FieldReader<u16>;
#[doc = "Field `ch3_a_touch_th` writer - Absolute threshold data of touch judgement for channel 3. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
pub type Ch3ATouchThW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch3_mbias(&self) -> Ch3MbiasR {
        Ch3MbiasR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 3. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch3_a_touch_th(&self) -> Ch3ATouchThR {
        Ch3ATouchThR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15: 0.25/0.25/0.25/0.25/0.5/0.5/1/2/2/4/8/8/16/32/32/64uA 0000000000000000:0uA 0000000000000001:0.25uA 0000000000000010:0.25uA ...1111111111111111:171uA"]
    #[inline(always)]
    pub fn ch3_mbias(&mut self) -> Ch3MbiasW<'_, Ch3MbiasAthSpec> {
        Ch3MbiasW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Absolute threshold data of touch judgement for channel 3. It can be updated by HW when baseline initial function or ETC function enabled, a_touch_th=baseline-d_touch_th, manual tuning when ETC disabled, (0x0~0xFFF). Recommended data=80%*signal"]
    #[inline(always)]
    pub fn ch3_a_touch_th(&mut self) -> Ch3ATouchThW<'_, Ch3MbiasAthSpec> {
        Ch3ATouchThW::new(self, 16)
    }
}
#[doc = "Channel 3 Mbias\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_mbias_ath::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_mbias_ath::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3MbiasAthSpec;
impl crate::RegisterSpec for Ch3MbiasAthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_mbias_ath::R`](R) reader structure"]
impl crate::Readable for Ch3MbiasAthSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_mbias_ath::W`](W) writer structure"]
impl crate::Writable for Ch3MbiasAthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_MBIAS_ATH to value 0"]
impl crate::Resettable for Ch3MbiasAthSpec {}
