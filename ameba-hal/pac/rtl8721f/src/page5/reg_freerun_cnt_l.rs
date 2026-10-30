#[doc = "Register `REG_FREERUN_CNT_L` reader"]
pub type R = crate::R<RegFreerunCntLSpec>;
#[doc = "Register `REG_FREERUN_CNT_L` writer"]
pub type W = crate::W<RegFreerunCntLSpec>;
#[doc = "Field `BIT_FREERUN_CNT_TIMER_L` reader - LOW 32bits of freerun cnt"]
pub type BitFreerunCntTimerLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_FREERUN_CNT_TIMER_L` writer - LOW 32bits of freerun cnt"]
pub type BitFreerunCntTimerLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LOW 32bits of freerun cnt"]
    #[inline(always)]
    pub fn bit_freerun_cnt_timer_l(&self) -> BitFreerunCntTimerLR {
        BitFreerunCntTimerLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LOW 32bits of freerun cnt"]
    #[inline(always)]
    pub fn bit_freerun_cnt_timer_l(&mut self) -> BitFreerunCntTimerLW<'_, RegFreerunCntLSpec> {
        BitFreerunCntTimerLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_freerun_cnt_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_freerun_cnt_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFreerunCntLSpec;
impl crate::RegisterSpec for RegFreerunCntLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_freerun_cnt_l::R`](R) reader structure"]
impl crate::Readable for RegFreerunCntLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_freerun_cnt_l::W`](W) writer structure"]
impl crate::Writable for RegFreerunCntLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FREERUN_CNT_L to value 0x01"]
impl crate::Resettable for RegFreerunCntLSpec {
    const RESET_VALUE: u32 = 0x01;
}
