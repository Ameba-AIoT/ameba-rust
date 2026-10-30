#[doc = "Register `REG_DWBCN0_CTRL` reader"]
pub type R = crate::R<RegDwbcn0CtrlSpec>;
#[doc = "Register `REG_DWBCN0_CTRL` writer"]
pub type W = crate::W<RegDwbcn0CtrlSpec>;
#[doc = "Field `BIT_BLK_DESC_OPT` reader - option to select usb descriptor number(according to 0x208\\[7:4\\]) 1 : usb_desc_num = 2 * r_blk_desc_num 0 : usb_desc_num = 4 * r_blk_desc_num"]
pub type BitBlkDescOptR = crate::BitReader;
#[doc = "Field `BIT_BLK_DESC_OPT` writer - option to select usb descriptor number(according to 0x208\\[7:4\\]) 1 : usb_desc_num = 2 * r_blk_desc_num 0 : usb_desc_num = 4 * r_blk_desc_num"]
pub type BitBlkDescOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_BLK_DESC_NUM` reader - The Most Number of Tx Descriptor per Bulk Out Only for USB"]
pub type BitBlkDescNumR = crate::FieldReader;
#[doc = "Field `BIT_BLK_DESC_NUM` writer - The Most Number of Tx Descriptor per Bulk Out Only for USB"]
pub type BitBlkDescNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_BCN_HEAD` reader - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from"]
pub type BitBcnHeadR = crate::FieldReader;
#[doc = "Field `BIT_BCN_HEAD` writer - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from"]
pub type BitBcnHeadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCN_VALID` reader - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW"]
pub type BitBcnValidR = crate::BitReader;
#[doc = "Field `BIT_RSVD_BYTE1` reader - RSVD"]
pub type BitRsvdByte1R = crate::FieldReader;
#[doc = "Field `BIT_RSVD_BYTE1` writer - RSVD"]
pub type BitRsvdByte1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `bit_llt_free_page` reader - RSVD"]
pub type BitLltFreePageR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - option to select usb descriptor number(according to 0x208\\[7:4\\]) 1 : usb_desc_num = 2 * r_blk_desc_num 0 : usb_desc_num = 4 * r_blk_desc_num"]
    #[inline(always)]
    pub fn bit_blk_desc_opt(&self) -> BitBlkDescOptR {
        BitBlkDescOptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - The Most Number of Tx Descriptor per Bulk Out Only for USB"]
    #[inline(always)]
    pub fn bit_blk_desc_num(&self) -> BitBlkDescNumR {
        BitBlkDescNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from"]
    #[inline(always)]
    pub fn bit_bcn_head(&self) -> BitBcnHeadR {
        BitBcnHeadR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW"]
    #[inline(always)]
    pub fn bit_bcn_valid(&self) -> BitBcnValidR {
        BitBcnValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - RSVD"]
    #[inline(always)]
    pub fn bit_rsvd_byte1(&self) -> BitRsvdByte1R {
        BitRsvdByte1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - RSVD"]
    #[inline(always)]
    pub fn bit_llt_free_page(&self) -> BitLltFreePageR {
        BitLltFreePageR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - option to select usb descriptor number(according to 0x208\\[7:4\\]) 1 : usb_desc_num = 2 * r_blk_desc_num 0 : usb_desc_num = 4 * r_blk_desc_num"]
    #[inline(always)]
    pub fn bit_blk_desc_opt(&mut self) -> BitBlkDescOptW<'_, RegDwbcn0CtrlSpec> {
        BitBlkDescOptW::new(self, 0)
    }
    #[doc = "Bits 1:3 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegDwbcn0CtrlSpec> {
        RsvdW::new(self, 1)
    }
    #[doc = "Bits 4:7 - The Most Number of Tx Descriptor per Bulk Out Only for USB"]
    #[inline(always)]
    pub fn bit_blk_desc_num(&mut self) -> BitBlkDescNumW<'_, RegDwbcn0CtrlSpec> {
        BitBlkDescNumW::new(self, 4)
    }
    #[doc = "Bits 8:15 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from"]
    #[inline(always)]
    pub fn bit_bcn_head(&mut self) -> BitBcnHeadW<'_, RegDwbcn0CtrlSpec> {
        BitBcnHeadW::new(self, 8)
    }
    #[doc = "Bits 17:23 - RSVD"]
    #[inline(always)]
    pub fn bit_rsvd_byte1(&mut self) -> BitRsvdByte1W<'_, RegDwbcn0CtrlSpec> {
        BitRsvdByte1W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dwbcn0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dwbcn0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDwbcn0CtrlSpec;
impl crate::RegisterSpec for RegDwbcn0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dwbcn0_ctrl::R`](R) reader structure"]
impl crate::Readable for RegDwbcn0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dwbcn0_ctrl::W`](W) writer structure"]
impl crate::Writable for RegDwbcn0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DWBCN0_CTRL to value 0xf610"]
impl crate::Resettable for RegDwbcn0CtrlSpec {
    const RESET_VALUE: u32 = 0xf610;
}
