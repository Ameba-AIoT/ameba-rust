#[doc = "Register `REG_FE4IMR` reader"]
pub type R = crate::R<RegFe4imrSpec>;
#[doc = "Register `REG_FE4IMR` writer"]
pub type W = crate::W<RegFe4imrSpec>;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_OK_INT_EN` reader - port 1 rx broadcast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_OK_INT_EN` writer - port 1 rx broadcast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_OK_INT_EN` reader - port 1 rx broadcast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_OK_INT_EN` writer - port 1 rx broadcast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_OK_INT_EN` reader - port 1 rx unicast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_OK_INT_EN` writer - port 1 rx unicast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_OK_INT_EN` reader - port 1 rx unicast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_OK_INT_EN` writer - port 1 rx unicast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB0_INT_EN` reader - MBSSID0 ATIM end window interrupt ."]
pub type BitFsAtimMb0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB0_INT_EN` writer - MBSSID0 ATIM end window interrupt ."]
pub type BitFsAtimMb0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB1_INT_EN` reader - MBSSID1 ATIM end window interrupt ."]
pub type BitFsAtimMb1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB1_INT_EN` writer - MBSSID1 ATIM end window interrupt ."]
pub type BitFsAtimMb1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB2_INT_EN` reader - MBSSID2 ATIM end window interrupt ."]
pub type BitFsAtimMb2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB2_INT_EN` writer - MBSSID2 ATIM end window interrupt ."]
pub type BitFsAtimMb2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB3_INT_EN` reader - MBSSID3 ATIM end window interrupt ."]
pub type BitFsAtimMb3IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB3_INT_EN` writer - MBSSID3 ATIM end window interrupt ."]
pub type BitFsAtimMb3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB4_INT_EN` reader - MBSSID4 ATIM end window interrupt ."]
pub type BitFsAtimMb4IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB4_INT_EN` writer - MBSSID4 ATIM end window interrupt ."]
pub type BitFsAtimMb4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB5_INT_EN` reader - MBSSID5 ATIM end window interrupt ."]
pub type BitFsAtimMb5IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB5_INT_EN` writer - MBSSID5 ATIM end window interrupt ."]
pub type BitFsAtimMb5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB6_INT_EN` reader - MBSSID6 ATIM end window interrupt ."]
pub type BitFsAtimMb6IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB6_INT_EN` writer - MBSSID6 ATIM end window interrupt ."]
pub type BitFsAtimMb6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_MB7_INT_EN` reader - MBSSID7 ATIM end window interrupt ."]
pub type BitFsAtimMb7IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB7_INT_EN` writer - MBSSID7 ATIM end window interrupt ."]
pub type BitFsAtimMb7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ATIM_PORT1_INT_EN` reader - PORT1 ATIM end window interrupt ."]
pub type BitFsAtimPort1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_PORT1_INT_EN` writer - PORT1 ATIM end window interrupt ."]
pub type BitFsAtimPort1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_PKTIN_INT_EN` reader - port 1 packet assign to queue interrupt"]
pub type BitFsPort1PktinIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_PKTIN_INT_EN` writer - port 1 packet assign to queue interrupt"]
pub type BitFsPort1PktinIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT0_PKTIN_INT_EN` reader - port 0 packet assign to queue interrupt"]
pub type BitFsPort0PktinIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_PKTIN_INT_EN` writer - port 0 packet assign to queue interrupt"]
pub type BitFsPort0PktinIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_state0_timeout_INT_EN` reader - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState0TimeoutIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_state0_timeout_INT_EN` writer - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState0TimeoutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_state1_timeout_INT_EN` reader - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState1TimeoutIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_state1_timeout_INT_EN` writer - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState1TimeoutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TRIG_LAT_AUDIO_CNT_INT_EN` reader - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitFsTrigLatAudioCntIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TRIG_LAT_AUDIO_CNT_INT_EN` writer - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitFsTrigLatAudioCntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB1_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB1_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB2_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB2_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB3_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb3IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB3_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB4_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb4IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB4_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB5_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb5IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB5_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB6_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb6IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB6_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA0_MB7_INT_EN` reader - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb7IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB7_INT_EN` writer - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA1_INT_EN` reader - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA1_INT_EN` writer - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PCIE_BCNDMAERR_INT_EN` reader - PCIE BCN DMA error interrupt"]
pub type BitFsPcieBcndmaerrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PCIE_BCNDMAERR_INT_EN` writer - PCIE BCN DMA error interrupt"]
pub type BitFsPcieBcndmaerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - port 1 rx broadcast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ok_int_en(&self) -> BitFsPort1Rxbcmd1OkIntEnR {
        BitFsPort1Rxbcmd1OkIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - port 1 rx broadcast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ok_int_en(&self) -> BitFsPort1Rxbcmd0OkIntEnR {
        BitFsPort1Rxbcmd0OkIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - port 1 rx unicast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ok_int_en(&self) -> BitFsPort1Rxucmd1OkIntEnR {
        BitFsPort1Rxucmd1OkIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port 1 rx unicast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ok_int_en(&self) -> BitFsPort1Rxucmd0OkIntEnR {
        BitFsPort1Rxucmd0OkIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - MBSSID0 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb0_int_en(&self) -> BitFsAtimMb0IntEnR {
        BitFsAtimMb0IntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MBSSID1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb1_int_en(&self) -> BitFsAtimMb1IntEnR {
        BitFsAtimMb1IntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MBSSID2 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb2_int_en(&self) -> BitFsAtimMb2IntEnR {
        BitFsAtimMb2IntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MBSSID3 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb3_int_en(&self) -> BitFsAtimMb3IntEnR {
        BitFsAtimMb3IntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MBSSID4 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb4_int_en(&self) -> BitFsAtimMb4IntEnR {
        BitFsAtimMb4IntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MBSSID5 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb5_int_en(&self) -> BitFsAtimMb5IntEnR {
        BitFsAtimMb5IntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MBSSID6 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb6_int_en(&self) -> BitFsAtimMb6IntEnR {
        BitFsAtimMb6IntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MBSSID7 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb7_int_en(&self) -> BitFsAtimMb7IntEnR {
        BitFsAtimMb7IntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PORT1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_port1_int_en(&self) -> BitFsAtimPort1IntEnR {
        BitFsAtimPort1IntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port 1 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_pktin_int_en(&self) -> BitFsPort1PktinIntEnR {
        BitFsPort1PktinIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port 0 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_pktin_int_en(&self) -> BitFsPort0PktinIntEnR {
        BitFsPort0PktinIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state0_timeout_int_en(&self) -> BitFsState0TimeoutIntEnR {
        BitFsState0TimeoutIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state1_timeout_int_en(&self) -> BitFsState1TimeoutIntEnR {
        BitFsState1TimeoutIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_fs_trig_lat_audio_cnt_int_en(&self) -> BitFsTrigLatAudioCntIntEnR {
        BitFsTrigLatAudioCntIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_int_en(&self) -> BitFsBcndma0IntEnR {
        BitFsBcndma0IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb1_int_en(&self) -> BitFsBcndma0Mb1IntEnR {
        BitFsBcndma0Mb1IntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb2_int_en(&self) -> BitFsBcndma0Mb2IntEnR {
        BitFsBcndma0Mb2IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb3_int_en(&self) -> BitFsBcndma0Mb3IntEnR {
        BitFsBcndma0Mb3IntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb4_int_en(&self) -> BitFsBcndma0Mb4IntEnR {
        BitFsBcndma0Mb4IntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb5_int_en(&self) -> BitFsBcndma0Mb5IntEnR {
        BitFsBcndma0Mb5IntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb6_int_en(&self) -> BitFsBcndma0Mb6IntEnR {
        BitFsBcndma0Mb6IntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb7_int_en(&self) -> BitFsBcndma0Mb7IntEnR {
        BitFsBcndma0Mb7IntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma1_int_en(&self) -> BitFsBcndma1IntEnR {
        BitFsBcndma1IntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PCIE BCN DMA error interrupt"]
    #[inline(always)]
    pub fn bit_fs_pcie_bcndmaerr_int_en(&self) -> BitFsPcieBcndmaerrIntEnR {
        BitFsPcieBcndmaerrIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - port 1 rx broadcast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ok_int_en(
        &mut self,
    ) -> BitFsPort1Rxbcmd1OkIntEnW<'_, RegFe4imrSpec> {
        BitFsPort1Rxbcmd1OkIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - port 1 rx broadcast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ok_int_en(
        &mut self,
    ) -> BitFsPort1Rxbcmd0OkIntEnW<'_, RegFe4imrSpec> {
        BitFsPort1Rxbcmd0OkIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - port 1 rx unicast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ok_int_en(
        &mut self,
    ) -> BitFsPort1Rxucmd1OkIntEnW<'_, RegFe4imrSpec> {
        BitFsPort1Rxucmd1OkIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - port 1 rx unicast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ok_int_en(
        &mut self,
    ) -> BitFsPort1Rxucmd0OkIntEnW<'_, RegFe4imrSpec> {
        BitFsPort1Rxucmd0OkIntEnW::new(self, 3)
    }
    #[doc = "Bit 7 - MBSSID0 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb0_int_en(&mut self) -> BitFsAtimMb0IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb0IntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - MBSSID1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb1_int_en(&mut self) -> BitFsAtimMb1IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb1IntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - MBSSID2 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb2_int_en(&mut self) -> BitFsAtimMb2IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb2IntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - MBSSID3 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb3_int_en(&mut self) -> BitFsAtimMb3IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb3IntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - MBSSID4 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb4_int_en(&mut self) -> BitFsAtimMb4IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb4IntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - MBSSID5 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb5_int_en(&mut self) -> BitFsAtimMb5IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb5IntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - MBSSID6 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb6_int_en(&mut self) -> BitFsAtimMb6IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb6IntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - MBSSID7 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb7_int_en(&mut self) -> BitFsAtimMb7IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimMb7IntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - PORT1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_port1_int_en(&mut self) -> BitFsAtimPort1IntEnW<'_, RegFe4imrSpec> {
        BitFsAtimPort1IntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - port 1 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_pktin_int_en(&mut self) -> BitFsPort1PktinIntEnW<'_, RegFe4imrSpec> {
        BitFsPort1PktinIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - port 0 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_pktin_int_en(&mut self) -> BitFsPort0PktinIntEnW<'_, RegFe4imrSpec> {
        BitFsPort0PktinIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state0_timeout_int_en(&mut self) -> BitFsState0TimeoutIntEnW<'_, RegFe4imrSpec> {
        BitFsState0TimeoutIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state1_timeout_int_en(&mut self) -> BitFsState1TimeoutIntEnW<'_, RegFe4imrSpec> {
        BitFsState1TimeoutIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_fs_trig_lat_audio_cnt_int_en(
        &mut self,
    ) -> BitFsTrigLatAudioCntIntEnW<'_, RegFe4imrSpec> {
        BitFsTrigLatAudioCntIntEnW::new(self, 20)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_int_en(&mut self) -> BitFsBcndma0IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb1_int_en(&mut self) -> BitFsBcndma0Mb1IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb1IntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb2_int_en(&mut self) -> BitFsBcndma0Mb2IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb2IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb3_int_en(&mut self) -> BitFsBcndma0Mb3IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb3IntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb4_int_en(&mut self) -> BitFsBcndma0Mb4IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb4IntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb5_int_en(&mut self) -> BitFsBcndma0Mb5IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb5IntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb6_int_en(&mut self) -> BitFsBcndma0Mb6IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb6IntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb7_int_en(&mut self) -> BitFsBcndma0Mb7IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma0Mb7IntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma1_int_en(&mut self) -> BitFsBcndma1IntEnW<'_, RegFe4imrSpec> {
        BitFsBcndma1IntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - PCIE BCN DMA error interrupt"]
    #[inline(always)]
    pub fn bit_fs_pcie_bcndmaerr_int_en(&mut self) -> BitFsPcieBcndmaerrIntEnW<'_, RegFe4imrSpec> {
        BitFsPcieBcndmaerrIntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe4imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe4imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe4imrSpec;
impl crate::RegisterSpec for RegFe4imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe4imr::R`](R) reader structure"]
impl crate::Readable for RegFe4imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe4imr::W`](W) writer structure"]
impl crate::Writable for RegFe4imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE4IMR to value 0"]
impl crate::Resettable for RegFe4imrSpec {}
