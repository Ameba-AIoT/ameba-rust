#[doc = "Register `IR_TX_CONFIG` reader"]
pub type R = crate::R<IrTxConfigSpec>;
#[doc = "Register `IR_TX_CONFIG` writer"]
pub type W = crate::W<IrTxConfigSpec>;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_EN` reader - Tx FIFO empty interrupt *0: Disable *1: Enable"]
pub type IrTxFifoEmptyIntEnR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_EN` writer - Tx FIFO empty interrupt *0: Disable *1: Enable"]
pub type IrTxFifoEmptyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_EN` reader - Tx FIFO level interrupt When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Disable *1: Enable"]
pub type IrTxFifoLevelIntEnR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_EN` writer - Tx FIFO level interrupt When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Disable *1: Enable"]
pub type IrTxFifoLevelIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_MASK` reader - Tx FIFO empty interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoEmptyIntMaskR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_MASK` writer - Tx FIFO empty interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoEmptyIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_MASK` reader - Tx FIFO level interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoLevelIntMaskR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_MASK` writer - Tx FIFO level interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoLevelIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_OVER_INT_EN` reader - Tx FIFO overflow interrupt *0: Disable *1: Enable"]
pub type IrTxFifoOverIntEnR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_OVER_INT_EN` writer - Tx FIFO overflow interrupt *0: Disable *1: Enable"]
pub type IrTxFifoOverIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_OVER_INT_MASK` reader - Tx FIFO overflow interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoOverIntMaskR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_OVER_INT_MASK` writer - Tx FIFO overflow interrupt *0: Unmask *1: Mask"]
pub type IrTxFifoOverIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_IDLE_STATE` reader - Tx output state in idle *0: Low *1: High"]
pub type IrTxIdleStateR = crate::BitReader;
#[doc = "Field `IR_TX_IDLE_STATE` writer - Tx output state in idle *0: Low *1: High"]
pub type IrTxIdleStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_LEVEL_TH` reader - Tx FIFO interrupt threshold. When Tx FIFO depth = < threshold value, interrupt is triggered."]
pub type IrTxFifoLevelThR = crate::FieldReader;
#[doc = "Field `IR_TX_FIFO_LEVEL_TH` writer - Tx FIFO interrupt threshold. When Tx FIFO depth = < threshold value, interrupt is triggered."]
pub type IrTxFifoLevelThW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IR_TX_DE_INVERSE` reader - *0: Not inverse FIFO define *1: Inverse FIFO define"]
pub type IrTxDeInverseR = crate::BitReader;
#[doc = "Field `IR_TX_DE_INVERSE` writer - *0: Not inverse FIFO define *1: Inverse FIFO define"]
pub type IrTxDeInverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_OUTPUT_INVERSE` reader - *0: Not inverse active output *1: Inverse active output"]
pub type IrTxOutputInverseR = crate::BitReader;
#[doc = "Field `IR_TX_OUTPUT_INVERSE` writer - *0: Not inverse active output *1: Inverse active output"]
pub type IrTxOutputInverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_DUTY_NUM` reader - Duty cycle setting for modulation frequency For example: for 1/3 duty cycle, IR_DUTY_NUM = (IR_DIV_NUM+1)/3 Note: Set this value equals to IR_DIV_NUM to generate 100% duty waveform."]
pub type IrTxDutyNumR = crate::FieldReader<u16>;
#[doc = "Field `IR_TX_DUTY_NUM` writer - Duty cycle setting for modulation frequency For example: for 1/3 duty cycle, IR_DUTY_NUM = (IR_DIV_NUM+1)/3 Note: Set this value equals to IR_DIV_NUM to generate 100% duty waveform."]
pub type IrTxDutyNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IR_TX_START` reader - *0: FSM stops at idle state. *1: FSM runs."]
pub type IrTxStartR = crate::BitReader;
#[doc = "Field `IR_TX_START` writer - *0: FSM stops at idle state. *1: FSM runs."]
pub type IrTxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_MODE_SEL` reader - *0: Tx mode *1: Rx mode"]
pub type IrModeSelR = crate::BitReader;
#[doc = "Field `IR_MODE_SEL` writer - *0: Tx mode *1: Rx mode"]
pub type IrModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tx FIFO empty interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_en(&self) -> IrTxFifoEmptyIntEnR {
        IrTxFifoEmptyIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO level interrupt When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_en(&self) -> IrTxFifoLevelIntEnR {
        IrTxFifoLevelIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx FIFO empty interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_mask(&self) -> IrTxFifoEmptyIntMaskR {
        IrTxFifoEmptyIntMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO level interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_mask(&self) -> IrTxFifoLevelIntMaskR {
        IrTxFifoLevelIntMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO overflow interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_en(&self) -> IrTxFifoOverIntEnR {
        IrTxFifoOverIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO overflow interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_mask(&self) -> IrTxFifoOverIntMaskR {
        IrTxFifoOverIntMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tx output state in idle *0: Low *1: High"]
    #[inline(always)]
    pub fn ir_tx_idle_state(&self) -> IrTxIdleStateR {
        IrTxIdleStateR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Tx FIFO interrupt threshold. When Tx FIFO depth = < threshold value, interrupt is triggered."]
    #[inline(always)]
    pub fn ir_tx_fifo_level_th(&self) -> IrTxFifoLevelThR {
        IrTxFifoLevelThR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - *0: Not inverse FIFO define *1: Inverse FIFO define"]
    #[inline(always)]
    pub fn ir_tx_de_inverse(&self) -> IrTxDeInverseR {
        IrTxDeInverseR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - *0: Not inverse active output *1: Inverse active output"]
    #[inline(always)]
    pub fn ir_tx_output_inverse(&self) -> IrTxOutputInverseR {
        IrTxOutputInverseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Duty cycle setting for modulation frequency For example: for 1/3 duty cycle, IR_DUTY_NUM = (IR_DIV_NUM+1)/3 Note: Set this value equals to IR_DIV_NUM to generate 100% duty waveform."]
    #[inline(always)]
    pub fn ir_tx_duty_num(&self) -> IrTxDutyNumR {
        IrTxDutyNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - *0: FSM stops at idle state. *1: FSM runs."]
    #[inline(always)]
    pub fn ir_tx_start(&self) -> IrTxStartR {
        IrTxStartR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - *0: Tx mode *1: Rx mode"]
    #[inline(always)]
    pub fn ir_mode_sel(&self) -> IrModeSelR {
        IrModeSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx FIFO empty interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_en(&mut self) -> IrTxFifoEmptyIntEnW<'_, IrTxConfigSpec> {
        IrTxFifoEmptyIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx FIFO level interrupt When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_en(&mut self) -> IrTxFifoLevelIntEnW<'_, IrTxConfigSpec> {
        IrTxFifoLevelIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Tx FIFO empty interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_mask(&mut self) -> IrTxFifoEmptyIntMaskW<'_, IrTxConfigSpec> {
        IrTxFifoEmptyIntMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO level interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_mask(&mut self) -> IrTxFifoLevelIntMaskW<'_, IrTxConfigSpec> {
        IrTxFifoLevelIntMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx FIFO overflow interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_en(&mut self) -> IrTxFifoOverIntEnW<'_, IrTxConfigSpec> {
        IrTxFifoOverIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx FIFO overflow interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_mask(&mut self) -> IrTxFifoOverIntMaskW<'_, IrTxConfigSpec> {
        IrTxFifoOverIntMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Tx output state in idle *0: Low *1: High"]
    #[inline(always)]
    pub fn ir_tx_idle_state(&mut self) -> IrTxIdleStateW<'_, IrTxConfigSpec> {
        IrTxIdleStateW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Tx FIFO interrupt threshold. When Tx FIFO depth = < threshold value, interrupt is triggered."]
    #[inline(always)]
    pub fn ir_tx_fifo_level_th(&mut self) -> IrTxFifoLevelThW<'_, IrTxConfigSpec> {
        IrTxFifoLevelThW::new(self, 8)
    }
    #[doc = "Bit 13 - *0: Not inverse FIFO define *1: Inverse FIFO define"]
    #[inline(always)]
    pub fn ir_tx_de_inverse(&mut self) -> IrTxDeInverseW<'_, IrTxConfigSpec> {
        IrTxDeInverseW::new(self, 13)
    }
    #[doc = "Bit 14 - *0: Not inverse active output *1: Inverse active output"]
    #[inline(always)]
    pub fn ir_tx_output_inverse(&mut self) -> IrTxOutputInverseW<'_, IrTxConfigSpec> {
        IrTxOutputInverseW::new(self, 14)
    }
    #[doc = "Bits 16:27 - Duty cycle setting for modulation frequency For example: for 1/3 duty cycle, IR_DUTY_NUM = (IR_DIV_NUM+1)/3 Note: Set this value equals to IR_DIV_NUM to generate 100% duty waveform."]
    #[inline(always)]
    pub fn ir_tx_duty_num(&mut self) -> IrTxDutyNumW<'_, IrTxConfigSpec> {
        IrTxDutyNumW::new(self, 16)
    }
    #[doc = "Bit 30 - *0: FSM stops at idle state. *1: FSM runs."]
    #[inline(always)]
    pub fn ir_tx_start(&mut self) -> IrTxStartW<'_, IrTxConfigSpec> {
        IrTxStartW::new(self, 30)
    }
    #[doc = "Bit 31 - *0: Tx mode *1: Rx mode"]
    #[inline(always)]
    pub fn ir_mode_sel(&mut self) -> IrModeSelW<'_, IrTxConfigSpec> {
        IrModeSelW::new(self, 31)
    }
}
#[doc = "IR Tx configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_tx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_tx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrTxConfigSpec;
impl crate::RegisterSpec for IrTxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_tx_config::R`](R) reader structure"]
impl crate::Readable for IrTxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`ir_tx_config::W`](W) writer structure"]
impl crate::Writable for IrTxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_TX_CONFIG to value 0"]
impl crate::Resettable for IrTxConfigSpec {}
