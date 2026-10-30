#[doc = "Register `REG_AUTO_LLT` reader"]
pub type R = crate::R<RegAutoLltSpec>;
#[doc = "Register `REG_AUTO_LLT` writer"]
pub type W = crate::W<RegAutoLltSpec>;
#[doc = "Field `BIT_Tx_OQT_NL_free_space` reader - Normal and low priority queue"]
pub type BitTxOqtNlFreeSpaceR = crate::FieldReader;
#[doc = "Field `BIT_Tx_OQT_HE_free_space` reader - High and Extra priority queue"]
pub type BitTxOqtHeFreeSpaceR = crate::FieldReader;
#[doc = "Field `BIT_Auto_Init_LLT` reader - Write 1 to enable HW init LLT, driver need polling to 0 meaning init success"]
pub type BitAutoInitLltR = crate::BitReader;
#[doc = "Field `BIT_Auto_Init_LLT` writer - Write 1 to enable HW init LLT, driver need polling to 0 meaning init success"]
pub type BitAutoInitLltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_MASK_QSEL_DIFF` reader - Mask the error bit(tx endpoint is unmatched with qsel, 0x210\\[10\\])"]
pub type BitMaskQselDiffR = crate::BitReader;
#[doc = "Field `BIT_MASK_QSEL_DIFF` writer - Mask the error bit(tx endpoint is unmatched with qsel, 0x210\\[10\\])"]
pub type BitMaskQselDiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDE_DBG_SEL` reader - debug signals selection of Tx DMA"]
pub type BitTdeDbgSelR = crate::BitReader;
#[doc = "Field `BIT_TDE_DBG_SEL` writer - debug signals selection of Tx DMA"]
pub type BitTdeDbgSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXPKTNUM_H` reader - Packet number in TXPKTBUF (High and Extra)"]
pub type BitTxpktnumHR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Normal and low priority queue"]
    #[inline(always)]
    pub fn bit_tx_oqt_nl_free_space(&self) -> BitTxOqtNlFreeSpaceR {
        BitTxOqtNlFreeSpaceR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High and Extra priority queue"]
    #[inline(always)]
    pub fn bit_tx_oqt_he_free_space(&self) -> BitTxOqtHeFreeSpaceR {
        BitTxOqtHeFreeSpaceR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Write 1 to enable HW init LLT, driver need polling to 0 meaning init success"]
    #[inline(always)]
    pub fn bit_auto_init_llt(&self) -> BitAutoInitLltR {
        BitAutoInitLltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Mask the error bit(tx endpoint is unmatched with qsel, 0x210\\[10\\])"]
    #[inline(always)]
    pub fn bit_mask_qsel_diff(&self) -> BitMaskQselDiffR {
        BitMaskQselDiffR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - debug signals selection of Tx DMA"]
    #[inline(always)]
    pub fn bit_tde_dbg_sel(&self) -> BitTdeDbgSelR {
        BitTdeDbgSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Packet number in TXPKTBUF (High and Extra)"]
    #[inline(always)]
    pub fn bit_txpktnum_h(&self) -> BitTxpktnumHR {
        BitTxpktnumHR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Write 1 to enable HW init LLT, driver need polling to 0 meaning init success"]
    #[inline(always)]
    pub fn bit_auto_init_llt(&mut self) -> BitAutoInitLltW<'_, RegAutoLltSpec> {
        BitAutoInitLltW::new(self, 16)
    }
    #[doc = "Bits 17:21 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegAutoLltSpec> {
        RsvdW::new(self, 17)
    }
    #[doc = "Bit 22 - Mask the error bit(tx endpoint is unmatched with qsel, 0x210\\[10\\])"]
    #[inline(always)]
    pub fn bit_mask_qsel_diff(&mut self) -> BitMaskQselDiffW<'_, RegAutoLltSpec> {
        BitMaskQselDiffW::new(self, 22)
    }
    #[doc = "Bit 23 - debug signals selection of Tx DMA"]
    #[inline(always)]
    pub fn bit_tde_dbg_sel(&mut self) -> BitTdeDbgSelW<'_, RegAutoLltSpec> {
        BitTdeDbgSelW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_auto_llt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_auto_llt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAutoLltSpec;
impl crate::RegisterSpec for RegAutoLltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_auto_llt::R`](R) reader structure"]
impl crate::Readable for RegAutoLltSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_auto_llt::W`](W) writer structure"]
impl crate::Writable for RegAutoLltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AUTO_LLT to value 0"]
impl crate::Resettable for RegAutoLltSpec {}
