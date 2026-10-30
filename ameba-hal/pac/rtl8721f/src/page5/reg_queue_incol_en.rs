#[doc = "Register `REG_QUEUE_INCOL_EN` reader"]
pub type R = crate::R<RegQueueIncolEnSpec>;
#[doc = "Register `REG_QUEUE_INCOL_EN` writer"]
pub type W = crate::W<RegQueueIncolEnSpec>;
#[doc = "Field `BIT_VO_TRIGGER_NUM` reader - The number is triggered by voq inter collision"]
pub type BitVoTriggerNumR = crate::FieldReader;
#[doc = "Field `BIT_VI_TRIGGER_NUM` reader - The number is triggered by viq inter collision"]
pub type BitViTriggerNumR = crate::FieldReader;
#[doc = "Field `BIT_BE_TRIGGER_NUM` reader - The number is triggered by beq inter collision"]
pub type BitBeTriggerNumR = crate::FieldReader;
#[doc = "Field `BIT_BK_TRIGGER_NUM` reader - The number is triggered by bkq inter collision"]
pub type BitBkTriggerNumR = crate::FieldReader;
#[doc = "Field `BIT_ACQ_MAX_INCOL_EN` reader - 1:Enable 0 :Disable"]
pub type BitAcqMaxIncolEnR = crate::BitReader;
#[doc = "Field `BIT_ACQ_MAX_INCOL_EN` writer - 1:Enable 0 :Disable"]
pub type BitAcqMaxIncolEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENABLE_INTERCOL_EDCA_REF` reader - When this bit set 1, enable inter-collision edca_ref"]
pub type BitEnableIntercolEdcaRefR = crate::BitReader;
#[doc = "Field `BIT_ENABLE_INTERCOL_EDCA_REF` writer - When this bit set 1, enable inter-collision edca_ref"]
pub type BitEnableIntercolEdcaRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENABLE_EDCA_REF_FUNCTION` reader - When this bit set 1, enable EDCA REF function"]
pub type BitEnableEdcaRefFunctionR = crate::BitReader;
#[doc = "Field `BIT_ENABLE_EDCA_REF_FUNCTION` writer - When this bit set 1, enable EDCA REF function"]
pub type BitEnableEdcaRefFunctionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENABLE_RANDOM_SHIFT_TX` reader - When this bit set 1, enable random shift TX function"]
pub type BitEnableRandomShiftTxR = crate::BitReader;
#[doc = "Field `BIT_ENABLE_RANDOM_SHIFT_TX` writer - When this bit set 1, enable random shift TX function"]
pub type BitEnableRandomShiftTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TBTT_AREA_BLK_4AC` reader - When set this bit, 4AC backoff is disabled on TBTT setup period"]
pub type BitEnTbttAreaBlk4acR = crate::BitReader;
#[doc = "Field `BIT_EN_TBTT_AREA_BLK_4AC` writer - When set this bit, 4AC backoff is disabled on TBTT setup period"]
pub type BitEnTbttAreaBlk4acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXOP_FAIL_BREAK` reader - 1: if tx pkt fail in txop, then tx cfend to end txop"]
pub type BitTxopFailBreakR = crate::BitReader;
#[doc = "Field `BIT_TXOP_FAIL_BREAK` writer - 1: if tx pkt fail in txop, then tx cfend to end txop"]
pub type BitTxopFailBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXOP_FAILBREAK_CNT` reader - if BIT_TXOP_FAIL_BREAK = 1, counter the times of txfail break TXOP"]
pub type BitTxopFailbreakCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The number is triggered by voq inter collision"]
    #[inline(always)]
    pub fn bit_vo_trigger_num(&self) -> BitVoTriggerNumR {
        BitVoTriggerNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The number is triggered by viq inter collision"]
    #[inline(always)]
    pub fn bit_vi_trigger_num(&self) -> BitViTriggerNumR {
        BitViTriggerNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The number is triggered by beq inter collision"]
    #[inline(always)]
    pub fn bit_be_trigger_num(&self) -> BitBeTriggerNumR {
        BitBeTriggerNumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The number is triggered by bkq inter collision"]
    #[inline(always)]
    pub fn bit_bk_trigger_num(&self) -> BitBkTriggerNumR {
        BitBkTriggerNumR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 1:Enable 0 :Disable"]
    #[inline(always)]
    pub fn bit_acq_max_incol_en(&self) -> BitAcqMaxIncolEnR {
        BitAcqMaxIncolEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit set 1, enable inter-collision edca_ref"]
    #[inline(always)]
    pub fn bit_enable_intercol_edca_ref(&self) -> BitEnableIntercolEdcaRefR {
        BitEnableIntercolEdcaRefR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit set 1, enable EDCA REF function"]
    #[inline(always)]
    pub fn bit_enable_edca_ref_function(&self) -> BitEnableEdcaRefFunctionR {
        BitEnableEdcaRefFunctionR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When this bit set 1, enable random shift TX function"]
    #[inline(always)]
    pub fn bit_enable_random_shift_tx(&self) -> BitEnableRandomShiftTxR {
        BitEnableRandomShiftTxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When set this bit, 4AC backoff is disabled on TBTT setup period"]
    #[inline(always)]
    pub fn bit_en_tbtt_area_blk_4ac(&self) -> BitEnTbttAreaBlk4acR {
        BitEnTbttAreaBlk4acR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: if tx pkt fail in txop, then tx cfend to end txop"]
    #[inline(always)]
    pub fn bit_txop_fail_break(&self) -> BitTxopFailBreakR {
        BitTxopFailBreakR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:29 - if BIT_TXOP_FAIL_BREAK = 1, counter the times of txfail break TXOP"]
    #[inline(always)]
    pub fn bit_txop_failbreak_cnt(&self) -> BitTxopFailbreakCntR {
        BitTxopFailbreakCntR::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - 1:Enable 0 :Disable"]
    #[inline(always)]
    pub fn bit_acq_max_incol_en(&mut self) -> BitAcqMaxIncolEnW<'_, RegQueueIncolEnSpec> {
        BitAcqMaxIncolEnW::new(self, 16)
    }
    #[doc = "Bit 17 - When this bit set 1, enable inter-collision edca_ref"]
    #[inline(always)]
    pub fn bit_enable_intercol_edca_ref(
        &mut self,
    ) -> BitEnableIntercolEdcaRefW<'_, RegQueueIncolEnSpec> {
        BitEnableIntercolEdcaRefW::new(self, 17)
    }
    #[doc = "Bit 18 - When this bit set 1, enable EDCA REF function"]
    #[inline(always)]
    pub fn bit_enable_edca_ref_function(
        &mut self,
    ) -> BitEnableEdcaRefFunctionW<'_, RegQueueIncolEnSpec> {
        BitEnableEdcaRefFunctionW::new(self, 18)
    }
    #[doc = "Bit 19 - When this bit set 1, enable random shift TX function"]
    #[inline(always)]
    pub fn bit_enable_random_shift_tx(
        &mut self,
    ) -> BitEnableRandomShiftTxW<'_, RegQueueIncolEnSpec> {
        BitEnableRandomShiftTxW::new(self, 19)
    }
    #[doc = "Bit 20 - When set this bit, 4AC backoff is disabled on TBTT setup period"]
    #[inline(always)]
    pub fn bit_en_tbtt_area_blk_4ac(&mut self) -> BitEnTbttAreaBlk4acW<'_, RegQueueIncolEnSpec> {
        BitEnTbttAreaBlk4acW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: if tx pkt fail in txop, then tx cfend to end txop"]
    #[inline(always)]
    pub fn bit_txop_fail_break(&mut self) -> BitTxopFailBreakW<'_, RegQueueIncolEnSpec> {
        BitTxopFailBreakW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_queue_incol_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_queue_incol_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQueueIncolEnSpec;
impl crate::RegisterSpec for RegQueueIncolEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_queue_incol_en::R`](R) reader structure"]
impl crate::Readable for RegQueueIncolEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_queue_incol_en::W`](W) writer structure"]
impl crate::Writable for RegQueueIncolEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_QUEUE_INCOL_EN to value 0x0010_0000"]
impl crate::Resettable for RegQueueIncolEnSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
