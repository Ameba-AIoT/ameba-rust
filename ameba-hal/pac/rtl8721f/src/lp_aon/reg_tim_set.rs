#[doc = "Register `REG_TIM_SET` reader"]
pub type R = crate::R<RegTimSetSpec>;
#[doc = "Register `REG_TIM_SET` writer"]
pub type W = crate::W<RegTimSetSpec>;
#[doc = "Field `aon_tim_cnt_set` reader - Set aon time a countdown value, when timer count down, it will trigger a aon_tim event."]
pub type AonTimCntSetR = crate::FieldReader<u32>;
#[doc = "Field `aon_tim_cnt_set` writer - Set aon time a countdown value, when timer count down, it will trigger a aon_tim event."]
pub type AonTimCntSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set aon time a countdown value, when timer count down, it will trigger a aon_tim event."]
    #[inline(always)]
    pub fn aon_tim_cnt_set(&self) -> AonTimCntSetR {
        AonTimCntSetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set aon time a countdown value, when timer count down, it will trigger a aon_tim event."]
    #[inline(always)]
    pub fn aon_tim_cnt_set(&mut self) -> AonTimCntSetW<'_, RegTimSetSpec> {
        AonTimCntSetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTimSetSpec;
impl crate::RegisterSpec for RegTimSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim_set::R`](R) reader structure"]
impl crate::Readable for RegTimSetSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim_set::W`](W) writer structure"]
impl crate::Writable for RegTimSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM_SET to value 0"]
impl crate::Resettable for RegTimSetSpec {}
