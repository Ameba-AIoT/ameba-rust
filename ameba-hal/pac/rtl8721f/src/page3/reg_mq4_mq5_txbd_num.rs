#[doc = "Register `REG_MQ4_MQ5_TXBD_NUM` reader"]
pub type R = crate::R<RegMq4Mq5TxbdNumSpec>;
#[doc = "Register `REG_MQ4_MQ5_TXBD_NUM` writer"]
pub type W = crate::W<RegMq4Mq5TxbdNumSpec>;
#[doc = "Field `BIT_MQ4DES_NUM` reader - M4 queue descriptor's number"]
pub type BitMq4desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ4DES_NUM` writer - M4 queue descriptor's number"]
pub type BitMq4desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ4_DESC_MODE` reader - MQ4 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq4DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ4_DESC_MODE` writer - MQ4 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq4DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ4_FLAG` reader - MQ4, read result will be MQ4flag"]
pub type BitMq4FlagR = crate::BitReader;
#[doc = "Field `BIT_MQ5DES_NUM` reader - M5 queue descriptor's number"]
pub type BitMq5desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ5DES_NUM` writer - M5 queue descriptor's number"]
pub type BitMq5desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ5_DESC_MODE` reader - MQ5 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq5DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ5_DESC_MODE` writer - MQ5 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq5DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ5_FLAG` reader - MQ5, read result will be MQ5flag"]
pub type BitMq5FlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - M4 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq4des_num(&self) -> BitMq4desNumR {
        BitMq4desNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - MQ4 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq4_desc_mode(&self) -> BitMq4DescModeR {
        BitMq4DescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MQ4, read result will be MQ4flag"]
    #[inline(always)]
    pub fn bit_mq4_flag(&self) -> BitMq4FlagR {
        BitMq4FlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - M5 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq5des_num(&self) -> BitMq5desNumR {
        BitMq5desNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - MQ5 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq5_desc_mode(&self) -> BitMq5DescModeR {
        BitMq5DescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MQ5, read result will be MQ5flag"]
    #[inline(always)]
    pub fn bit_mq5_flag(&self) -> BitMq5FlagR {
        BitMq5FlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - M4 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq4des_num(&mut self) -> BitMq4desNumW<'_, RegMq4Mq5TxbdNumSpec> {
        BitMq4desNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - MQ4 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq4_desc_mode(&mut self) -> BitMq4DescModeW<'_, RegMq4Mq5TxbdNumSpec> {
        BitMq4DescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - M5 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq5des_num(&mut self) -> BitMq5desNumW<'_, RegMq4Mq5TxbdNumSpec> {
        BitMq5desNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - MQ5 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq5_desc_mode(&mut self) -> BitMq5DescModeW<'_, RegMq4Mq5TxbdNumSpec> {
        BitMq5DescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq4_mq5_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq4_mq5_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq4Mq5TxbdNumSpec;
impl crate::RegisterSpec for RegMq4Mq5TxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq4_mq5_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegMq4Mq5TxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq4_mq5_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegMq4Mq5TxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ4_MQ5_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegMq4Mq5TxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
