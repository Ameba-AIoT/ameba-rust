#[doc = "Register `REG_FE4ISR` reader"]
pub type R = crate::R<RegFe4isrSpec>;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_OK_INT` reader - port 1 rx broadcast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1OkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_OK_INT` reader - port 1 rx broadcast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0OkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_OK_INT` reader - port 1 rx unicast packet with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1OkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_OK_INT` reader - port 1 rx unicast packet with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0OkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB0_INT` reader - MBSSID0 ATIM end window interrupt ."]
pub type BitFsAtimMb0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB1_INT` reader - MBSSID1 ATIM end window interrupt ."]
pub type BitFsAtimMb1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB2_INT` reader - MBSSID2 ATIM end window interrupt ."]
pub type BitFsAtimMb2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB3_INT` reader - MBSSID3 ATIM end window interrupt ."]
pub type BitFsAtimMb3IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB4_INT` reader - MBSSID4 ATIM end window interrupt ."]
pub type BitFsAtimMb4IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB5_INT` reader - MBSSID5 ATIM end window interrupt ."]
pub type BitFsAtimMb5IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB6_INT` reader - MBSSID6 ATIM end window interrupt ."]
pub type BitFsAtimMb6IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_MB7_INT` reader - MBSSID7 ATIM end window interrupt ."]
pub type BitFsAtimMb7IntR = crate::BitReader;
#[doc = "Field `BIT_FS_ATIM_PORT1_INT` reader - PORT1 ATIM end window interrupt ."]
pub type BitFsAtimPort1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_PKTIN_INT` reader - port 1 packet assign to queue interrupt"]
pub type BitFsPort1PktinIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_PKTIN_INT` reader - port 0 packet assign to queue interrupt"]
pub type BitFsPort0PktinIntR = crate::BitReader;
#[doc = "Field `BIT_FS_state0_timeout_int` reader - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState0TimeoutIntR = crate::BitReader;
#[doc = "Field `BIT_FS_state1_timeout_int` reader - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
pub type BitFsState1TimeoutIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TRIG_LAT_AUDIO_CNT_INT` reader - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitFsTrigLatAudioCntIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_INT` reader - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB1_INT` reader - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB2_INT` reader - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB3_INT` reader - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb3IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB4_INT` reader - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb4IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB5_INT` reader - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb5IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB6_INT` reader - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb6IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA0_MB7_INT` reader - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma0Mb7IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA1_INT` reader - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PCIE_BCNDMAERR_INT` reader - PCIE BCN DMA error interrupt"]
pub type BitFsPcieBcndmaerrIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - port 1 rx broadcast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ok_int(&self) -> BitFsPort1Rxbcmd1OkIntR {
        BitFsPort1Rxbcmd1OkIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - port 1 rx broadcast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ok_int(&self) -> BitFsPort1Rxbcmd0OkIntR {
        BitFsPort1Rxbcmd0OkIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - port 1 rx unicast packet with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ok_int(&self) -> BitFsPort1Rxucmd1OkIntR {
        BitFsPort1Rxucmd1OkIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port 1 rx unicast packet with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ok_int(&self) -> BitFsPort1Rxucmd0OkIntR {
        BitFsPort1Rxucmd0OkIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - MBSSID0 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb0_int(&self) -> BitFsAtimMb0IntR {
        BitFsAtimMb0IntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MBSSID1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb1_int(&self) -> BitFsAtimMb1IntR {
        BitFsAtimMb1IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MBSSID2 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb2_int(&self) -> BitFsAtimMb2IntR {
        BitFsAtimMb2IntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MBSSID3 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb3_int(&self) -> BitFsAtimMb3IntR {
        BitFsAtimMb3IntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MBSSID4 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb4_int(&self) -> BitFsAtimMb4IntR {
        BitFsAtimMb4IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MBSSID5 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb5_int(&self) -> BitFsAtimMb5IntR {
        BitFsAtimMb5IntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MBSSID6 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb6_int(&self) -> BitFsAtimMb6IntR {
        BitFsAtimMb6IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MBSSID7 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_mb7_int(&self) -> BitFsAtimMb7IntR {
        BitFsAtimMb7IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PORT1 ATIM end window interrupt ."]
    #[inline(always)]
    pub fn bit_fs_atim_port1_int(&self) -> BitFsAtimPort1IntR {
        BitFsAtimPort1IntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port 1 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_pktin_int(&self) -> BitFsPort1PktinIntR {
        BitFsPort1PktinIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port 0 packet assign to queue interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_pktin_int(&self) -> BitFsPort0PktinIntR {
        BitFsPort0PktinIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state0_timeout_int(&self) -> BitFsState0TimeoutIntR {
        BitFsState0TimeoutIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when state0_timeout_timer is equal to r_state0_timeout_thres, which means state_0 is hang, hw will set this interrupt to inform sw"]
    #[inline(always)]
    pub fn bit_fs_state1_timeout_int(&self) -> BitFsState1TimeoutIntR {
        BitFsState1TimeoutIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_fs_trig_lat_audio_cnt_int(&self) -> BitFsTrigLatAudioCntIntR {
        BitFsTrigLatAudioCntIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT0_MB0, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_int(&self) -> BitFsBcndma0IntR {
        BitFsBcndma0IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT0_MB1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb1_int(&self) -> BitFsBcndma0Mb1IntR {
        BitFsBcndma0Mb1IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT0_MB2, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb2_int(&self) -> BitFsBcndma0Mb2IntR {
        BitFsBcndma0Mb2IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When BCNDMA interval arrives before TBTT0_MB3, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb3_int(&self) -> BitFsBcndma0Mb3IntR {
        BitFsBcndma0Mb3IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When BCNDMA interval arrives before TBTT0_MB4, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb4_int(&self) -> BitFsBcndma0Mb4IntR {
        BitFsBcndma0Mb4IntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When BCNDMA interval arrives before TBTT0_MB5, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb5_int(&self) -> BitFsBcndma0Mb5IntR {
        BitFsBcndma0Mb5IntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When BCNDMA interval arrives before TBTT0_MB6, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb6_int(&self) -> BitFsBcndma0Mb6IntR {
        BitFsBcndma0Mb6IntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When BCNDMA interval arrives before TBTT0_MB7, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma0_mb7_int(&self) -> BitFsBcndma0Mb7IntR {
        BitFsBcndma0Mb7IntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When BCNDMA interval arrives before TBTT1, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma1_int(&self) -> BitFsBcndma1IntR {
        BitFsBcndma1IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PCIE BCN DMA error interrupt"]
    #[inline(always)]
    pub fn bit_fs_pcie_bcndmaerr_int(&self) -> BitFsPcieBcndmaerrIntR {
        BitFsPcieBcndmaerrIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe4isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe4isrSpec;
impl crate::RegisterSpec for RegFe4isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe4isr::R`](R) reader structure"]
impl crate::Readable for RegFe4isrSpec {}
#[doc = "`reset()` method sets REG_FE4ISR to value 0"]
impl crate::Resettable for RegFe4isrSpec {}
