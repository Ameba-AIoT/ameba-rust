#[doc = "Register `REG_NAN_SETTING_1` reader"]
pub type R = crate::R<RegNanSetting1Spec>;
#[doc = "Register `REG_NAN_SETTING_1` writer"]
pub type W = crate::W<RegNanSetting1Spec>;
#[doc = "Field `BIT_EARLY_128US_P2` reader - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the ERLYINTIM x 1ms + EARLY_128us x 128us."]
pub type BitEarly128usP2R = crate::FieldReader;
#[doc = "Field `BIT_EARLY_128US_P2` writer - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the ERLYINTIM x 1ms + EARLY_128us x 128us."]
pub type BitEarly128usP2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_EVTQ_EARLY` reader - the early time of evtq tx window. unit is TU"]
pub type BitEvtqEarlyR = crate::FieldReader;
#[doc = "Field `BIT_EVTQ_EARLY` writer - the early time of evtq tx window. unit is TU"]
pub type BitEvtqEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_mDW_EVTQ` reader - evtq packet will tx in the m-th SLOT of the m-th DWs"]
pub type BitMDwEvtqR = crate::FieldReader;
#[doc = "Field `BIT_mDW_EVTQ` writer - evtq packet will tx in the m-th SLOT of the m-th DWs"]
pub type BitMDwEvtqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_mSLOT_EVTQ` reader - evtq packet will tx in the m-th SLOT of the m-th DWs"]
pub type BitMSlotEvtqR = crate::FieldReader;
#[doc = "Field `BIT_mSLOT_EVTQ` writer - evtq packet will tx in the m-th SLOT of the m-th DWs"]
pub type BitMSlotEvtqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NAN_ROLE` reader - the role of NAN device, including master, no-master sync, and no-master no-sync."]
pub type BitNanRoleR = crate::FieldReader;
#[doc = "Field `BIT_NAN_ROLE` writer - the role of NAN device, including master, no-master sync, and no-master no-sync."]
pub type BitNanRoleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_DW_END_EARLY` reader - when set, the DW window end earlier than spec definition. When sync bcn and SDF had tx, and rx enough sync bcns defined by 0x5DC\\[31:27\\]. DW can early end"]
pub type BitDwEndEarlyR = crate::BitReader;
#[doc = "Field `BIT_DW_END_EARLY` writer - when set, the DW window end earlier than spec definition. When sync bcn and SDF had tx, and rx enough sync bcns defined by 0x5DC\\[31:27\\]. DW can early end"]
pub type BitDwEndEarlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SYNCBCN_RXNUM` reader - if DW is permitted to end early, the number of rx sync beacons is defined in this register"]
pub type BitSyncbcnRxnumR = crate::FieldReader;
#[doc = "Field `BIT_SYNCBCN_RXNUM` writer - if DW is permitted to end early, the number of rx sync beacons is defined in this register"]
pub type BitSyncbcnRxnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the ERLYINTIM x 1ms + EARLY_128us x 128us."]
    #[inline(always)]
    pub fn bit_early_128us_p2(&self) -> BitEarly128usP2R {
        BitEarly128usP2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - the early time of evtq tx window. unit is TU"]
    #[inline(always)]
    pub fn bit_evtq_early(&self) -> BitEvtqEarlyR {
        BitEvtqEarlyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - evtq packet will tx in the m-th SLOT of the m-th DWs"]
    #[inline(always)]
    pub fn bit_m_dw_evtq(&self) -> BitMDwEvtqR {
        BitMDwEvtqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - evtq packet will tx in the m-th SLOT of the m-th DWs"]
    #[inline(always)]
    pub fn bit_m_slot_evtq(&self) -> BitMSlotEvtqR {
        BitMSlotEvtqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - the role of NAN device, including master, no-master sync, and no-master no-sync."]
    #[inline(always)]
    pub fn bit_nan_role(&self) -> BitNanRoleR {
        BitNanRoleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - when set, the DW window end earlier than spec definition. When sync bcn and SDF had tx, and rx enough sync bcns defined by 0x5DC\\[31:27\\]. DW can early end"]
    #[inline(always)]
    pub fn bit_dw_end_early(&self) -> BitDwEndEarlyR {
        BitDwEndEarlyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - if DW is permitted to end early, the number of rx sync beacons is defined in this register"]
    #[inline(always)]
    pub fn bit_syncbcn_rxnum(&self) -> BitSyncbcnRxnumR {
        BitSyncbcnRxnumR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the ERLYINTIM x 1ms + EARLY_128us x 128us."]
    #[inline(always)]
    pub fn bit_early_128us_p2(&mut self) -> BitEarly128usP2W<'_, RegNanSetting1Spec> {
        BitEarly128usP2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - the early time of evtq tx window. unit is TU"]
    #[inline(always)]
    pub fn bit_evtq_early(&mut self) -> BitEvtqEarlyW<'_, RegNanSetting1Spec> {
        BitEvtqEarlyW::new(self, 4)
    }
    #[doc = "Bits 8:15 - evtq packet will tx in the m-th SLOT of the m-th DWs"]
    #[inline(always)]
    pub fn bit_m_dw_evtq(&mut self) -> BitMDwEvtqW<'_, RegNanSetting1Spec> {
        BitMDwEvtqW::new(self, 8)
    }
    #[doc = "Bits 16:23 - evtq packet will tx in the m-th SLOT of the m-th DWs"]
    #[inline(always)]
    pub fn bit_m_slot_evtq(&mut self) -> BitMSlotEvtqW<'_, RegNanSetting1Spec> {
        BitMSlotEvtqW::new(self, 16)
    }
    #[doc = "Bits 24:25 - the role of NAN device, including master, no-master sync, and no-master no-sync."]
    #[inline(always)]
    pub fn bit_nan_role(&mut self) -> BitNanRoleW<'_, RegNanSetting1Spec> {
        BitNanRoleW::new(self, 24)
    }
    #[doc = "Bit 26 - when set, the DW window end earlier than spec definition. When sync bcn and SDF had tx, and rx enough sync bcns defined by 0x5DC\\[31:27\\]. DW can early end"]
    #[inline(always)]
    pub fn bit_dw_end_early(&mut self) -> BitDwEndEarlyW<'_, RegNanSetting1Spec> {
        BitDwEndEarlyW::new(self, 26)
    }
    #[doc = "Bits 27:31 - if DW is permitted to end early, the number of rx sync beacons is defined in this register"]
    #[inline(always)]
    pub fn bit_syncbcn_rxnum(&mut self) -> BitSyncbcnRxnumW<'_, RegNanSetting1Spec> {
        BitSyncbcnRxnumW::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_setting_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_setting_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanSetting1Spec;
impl crate::RegisterSpec for RegNanSetting1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_setting_1::R`](R) reader structure"]
impl crate::Readable for RegNanSetting1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_setting_1::W`](W) writer structure"]
impl crate::Writable for RegNanSetting1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_SETTING_1 to value 0x10"]
impl crate::Resettable for RegNanSetting1Spec {
    const RESET_VALUE: u32 = 0x10;
}
