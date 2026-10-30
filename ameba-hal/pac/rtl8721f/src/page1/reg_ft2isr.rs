#[doc = "Register `REG_FT2ISR` reader"]
pub type R = crate::R<RegFt2isrSpec>;
#[doc = "Field `BIT_FS_NOA0_TSF_BIT32_INT` reader - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa0TsfBit32IntR = crate::BitReader;
#[doc = "Field `BIT_FS_NOA1_TSFT_BIT32_INT` reader - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa1TsftBit32IntR = crate::BitReader;
#[doc = "Field `BIT_FS_NOA2_TSFT_BIT32_INT` reader - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa2TsftBit32IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF0_INT` reader - P2P NoA RF off time indication 0 interrupt"]
pub type BitFsP2pRfoff0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON0_INT` reader - P2P NoA RF on time indication 0 interrupt"]
pub type BitFsP2pRfon0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF1_INT` reader - P2P NoA RF off time indication 1 interrupt"]
pub type BitFsP2pRfoff1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON1_INT` reader - P2P NoA RF on time indication 1 interrupt"]
pub type BitFsP2pRfon1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF2_INT` reader - P2P NoA RF off time indication 2 interrupt"]
pub type BitFsP2pRfoff2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON2_INT` reader - P2P NoA RF on time indication 2 interrupt"]
pub type BitFsP2pRfon2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd0_INT` reader - CTWindow End NOA0 interrupt."]
pub type BitFsCtwend0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd1_INT` reader - CTWindow End NOA1 interrupt."]
pub type BitFsCtwend1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd2_INT` reader - CTWindow End NOA2 interrupt."]
pub type BitFsCtwend2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT0` reader - port0 CTWindow end interrupt for p2p."]
pub type BitFsCtweInt0R = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT1` reader - port1 CTWindow end interrupt for p2p."]
pub type BitFsCtweInt1R = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT2` reader - port2 CTWindow end interrupt for p2p."]
pub type BitFsCtweInt2R = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa0_tsf_bit32_int(&self) -> BitFsNoa0TsfBit32IntR {
        BitFsNoa0TsfBit32IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa1_tsft_bit32_int(&self) -> BitFsNoa1TsftBit32IntR {
        BitFsNoa1TsftBit32IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa2_tsft_bit32_int(&self) -> BitFsNoa2TsftBit32IntR {
        BitFsNoa2TsftBit32IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff0_int(&self) -> BitFsP2pRfoff0IntR {
        BitFsP2pRfoff0IntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon0_int(&self) -> BitFsP2pRfon0IntR {
        BitFsP2pRfon0IntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff1_int(&self) -> BitFsP2pRfoff1IntR {
        BitFsP2pRfoff1IntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon1_int(&self) -> BitFsP2pRfon1IntR {
        BitFsP2pRfon1IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff2_int(&self) -> BitFsP2pRfoff2IntR {
        BitFsP2pRfoff2IntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon2_int(&self) -> BitFsP2pRfon2IntR {
        BitFsP2pRfon2IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend0_int(&self) -> BitFsCtwend0IntR {
        BitFsCtwend0IntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend1_int(&self) -> BitFsCtwend1IntR {
        BitFsCtwend1IntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend2_int(&self) -> BitFsCtwend2IntR {
        BitFsCtwend2IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port0 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int0(&self) -> BitFsCtweInt0R {
        BitFsCtweInt0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port1 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int1(&self) -> BitFsCtweInt1R {
        BitFsCtweInt1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int2(&self) -> BitFsCtweInt2R {
        BitFsCtweInt2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ft2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFt2isrSpec;
impl crate::RegisterSpec for RegFt2isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ft2isr::R`](R) reader structure"]
impl crate::Readable for RegFt2isrSpec {}
#[doc = "`reset()` method sets REG_FT2ISR to value 0"]
impl crate::Resettable for RegFt2isrSpec {}
