#[doc = "Register `REG_SR_CONTROL_DBG1` reader"]
pub type R = crate::R<RegSrControlDbg1Spec>;
#[doc = "Register `REG_SR_CONTROL_DBG1` writer"]
pub type W = crate::W<RegSrControlDbg1Spec>;
#[doc = "Field `BIT_SR_USABLE_TIMER_THRES` reader - If received pkt can operate spatial reuse,the min threshold for sr_timer_tx. unit:256us(default:1024us)"]
pub type BitSrUsableTimerThresR = crate::FieldReader;
#[doc = "Field `BIT_SR_USABLE_TIMER_THRES` writer - If received pkt can operate spatial reuse,the min threshold for sr_timer_tx. unit:256us(default:1024us)"]
pub type BitSrUsableTimerThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - If received pkt can operate spatial reuse,the min threshold for sr_timer_tx. unit:256us(default:1024us)"]
    #[inline(always)]
    pub fn bit_sr_usable_timer_thres(&self) -> BitSrUsableTimerThresR {
        BitSrUsableTimerThresR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - If received pkt can operate spatial reuse,the min threshold for sr_timer_tx. unit:256us(default:1024us)"]
    #[inline(always)]
    pub fn bit_sr_usable_timer_thres(
        &mut self,
    ) -> BitSrUsableTimerThresW<'_, RegSrControlDbg1Spec> {
        BitSrUsableTimerThresW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sr_control_dbg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sr_control_dbg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSrControlDbg1Spec;
impl crate::RegisterSpec for RegSrControlDbg1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`reg_sr_control_dbg1::R`](R) reader structure"]
impl crate::Readable for RegSrControlDbg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sr_control_dbg1::W`](W) writer structure"]
impl crate::Writable for RegSrControlDbg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SR_CONTROL_DBG1 to value 0x04"]
impl crate::Resettable for RegSrControlDbg1Spec {
    const RESET_VALUE: u8 = 0x04;
}
