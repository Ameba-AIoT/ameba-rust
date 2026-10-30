#[doc = "Register `REG_TSFT_SYN_OFFSET_SLOT` reader"]
pub type R = crate::R<RegTsftSynOffsetSlotSpec>;
#[doc = "Register `REG_TSFT_SYN_OFFSET_SLOT` writer"]
pub type W = crate::W<RegTsftSynOffsetSlotSpec>;
#[doc = "Field `BIT_TSFTR_SYNC_OFFSET` reader - tsft sync. Offset (bcn0 sync to bcn1 offset or bcn1 sync to bcn0 offset). Unit : 32us, default is 50ms"]
pub type BitTsftrSyncOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TSFTR_SYNC_OFFSET` writer - tsft sync. Offset (bcn0 sync to bcn1 offset or bcn1 sync to bcn0 offset). Unit : 32us, default is 50ms"]
pub type BitTsftrSyncOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_AGGR_BK_TIME` reader - Hw will break aggr. In sifs condition"]
pub type BitAggrBkTimeR = crate::FieldReader;
#[doc = "Field `BIT_AGGR_BK_TIME` writer - Hw will break aggr. In sifs condition"]
pub type BitAggrBkTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SLOTTIME` reader - This register sets the slot time value mentioned in 802.11 specification. It is in units of us. Default: 0x9"]
pub type BitSlottimeR = crate::FieldReader;
#[doc = "Field `BIT_SLOTTIME` writer - This register sets the slot time value mentioned in 802.11 specification. It is in units of us. Default: 0x9"]
pub type BitSlottimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - tsft sync. Offset (bcn0 sync to bcn1 offset or bcn1 sync to bcn0 offset). Unit : 32us, default is 50ms"]
    #[inline(always)]
    pub fn bit_tsftr_sync_offset(&self) -> BitTsftrSyncOffsetR {
        BitTsftrSyncOffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Hw will break aggr. In sifs condition"]
    #[inline(always)]
    pub fn bit_aggr_bk_time(&self) -> BitAggrBkTimeR {
        BitAggrBkTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register sets the slot time value mentioned in 802.11 specification. It is in units of us. Default: 0x9"]
    #[inline(always)]
    pub fn bit_slottime(&self) -> BitSlottimeR {
        BitSlottimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - tsft sync. Offset (bcn0 sync to bcn1 offset or bcn1 sync to bcn0 offset). Unit : 32us, default is 50ms"]
    #[inline(always)]
    pub fn bit_tsftr_sync_offset(&mut self) -> BitTsftrSyncOffsetW<'_, RegTsftSynOffsetSlotSpec> {
        BitTsftrSyncOffsetW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Hw will break aggr. In sifs condition"]
    #[inline(always)]
    pub fn bit_aggr_bk_time(&mut self) -> BitAggrBkTimeW<'_, RegTsftSynOffsetSlotSpec> {
        BitAggrBkTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This register sets the slot time value mentioned in 802.11 specification. It is in units of us. Default: 0x9"]
    #[inline(always)]
    pub fn bit_slottime(&mut self) -> BitSlottimeW<'_, RegTsftSynOffsetSlotSpec> {
        BitSlottimeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsft_syn_offset_slot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tsft_syn_offset_slot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsftSynOffsetSlotSpec;
impl crate::RegisterSpec for RegTsftSynOffsetSlotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsft_syn_offset_slot::R`](R) reader structure"]
impl crate::Readable for RegTsftSynOffsetSlotSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tsft_syn_offset_slot::W`](W) writer structure"]
impl crate::Writable for RegTsftSynOffsetSlotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TSFT_SYN_OFFSET_SLOT to value 0x0914_0640"]
impl crate::Resettable for RegTsftSynOffsetSlotSpec {
    const RESET_VALUE: u32 = 0x0914_0640;
}
