#[doc = "Register `REG_WMAC_PLCP_MONITOR` reader"]
pub type R = crate::R<RegWmacPlcpMonitorSpec>;
#[doc = "Register `REG_WMAC_PLCP_MONITOR` writer"]
pub type W = crate::W<RegWmacPlcpMonitorSpec>;
#[doc = "Field `BIT_WMAC_PLCP_RDSIG` reader - Selected signal field to read out"]
pub type BitWmacPlcpRdsigR = crate::FieldReader<u32>;
#[doc = "Field `BIT_WMAC_RATE_IDX` reader - PPDU type 0:Long PLCP, 1:Short PLCP, 2: Legacy, 3: HT_MF, 4:HT_GF, 5: VHT-SU, 6: VHT-MU, 7: HE-SU, 8: HE-ER_SU, 9: HE_MU, 10: HE_TB, 11~15:RSVD U(4,0)"]
pub type BitWmacRateIdxR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_PLCP_RDSIG_SEL` reader - Selection of plcp header signal field 0:LSIG/CCK PLCP 1:HTSIG1/VHTSIGA1/HESIGA1 2:HTSIG2/VHTSIGA2/HESIGA2 3:VHTSIGB/HESIGB, low 4bit is dummy 4: HE USER FIELD 5: SERVICE"]
pub type BitWmacPlcpRdsigSelR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_PLCP_RDSIG_SEL` writer - Selection of plcp header signal field 0:LSIG/CCK PLCP 1:HTSIG1/VHTSIGA1/HESIGA1 2:HTSIG2/VHTSIGA2/HESIGA2 3:VHTSIGB/HESIGB, low 4bit is dummy 4: HE USER FIELD 5: SERVICE"]
pub type BitWmacPlcpRdsigSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WMAC_PLCP_TRX_SEL` reader - Selection of Tx/Rx plcp header 0:Rx 1:Tx"]
pub type BitWmacPlcpTrxSelR = crate::BitReader;
#[doc = "Field `BIT_WMAC_PLCP_TRX_SEL` writer - Selection of Tx/Rx plcp header 0:Rx 1:Tx"]
pub type BitWmacPlcpTrxSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Selected signal field to read out"]
    #[inline(always)]
    pub fn bit_wmac_plcp_rdsig(&self) -> BitWmacPlcpRdsigR {
        BitWmacPlcpRdsigR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - PPDU type 0:Long PLCP, 1:Short PLCP, 2: Legacy, 3: HT_MF, 4:HT_GF, 5: VHT-SU, 6: VHT-MU, 7: HE-SU, 8: HE-ER_SU, 9: HE_MU, 10: HE_TB, 11~15:RSVD U(4,0)"]
    #[inline(always)]
    pub fn bit_wmac_rate_idx(&self) -> BitWmacRateIdxR {
        BitWmacRateIdxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Selection of plcp header signal field 0:LSIG/CCK PLCP 1:HTSIG1/VHTSIGA1/HESIGA1 2:HTSIG2/VHTSIGA2/HESIGA2 3:VHTSIGB/HESIGB, low 4bit is dummy 4: HE USER FIELD 5: SERVICE"]
    #[inline(always)]
    pub fn bit_wmac_plcp_rdsig_sel(&self) -> BitWmacPlcpRdsigSelR {
        BitWmacPlcpRdsigSelR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Selection of Tx/Rx plcp header 0:Rx 1:Tx"]
    #[inline(always)]
    pub fn bit_wmac_plcp_trx_sel(&self) -> BitWmacPlcpTrxSelR {
        BitWmacPlcpTrxSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - Selection of plcp header signal field 0:LSIG/CCK PLCP 1:HTSIG1/VHTSIGA1/HESIGA1 2:HTSIG2/VHTSIGA2/HESIGA2 3:VHTSIGB/HESIGB, low 4bit is dummy 4: HE USER FIELD 5: SERVICE"]
    #[inline(always)]
    pub fn bit_wmac_plcp_rdsig_sel(&mut self) -> BitWmacPlcpRdsigSelW<'_, RegWmacPlcpMonitorSpec> {
        BitWmacPlcpRdsigSelW::new(self, 28)
    }
    #[doc = "Bit 31 - Selection of Tx/Rx plcp header 0:Rx 1:Tx"]
    #[inline(always)]
    pub fn bit_wmac_plcp_trx_sel(&mut self) -> BitWmacPlcpTrxSelW<'_, RegWmacPlcpMonitorSpec> {
        BitWmacPlcpTrxSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_plcp_monitor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_plcp_monitor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacPlcpMonitorSpec;
impl crate::RegisterSpec for RegWmacPlcpMonitorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_plcp_monitor::R`](R) reader structure"]
impl crate::Readable for RegWmacPlcpMonitorSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_plcp_monitor::W`](W) writer structure"]
impl crate::Writable for RegWmacPlcpMonitorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_PLCP_MONITOR to value 0"]
impl crate::Resettable for RegWmacPlcpMonitorSpec {}
