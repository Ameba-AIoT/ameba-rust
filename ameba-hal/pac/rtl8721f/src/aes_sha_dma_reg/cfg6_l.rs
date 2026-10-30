#[doc = "Register `CFG6_L` reader"]
pub type R = crate::R<Cfg6LSpec>;
#[doc = "Register `CFG6_L` writer"]
pub type W = crate::W<Cfg6LSpec>;
#[doc = "Field `CFG6_L_INACTIVE` reader - Indicates if the channel is inactive. Can be used in conjunction with CFGx_L.CH_SUSP to cleanly disable a channel when destination is peripheral. For more information, refer to \"Disabling a Channel Prior to Transfer Completion\" . 1 = Channel Inactive for axi interface 0 = Channel not Inactive for axi interface"]
pub type Cfg6LInactiveR = crate::BitReader;
#[doc = "Field `CFG6_L_SRC_PCTL_OVER` reader - DMA Source bus protocol finish, indicating the current read command has finished."]
pub type Cfg6LSrcPctlOverR = crate::BitReader;
#[doc = "Field `CFG6_L_DST_PCTL_OVER` reader - DMA destination bus protocol finish, indicating the current write command has finished."]
pub type Cfg6LDstPctlOverR = crate::BitReader;
#[doc = "Field `CFG6_L_CH_PRIOR` reader - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
pub type Cfg6LChPriorR = crate::FieldReader;
#[doc = "Field `CFG6_L_CH_PRIOR` writer - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
pub type Cfg6LChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG6_L_CH_SUSP` reader - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
pub type Cfg6LChSuspR = crate::BitReader;
#[doc = "Field `CFG6_L_CH_SUSP` writer - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
pub type Cfg6LChSuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO. 1 = Channel FIFO empty 0 = Channel FIFO not empty"]
pub type Cfg6LFifoEmptyR = crate::BitReader;
#[doc = "Field `CFG6_L_HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
pub type Cfg6LHsSelDstR = crate::BitReader;
#[doc = "Field `CFG6_L_HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
pub type Cfg6LHsSelDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
pub type Cfg6LHsSelSrcR = crate::BitReader;
#[doc = "Field `CFG6_L_HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
pub type Cfg6LHsSelSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_DST_HS_POL` reader - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
pub type Cfg6LDstHsPolR = crate::BitReader;
#[doc = "Field `CFG6_L_DST_HS_POL` writer - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
pub type Cfg6LDstHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_SRC_HS_POL` reader - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
pub type Cfg6LSrcHsPolR = crate::BitReader;
#[doc = "Field `CFG6_L_SRC_HS_POL` writer - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
pub type Cfg6LSrcHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_RELOAD_SRC` reader - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg6LReloadSrcR = crate::BitReader;
#[doc = "Field `CFG6_L_RELOAD_SRC` writer - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg6LReloadSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG6_L_RELOAD_DST` reader - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg6LReloadDstR = crate::BitReader;
#[doc = "Field `CFG6_L_RELOAD_DST` writer - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
pub type Cfg6LReloadDstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if the channel is inactive. Can be used in conjunction with CFGx_L.CH_SUSP to cleanly disable a channel when destination is peripheral. For more information, refer to \"Disabling a Channel Prior to Transfer Completion\" . 1 = Channel Inactive for axi interface 0 = Channel not Inactive for axi interface"]
    #[inline(always)]
    pub fn cfg6_l_inactive(&self) -> Cfg6LInactiveR {
        Cfg6LInactiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Source bus protocol finish, indicating the current read command has finished."]
    #[inline(always)]
    pub fn cfg6_l_src_pctl_over(&self) -> Cfg6LSrcPctlOverR {
        Cfg6LSrcPctlOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA destination bus protocol finish, indicating the current write command has finished."]
    #[inline(always)]
    pub fn cfg6_l_dst_pctl_over(&self) -> Cfg6LDstPctlOverR {
        Cfg6LDstPctlOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
    #[inline(always)]
    pub fn cfg6_l_ch_prior(&self) -> Cfg6LChPriorR {
        Cfg6LChPriorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
    #[inline(always)]
    pub fn cfg6_l_ch_susp(&self) -> Cfg6LChSuspR {
        Cfg6LChSuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO. 1 = Channel FIFO empty 0 = Channel FIFO not empty"]
    #[inline(always)]
    pub fn cfg6_l_fifo_empty(&self) -> Cfg6LFifoEmptyR {
        Cfg6LFifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg6_l_hs_sel_dst(&self) -> Cfg6LHsSelDstR {
        Cfg6LHsSelDstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg6_l_hs_sel_src(&self) -> Cfg6LHsSelSrcR {
        Cfg6LHsSelSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
    #[inline(always)]
    pub fn cfg6_l_dst_hs_pol(&self) -> Cfg6LDstHsPolR {
        Cfg6LDstHsPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
    #[inline(always)]
    pub fn cfg6_l_src_hs_pol(&self) -> Cfg6LSrcHsPolR {
        Cfg6LSrcHsPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg6_l_reload_src(&self) -> Cfg6LReloadSrcR {
        Cfg6LReloadSrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg6_l_reload_dst(&self) -> Cfg6LReloadDstR {
        Cfg6LReloadDstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Channel priority. RTK DMAC supports three kinds of channel priority now, fixed priority, round robin priority and programmable priority. The default way is fixed priority to reduce area, and channel0 has the highest priority, channel15 has the lowest priority. Changing the definition of parameter FIX_PRIORITY or Programmable_PRIORITY can change priority way. If RTK DMAC is configured to support Programmable priority, this field will be used to set priority between channels. CH_PRIOR's legal value is 0 ~ (DMAC_NUM_CHANNELS-1), and 0 is the highest priority value, (DMAC_NUM_CHANNELS-1) is the lowest priroty value. If more than one channel are set with the same priroty value, the channel with small channel number index has higher priority. If DMAC is configured to support fix priority or round-robin priority, this field is not exist. Note: if user not define ProgrammablePriority, it only can be readed."]
    #[inline(always)]
    pub fn cfg6_l_ch_prior(&mut self) -> Cfg6LChPriorW<'_, Cfg6LSpec> {
        Cfg6LChPriorW::new(self, 4)
    }
    #[doc = "Bit 8 - Channel Suspend. Suspends all DMA data transfers from the source until this bit is cleared. There is no guarantee that the current transaction will complete. Can also be used in conjunction with CFGx_l.INACTIVE to cleanly disable a channel without losing any data. 0 = Not suspended 1 = Suspend DMA transfer from the source. This field will be reset to 0 after \"Disabling a Channel Prior to Transfer Completion\" automatically by hardware.For more information, refer to \"7.7 Disabling a Channel Prior to Transfer Completion\" . Note: if user not define DMAC_CH*_SUPPORT_SUSPEND, it only can be readed."]
    #[inline(always)]
    pub fn cfg6_l_ch_susp(&mut self) -> Cfg6LChSuspW<'_, Cfg6LSpec> {
        Cfg6LChSuspW::new(self, 8)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for destination requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware- initiated transaction requests are ignored. If the destination peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg6_l_hs_sel_dst(&mut self) -> Cfg6LHsSelDstW<'_, Cfg6LSpec> {
        Cfg6LHsSelDstW::new(self, 10)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select. This register selects which of the handshaking interfaces - hardware or software - is active for source requests on this channel. 0 = Hardware handshaking interface. Software-initiated transaction requests are ignored. 1 = Software handshaking interface. Hardware-initiated transaction requests are ignored. If the source peripheral is memory, then this bit is ignored."]
    #[inline(always)]
    pub fn cfg6_l_hs_sel_src(&mut self) -> Cfg6LHsSelSrcW<'_, Cfg6LSpec> {
        Cfg6LHsSelSrcW::new(self, 11)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity. 0 = Active high 1 = Active low For information on this, refer to \"Handshaking Interface\" ."]
    #[inline(always)]
    pub fn cfg6_l_dst_hs_pol(&mut self) -> Cfg6LDstHsPolW<'_, Cfg6LSpec> {
        Cfg6LDstHsPolW::new(self, 18)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity. 0 = Active high 1 = Active low"]
    #[inline(always)]
    pub fn cfg6_l_src_hs_pol(&mut self) -> Cfg6LSrcHsPolW<'_, Cfg6LSpec> {
        Cfg6LSrcHsPolW::new(self, 19)
    }
    #[doc = "Bit 30 - Automatic Source Reload. The SARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated. This field does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg6_l_reload_src(&mut self) -> Cfg6LReloadSrcW<'_, Cfg6LSpec> {
        Cfg6LReloadSrcW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatic Destination Reload. The DARx register can be automatically reloaded from its initial value at the end of every block for multi-block transfers. A new block transfer is then initiated.This register does not exist if the configuration parameter DMAC_CHx_MULTI_BLK_EN is not selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn cfg6_l_reload_dst(&mut self) -> Cfg6LReloadDstW<'_, Cfg6LSpec> {
        Cfg6LReloadDstW::new(self, 31)
    }
}
#[doc = "Channel 6 low-32 bits Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg6_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg6_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg6LSpec;
impl crate::RegisterSpec for Cfg6LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg6_l::R`](R) reader structure"]
impl crate::Readable for Cfg6LSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg6_l::W`](W) writer structure"]
impl crate::Writable for Cfg6LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG6_L to value 0x0e07"]
impl crate::Resettable for Cfg6LSpec {
    const RESET_VALUE: u32 = 0x0e07;
}
