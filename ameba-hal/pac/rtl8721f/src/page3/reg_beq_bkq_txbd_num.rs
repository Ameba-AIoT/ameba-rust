#[doc = "Register `REG_BEQ_BKQ_TXBD_NUM` reader"]
pub type R = crate::R<RegBeqBkqTxbdNumSpec>;
#[doc = "Register `REG_BEQ_BKQ_TXBD_NUM` writer"]
pub type W = crate::W<RegBeqBkqTxbdNumSpec>;
#[doc = "Field `BIT_BEQDES_NUM` reader - BE queue descriptor's number"]
pub type BitBeqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BEQDES_NUM` writer - BE queue descriptor's number"]
pub type BitBeqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_BEQ_DESC_MODE` reader - BEQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBeqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_BEQ_DESC_MODE` writer - BEQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBeqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BEQ_FLAG` reader - BEQ, read result will be beqflag"]
pub type BitBeqFlagR = crate::BitReader;
#[doc = "Field `BIT_BKQDES_NUM` reader - BK queue descriptor's number"]
pub type BitBkqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BKQDES_NUM` writer - BK queue descriptor's number"]
pub type BitBkqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_BKQ_DESC_MODE` reader - BKQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBkqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_BKQ_DESC_MODE` writer - BKQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBkqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BKQ_FLAG` reader - BKQ, read result will be bkqflag"]
pub type BitBkqFlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - BE queue descriptor's number"]
    #[inline(always)]
    pub fn bit_beqdes_num(&self) -> BitBeqdesNumR {
        BitBeqdesNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - BEQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_beq_desc_mode(&self) -> BitBeqDescModeR {
        BitBeqDescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - BEQ, read result will be beqflag"]
    #[inline(always)]
    pub fn bit_beq_flag(&self) -> BitBeqFlagR {
        BitBeqFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - BK queue descriptor's number"]
    #[inline(always)]
    pub fn bit_bkqdes_num(&self) -> BitBkqdesNumR {
        BitBkqdesNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - BKQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_bkq_desc_mode(&self) -> BitBkqDescModeR {
        BitBkqDescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - BKQ, read result will be bkqflag"]
    #[inline(always)]
    pub fn bit_bkq_flag(&self) -> BitBkqFlagR {
        BitBkqFlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - BE queue descriptor's number"]
    #[inline(always)]
    pub fn bit_beqdes_num(&mut self) -> BitBeqdesNumW<'_, RegBeqBkqTxbdNumSpec> {
        BitBeqdesNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - BEQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_beq_desc_mode(&mut self) -> BitBeqDescModeW<'_, RegBeqBkqTxbdNumSpec> {
        BitBeqDescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - BK queue descriptor's number"]
    #[inline(always)]
    pub fn bit_bkqdes_num(&mut self) -> BitBkqdesNumW<'_, RegBeqBkqTxbdNumSpec> {
        BitBkqdesNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - BKQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_bkq_desc_mode(&mut self) -> BitBkqDescModeW<'_, RegBeqBkqTxbdNumSpec> {
        BitBkqDescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beq_bkq_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beq_bkq_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeqBkqTxbdNumSpec;
impl crate::RegisterSpec for RegBeqBkqTxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beq_bkq_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegBeqBkqTxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beq_bkq_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegBeqBkqTxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEQ_BKQ_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegBeqBkqTxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
