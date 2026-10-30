#[doc = "Register `REG_WMAC_PKTCNT_RWD` reader"]
pub type R = crate::R<RegWmacPktcntRwdSpec>;
#[doc = "Field `BIT_PKTCNT_CNTRST` reader - Write 1 to reset Counter after DRV read all counters , DRV set this bit and HW return 0 after reset"]
pub type BitPktcntCntrstR = crate::BitReader;
#[doc = "Field `BIT_PKTCNT_BSSIDMAP` reader - To decide this counter is for which MBSSID (16 groups), default : 0 for counter 0 1 for counter 1 2 for counter 2 3 for counter 3 4 for counter 4 5 for counter 5 6 for counter 6 7 for counter 7"]
pub type BitPktcntBssidmapR = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - Write 1 to reset Counter after DRV read all counters , DRV set this bit and HW return 0 after reset"]
    #[inline(always)]
    pub fn bit_pktcnt_cntrst(&self) -> BitPktcntCntrstR {
        BitPktcntCntrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - To decide this counter is for which MBSSID (16 groups), default : 0 for counter 0 1 for counter 1 2 for counter 2 3 for counter 3 4 for counter 4 5 for counter 5 6 for counter 6 7 for counter 7"]
    #[inline(always)]
    pub fn bit_pktcnt_bssidmap(&self) -> BitPktcntBssidmapR {
        BitPktcntBssidmapR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_pktcnt_rwd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacPktcntRwdSpec;
impl crate::RegisterSpec for RegWmacPktcntRwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_pktcnt_rwd::R`](R) reader structure"]
impl crate::Readable for RegWmacPktcntRwdSpec {}
#[doc = "`reset()` method sets REG_WMAC_PKTCNT_RWD to value 0"]
impl crate::Resettable for RegWmacPktcntRwdSpec {}
