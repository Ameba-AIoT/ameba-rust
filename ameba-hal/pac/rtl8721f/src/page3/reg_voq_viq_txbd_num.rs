#[doc = "Register `REG_VOQ_VIQ_TXBD_NUM` reader"]
pub type R = crate::R<RegVoqViqTxbdNumSpec>;
#[doc = "Register `REG_VOQ_VIQ_TXBD_NUM` writer"]
pub type W = crate::W<RegVoqViqTxbdNumSpec>;
#[doc = "Field `BIT_VOQDES_NUM` reader - VO queue descriptor's number"]
pub type BitVoqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VOQDES_NUM` writer - VO queue descriptor's number"]
pub type BitVoqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_VOQ_DESC_MODE` reader - VOQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitVoqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_VOQ_DESC_MODE` writer - VOQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitVoqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_VOQ_FLAG` reader - VOQ, read result will be voqflag"]
pub type BitVoqFlagR = crate::BitReader;
#[doc = "Field `BIT_VIQDES_NUM` reader - VI queue descriptor's number"]
pub type BitViqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VIQDES_NUM` writer - VI queue descriptor's number"]
pub type BitViqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_VIQ_DESC_MODE` reader - VIQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitViqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_VIQ_DESC_MODE` writer - VIQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitViqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_VIQ_FLAG` reader - VIQ, read result will be viqflag"]
pub type BitViqFlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - VO queue descriptor's number"]
    #[inline(always)]
    pub fn bit_voqdes_num(&self) -> BitVoqdesNumR {
        BitVoqdesNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - VOQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_voq_desc_mode(&self) -> BitVoqDescModeR {
        BitVoqDescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - VOQ, read result will be voqflag"]
    #[inline(always)]
    pub fn bit_voq_flag(&self) -> BitVoqFlagR {
        BitVoqFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - VI queue descriptor's number"]
    #[inline(always)]
    pub fn bit_viqdes_num(&self) -> BitViqdesNumR {
        BitViqdesNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - VIQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_viq_desc_mode(&self) -> BitViqDescModeR {
        BitViqDescModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - VIQ, read result will be viqflag"]
    #[inline(always)]
    pub fn bit_viq_flag(&self) -> BitViqFlagR {
        BitViqFlagR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - VO queue descriptor's number"]
    #[inline(always)]
    pub fn bit_voqdes_num(&mut self) -> BitVoqdesNumW<'_, RegVoqViqTxbdNumSpec> {
        BitVoqdesNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - VOQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_voq_desc_mode(&mut self) -> BitVoqDescModeW<'_, RegVoqViqTxbdNumSpec> {
        BitVoqDescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - VI queue descriptor's number"]
    #[inline(always)]
    pub fn bit_viqdes_num(&mut self) -> BitViqdesNumW<'_, RegVoqViqTxbdNumSpec> {
        BitViqdesNumW::new(self, 16)
    }
    #[doc = "Bits 28:29 - VIQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_viq_desc_mode(&mut self) -> BitViqDescModeW<'_, RegVoqViqTxbdNumSpec> {
        BitViqDescModeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_voq_viq_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_voq_viq_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegVoqViqTxbdNumSpec;
impl crate::RegisterSpec for RegVoqViqTxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_voq_viq_txbd_num::R`](R) reader structure"]
impl crate::Readable for RegVoqViqTxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_voq_viq_txbd_num::W`](W) writer structure"]
impl crate::Writable for RegVoqViqTxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VOQ_VIQ_TXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegVoqViqTxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
