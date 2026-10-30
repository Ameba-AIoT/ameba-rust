#[doc = "Register `REG_MQ6_MQ7_TXBD_NUM` reader"]
pub type R = crate::R<RegMq6Mq7TxbdNumSpec>;
#[doc = "Register `REG_MQ6_MQ7_TXBD_NUM` writer"]
pub type W = crate::W<RegMq6Mq7TxbdNumSpec>;
#[doc = "Field `BIT_MQ6DES_NUM` reader - M6 queue descriptor's number"]
pub type BitMq6desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ6DES_NUM` writer - M6 queue descriptor's number"]
pub type BitMq6desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ6_DESC_MODE` reader - MQ6 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq6DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ6_DESC_MODE` writer - MQ6 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq6DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ6_FLAG` reader - MQ6, read result will be MQ6flag"]
pub type BitMq6FlagR = crate::BitReader;
#[doc = "Field `BIT_MQ7DES_NUM` reader - M7 queue descriptor's number"]
pub type BitMq7desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ7DES_NUM` writer - M7 queue descriptor's number"]
pub type BitMq7desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ7_DESC_MODE` reader - MQ7 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq7DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ7_DESC_MODE` writer - MQ7 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq7DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ7_FLAG` reader - MQ7, read result will be MQ7flag"]
pub type BitMq7FlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - M6 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq6des_num(&self) -> BitMq6desNumR {
        BitMq6desNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - MQ6 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq6_desc_mode(&self) -> BitMq6DescModeR {
        BitMq6DescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MQ6, read result will be MQ6flag"]
    #[inline(always)]
    pub fn bit_mq6_flag(&self) -> BitMq6FlagR {
        BitMq6FlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - M7 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq7des_num(&self) -> BitMq7desNumR {
        BitMq7desNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - MQ7 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq7_desc_mode(&self) -> BitMq7DescModeR {
        BitMq7DescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MQ7, read result will be MQ7flag"]
    #[inline(always)]
    pub fn bit_mq7_flag(&self) -> BitMq7FlagR {
        BitMq7FlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - M6 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq6des_num(&mut self) -> BitMq6desNumW<'_, RegMq6Mq7TxbdNumSpec> {
        BitMq6desNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - MQ6 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq6_desc_mode(&mut self) -> BitMq6DescModeW<'_, RegMq6Mq7TxbdNumSpec> {
        BitMq6DescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - M7 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq7des_num(&mut self) -> BitMq7desNumW<'_, RegMq6Mq7TxbdNumSpec> {
        BitMq7desNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - MQ7 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq7_desc_mode(&mut self) -> BitMq7DescModeW<'_, RegMq6Mq7TxbdNumSpec> {
        BitMq7DescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq6_mq7_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq6_mq7_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq6Mq7TxbdNumSpec;
impl crate::RegisterSpec for RegMq6Mq7TxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq6_mq7_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegMq6Mq7TxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq6_mq7_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegMq6Mq7TxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ6_MQ7_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegMq6Mq7TxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
