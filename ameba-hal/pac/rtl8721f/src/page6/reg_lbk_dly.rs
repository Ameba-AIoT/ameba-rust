#[doc = "Register `REG_LBK_DLY` reader"]
pub type R = crate::R<RegLbkDlySpec>;
#[doc = "Register `REG_LBK_DLY` writer"]
pub type W = crate::W<RegLbkDlySpec>;
#[doc = "Field `BIT_LBK_DLY` reader - Loopback Delay. In unit of 'us'; indicates the delay after which the tdrdy_mac starts. In wep/tkip mode, the delay should be larger than 15, because the initial delay of security is about 1320 cycles, approximately to 16.5 us."]
pub type BitLbkDlyR = crate::FieldReader;
#[doc = "Field `BIT_LBK_DLY` writer - Loopback Delay. In unit of 'us'; indicates the delay after which the tdrdy_mac starts. In wep/tkip mode, the delay should be larger than 15, because the initial delay of security is about 1320 cycles, approximately to 16.5 us."]
pub type BitLbkDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_BACAM_RPMEN` reader - 1:Enable repeat mode 0:Disable function"]
pub type BitBacamRpmenR = crate::BitReader;
#[doc = "Field `BIT_BACAM_RPMEN` writer - 1:Enable repeat mode 0:Disable function"]
pub type BitBacamRpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BITMAP_EN` reader - 1:Enable of counter and WMAC_BITMAP_CTL(0x663) 0:Disable function"]
pub type BitBitmapEnR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_EN` writer - 1:Enable of counter and WMAC_BITMAP_CTL(0x663) 0:Disable function"]
pub type BitBitmapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BITMAP_SSNBK_COUNTER` reader - Count the sequence number out of BITMAP condition (0x663\\[3:2\\])."]
pub type BitBitmapSsnbkCounterR = crate::FieldReader;
#[doc = "Field `BIT_RXACT_IND` reader - Indicate MAC is busy to rx packet"]
pub type BitRxactIndR = crate::BitReader;
#[doc = "Field `BIT_TXACT_IND` reader - Indicate MAC/PHY is busy to tx packet"]
pub type BitTxactIndR = crate::BitReader;
#[doc = "Field `BIT_RXPKT_TYPE` reader - The type and subtype field of frame control of rx packet"]
pub type BitRxpktTypeR = crate::FieldReader;
#[doc = "Field `BIT_BITMAP_FORCE` reader - Under the BIT_BITMAP_EN = 1 condition. 1: Rx's packet always checks sequence number by BIT_BITMAP_CONDITION. 0: Disable"]
pub type BitBitmapForceR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_FORCE` writer - Under the BIT_BITMAP_EN = 1 condition. 1: Rx's packet always checks sequence number by BIT_BITMAP_CONDITION. 0: Disable"]
pub type BitBitmapForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BITMAP_SSNBK_COUNTER_CLR` reader - Clear BIT_BITMAP_SSNBK_COUNTER."]
pub type BitBitmapSsnbkCounterClrR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_CONDITION` reader - ADDBA condition: 0: HW start sequence - 64. 1: HW start sequence - 128. 2: HW start sequence - 256. 3: HW start sequence - 512."]
pub type BitBitmapConditionR = crate::FieldReader;
#[doc = "Field `BIT_BITMAP_CONDITION` writer - ADDBA condition: 0: HW start sequence - 64. 1: HW start sequence - 128. 2: HW start sequence - 256. 3: HW start sequence - 512."]
pub type BitBitmapConditionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BITMAP_FLAG_BK` reader - When BIT_BITMAP_BK = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
pub type BitBitmapFlagBkR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_FLAG_BE` reader - When BIT_BITMAP_BE = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
pub type BitBitmapFlagBeR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_FLAG_VI` reader - When BIT_BITMAP_VI = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
pub type BitBitmapFlagViR = crate::BitReader;
#[doc = "Field `BIT_BITMAP_FLAG_VO` reader - When BIT_BITMAP_VO = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
pub type BitBitmapFlagVoR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Loopback Delay. In unit of 'us'; indicates the delay after which the tdrdy_mac starts. In wep/tkip mode, the delay should be larger than 15, because the initial delay of security is about 1320 cycles, approximately to 16.5 us."]
    #[inline(always)]
    pub fn bit_lbk_dly(&self) -> BitLbkDlyR {
        BitLbkDlyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 1:Enable repeat mode 0:Disable function"]
    #[inline(always)]
    pub fn bit_bacam_rpmen(&self) -> BitBacamRpmenR {
        BitBacamRpmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1:Enable of counter and WMAC_BITMAP_CTL(0x663) 0:Disable function"]
    #[inline(always)]
    pub fn bit_bitmap_en(&self) -> BitBitmapEnR {
        BitBitmapEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Count the sequence number out of BITMAP condition (0x663\\[3:2\\])."]
    #[inline(always)]
    pub fn bit_bitmap_ssnbk_counter(&self) -> BitBitmapSsnbkCounterR {
        BitBitmapSsnbkCounterR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Indicate MAC is busy to rx packet"]
    #[inline(always)]
    pub fn bit_rxact_ind(&self) -> BitRxactIndR {
        BitRxactIndR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicate MAC/PHY is busy to tx packet"]
    #[inline(always)]
    pub fn bit_txact_ind(&self) -> BitTxactIndR {
        BitTxactIndR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - The type and subtype field of frame control of rx packet"]
    #[inline(always)]
    pub fn bit_rxpkt_type(&self) -> BitRxpktTypeR {
        BitRxpktTypeR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Under the BIT_BITMAP_EN = 1 condition. 1: Rx's packet always checks sequence number by BIT_BITMAP_CONDITION. 0: Disable"]
    #[inline(always)]
    pub fn bit_bitmap_force(&self) -> BitBitmapForceR {
        BitBitmapForceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clear BIT_BITMAP_SSNBK_COUNTER."]
    #[inline(always)]
    pub fn bit_bitmap_ssnbk_counter_clr(&self) -> BitBitmapSsnbkCounterClrR {
        BitBitmapSsnbkCounterClrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - ADDBA condition: 0: HW start sequence - 64. 1: HW start sequence - 128. 2: HW start sequence - 256. 3: HW start sequence - 512."]
    #[inline(always)]
    pub fn bit_bitmap_condition(&self) -> BitBitmapConditionR {
        BitBitmapConditionR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - When BIT_BITMAP_BK = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
    #[inline(always)]
    pub fn bit_bitmap_flag_bk(&self) -> BitBitmapFlagBkR {
        BitBitmapFlagBkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When BIT_BITMAP_BE = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
    #[inline(always)]
    pub fn bit_bitmap_flag_be(&self) -> BitBitmapFlagBeR {
        BitBitmapFlagBeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When BIT_BITMAP_VI = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
    #[inline(always)]
    pub fn bit_bitmap_flag_vi(&self) -> BitBitmapFlagViR {
        BitBitmapFlagViR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When BIT_BITMAP_VO = 1, HW will check received packet's sequence number by BIT_BITMAP_CONDITION, and clear the bit after occur one time which received the correct sequence number."]
    #[inline(always)]
    pub fn bit_bitmap_flag_vo(&self) -> BitBitmapFlagVoR {
        BitBitmapFlagVoR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Loopback Delay. In unit of 'us'; indicates the delay after which the tdrdy_mac starts. In wep/tkip mode, the delay should be larger than 15, because the initial delay of security is about 1320 cycles, approximately to 16.5 us."]
    #[inline(always)]
    pub fn bit_lbk_dly(&mut self) -> BitLbkDlyW<'_, RegLbkDlySpec> {
        BitLbkDlyW::new(self, 0)
    }
    #[doc = "Bit 8 - 1:Enable repeat mode 0:Disable function"]
    #[inline(always)]
    pub fn bit_bacam_rpmen(&mut self) -> BitBacamRpmenW<'_, RegLbkDlySpec> {
        BitBacamRpmenW::new(self, 8)
    }
    #[doc = "Bit 9 - 1:Enable of counter and WMAC_BITMAP_CTL(0x663) 0:Disable function"]
    #[inline(always)]
    pub fn bit_bitmap_en(&mut self) -> BitBitmapEnW<'_, RegLbkDlySpec> {
        BitBitmapEnW::new(self, 9)
    }
    #[doc = "Bit 24 - Under the BIT_BITMAP_EN = 1 condition. 1: Rx's packet always checks sequence number by BIT_BITMAP_CONDITION. 0: Disable"]
    #[inline(always)]
    pub fn bit_bitmap_force(&mut self) -> BitBitmapForceW<'_, RegLbkDlySpec> {
        BitBitmapForceW::new(self, 24)
    }
    #[doc = "Bits 26:27 - ADDBA condition: 0: HW start sequence - 64. 1: HW start sequence - 128. 2: HW start sequence - 256. 3: HW start sequence - 512."]
    #[inline(always)]
    pub fn bit_bitmap_condition(&mut self) -> BitBitmapConditionW<'_, RegLbkDlySpec> {
        BitBitmapConditionW::new(self, 26)
    }
}
#[doc = "Loopback Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lbk_dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lbk_dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLbkDlySpec;
impl crate::RegisterSpec for RegLbkDlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lbk_dly::R`](R) reader structure"]
impl crate::Readable for RegLbkDlySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lbk_dly::W`](W) writer structure"]
impl crate::Writable for RegLbkDlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LBK_DLY to value 0x19"]
impl crate::Resettable for RegLbkDlySpec {
    const RESET_VALUE: u32 = 0x19;
}
