#[doc = "Register `CFG3_L` reader"]
pub type R = crate::R<Cfg3LSpec>;
#[doc = "Register `CFG3_L` writer"]
pub type W = crate::W<Cfg3LSpec>;
#[doc = "Field `CFG3_L_INACTIVE` reader - Indicates if the channel is inactive. Can be used in conjunction with CFGx_L.CH_SUSP to cleanly disable a channel when destination is peripheral. For more information, refer to \"Disabling a Channel Prior to Transfer Completion\" . 1 = Channel Inactive for axi interface 0 = Channel not Inactive for axi interface"]
pub type Cfg3LInactiveR = crate::BitReader;
#[doc = "Field `CFG3_L_SRC_PCTL_OVER` reader - DMA Source bus protocol finish, indicating the current read command has finished."]
pub type Cfg3LSrcPctlOverR = crate::BitReader;
#[doc = "Field `CFG3_L_DST_PCTL_OVER` reader - DMA destination bus protocol finish, indicating the current write command has finished."]
pub type Cfg3LDstPctlOverR = crate::BitReader;
#[doc = "Field `CFG3_L_CH_PRIOR` reader - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
pub type Cfg3LChPriorR = crate::FieldReader;
#[doc = "Field `CFG3_L_CH_PRIOR` writer - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
pub type Cfg3LChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG3_L_CH_SUSP` reader - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
pub type Cfg3LChSuspR = crate::BitReader;
#[doc = "Field `CFG3_L_CH_SUSP` writer - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
pub type Cfg3LChSuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO. 1 = Channel FIFO empty 0 = Channel FIFO not empty"]
pub type Cfg3LFifoEmptyR = crate::BitReader;
#[doc = "Field `CFG3_L_HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
pub type Cfg3LHsSelDstR = crate::BitReader;
#[doc = "Field `CFG3_L_HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
pub type Cfg3LHsSelDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
pub type Cfg3LHsSelSrcR = crate::BitReader;
#[doc = "Field `CFG3_L_HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
pub type Cfg3LHsSelSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_DST_HS_POL` reader - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
pub type Cfg3LDstHsPolR = crate::BitReader;
#[doc = "Field `CFG3_L_DST_HS_POL` writer - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
pub type Cfg3LDstHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_SRC_HS_POL` reader - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
pub type Cfg3LSrcHsPolR = crate::BitReader;
#[doc = "Field `CFG3_L_SRC_HS_POL` writer - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
pub type Cfg3LSrcHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_RELOAD_SRC` reader - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg3LReloadSrcR = crate::BitReader;
#[doc = "Field `CFG3_L_RELOAD_SRC` writer - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg3LReloadSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG3_L_RELOAD_DST` reader - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg3LReloadDstR = crate::BitReader;
#[doc = "Field `CFG3_L_RELOAD_DST` writer - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg3LReloadDstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if the channel is inactive. Can be used in conjunction with CFGx_L.CH_SUSP to cleanly disable a channel when destination is peripheral. For more information, refer to \"Disabling a Channel Prior to Transfer Completion\" . 1 = Channel Inactive for axi interface 0 = Channel not Inactive for axi interface"]
    #[inline(always)]
    pub fn cfg3_l_inactive(&self) -> Cfg3LInactiveR {
        Cfg3LInactiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Source bus protocol finish, indicating the current read command has finished."]
    #[inline(always)]
    pub fn cfg3_l_src_pctl_over(&self) -> Cfg3LSrcPctlOverR {
        Cfg3LSrcPctlOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA destination bus protocol finish, indicating the current write command has finished."]
    #[inline(always)]
    pub fn cfg3_l_dst_pctl_over(&self) -> Cfg3LDstPctlOverR {
        Cfg3LDstPctlOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
    #[inline(always)]
    pub fn cfg3_l_ch_prior(&self) -> Cfg3LChPriorR {
        Cfg3LChPriorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
    #[inline(always)]
    pub fn cfg3_l_ch_susp(&self) -> Cfg3LChSuspR {
        Cfg3LChSuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO. 1 = Channel FIFO empty 0 = Channel FIFO not empty"]
    #[inline(always)]
    pub fn cfg3_l_fifo_empty(&self) -> Cfg3LFifoEmptyR {
        Cfg3LFifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg3_l_hs_sel_dst(&self) -> Cfg3LHsSelDstR {
        Cfg3LHsSelDstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg3_l_hs_sel_src(&self) -> Cfg3LHsSelSrcR {
        Cfg3LHsSelSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
    #[inline(always)]
    pub fn cfg3_l_dst_hs_pol(&self) -> Cfg3LDstHsPolR {
        Cfg3LDstHsPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
    #[inline(always)]
    pub fn cfg3_l_src_hs_pol(&self) -> Cfg3LSrcHsPolR {
        Cfg3LSrcHsPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg3_l_reload_src(&self) -> Cfg3LReloadSrcR {
        Cfg3LReloadSrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg3_l_reload_dst(&self) -> Cfg3LReloadDstR {
        Cfg3LReloadDstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
    #[inline(always)]
    pub fn cfg3_l_ch_prior(&mut self) -> Cfg3LChPriorW<'_, Cfg3LSpec> {
        Cfg3LChPriorW::new(self, 4)
    }
    #[doc = "Bit 8 - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
    #[inline(always)]
    pub fn cfg3_l_ch_susp(&mut self) -> Cfg3LChSuspW<'_, Cfg3LSpec> {
        Cfg3LChSuspW::new(self, 8)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg3_l_hs_sel_dst(&mut self) -> Cfg3LHsSelDstW<'_, Cfg3LSpec> {
        Cfg3LHsSelDstW::new(self, 10)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg3_l_hs_sel_src(&mut self) -> Cfg3LHsSelSrcW<'_, Cfg3LSpec> {
        Cfg3LHsSelSrcW::new(self, 11)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
    #[inline(always)]
    pub fn cfg3_l_dst_hs_pol(&mut self) -> Cfg3LDstHsPolW<'_, Cfg3LSpec> {
        Cfg3LDstHsPolW::new(self, 18)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
    #[inline(always)]
    pub fn cfg3_l_src_hs_pol(&mut self) -> Cfg3LSrcHsPolW<'_, Cfg3LSpec> {
        Cfg3LSrcHsPolW::new(self, 19)
    }
    #[doc = "Bit 30 - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg3_l_reload_src(&mut self) -> Cfg3LReloadSrcW<'_, Cfg3LSpec> {
        Cfg3LReloadSrcW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg3_l_reload_dst(&mut self) -> Cfg3LReloadDstW<'_, Cfg3LSpec> {
        Cfg3LReloadDstW::new(self, 31)
    }
}
#[doc = "Channel 3 low-32 bits Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg3_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg3LSpec;
impl crate::RegisterSpec for Cfg3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3_l::R`](R) reader structure"]
impl crate::Readable for Cfg3LSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg3_l::W`](W) writer structure"]
impl crate::Writable for Cfg3LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG3_L to value 0x0e07"]
impl crate::Resettable for Cfg3LSpec {
    const RESET_VALUE: u32 = 0x0e07;
}
