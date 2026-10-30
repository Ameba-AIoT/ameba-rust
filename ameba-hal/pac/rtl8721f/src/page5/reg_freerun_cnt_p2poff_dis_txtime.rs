#[doc = "Register `REG_FREERUN_CNT_P2POFF_DIS_TXTIME` reader"]
pub type R = crate::R<RegFreerunCntP2poffDisTxtimeSpec>;
#[doc = "Register `REG_FREERUN_CNT_P2POFF_DIS_TXTIME` writer"]
pub type W = crate::W<RegFreerunCntP2poffDisTxtimeSpec>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_RESET_FREECNT_P` writer - reset pulse of freerun tsft"]
pub type BitResetFreecntPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_FREECNT` reader - enable freerun tsft"]
pub type BitEnFreecntR = crate::BitReader;
#[doc = "Field `BIT_EN_FREECNT` writer - enable freerun tsft"]
pub type BitEnFreecntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2POFF_DIS_TXTIME` reader - The time of the disable TX period after the P2P RF off interrupt occur. Unit: 64us."]
pub type BitP2poffDisTxtimeR = crate::FieldReader;
#[doc = "Field `BIT_P2POFF_DIS_TXTIME` writer - The time of the disable TX period after the P2P RF off interrupt occur. Unit: 64us."]
pub type BitP2poffDisTxtimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - enable freerun tsft"]
    #[inline(always)]
    pub fn bit_en_freecnt(&self) -> BitEnFreecntR {
        BitEnFreecntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 24:31 - The time of the disable TX period after the P2P RF off interrupt occur. Unit: 64us."]
    #[inline(always)]
    pub fn bit_p2poff_dis_txtime(&self) -> BitP2poffDisTxtimeR {
        BitP2poffDisTxtimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegFreerunCntP2poffDisTxtimeSpec> {
        DummyW::new(self, 0)
    }
    #[doc = "Bit 12 - reset pulse of freerun tsft"]
    #[inline(always)]
    pub fn bit_reset_freecnt_p(
        &mut self,
    ) -> BitResetFreecntPW<'_, RegFreerunCntP2poffDisTxtimeSpec> {
        BitResetFreecntPW::new(self, 12)
    }
    #[doc = "Bit 13 - enable freerun tsft"]
    #[inline(always)]
    pub fn bit_en_freecnt(&mut self) -> BitEnFreecntW<'_, RegFreerunCntP2poffDisTxtimeSpec> {
        BitEnFreecntW::new(self, 13)
    }
    #[doc = "Bits 24:31 - The time of the disable TX period after the P2P RF off interrupt occur. Unit: 64us."]
    #[inline(always)]
    pub fn bit_p2poff_dis_txtime(
        &mut self,
    ) -> BitP2poffDisTxtimeW<'_, RegFreerunCntP2poffDisTxtimeSpec> {
        BitP2poffDisTxtimeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_freerun_cnt_p2poff_dis_txtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_freerun_cnt_p2poff_dis_txtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFreerunCntP2poffDisTxtimeSpec;
impl crate::RegisterSpec for RegFreerunCntP2poffDisTxtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_freerun_cnt_p2poff_dis_txtime::R`](R) reader structure"]
impl crate::Readable for RegFreerunCntP2poffDisTxtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_freerun_cnt_p2poff_dis_txtime::W`](W) writer structure"]
impl crate::Writable for RegFreerunCntP2poffDisTxtimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FREERUN_CNT_P2POFF_DIS_TXTIME to value 0"]
impl crate::Resettable for RegFreerunCntP2poffDisTxtimeSpec {}
