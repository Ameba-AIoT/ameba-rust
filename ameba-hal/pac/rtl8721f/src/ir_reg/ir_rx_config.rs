#[doc = "Register `IR_RX_CONFIG` reader"]
pub type R = crate::R<IrRxConfigSpec>;
#[doc = "Register `IR_RX_CONFIG` writer"]
pub type W = crate::W<IrRxConfigSpec>;
#[doc = "Field `IR_RX_FIFO_FULL_INT_EN` reader - Rx FIFO full interrupt *0: Disable *1: Enable"]
pub type IrRxFifoFullIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_FULL_INT_EN` writer - Rx FIFO full interrupt *0: Disable *1: Enable"]
pub type IrRxFifoFullIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_EN` reader - Rx FIFO level interrupt When Rx FIFO offset >= threshold value, this interrupt is triggered. *0: Disable *1: Enable"]
pub type IrRxFifoLevelIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_EN` writer - Rx FIFO level interrupt When Rx FIFO offset >= threshold value, this interrupt is triggered. *0: Disable *1: Enable"]
pub type IrRxFifoLevelIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_OF_INT_EN` reader - RX counter overflow *0: Disable *1: Enable"]
pub type IrRxCntOfIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_OF_INT_EN` writer - RX counter overflow *0: Disable *1: Enable"]
pub type IrRxCntOfIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_OF_INT_EN` reader - RX FIFO overflow *0: Disable *1: Enable"]
pub type IrRxFifoOfIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_OF_INT_EN` writer - RX FIFO overflow *0: Disable *1: Enable"]
pub type IrRxFifoOfIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_THR_INT_EN` reader - Rx count threshold interrupt *0: Disable *1: Enable"]
pub type IrRxCntThrIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_THR_INT_EN` writer - Rx count threshold interrupt *0: Disable *1: Enable"]
pub type IrRxCntThrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_EN` reader - Rx FIFO error read interrupt When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Disable *1: Enable"]
pub type IrRxFifoErrorIntEnR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_EN` writer - Rx FIFO error read interrupt When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Disable *1: Enable"]
pub type IrRxFifoErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_LEVEL_TH` reader - Rx FIFO interrupt threshold When Rx FIFO depth > threshold value, this interrupt is triggered."]
pub type IrRxFifoLevelThR = crate::FieldReader;
#[doc = "Field `IR_RX_FIFO_LEVEL_TH` writer - Rx FIFO interrupt threshold When Rx FIFO depth > threshold value, this interrupt is triggered."]
pub type IrRxFifoLevelThW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IR_RX_FIFO_DISCARD_SET` reader - When FIFO is full, new data is send to FIFO. *0: Discard oldest data in FIFO. *1: Reject new data sending to FIFO."]
pub type IrRxFifoDiscardSetR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_DISCARD_SET` writer - When FIFO is full, new data is send to FIFO. *0: Discard oldest data in FIFO. *1: Reject new data sending to FIFO."]
pub type IrRxFifoDiscardSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_FULL_INT_MASK` reader - Rx FIFO full interrupt *0: Unmask *1: Mask"]
pub type IrRxFifoFullIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_FULL_INT_MASK` writer - Rx FIFO full interrupt *0: Unmask *1: Mask"]
pub type IrRxFifoFullIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_MASK` reader - Rx FIFO level interrupt *0: Unmask *1: Mask When Rx FIFO offset >= threshold value, this interrupt is triggered."]
pub type IrRxFifoLevelIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_MASK` writer - Rx FIFO level interrupt *0: Unmask *1: Mask When Rx FIFO offset >= threshold value, this interrupt is triggered."]
pub type IrRxFifoLevelIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_OF_INT_MASK` reader - RX counter overflow *0: Unmask *1: Mask"]
pub type IrRxCntOfIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_OF_INT_MASK` writer - RX counter overflow *0: Unmask *1: Mask"]
pub type IrRxCntOfIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_OF_INT_MASK` reader - Rx FIFO overflow *0: Unmask *1: Mask"]
pub type IrRxFifoOfIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_OF_INT_MASK` writer - Rx FIFO overflow *0: Unmask *1: Mask"]
pub type IrRxFifoOfIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_THR_INT_MASK` reader - Rx count threshold interrupt *0: Unmask *1: Mask"]
pub type IrRxCntThrIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_THR_INT_MASK` writer - Rx count threshold interrupt *0: Unmask *1: Mask"]
pub type IrRxCntThrIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_MASK` reader - Rx FIFO error read interrupt When Rx FIFO is empty, read Rx FIFO and trigger interrupt. *0: Unmask *1: Mask"]
pub type IrRxFifoErrorIntMaskR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_MASK` writer - Rx FIFO error read interrupt When Rx FIFO is empty, read Rx FIFO and trigger interrupt. *0: Unmask *1: Mask"]
pub type IrRxFifoErrorIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FILTER_STAGETX` reader - *0x0: Filter <= 20ns glitch *0x1: Filter <= 30ns glitch *0x2: Filter <= 40ns glitch *0x3: Filter <= 50ns glitch ..."]
pub type IrRxFilterStagetxR = crate::FieldReader;
#[doc = "Field `IR_RX_FILTER_STAGETX` writer - *0x0: Filter <= 20ns glitch *0x1: Filter <= 30ns glitch *0x2: Filter <= 40ns glitch *0x3: Filter <= 50ns glitch ..."]
pub type IrRxFilterStagetxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IR_RX_TRIGGER_MODE` reader - *00: High --> low trigger *01: Low -> high trigger *10: High -> low or low ->high trigger"]
pub type IrRxTriggerModeR = crate::FieldReader;
#[doc = "Field `IR_RX_TRIGGER_MODE` writer - *00: High --> low trigger *01: Low -> high trigger *10: High -> low or low ->high trigger"]
pub type IrRxTriggerModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IR_RX_MAN_START` reader - If IR_RX_TRIGGER_MODE =0, writing 1 means starting to check the waveform."]
pub type IrRxManStartR = crate::BitReader;
#[doc = "Field `IR_RX_MAN_START` writer - If IR_RX_TRIGGER_MODE =0, writing 1 means starting to check the waveform."]
pub type IrRxManStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_START_MODE` reader - *0: Manual mode, IR_RX_MAN_START control *1: Auto-mode, Trigger-mode cont"]
pub type IrRxStartModeR = crate::BitReader;
#[doc = "Field `IR_RX_START_MODE` writer - *0: Manual mode, IR_RX_MAN_START control *1: Auto-mode, Trigger-mode cont"]
pub type IrRxStartModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_START` reader - *0: FSM stops at idle state. *1: FSM runs."]
pub type IrRxStartR = crate::BitReader;
#[doc = "Field `IR_RX_START` writer - *0: FSM stops at idle state. *1: FSM runs."]
pub type IrRxStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO full interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_en(&self) -> IrRxFifoFullIntEnR {
        IrRxFifoFullIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO level interrupt When Rx FIFO offset >= threshold value, this interrupt is triggered. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_en(&self) -> IrRxFifoLevelIntEnR {
        IrRxFifoLevelIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX counter overflow *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_en(&self) -> IrRxCntOfIntEnR {
        IrRxCntOfIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO overflow *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_en(&self) -> IrRxFifoOfIntEnR {
        IrRxFifoOfIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx count threshold interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_en(&self) -> IrRxCntThrIntEnR {
        IrRxCntThrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO error read interrupt When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_en(&self) -> IrRxFifoErrorIntEnR {
        IrRxFifoErrorIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Rx FIFO interrupt threshold When Rx FIFO depth > threshold value, this interrupt is triggered."]
    #[inline(always)]
    pub fn ir_rx_fifo_level_th(&self) -> IrRxFifoLevelThR {
        IrRxFifoLevelThR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - When FIFO is full, new data is send to FIFO. *0: Discard oldest data in FIFO. *1: Reject new data sending to FIFO."]
    #[inline(always)]
    pub fn ir_rx_fifo_discard_set(&self) -> IrRxFifoDiscardSetR {
        IrRxFifoDiscardSetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx FIFO full interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_mask(&self) -> IrRxFifoFullIntMaskR {
        IrRxFifoFullIntMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO level interrupt *0: Unmask *1: Mask When Rx FIFO offset >= threshold value, this interrupt is triggered."]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_mask(&self) -> IrRxFifoLevelIntMaskR {
        IrRxFifoLevelIntMaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX counter overflow *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_mask(&self) -> IrRxCntOfIntMaskR {
        IrRxCntOfIntMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO overflow *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_mask(&self) -> IrRxFifoOfIntMaskR {
        IrRxFifoOfIntMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rx count threshold interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_mask(&self) -> IrRxCntThrIntMaskR {
        IrRxCntThrIntMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO error read interrupt When Rx FIFO is empty, read Rx FIFO and trigger interrupt. *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_mask(&self) -> IrRxFifoErrorIntMaskR {
        IrRxFifoErrorIntMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:23 - *0x0: Filter <= 20ns glitch *0x1: Filter <= 30ns glitch *0x2: Filter <= 40ns glitch *0x3: Filter <= 50ns glitch ..."]
    #[inline(always)]
    pub fn ir_rx_filter_stagetx(&self) -> IrRxFilterStagetxR {
        IrRxFilterStagetxR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - *00: High --> low trigger *01: Low -> high trigger *10: High -> low or low ->high trigger"]
    #[inline(always)]
    pub fn ir_rx_trigger_mode(&self) -> IrRxTriggerModeR {
        IrRxTriggerModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - If IR_RX_TRIGGER_MODE =0, writing 1 means starting to check the waveform."]
    #[inline(always)]
    pub fn ir_rx_man_start(&self) -> IrRxManStartR {
        IrRxManStartR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - *0: Manual mode, IR_RX_MAN_START control *1: Auto-mode, Trigger-mode cont"]
    #[inline(always)]
    pub fn ir_rx_start_mode(&self) -> IrRxStartModeR {
        IrRxStartModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - *0: FSM stops at idle state. *1: FSM runs."]
    #[inline(always)]
    pub fn ir_rx_start(&self) -> IrRxStartR {
        IrRxStartR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO full interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_en(&mut self) -> IrRxFifoFullIntEnW<'_, IrRxConfigSpec> {
        IrRxFifoFullIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO level interrupt When Rx FIFO offset >= threshold value, this interrupt is triggered. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_en(&mut self) -> IrRxFifoLevelIntEnW<'_, IrRxConfigSpec> {
        IrRxFifoLevelIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RX counter overflow *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_en(&mut self) -> IrRxCntOfIntEnW<'_, IrRxConfigSpec> {
        IrRxCntOfIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO overflow *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_en(&mut self) -> IrRxFifoOfIntEnW<'_, IrRxConfigSpec> {
        IrRxFifoOfIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx count threshold interrupt *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_en(&mut self) -> IrRxCntThrIntEnW<'_, IrRxConfigSpec> {
        IrRxCntThrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO error read interrupt When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_en(&mut self) -> IrRxFifoErrorIntEnW<'_, IrRxConfigSpec> {
        IrRxFifoErrorIntEnW::new(self, 5)
    }
    #[doc = "Bits 8:12 - Rx FIFO interrupt threshold When Rx FIFO depth > threshold value, this interrupt is triggered."]
    #[inline(always)]
    pub fn ir_rx_fifo_level_th(&mut self) -> IrRxFifoLevelThW<'_, IrRxConfigSpec> {
        IrRxFifoLevelThW::new(self, 8)
    }
    #[doc = "Bit 13 - When FIFO is full, new data is send to FIFO. *0: Discard oldest data in FIFO. *1: Reject new data sending to FIFO."]
    #[inline(always)]
    pub fn ir_rx_fifo_discard_set(&mut self) -> IrRxFifoDiscardSetW<'_, IrRxConfigSpec> {
        IrRxFifoDiscardSetW::new(self, 13)
    }
    #[doc = "Bit 14 - Rx FIFO full interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_mask(&mut self) -> IrRxFifoFullIntMaskW<'_, IrRxConfigSpec> {
        IrRxFifoFullIntMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Rx FIFO level interrupt *0: Unmask *1: Mask When Rx FIFO offset >= threshold value, this interrupt is triggered."]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_mask(&mut self) -> IrRxFifoLevelIntMaskW<'_, IrRxConfigSpec> {
        IrRxFifoLevelIntMaskW::new(self, 15)
    }
    #[doc = "Bit 16 - RX counter overflow *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_mask(&mut self) -> IrRxCntOfIntMaskW<'_, IrRxConfigSpec> {
        IrRxCntOfIntMaskW::new(self, 16)
    }
    #[doc = "Bit 17 - Rx FIFO overflow *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_mask(&mut self) -> IrRxFifoOfIntMaskW<'_, IrRxConfigSpec> {
        IrRxFifoOfIntMaskW::new(self, 17)
    }
    #[doc = "Bit 18 - Rx count threshold interrupt *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_mask(&mut self) -> IrRxCntThrIntMaskW<'_, IrRxConfigSpec> {
        IrRxCntThrIntMaskW::new(self, 18)
    }
    #[doc = "Bit 19 - Rx FIFO error read interrupt When Rx FIFO is empty, read Rx FIFO and trigger interrupt. *0: Unmask *1: Mask"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_mask(&mut self) -> IrRxFifoErrorIntMaskW<'_, IrRxConfigSpec> {
        IrRxFifoErrorIntMaskW::new(self, 19)
    }
    #[doc = "Bits 21:23 - *0x0: Filter <= 20ns glitch *0x1: Filter <= 30ns glitch *0x2: Filter <= 40ns glitch *0x3: Filter <= 50ns glitch ..."]
    #[inline(always)]
    pub fn ir_rx_filter_stagetx(&mut self) -> IrRxFilterStagetxW<'_, IrRxConfigSpec> {
        IrRxFilterStagetxW::new(self, 21)
    }
    #[doc = "Bits 24:25 - *00: High --> low trigger *01: Low -> high trigger *10: High -> low or low ->high trigger"]
    #[inline(always)]
    pub fn ir_rx_trigger_mode(&mut self) -> IrRxTriggerModeW<'_, IrRxConfigSpec> {
        IrRxTriggerModeW::new(self, 24)
    }
    #[doc = "Bit 26 - If IR_RX_TRIGGER_MODE =0, writing 1 means starting to check the waveform."]
    #[inline(always)]
    pub fn ir_rx_man_start(&mut self) -> IrRxManStartW<'_, IrRxConfigSpec> {
        IrRxManStartW::new(self, 26)
    }
    #[doc = "Bit 27 - *0: Manual mode, IR_RX_MAN_START control *1: Auto-mode, Trigger-mode cont"]
    #[inline(always)]
    pub fn ir_rx_start_mode(&mut self) -> IrRxStartModeW<'_, IrRxConfigSpec> {
        IrRxStartModeW::new(self, 27)
    }
    #[doc = "Bit 28 - *0: FSM stops at idle state. *1: FSM runs."]
    #[inline(always)]
    pub fn ir_rx_start(&mut self) -> IrRxStartW<'_, IrRxConfigSpec> {
        IrRxStartW::new(self, 28)
    }
}
#[doc = "IR Rx configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_rx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_rx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrRxConfigSpec;
impl crate::RegisterSpec for IrRxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_rx_config::R`](R) reader structure"]
impl crate::Readable for IrRxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`ir_rx_config::W`](W) writer structure"]
impl crate::Writable for IrRxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_RX_CONFIG to value 0"]
impl crate::Resettable for IrRxConfigSpec {}
