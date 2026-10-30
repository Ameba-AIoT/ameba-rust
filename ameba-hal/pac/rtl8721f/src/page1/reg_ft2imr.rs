#[doc = "Register `REG_FT2IMR` reader"]
pub type R = crate::R<RegFt2imrSpec>;
#[doc = "Register `REG_FT2IMR` writer"]
pub type W = crate::W<RegFt2imrSpec>;
#[doc = "Field `BIT_FS_NOA0_TSF_BIT32_INT_EN` reader - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa0TsfBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_NOA0_TSF_BIT32_INT_EN` writer - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa0TsfBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_NOA1_TSFT_BIT32_INT_EN` reader - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa1TsftBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_NOA1_TSFT_BIT32_INT_EN` writer - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa1TsftBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_NOA2_TSFT_BIT32_INT_EN` reader - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa2TsftBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_NOA2_TSFT_BIT32_INT_EN` writer - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitFsNoa2TsftBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFOFF0_INT_EN` reader - P2P NoA RF off time indication 0 interrupt"]
pub type BitFsP2pRfoff0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF0_INT_EN` writer - P2P NoA RF off time indication 0 interrupt"]
pub type BitFsP2pRfoff0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFON0_INT_EN` reader - P2P NoA RF on time indication 0 interrupt"]
pub type BitFsP2pRfon0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON0_INT_EN` writer - P2P NoA RF on time indication 0 interrupt"]
pub type BitFsP2pRfon0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFOFF1_INT_EN` reader - P2P NoA RF off time indication 1 interrupt"]
pub type BitFsP2pRfoff1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF1_INT_EN` writer - P2P NoA RF off time indication 1 interrupt"]
pub type BitFsP2pRfoff1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFON1_INT_EN` reader - P2P NoA RF on time indication 1 interrupt"]
pub type BitFsP2pRfon1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON1_INT_EN` writer - P2P NoA RF on time indication 1 interrupt"]
pub type BitFsP2pRfon1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFOFF2_INT_EN` reader - P2P NoA RF off time indication 2 interrupt"]
pub type BitFsP2pRfoff2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFOFF2_INT_EN` writer - P2P NoA RF off time indication 2 interrupt"]
pub type BitFsP2pRfoff2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_P2P_RFON2_INT_EN` reader - P2P NoA RF on time indication 2 interrupt"]
pub type BitFsP2pRfon2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_P2P_RFON2_INT_EN` writer - P2P NoA RF on time indication 2 interrupt"]
pub type BitFsP2pRfon2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWEnd0_INT_EN` reader - CTWindow End NOA0 interrupt."]
pub type BitFsCtwend0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd0_INT_EN` writer - CTWindow End NOA0 interrupt."]
pub type BitFsCtwend0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWEnd1_INT_EN` reader - CTWindow End NOA1 interrupt."]
pub type BitFsCtwend1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd1_INT_EN` writer - CTWindow End NOA1 interrupt."]
pub type BitFsCtwend1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWEnd2_INT_EN` reader - CTWindow End NOA2 interrupt."]
pub type BitFsCtwend2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWEnd2_INT_EN` writer - CTWindow End NOA2 interrupt."]
pub type BitFsCtwend2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWE_INT_EN1` reader - port1 CTWindow end interrupt for p2p."]
pub type BitFsCtweIntEn1R = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT_EN1` writer - port1 CTWindow end interrupt for p2p."]
pub type BitFsCtweIntEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWE_INT_EN0` reader - port0 CTWindow end interrupt for p2p."]
pub type BitFsCtweIntEn0R = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT_EN0` writer - port0 CTWindow end interrupt for p2p."]
pub type BitFsCtweIntEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CTWE_INT2_EN` reader - port2 CTWindow end interrupt for p2p."]
pub type BitFsCtweInt2EnR = crate::BitReader;
#[doc = "Field `BIT_FS_CTWE_INT2_EN` writer - port2 CTWindow end interrupt for p2p."]
pub type BitFsCtweInt2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa0_tsf_bit32_int_en(&self) -> BitFsNoa0TsfBit32IntEnR {
        BitFsNoa0TsfBit32IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa1_tsft_bit32_int_en(&self) -> BitFsNoa1TsftBit32IntEnR {
        BitFsNoa1TsftBit32IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa2_tsft_bit32_int_en(&self) -> BitFsNoa2TsftBit32IntEnR {
        BitFsNoa2TsftBit32IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff0_int_en(&self) -> BitFsP2pRfoff0IntEnR {
        BitFsP2pRfoff0IntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon0_int_en(&self) -> BitFsP2pRfon0IntEnR {
        BitFsP2pRfon0IntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff1_int_en(&self) -> BitFsP2pRfoff1IntEnR {
        BitFsP2pRfoff1IntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon1_int_en(&self) -> BitFsP2pRfon1IntEnR {
        BitFsP2pRfon1IntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff2_int_en(&self) -> BitFsP2pRfoff2IntEnR {
        BitFsP2pRfoff2IntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon2_int_en(&self) -> BitFsP2pRfon2IntEnR {
        BitFsP2pRfon2IntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend0_int_en(&self) -> BitFsCtwend0IntEnR {
        BitFsCtwend0IntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend1_int_en(&self) -> BitFsCtwend1IntEnR {
        BitFsCtwend1IntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend2_int_en(&self) -> BitFsCtwend2IntEnR {
        BitFsCtwend2IntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port1 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int_en1(&self) -> BitFsCtweIntEn1R {
        BitFsCtweIntEn1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port0 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int_en0(&self) -> BitFsCtweIntEn0R {
        BitFsCtweIntEn0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int2_en(&self) -> BitFsCtweInt2EnR {
        BitFsCtweInt2EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa0_tsf_bit32_int_en(&mut self) -> BitFsNoa0TsfBit32IntEnW<'_, RegFt2imrSpec> {
        BitFsNoa0TsfBit32IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa1_tsft_bit32_int_en(&mut self) -> BitFsNoa1TsftBit32IntEnW<'_, RegFt2imrSpec> {
        BitFsNoa1TsftBit32IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_fs_noa2_tsft_bit32_int_en(&mut self) -> BitFsNoa2TsftBit32IntEnW<'_, RegFt2imrSpec> {
        BitFsNoa2TsftBit32IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff0_int_en(&mut self) -> BitFsP2pRfoff0IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfoff0IntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon0_int_en(&mut self) -> BitFsP2pRfon0IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfon0IntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff1_int_en(&mut self) -> BitFsP2pRfoff1IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfoff1IntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon1_int_en(&mut self) -> BitFsP2pRfon1IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfon1IntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfoff2_int_en(&mut self) -> BitFsP2pRfoff2IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfoff2IntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_fs_p2p_rfon2_int_en(&mut self) -> BitFsP2pRfon2IntEnW<'_, RegFt2imrSpec> {
        BitFsP2pRfon2IntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend0_int_en(&mut self) -> BitFsCtwend0IntEnW<'_, RegFt2imrSpec> {
        BitFsCtwend0IntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend1_int_en(&mut self) -> BitFsCtwend1IntEnW<'_, RegFt2imrSpec> {
        BitFsCtwend1IntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_fs_ctwend2_int_en(&mut self) -> BitFsCtwend2IntEnW<'_, RegFt2imrSpec> {
        BitFsCtwend2IntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - port1 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int_en1(&mut self) -> BitFsCtweIntEn1W<'_, RegFt2imrSpec> {
        BitFsCtweIntEn1W::new(self, 12)
    }
    #[doc = "Bit 13 - port0 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int_en0(&mut self) -> BitFsCtweIntEn0W<'_, RegFt2imrSpec> {
        BitFsCtweIntEn0W::new(self, 13)
    }
    #[doc = "Bit 14 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_fs_ctwe_int2_en(&mut self) -> BitFsCtweInt2EnW<'_, RegFt2imrSpec> {
        BitFsCtweInt2EnW::new(self, 14)
    }
    #[doc = "Bits 15:19 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFt2imrSpec> {
        RsvdW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ft2imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ft2imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFt2imrSpec;
impl crate::RegisterSpec for RegFt2imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ft2imr::R`](R) reader structure"]
impl crate::Readable for RegFt2imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ft2imr::W`](W) writer structure"]
impl crate::Writable for RegFt2imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FT2IMR to value 0"]
impl crate::Resettable for RegFt2imrSpec {}
