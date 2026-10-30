#[doc = "Register `REG_H2CQ_TXBD_NUM` reader"]
pub type R = crate::R<RegH2cqTxbdNumSpec>;
#[doc = "Register `REG_H2CQ_TXBD_NUM` writer"]
pub type W = crate::W<RegH2cqTxbdNumSpec>;
#[doc = "Field `BIT_H2CQDES_NUM` reader - H2C queue descriptor's number"]
pub type BitH2cqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_H2CQDES_NUM` writer - H2C queue descriptor's number"]
pub type BitH2cqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_H2CQ_DESC_MODE` reader - H2CQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitH2cqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_H2CQ_DESC_MODE` writer - H2CQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitH2cqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_H2CQ_FLAG` reader - H2CQ, read result will be H2CQflag"]
pub type BitH2cqFlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - H2C queue descriptor's number"]
    #[inline(always)]
    pub fn bit_h2cqdes_num(&self) -> BitH2cqdesNumR {
        BitH2cqdesNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - H2CQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_h2cq_desc_mode(&self) -> BitH2cqDescModeR {
        BitH2cqDescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - H2CQ, read result will be H2CQflag"]
    #[inline(always)]
    pub fn bit_h2cq_flag(&self) -> BitH2cqFlagR {
        BitH2cqFlagR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - H2C queue descriptor's number"]
    #[inline(always)]
    pub fn bit_h2cqdes_num(&mut self) -> BitH2cqdesNumW<'_, RegH2cqTxbdNumSpec> {
        BitH2cqdesNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - H2CQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_h2cq_desc_mode(&mut self) -> BitH2cqDescModeW<'_, RegH2cqTxbdNumSpec> {
        BitH2cqDescModeW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_h2cq_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_h2cq_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegH2cqTxbdNumSpec;
impl crate::RegisterSpec for RegH2cqTxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_h2cq_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegH2cqTxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_h2cq_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegH2cqTxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_H2CQ_TXBD_NUM to value 0x0100"]
impl crate::Resettable for RegH2cqTxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100;
}
