#[doc = "Register `IR_RX_CNT_INT_SEL` reader"]
pub type R = crate::R<IrRxCntIntSelSpec>;
#[doc = "Register `IR_RX_CNT_INT_SEL` writer"]
pub type W = crate::W<IrRxCntIntSelSpec>;
#[doc = "Field `IR_RX_CNT_THR` reader - 31-bits threshold"]
pub type IrRxCntThrR = crate::FieldReader<u32>;
#[doc = "Field `IR_RX_CNT_THR` writer - 31-bits threshold"]
pub type IrRxCntThrW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `IR_RX_CNT_THR_TRIGGER_LV` reader - Trigger level *0: When low level counter >= threshold, trigger interrupt *1: When high level counter >= threshold, trigger interrupt"]
pub type IrRxCntThrTriggerLvR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_THR_TRIGGER_LV` writer - Trigger level *0: When low level counter >= threshold, trigger interrupt *1: When high level counter >= threshold, trigger interrupt"]
pub type IrRxCntThrTriggerLvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - 31-bits threshold"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr(&self) -> IrRxCntThrR {
        IrRxCntThrR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Trigger level *0: When low level counter >= threshold, trigger interrupt *1: When high level counter >= threshold, trigger interrupt"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_trigger_lv(&self) -> IrRxCntThrTriggerLvR {
        IrRxCntThrTriggerLvR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - 31-bits threshold"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr(&mut self) -> IrRxCntThrW<'_, IrRxCntIntSelSpec> {
        IrRxCntThrW::new(self, 0)
    }
    #[doc = "Bit 31 - Trigger level *0: When low level counter >= threshold, trigger interrupt *1: When high level counter >= threshold, trigger interrupt"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_trigger_lv(&mut self) -> IrRxCntThrTriggerLvW<'_, IrRxCntIntSelSpec> {
        IrRxCntThrTriggerLvW::new(self, 31)
    }
}
#[doc = "IR Rx count threshold configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_rx_cnt_int_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_rx_cnt_int_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrRxCntIntSelSpec;
impl crate::RegisterSpec for IrRxCntIntSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_rx_cnt_int_sel::R`](R) reader structure"]
impl crate::Readable for IrRxCntIntSelSpec {}
#[doc = "`write(|w| ..)` method takes [`ir_rx_cnt_int_sel::W`](W) writer structure"]
impl crate::Writable for IrRxCntIntSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_RX_CNT_INT_SEL to value 0"]
impl crate::Resettable for IrRxCntIntSelSpec {}
