#[doc = "Register `REG_MQ2_MQ3_TXBD_NUM` reader"]
pub type R = crate::R<RegMq2Mq3TxbdNumSpec>;
#[doc = "Register `REG_MQ2_MQ3_TXBD_NUM` writer"]
pub type W = crate::W<RegMq2Mq3TxbdNumSpec>;
#[doc = "Field `BIT_MQ2DES_NUM` reader - M2 queue descriptor's number"]
pub type BitMq2desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ2DES_NUM` writer - M2 queue descriptor's number"]
pub type BitMq2desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ2_DESC_MODE` reader - MQ2 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq2DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ2_DESC_MODE` writer - MQ2 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq2DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ2_FLAG` reader - MQ2, read result will be MQ2flag"]
pub type BitMq2FlagR = crate::BitReader;
#[doc = "Field `BIT_MQ3DES_NUM` reader - M3 queue descriptor's number"]
pub type BitMq3desNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ3DES_NUM` writer - M3 queue descriptor's number"]
pub type BitMq3desNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ3_DESC_MODE` reader - MQ3 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq3DescModeR = crate::FieldReader;
#[doc = "Field `BIT_MQ3_DESC_MODE` writer - MQ3 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMq3DescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MQ3_FLAG` reader - MQ3, read result will be MQ3flag"]
pub type BitMq3FlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - M2 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq2des_num(&self) -> BitMq2desNumR {
        BitMq2desNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - MQ2 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq2_desc_mode(&self) -> BitMq2DescModeR {
        BitMq2DescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MQ2, read result will be MQ2flag"]
    #[inline(always)]
    pub fn bit_mq2_flag(&self) -> BitMq2FlagR {
        BitMq2FlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - M3 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq3des_num(&self) -> BitMq3desNumR {
        BitMq3desNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - MQ3 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq3_desc_mode(&self) -> BitMq3DescModeR {
        BitMq3DescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MQ3, read result will be MQ3flag"]
    #[inline(always)]
    pub fn bit_mq3_flag(&self) -> BitMq3FlagR {
        BitMq3FlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - M2 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq2des_num(&mut self) -> BitMq2desNumW<'_, RegMq2Mq3TxbdNumSpec> {
        BitMq2desNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - MQ2 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq2_desc_mode(&mut self) -> BitMq2DescModeW<'_, RegMq2Mq3TxbdNumSpec> {
        BitMq2DescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - M3 queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mq3des_num(&mut self) -> BitMq3desNumW<'_, RegMq2Mq3TxbdNumSpec> {
        BitMq3desNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - MQ3 descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mq3_desc_mode(&mut self) -> BitMq3DescModeW<'_, RegMq2Mq3TxbdNumSpec> {
        BitMq3DescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq2_mq3_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq2_mq3_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq2Mq3TxbdNumSpec;
impl crate::RegisterSpec for RegMq2Mq3TxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq2_mq3_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegMq2Mq3TxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq2_mq3_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegMq2Mq3TxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ2_MQ3_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegMq2Mq3TxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
