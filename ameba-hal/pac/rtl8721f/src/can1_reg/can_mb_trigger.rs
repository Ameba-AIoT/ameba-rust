#[doc = "Register `CAN_MB_TRIGGER` reader"]
pub type R = crate::R<CanMbTriggerSpec>;
#[doc = "Register `CAN_MB_TRIGGER` writer"]
pub type W = crate::W<CanMbTriggerSpec>;
#[doc = "Field `tx_trigger_begin` reader - start of trigger time"]
pub type TxTriggerBeginR = crate::FieldReader<u16>;
#[doc = "Field `tx_trigger_begin` writer - start of trigger time"]
pub type TxTriggerBeginW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tx_trigger_close_offset` reader - end of trigger time = <$bit:tx_trigger_begin> + <$bit:tx_trigger_close_offset>"]
pub type TxTriggerCloseOffsetR = crate::FieldReader;
#[doc = "Field `tx_trigger_close_offset` writer - end of trigger time = <$bit:tx_trigger_begin> + <$bit:tx_trigger_close_offset>"]
pub type TxTriggerCloseOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tx_trigger_en` reader - End of message buffer base address in RAM"]
pub type TxTriggerEnR = crate::BitReader;
#[doc = "Field `tx_trigger_en` writer - End of message buffer base address in RAM"]
pub type TxTriggerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - start of trigger time"]
    #[inline(always)]
    pub fn tx_trigger_begin(&self) -> TxTriggerBeginR {
        TxTriggerBeginR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - end of trigger time = <$bit:tx_trigger_begin> + <$bit:tx_trigger_close_offset>"]
    #[inline(always)]
    pub fn tx_trigger_close_offset(&self) -> TxTriggerCloseOffsetR {
        TxTriggerCloseOffsetR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - End of message buffer base address in RAM"]
    #[inline(always)]
    pub fn tx_trigger_en(&self) -> TxTriggerEnR {
        TxTriggerEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - start of trigger time"]
    #[inline(always)]
    pub fn tx_trigger_begin(&mut self) -> TxTriggerBeginW<'_, CanMbTriggerSpec> {
        TxTriggerBeginW::new(self, 0)
    }
    #[doc = "Bits 16:23 - end of trigger time = <$bit:tx_trigger_begin> + <$bit:tx_trigger_close_offset>"]
    #[inline(always)]
    pub fn tx_trigger_close_offset(&mut self) -> TxTriggerCloseOffsetW<'_, CanMbTriggerSpec> {
        TxTriggerCloseOffsetW::new(self, 16)
    }
    #[doc = "Bit 24 - End of message buffer base address in RAM"]
    #[inline(always)]
    pub fn tx_trigger_en(&mut self) -> TxTriggerEnW<'_, CanMbTriggerSpec> {
        TxTriggerEnW::new(self, 24)
    }
}
#[doc = "CAN tx trigger by timer configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb_trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb_trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMbTriggerSpec;
impl crate::RegisterSpec for CanMbTriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb_trigger::R`](R) reader structure"]
impl crate::Readable for CanMbTriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb_trigger::W`](W) writer structure"]
impl crate::Writable for CanMbTriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB_TRIGGER to value 0"]
impl crate::Resettable for CanMbTriggerSpec {}
