#[doc = "Register `REG_RX_FILTER_FUNCTION` reader"]
pub type R = crate::R<RegRxFilterFunctionSpec>;
#[doc = "Register `REG_RX_FILTER_FUNCTION` writer"]
pub type W = crate::W<RegRxFilterFunctionSpec>;
#[doc = "Field `BIT_r_WMAC_RX_FIL_LEN_2` reader - use with BIT_r_WMAC_FIL_CTLPKTLEN"]
pub type BitRWmacRxFilLen2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_RX_FIL_LEN_2` writer - use with BIT_r_WMAC_FIL_CTLPKTLEN"]
pub type BitRWmacRxFilLen2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_r_WMAC_RXFIL_MASKM` reader - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacRxfilMaskmR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXFIL_MASKM` writer - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacRxfilMaskmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXFIL_MESH_DEL` reader - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacRxfilMeshDelR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXFIL_MESH_DEL` writer - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacRxfilMeshDelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXFIL_FCSE` reader - If rx pkt is not ampdu and rejected, reset rxpkt_ctl FSM immediately after parsing header"]
pub type BitRWmacRxfilFcseR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXFIL_FCSE` writer - If rx pkt is not ampdu and rejected, reset rxpkt_ctl FSM immediately after parsing header"]
pub type BitRWmacRxfilFcseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXFIL_RDEL` reader - If new rx pkt is coming, reset the rxpkt_ctl FSM after parsing plcp."]
pub type BitRWmacRxfilRdelR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXFIL_RDEL` writer - If new rx pkt is coming, reset the rxpkt_ctl FSM after parsing plcp."]
pub type BitRWmacRxfilRdelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_CLRRXSEC` reader - If new rx pkt is coming, reset the previous rx key search result."]
pub type BitRWmacClrrxsecR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CLRRXSEC` writer - If new rx pkt is coming, reset the previous rx key search result."]
pub type BitRWmacClrrxsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_MPDURDY_CLR` reader - when the bit is set 1, the last MPDU which will be transferred to rxpktfifo is ready and error occurs, mpdu_rdy signal is also valid(is means all MPDU data has been transferred to rxpktbuffer)"]
pub type BitRWmacMpdurdyClrR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_MPDURDY_CLR` writer - when the bit is set 1, the last MPDU which will be transferred to rxpktfifo is ready and error occurs, mpdu_rdy signal is also valid(is means all MPDU data has been transferred to rxpktbuffer)"]
pub type BitRWmacMpdurdyClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXFIL_REND` reader - when this bit set 1,icvmic_valid signal is cleared when icvmic_rdy is valid(ICV/MIC is parsed)"]
pub type BitRWmacRxfilRendR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXFIL_REND` writer - when this bit set 1,icvmic_valid signal is cleared when icvmic_rdy is valid(ICV/MIC is parsed)"]
pub type BitRWmacRxfilRendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_LATCH_MACHRDY` reader - when this bit set 1,mach_rdy signal is latched to make sure that rxpktfifo_ctl can detect it"]
pub type BitRLatchMachrdyR = crate::BitReader;
#[doc = "Field `BIT_r_LATCH_MACHRDY` writer - when this bit set 1,mach_rdy signal is latched to make sure that rxpktfifo_ctl can detect it"]
pub type BitRLatchMachrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_RXPKTCTL_FSM_BASED_MPDURDY` reader - when this bit is set 1, rxpktfifo_ctl FSM jumps based on mpdu_valid(all MPDU data has been transferred to rxpktbuffer), when this bit is set 0, rxpktfifo_ctl FSM jumps based on fcs_valid(all MPDU data has been parsed by rx_parser)"]
pub type BitRRxpktctlFsmBasedMpdurdyR = crate::BitReader;
#[doc = "Field `BIT_r_RXPKTCTL_FSM_BASED_MPDURDY` writer - when this bit is set 1, rxpktfifo_ctl FSM jumps based on mpdu_valid(all MPDU data has been transferred to rxpktbuffer), when this bit is set 0, rxpktfifo_ctl FSM jumps based on fcs_valid(all MPDU data has been parsed by rx_parser)"]
pub type BitRRxpktctlFsmBasedMpdurdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_REAPTER_ADDR_MATCH` reader - when this bit is set 1, physical address is matched when rx A1 is equal to tx A2, used in repeater mode"]
pub type BitRReapterAddrMatchR = crate::BitReader;
#[doc = "Field `BIT_r_REAPTER_ADDR_MATCH` writer - when this bit is set 1, physical address is matched when rx A1 is equal to tx A2, used in repeater mode"]
pub type BitRReapterAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_CHK_DELIMIT_LEN` reader - Drop mpdu if delimeter mpdu length smaller than mac header length + 4."]
pub type BitRChkDelimitLenR = crate::BitReader;
#[doc = "Field `BIT_r_CHK_DELIMIT_LEN` writer - Drop mpdu if delimeter mpdu length smaller than mac header length + 4."]
pub type BitRChkDelimitLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_DIS_VHT_PLCP_CHK_MU` reader - Disable check sigb crc value of MU plcp header."]
pub type BitWmacDisVhtPlcpChkMuR = crate::BitReader;
#[doc = "Field `BIT_WMAC_DIS_VHT_PLCP_CHK_MU` writer - Disable check sigb crc value of MU plcp header."]
pub type BitWmacDisVhtPlcpChkMuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_RXPKTCTL_FSM_BASED_MPDURDY1` reader - Adjust mpdu_rdy timing to solve FSM behavier error."]
pub type BitRRxpktctlFsmBasedMpdurdy1R = crate::BitReader;
#[doc = "Field `BIT_r_RXPKTCTL_FSM_BASED_MPDURDY1` writer - Adjust mpdu_rdy timing to solve FSM behavier error."]
pub type BitRRxpktctlFsmBasedMpdurdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_MHRDDY_CLR` reader - Reset mach_rdy_latch when rx parser begin parsing mac header."]
pub type BitRWmacMhrddyClrR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_MHRDDY_CLR` writer - Reset mach_rdy_latch when rx parser begin parsing mac header."]
pub type BitRWmacMhrddyClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_MHRDDY_LATCH` reader - Adjust mach_rdy timing if rx_parser and rxpktfifo FSM is not sync."]
pub type BitRWmacMhrddyLatchR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_MHRDDY_LATCH` writer - Adjust mach_rdy timing if rx_parser and rxpktfifo FSM is not sync."]
pub type BitRWmacMhrddyLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXHANG_EN` reader - Force read error content from fifo to solve rxhang."]
pub type BitRxhangEnR = crate::BitReader;
#[doc = "Field `BIT_RXHANG_EN` writer - Force read error content from fifo to solve rxhang."]
pub type BitRxhangEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - use with BIT_r_WMAC_FIL_CTLPKTLEN"]
    #[inline(always)]
    pub fn bit_r_wmac_rx_fil_len_2(&self) -> BitRWmacRxFilLen2R {
        BitRWmacRxFilLen2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_maskm(&self) -> BitRWmacRxfilMaskmR {
        BitRWmacRxfilMaskmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_mesh_del(&self) -> BitRWmacRxfilMeshDelR {
        BitRWmacRxfilMeshDelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If rx pkt is not ampdu and rejected, reset rxpkt_ctl FSM immediately after parsing header"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_fcse(&self) -> BitRWmacRxfilFcseR {
        BitRWmacRxfilFcseR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If new rx pkt is coming, reset the rxpkt_ctl FSM after parsing plcp."]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_rdel(&self) -> BitRWmacRxfilRdelR {
        BitRWmacRxfilRdelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If new rx pkt is coming, reset the previous rx key search result."]
    #[inline(always)]
    pub fn bit_r_wmac_clrrxsec(&self) -> BitRWmacClrrxsecR {
        BitRWmacClrrxsecR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - when the bit is set 1, the last MPDU which will be transferred to rxpktfifo is ready and error occurs, mpdu_rdy signal is also valid(is means all MPDU data has been transferred to rxpktbuffer)"]
    #[inline(always)]
    pub fn bit_r_wmac_mpdurdy_clr(&self) -> BitRWmacMpdurdyClrR {
        BitRWmacMpdurdyClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - when this bit set 1,icvmic_valid signal is cleared when icvmic_rdy is valid(ICV/MIC is parsed)"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_rend(&self) -> BitRWmacRxfilRendR {
        BitRWmacRxfilRendR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - when this bit set 1,mach_rdy signal is latched to make sure that rxpktfifo_ctl can detect it"]
    #[inline(always)]
    pub fn bit_r_latch_machrdy(&self) -> BitRLatchMachrdyR {
        BitRLatchMachrdyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - when this bit is set 1, rxpktfifo_ctl FSM jumps based on mpdu_valid(all MPDU data has been transferred to rxpktbuffer), when this bit is set 0, rxpktfifo_ctl FSM jumps based on fcs_valid(all MPDU data has been parsed by rx_parser)"]
    #[inline(always)]
    pub fn bit_r_rxpktctl_fsm_based_mpdurdy(&self) -> BitRRxpktctlFsmBasedMpdurdyR {
        BitRRxpktctlFsmBasedMpdurdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - when this bit is set 1, physical address is matched when rx A1 is equal to tx A2, used in repeater mode"]
    #[inline(always)]
    pub fn bit_r_reapter_addr_match(&self) -> BitRReapterAddrMatchR {
        BitRReapterAddrMatchR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Drop mpdu if delimeter mpdu length smaller than mac header length + 4."]
    #[inline(always)]
    pub fn bit_r_chk_delimit_len(&self) -> BitRChkDelimitLenR {
        BitRChkDelimitLenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable check sigb crc value of MU plcp header."]
    #[inline(always)]
    pub fn bit_wmac_dis_vht_plcp_chk_mu(&self) -> BitWmacDisVhtPlcpChkMuR {
        BitWmacDisVhtPlcpChkMuR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Adjust mpdu_rdy timing to solve FSM behavier error."]
    #[inline(always)]
    pub fn bit_r_rxpktctl_fsm_based_mpdurdy1(&self) -> BitRRxpktctlFsmBasedMpdurdy1R {
        BitRRxpktctlFsmBasedMpdurdy1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset mach_rdy_latch when rx parser begin parsing mac header."]
    #[inline(always)]
    pub fn bit_r_wmac_mhrddy_clr(&self) -> BitRWmacMhrddyClrR {
        BitRWmacMhrddyClrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Adjust mach_rdy timing if rx_parser and rxpktfifo FSM is not sync."]
    #[inline(always)]
    pub fn bit_r_wmac_mhrddy_latch(&self) -> BitRWmacMhrddyLatchR {
        BitRWmacMhrddyLatchR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Force read error content from fifo to solve rxhang."]
    #[inline(always)]
    pub fn bit_rxhang_en(&self) -> BitRxhangEnR {
        BitRxhangEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - use with BIT_r_WMAC_FIL_CTLPKTLEN"]
    #[inline(always)]
    pub fn bit_r_wmac_rx_fil_len_2(&mut self) -> BitRWmacRxFilLen2W<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxFilLen2W::new(self, 0)
    }
    #[doc = "Bit 16 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_maskm(&mut self) -> BitRWmacRxfilMaskmW<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxfilMaskmW::new(self, 16)
    }
    #[doc = "Bit 17 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_mesh_del(
        &mut self,
    ) -> BitRWmacRxfilMeshDelW<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxfilMeshDelW::new(self, 17)
    }
    #[doc = "Bit 18 - If rx pkt is not ampdu and rejected, reset rxpkt_ctl FSM immediately after parsing header"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_fcse(&mut self) -> BitRWmacRxfilFcseW<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxfilFcseW::new(self, 18)
    }
    #[doc = "Bit 19 - If new rx pkt is coming, reset the rxpkt_ctl FSM after parsing plcp."]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_rdel(&mut self) -> BitRWmacRxfilRdelW<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxfilRdelW::new(self, 19)
    }
    #[doc = "Bit 20 - If new rx pkt is coming, reset the previous rx key search result."]
    #[inline(always)]
    pub fn bit_r_wmac_clrrxsec(&mut self) -> BitRWmacClrrxsecW<'_, RegRxFilterFunctionSpec> {
        BitRWmacClrrxsecW::new(self, 20)
    }
    #[doc = "Bit 21 - when the bit is set 1, the last MPDU which will be transferred to rxpktfifo is ready and error occurs, mpdu_rdy signal is also valid(is means all MPDU data has been transferred to rxpktbuffer)"]
    #[inline(always)]
    pub fn bit_r_wmac_mpdurdy_clr(&mut self) -> BitRWmacMpdurdyClrW<'_, RegRxFilterFunctionSpec> {
        BitRWmacMpdurdyClrW::new(self, 21)
    }
    #[doc = "Bit 22 - when this bit set 1,icvmic_valid signal is cleared when icvmic_rdy is valid(ICV/MIC is parsed)"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfil_rend(&mut self) -> BitRWmacRxfilRendW<'_, RegRxFilterFunctionSpec> {
        BitRWmacRxfilRendW::new(self, 22)
    }
    #[doc = "Bit 23 - when this bit set 1,mach_rdy signal is latched to make sure that rxpktfifo_ctl can detect it"]
    #[inline(always)]
    pub fn bit_r_latch_machrdy(&mut self) -> BitRLatchMachrdyW<'_, RegRxFilterFunctionSpec> {
        BitRLatchMachrdyW::new(self, 23)
    }
    #[doc = "Bit 24 - when this bit is set 1, rxpktfifo_ctl FSM jumps based on mpdu_valid(all MPDU data has been transferred to rxpktbuffer), when this bit is set 0, rxpktfifo_ctl FSM jumps based on fcs_valid(all MPDU data has been parsed by rx_parser)"]
    #[inline(always)]
    pub fn bit_r_rxpktctl_fsm_based_mpdurdy(
        &mut self,
    ) -> BitRRxpktctlFsmBasedMpdurdyW<'_, RegRxFilterFunctionSpec> {
        BitRRxpktctlFsmBasedMpdurdyW::new(self, 24)
    }
    #[doc = "Bit 25 - when this bit is set 1, physical address is matched when rx A1 is equal to tx A2, used in repeater mode"]
    #[inline(always)]
    pub fn bit_r_reapter_addr_match(
        &mut self,
    ) -> BitRReapterAddrMatchW<'_, RegRxFilterFunctionSpec> {
        BitRReapterAddrMatchW::new(self, 25)
    }
    #[doc = "Bit 26 - Drop mpdu if delimeter mpdu length smaller than mac header length + 4."]
    #[inline(always)]
    pub fn bit_r_chk_delimit_len(&mut self) -> BitRChkDelimitLenW<'_, RegRxFilterFunctionSpec> {
        BitRChkDelimitLenW::new(self, 26)
    }
    #[doc = "Bit 27 - Disable check sigb crc value of MU plcp header."]
    #[inline(always)]
    pub fn bit_wmac_dis_vht_plcp_chk_mu(
        &mut self,
    ) -> BitWmacDisVhtPlcpChkMuW<'_, RegRxFilterFunctionSpec> {
        BitWmacDisVhtPlcpChkMuW::new(self, 27)
    }
    #[doc = "Bit 28 - Adjust mpdu_rdy timing to solve FSM behavier error."]
    #[inline(always)]
    pub fn bit_r_rxpktctl_fsm_based_mpdurdy1(
        &mut self,
    ) -> BitRRxpktctlFsmBasedMpdurdy1W<'_, RegRxFilterFunctionSpec> {
        BitRRxpktctlFsmBasedMpdurdy1W::new(self, 28)
    }
    #[doc = "Bit 29 - Reset mach_rdy_latch when rx parser begin parsing mac header."]
    #[inline(always)]
    pub fn bit_r_wmac_mhrddy_clr(&mut self) -> BitRWmacMhrddyClrW<'_, RegRxFilterFunctionSpec> {
        BitRWmacMhrddyClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Adjust mach_rdy timing if rx_parser and rxpktfifo FSM is not sync."]
    #[inline(always)]
    pub fn bit_r_wmac_mhrddy_latch(&mut self) -> BitRWmacMhrddyLatchW<'_, RegRxFilterFunctionSpec> {
        BitRWmacMhrddyLatchW::new(self, 30)
    }
    #[doc = "Bit 31 - Force read error content from fifo to solve rxhang."]
    #[inline(always)]
    pub fn bit_rxhang_en(&mut self) -> BitRxhangEnW<'_, RegRxFilterFunctionSpec> {
        BitRxhangEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_filter_function::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_filter_function::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxFilterFunctionSpec;
impl crate::RegisterSpec for RegRxFilterFunctionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_filter_function::R`](R) reader structure"]
impl crate::Readable for RegRxFilterFunctionSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_filter_function::W`](W) writer structure"]
impl crate::Writable for RegRxFilterFunctionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_FILTER_FUNCTION to value 0x3081_0041"]
impl crate::Resettable for RegRxFilterFunctionSpec {
    const RESET_VALUE: u32 = 0x3081_0041;
}
