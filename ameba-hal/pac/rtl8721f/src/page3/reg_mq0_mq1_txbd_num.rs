#[doc = "Register `REG_MQ0_MQ1_TXBD_NUM` reader"]
pub type R = crate::R<RegMq0Mq1TxbdNumSpec>;
#[doc = "Register `REG_MQ0_MQ1_TXBD_NUM` writer"]
pub type W = crate::W<RegMq0Mq1TxbdNumSpec>;
#[doc = "Field `BIT_MQ0DES_NUM` reader - M0 queue descriptor's number"]
pub type BitMq0desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ0DES_NUM` writer - M0 queue descriptor's number"]
pub type BitMq0desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ0_DESC_MODE` reader - MQ0 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq0DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ0_DESC_MODE` writer - MQ0 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq0DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ0_FLAG` reader - MQ0, read result will be MQ0flag"]
pub type BitMq0FlagR = crate::BitReader;
#[doc = "Field `BIT_MQ1DES_NUM` reader - M1 queue descriptor's number"]
pub type BitMq1desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ1DES_NUM` writer - M1 queue descriptor's number"]
pub type BitMq1desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ1_DESC_MODE` reader - MQ1 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq1DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ1_DESC_MODE` writer - MQ1 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq1DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ1_FLAG` reader - MQ1, read result will be MQ1flag"]
pub type BitMq1FlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - M0 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq0des_num(&self) -> BitMq0desNumR {
        BitMq0desNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - MQ0 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq0_desc_mode(&self) -> BitMq0DescModeR {
        BitMq0DescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MQ0, read result will be MQ0flag"]
    #[inline(always)]
    pub fn bit_mq0_flag(&self) -> BitMq0FlagR {
        BitMq0FlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - M1 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq1des_num(&self) -> BitMq1desNumR {
        BitMq1desNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - MQ1 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq1_desc_mode(&self) -> BitMq1DescModeR {
        BitMq1DescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MQ1, read result will be MQ1flag"]
    #[inline(always)]
    pub fn bit_mq1_flag(&self) -> BitMq1FlagR {
        BitMq1FlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - M0 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq0des_num(&mut self) -> BitMq0desNumW<'_, RegMq0Mq1TxbdNumSpec> {
        BitMq0desNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - MQ0 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq0_desc_mode(&mut self) -> BitMq0DescModeW<'_, RegMq0Mq1TxbdNumSpec> {
        BitMq0DescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - M1 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq1des_num(&mut self) -> BitMq1desNumW<'_, RegMq0Mq1TxbdNumSpec> {
        BitMq1desNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - MQ1 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq1_desc_mode(&mut self) -> BitMq1DescModeW<'_, RegMq0Mq1TxbdNumSpec> {
        BitMq1DescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq0_mq1_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq0_mq1_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq0Mq1TxbdNumSpec;
impl crate::RegisterSpec for RegMq0Mq1TxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq0_mq1_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegMq0Mq1TxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq0_mq1_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegMq0Mq1TxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ0_MQ1_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegMq0Mq1TxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
