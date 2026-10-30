#[doc = "Register `REG_NAN_SETTING_3` reader"]
pub type R = crate::R<RegNanSetting3Spec>;
#[doc = "Register `REG_NAN_SETTING_3` writer"]
pub type W = crate::W<RegNanSetting3Spec>;
#[doc = "Field `EN_5G_NAN` reader - 1: enable 5G NAN mode 0: only 2.4G NAN"]
pub type En5gNanR = crate::BitReader;
#[doc = "Field `EN_5G_NAN` writer - 1: enable 5G NAN mode 0: only 2.4G NAN"]
pub type En5gNanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_TSFT_AUTO_SYNC_EN` reader - 1: enable HW auto tsft sync, when sync beacon frames are received, auto update RX.timestamp + RX path delay to TSFT_P2_0"]
pub type P2TsftAutoSyncEnR = crate::BitReader;
#[doc = "Field `P2_TSFT_AUTO_SYNC_EN` writer - 1: enable HW auto tsft sync, when sync beacon frames are received, auto update RX.timestamp + RX path delay to TSFT_P2_0"]
pub type P2TsftAutoSyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPD_TSFT_DIS_CHK_TA` reader - 1: don't check TA_FIT when update NAN tsft"]
pub type UpdTsftDisChkTaR = crate::BitReader;
#[doc = "Field `UPD_TSFT_DIS_CHK_TA` writer - 1: don't check TA_FIT when update NAN tsft"]
pub type UpdTsftDisChkTaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_TSFT_BACKUP_EN` reader - 1:en p2 backup tsft"]
pub type P2TsftBackupEnR = crate::BitReader;
#[doc = "Field `P2_TSFT_BACKUP_EN` writer - 1:en p2 backup tsft"]
pub type P2TsftBackupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_PORT_ROND` reader - 1: enable round-robin of P2 disc beacon and P0/1 beacon"]
pub type EnPortRondR = crate::BitReader;
#[doc = "Field `EN_PORT_ROND` writer - 1: enable round-robin of P2 disc beacon and P0/1 beacon"]
pub type EnPortRondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCN_PORT_PRI` reader - the priority of ports 000: no priority 001: port2 sync bcn has higher priority than P0/P1, P2 sync > P0/P1 010: port0 has higher priority than P2 sync bcn, P0 > P2 sync > P1 011: port1 has higher priority than P2 sync bcn, P1 > P2 sync > P0 100: port0 and port1 has higher priority than P2 sync bcn, P0/P1 > P2 sync"]
pub type BcnPortPriR = crate::FieldReader;
#[doc = "Field `BCN_PORT_PRI` writer - the priority of ports 000: no priority 001: port2 sync bcn has higher priority than P0/P1, P2 sync > P0/P1 010: port0 has higher priority than P2 sync bcn, P0 > P2 sync > P1 011: port1 has higher priority than P2 sync bcn, P1 > P2 sync > P0 100: port0 and port1 has higher priority than P2 sync bcn, P0/P1 > P2 sync"]
pub type BcnPortPriW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BCN_OFFSET_5G_NAN` reader - Unit is ms. the tbtt offset between 2.4G sync beacon and 5G sync bcn. Default is 128ms"]
pub type BcnOffset5gNanR = crate::FieldReader;
#[doc = "Field `BCN_OFFSET_5G_NAN` writer - Unit is ms. the tbtt offset between 2.4G sync beacon and 5G sync bcn. Default is 128ms"]
pub type BcnOffset5gNanW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCN_SPACE5` reader - Unit is ms. the beacon space of port2 sync beacon. default is 512ms"]
pub type BitBcnSpace5R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_SPACE5` writer - Unit is ms. the beacon space of port2 sync beacon. default is 512ms"]
pub type BitBcnSpace5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 1: enable 5G NAN mode 0: only 2.4G NAN"]
    #[inline(always)]
    pub fn en_5g_nan(&self) -> En5gNanR {
        En5gNanR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable HW auto tsft sync, when sync beacon frames are received, auto update RX.timestamp + RX path delay to TSFT_P2_0"]
    #[inline(always)]
    pub fn p2_tsft_auto_sync_en(&self) -> P2TsftAutoSyncEnR {
        P2TsftAutoSyncEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: don't check TA_FIT when update NAN tsft"]
    #[inline(always)]
    pub fn upd_tsft_dis_chk_ta(&self) -> UpdTsftDisChkTaR {
        UpdTsftDisChkTaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:en p2 backup tsft"]
    #[inline(always)]
    pub fn p2_tsft_backup_en(&self) -> P2TsftBackupEnR {
        P2TsftBackupEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable round-robin of P2 disc beacon and P0/1 beacon"]
    #[inline(always)]
    pub fn en_port_rond(&self) -> EnPortRondR {
        EnPortRondR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - the priority of ports 000: no priority 001: port2 sync bcn has higher priority than P0/P1, P2 sync > P0/P1 010: port0 has higher priority than P2 sync bcn, P0 > P2 sync > P1 011: port1 has higher priority than P2 sync bcn, P1 > P2 sync > P0 100: port0 and port1 has higher priority than P2 sync bcn, P0/P1 > P2 sync"]
    #[inline(always)]
    pub fn bcn_port_pri(&self) -> BcnPortPriR {
        BcnPortPriR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Unit is ms. the tbtt offset between 2.4G sync beacon and 5G sync bcn. Default is 128ms"]
    #[inline(always)]
    pub fn bcn_offset_5g_nan(&self) -> BcnOffset5gNanR {
        BcnOffset5gNanR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Unit is ms. the beacon space of port2 sync beacon. default is 512ms"]
    #[inline(always)]
    pub fn bit_bcn_space5(&self) -> BitBcnSpace5R {
        BitBcnSpace5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable 5G NAN mode 0: only 2.4G NAN"]
    #[inline(always)]
    pub fn en_5g_nan(&mut self) -> En5gNanW<'_, RegNanSetting3Spec> {
        En5gNanW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable HW auto tsft sync, when sync beacon frames are received, auto update RX.timestamp + RX path delay to TSFT_P2_0"]
    #[inline(always)]
    pub fn p2_tsft_auto_sync_en(&mut self) -> P2TsftAutoSyncEnW<'_, RegNanSetting3Spec> {
        P2TsftAutoSyncEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: don't check TA_FIT when update NAN tsft"]
    #[inline(always)]
    pub fn upd_tsft_dis_chk_ta(&mut self) -> UpdTsftDisChkTaW<'_, RegNanSetting3Spec> {
        UpdTsftDisChkTaW::new(self, 2)
    }
    #[doc = "Bit 3 - 1:en p2 backup tsft"]
    #[inline(always)]
    pub fn p2_tsft_backup_en(&mut self) -> P2TsftBackupEnW<'_, RegNanSetting3Spec> {
        P2TsftBackupEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: enable round-robin of P2 disc beacon and P0/1 beacon"]
    #[inline(always)]
    pub fn en_port_rond(&mut self) -> EnPortRondW<'_, RegNanSetting3Spec> {
        EnPortRondW::new(self, 4)
    }
    #[doc = "Bits 5:7 - the priority of ports 000: no priority 001: port2 sync bcn has higher priority than P0/P1, P2 sync > P0/P1 010: port0 has higher priority than P2 sync bcn, P0 > P2 sync > P1 011: port1 has higher priority than P2 sync bcn, P1 > P2 sync > P0 100: port0 and port1 has higher priority than P2 sync bcn, P0/P1 > P2 sync"]
    #[inline(always)]
    pub fn bcn_port_pri(&mut self) -> BcnPortPriW<'_, RegNanSetting3Spec> {
        BcnPortPriW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Unit is ms. the tbtt offset between 2.4G sync beacon and 5G sync bcn. Default is 128ms"]
    #[inline(always)]
    pub fn bcn_offset_5g_nan(&mut self) -> BcnOffset5gNanW<'_, RegNanSetting3Spec> {
        BcnOffset5gNanW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Unit is ms. the beacon space of port2 sync beacon. default is 512ms"]
    #[inline(always)]
    pub fn bit_bcn_space5(&mut self) -> BitBcnSpace5W<'_, RegNanSetting3Spec> {
        BitBcnSpace5W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_setting_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_setting_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanSetting3Spec;
impl crate::RegisterSpec for RegNanSetting3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_setting_3::R`](R) reader structure"]
impl crate::Readable for RegNanSetting3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_setting_3::W`](W) writer structure"]
impl crate::Writable for RegNanSetting3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_SETTING_3 to value 0x0200_8022"]
impl crate::Resettable for RegNanSetting3Spec {
    const RESET_VALUE: u32 = 0x0200_8022;
}
