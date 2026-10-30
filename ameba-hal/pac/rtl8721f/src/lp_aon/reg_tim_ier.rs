#[doc = "Register `REG_TIM_IER` reader"]
pub type R = crate::R<RegTimIerSpec>;
#[doc = "Register `REG_TIM_IER` writer"]
pub type W = crate::W<RegTimIerSpec>;
#[doc = "Field `AON_TIM_IE` reader - Timer interrupt enable 0:disable 1:enable"]
pub type AonTimIeR = crate::BitReader;
#[doc = "Field `AON_TIM_IE` writer - Timer interrupt enable 0:disable 1:enable"]
pub type AonTimIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer interrupt enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn aon_tim_ie(&self) -> AonTimIeR {
        AonTimIeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer interrupt enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn aon_tim_ie(&mut self) -> AonTimIeW<'_, RegTimIerSpec> {
        AonTimIeW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTimIerSpec;
impl crate::RegisterSpec for RegTimIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim_ier::R`](R) reader structure"]
impl crate::Readable for RegTimIerSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim_ier::W`](W) writer structure"]
impl crate::Writable for RegTimIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM_IER to value 0"]
impl crate::Resettable for RegTimIerSpec {}
