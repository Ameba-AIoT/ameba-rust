#[doc = "Register `REG_RXBCN_TIMER` reader"]
pub type R = crate::R<RegRxbcnTimerSpec>;
#[doc = "Register `REG_RXBCN_TIMER` writer"]
pub type W = crate::W<RegRxbcnTimerSpec>;
#[doc = "Field `TBTT_HOLD_TIMER` reader - start form tbtt int,unit:1us"]
pub type TbttHoldTimerR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT_THRES` reader - threshold for rxbcn timeout counter,when 0x17C\\[30:27\\] equals to this value,generate interrupt"]
pub type BitRxbcnTimeupCountThresR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT_THRES` writer - threshold for rxbcn timeout counter,when 0x17C\\[30:27\\] equals to this value,generate interrupt"]
pub type BitRxbcnTimeupCountThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT_CLR` reader - clear rxbcn timeout counter"]
pub type BitRxbcnTimeupCountClrR = crate::BitReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT_CLR` writer - clear rxbcn timeout counter"]
pub type BitRxbcnTimeupCountClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WAIT_RXBCN_FINISH` reader - when rxbcn frame, turn off rf or generate interrupt ablout rxbcn after beacon fcs is parsered"]
pub type BitFsWaitRxbcnFinishR = crate::BitReader;
#[doc = "Field `BIT_FS_WAIT_RXBCN_FINISH` writer - when rxbcn frame, turn off rf or generate interrupt ablout rxbcn after beacon fcs is parsered"]
pub type BitFsWaitRxbcnFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT` reader - rxbcn timeout counter"]
pub type BitRxbcnTimeupCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - start form tbtt int,unit:1us"]
    #[inline(always)]
    pub fn tbtt_hold_timer(&self) -> TbttHoldTimerR {
        TbttHoldTimerR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 20:23 - threshold for rxbcn timeout counter,when 0x17C\\[30:27\\] equals to this value,generate interrupt"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count_thres(&self) -> BitRxbcnTimeupCountThresR {
        BitRxbcnTimeupCountThresR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - clear rxbcn timeout counter"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count_clr(&self) -> BitRxbcnTimeupCountClrR {
        BitRxbcnTimeupCountClrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - when rxbcn frame, turn off rf or generate interrupt ablout rxbcn after beacon fcs is parsered"]
    #[inline(always)]
    pub fn bit_fs_wait_rxbcn_finish(&self) -> BitFsWaitRxbcnFinishR {
        BitFsWaitRxbcnFinishR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:30 - rxbcn timeout counter"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count(&self) -> BitRxbcnTimeupCountR {
        BitRxbcnTimeupCountR::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - threshold for rxbcn timeout counter,when 0x17C\\[30:27\\] equals to this value,generate interrupt"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count_thres(
        &mut self,
    ) -> BitRxbcnTimeupCountThresW<'_, RegRxbcnTimerSpec> {
        BitRxbcnTimeupCountThresW::new(self, 20)
    }
    #[doc = "Bit 24 - clear rxbcn timeout counter"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count_clr(&mut self) -> BitRxbcnTimeupCountClrW<'_, RegRxbcnTimerSpec> {
        BitRxbcnTimeupCountClrW::new(self, 24)
    }
    #[doc = "Bit 25 - when rxbcn frame, turn off rf or generate interrupt ablout rxbcn after beacon fcs is parsered"]
    #[inline(always)]
    pub fn bit_fs_wait_rxbcn_finish(&mut self) -> BitFsWaitRxbcnFinishW<'_, RegRxbcnTimerSpec> {
        BitFsWaitRxbcnFinishW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxbcn_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxbcn_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxbcnTimerSpec;
impl crate::RegisterSpec for RegRxbcnTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxbcn_timer::R`](R) reader structure"]
impl crate::Readable for RegRxbcnTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxbcn_timer::W`](W) writer structure"]
impl crate::Writable for RegRxbcnTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXBCN_TIMER to value 0x0030_0000"]
impl crate::Resettable for RegRxbcnTimerSpec {
    const RESET_VALUE: u32 = 0x0030_0000;
}
