#[doc = "Register `REG_DWBCN1_CTRL` reader"]
pub type R = crate::R<RegDwbcn1CtrlSpec>;
#[doc = "Register `REG_DWBCN1_CTRL` writer"]
pub type W = crate::W<RegDwbcn1CtrlSpec>;
#[doc = "Field `BIT_Max_tx_pkt` reader - Unit : packet number Max_tx_pkt_for_USB_and_SDIO"]
pub type BitMaxTxPktR = crate::FieldReader;
#[doc = "Field `BIT_Max_tx_pkt` writer - Unit : packet number Max_tx_pkt_for_USB_and_SDIO"]
pub type BitMaxTxPktW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCN_HEAD_1` reader - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
pub type BitBcnHead1R = crate::FieldReader;
#[doc = "Field `BIT_BCN_HEAD_1` writer - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
pub type BitBcnHead1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCN_VALID_1` reader - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
pub type BitBcnValid1R = crate::BitReader;
#[doc = "Field `BIT_BCN_VALID_1` writer - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
pub type BitBcnValid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SW_BCN_SEL_EN` reader - Enable Bcnq head selection from SW setting"]
pub type BitSwBcnSelEnR = crate::BitReader;
#[doc = "Field `BIT_SW_BCN_SEL_EN` writer - Enable Bcnq head selection from SW setting"]
pub type BitSwBcnSelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_VALID_2` reader - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
pub type BitBcnValid2R = crate::BitReader;
#[doc = "Field `BIT_BCN_VALID_2` writer - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
pub type BitBcnValid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SW_BCN_SEL` reader - Bcnq head selection from SW setting 2'b01 : r_bcn_head_1 2'b10 : r_bcn_head_2"]
pub type BitSwBcnSelR = crate::FieldReader;
#[doc = "Field `BIT_SW_BCN_SEL` writer - Bcnq head selection from SW setting 2'b01 : r_bcn_head_1 2'b10 : r_bcn_head_2"]
pub type BitSwBcnSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BCN_HEAD_2` reader - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
pub type BitBcnHead2R = crate::FieldReader;
#[doc = "Field `BIT_BCN_HEAD_2` writer - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
pub type BitBcnHead2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Unit : packet number Max_tx_pkt_for_USB_and_SDIO"]
    #[inline(always)]
    pub fn bit_max_tx_pkt(&self) -> BitMaxTxPktR {
        BitMaxTxPktR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
    #[inline(always)]
    pub fn bit_bcn_head_1(&self) -> BitBcnHead1R {
        BitBcnHead1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
    #[inline(always)]
    pub fn bit_bcn_valid_1(&self) -> BitBcnValid1R {
        BitBcnValid1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bcnq head selection from SW setting"]
    #[inline(always)]
    pub fn bit_sw_bcn_sel_en(&self) -> BitSwBcnSelEnR {
        BitSwBcnSelEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
    #[inline(always)]
    pub fn bit_bcn_valid_2(&self) -> BitBcnValid2R {
        BitBcnValid2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Bcnq head selection from SW setting 2'b01 : r_bcn_head_1 2'b10 : r_bcn_head_2"]
    #[inline(always)]
    pub fn bit_sw_bcn_sel(&self) -> BitSwBcnSelR {
        BitSwBcnSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
    #[inline(always)]
    pub fn bit_bcn_head_2(&self) -> BitBcnHead2R {
        BitBcnHead2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Unit : packet number Max_tx_pkt_for_USB_and_SDIO"]
    #[inline(always)]
    pub fn bit_max_tx_pkt(&mut self) -> BitMaxTxPktW<'_, RegDwbcn1CtrlSpec> {
        BitMaxTxPktW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
    #[inline(always)]
    pub fn bit_bcn_head_1(&mut self) -> BitBcnHead1W<'_, RegDwbcn1CtrlSpec> {
        BitBcnHead1W::new(self, 8)
    }
    #[doc = "Bit 16 - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
    #[inline(always)]
    pub fn bit_bcn_valid_1(&mut self) -> BitBcnValid1W<'_, RegDwbcn1CtrlSpec> {
        BitBcnValid1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Bcnq head selection from SW setting"]
    #[inline(always)]
    pub fn bit_sw_bcn_sel_en(&mut self) -> BitSwBcnSelEnW<'_, RegDwbcn1CtrlSpec> {
        BitSwBcnSelEnW::new(self, 17)
    }
    #[doc = "Bit 18 - When this bit is set, the beacon packet has finished to write to txpktbuffer. Clear by SW."]
    #[inline(always)]
    pub fn bit_bcn_valid_2(&mut self) -> BitBcnValid2W<'_, RegDwbcn1CtrlSpec> {
        BitBcnValid2W::new(self, 18)
    }
    #[doc = "Bit 19 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegDwbcn1CtrlSpec> {
        Rsvd2W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Bcnq head selection from SW setting 2'b01 : r_bcn_head_1 2'b10 : r_bcn_head_2"]
    #[inline(always)]
    pub fn bit_sw_bcn_sel(&mut self) -> BitSwBcnSelW<'_, RegDwbcn1CtrlSpec> {
        BitSwBcnSelW::new(self, 20)
    }
    #[doc = "Bits 22:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegDwbcn1CtrlSpec> {
        RsvdW::new(self, 22)
    }
    #[doc = "Bits 24:31 - The head page of packet of Bcnq which is Tx DMA filled Bcnq packet from."]
    #[inline(always)]
    pub fn bit_bcn_head_2(&mut self) -> BitBcnHead2W<'_, RegDwbcn1CtrlSpec> {
        BitBcnHead2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dwbcn1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dwbcn1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDwbcn1CtrlSpec;
impl crate::RegisterSpec for RegDwbcn1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dwbcn1_ctrl::R`](R) reader structure"]
impl crate::Readable for RegDwbcn1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dwbcn1_ctrl::W`](W) writer structure"]
impl crate::Writable for RegDwbcn1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DWBCN1_CTRL to value 0xfa00_f800"]
impl crate::Resettable for RegDwbcn1CtrlSpec {
    const RESET_VALUE: u32 = 0xfa00_f800;
}
