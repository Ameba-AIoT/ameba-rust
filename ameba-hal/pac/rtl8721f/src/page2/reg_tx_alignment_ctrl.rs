#[doc = "Register `REG_TX_ALIGNMENT_CTRL` reader"]
pub type R = crate::R<RegTxAlignmentCtrlSpec>;
#[doc = "Register `REG_TX_ALIGNMENT_CTRL` writer"]
pub type W = crate::W<RegTxAlignmentCtrlSpec>;
#[doc = "Field `BIT_ALIGNMENT_SIZE` reader - The alignment size of blocks in an aggregated-packet. Unit: Byte"]
pub type BitAlignmentSizeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ALIGNMENT_SIZE` writer - The alignment size of blocks in an aggregated-packet. Unit: Byte"]
pub type BitAlignmentSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_ADJUSTABLE_SIZE_EN` reader - Enable tx alignment for sdio. SW can assign the alignment size of blocks in an aggregated-packet."]
pub type BitAdjustableSizeEnR = crate::BitReader;
#[doc = "Field `BIT_ADJUSTABLE_SIZE_EN` writer - Enable tx alignment for sdio. SW can assign the alignment size of blocks in an aggregated-packet."]
pub type BitAdjustableSizeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - The alignment size of blocks in an aggregated-packet. Unit: Byte"]
    #[inline(always)]
    pub fn bit_alignment_size(&self) -> BitAlignmentSizeR {
        BitAlignmentSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable tx alignment for sdio. SW can assign the alignment size of blocks in an aggregated-packet."]
    #[inline(always)]
    pub fn bit_adjustable_size_en(&self) -> BitAdjustableSizeEnR {
        BitAdjustableSizeEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The alignment size of blocks in an aggregated-packet. Unit: Byte"]
    #[inline(always)]
    pub fn bit_alignment_size(&mut self) -> BitAlignmentSizeW<'_, RegTxAlignmentCtrlSpec> {
        BitAlignmentSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegTxAlignmentCtrlSpec> {
        Rsvd2W::new(self, 12)
    }
    #[doc = "Bit 15 - Enable tx alignment for sdio. SW can assign the alignment size of blocks in an aggregated-packet."]
    #[inline(always)]
    pub fn bit_adjustable_size_en(&mut self) -> BitAdjustableSizeEnW<'_, RegTxAlignmentCtrlSpec> {
        BitAdjustableSizeEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTxAlignmentCtrlSpec> {
        RsvdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_alignment_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_alignment_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxAlignmentCtrlSpec;
impl crate::RegisterSpec for RegTxAlignmentCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_alignment_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTxAlignmentCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_alignment_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTxAlignmentCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_ALIGNMENT_CTRL to value 0"]
impl crate::Resettable for RegTxAlignmentCtrlSpec {}
