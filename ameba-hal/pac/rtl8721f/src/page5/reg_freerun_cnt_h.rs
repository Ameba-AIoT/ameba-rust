#[doc = "Register `REG_FREERUN_CNT_H` reader"]
pub type R = crate::R<RegFreerunCntHSpec>;
#[doc = "Register `REG_FREERUN_CNT_H` writer"]
pub type W = crate::W<RegFreerunCntHSpec>;
#[doc = "Field `BIT_FREERUN_CNT_TIMER_H` reader - high 32bits of freerun cnt"]
pub type BitFreerunCntTimerHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_FREERUN_CNT_TIMER_H` writer - high 32bits of freerun cnt"]
pub type BitFreerunCntTimerHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits of freerun cnt"]
    #[inline(always)]
    pub fn bit_freerun_cnt_timer_h(&self) -> BitFreerunCntTimerHR {
        BitFreerunCntTimerHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits of freerun cnt"]
    #[inline(always)]
    pub fn bit_freerun_cnt_timer_h(&mut self) -> BitFreerunCntTimerHW<'_, RegFreerunCntHSpec> {
        BitFreerunCntTimerHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_freerun_cnt_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_freerun_cnt_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFreerunCntHSpec;
impl crate::RegisterSpec for RegFreerunCntHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_freerun_cnt_h::R`](R) reader structure"]
impl crate::Readable for RegFreerunCntHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_freerun_cnt_h::W`](W) writer structure"]
impl crate::Writable for RegFreerunCntHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FREERUN_CNT_H to value 0x01"]
impl crate::Resettable for RegFreerunCntHSpec {
    const RESET_VALUE: u32 = 0x01;
}
