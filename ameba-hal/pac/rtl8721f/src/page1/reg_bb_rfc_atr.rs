#[doc = "Register `REG_BB_RFC_ATR` reader"]
pub type R = crate::R<RegBbRfcAtrSpec>;
#[doc = "Register `REG_BB_RFC_ATR` writer"]
pub type W = crate::W<RegBbRfcAtrSpec>;
#[doc = "Field `BIT_PHYRD_WAIT_CNT` reader - the timing RDATA stable after ADDR for host/cpu access BB/RFC REG"]
pub type BitPhyrdWaitCntR = crate::FieldReader;
#[doc = "Field `BIT_PHYRD_WAIT_CNT` writer - the timing RDATA stable after ADDR for host/cpu access BB/RFC REG"]
pub type BitPhyrdWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PHYWR_HOLD_CNT` reader - the timing WDATA stable after WE for host/cpu access BB/RFC REG"]
pub type BitPhywrHoldCntR = crate::FieldReader;
#[doc = "Field `BIT_PHYWR_HOLD_CNT` writer - the timing WDATA stable after WE for host/cpu access BB/RFC REG"]
pub type BitPhywrHoldCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PHYWR_SETUP_CNT` reader - the timing WDATA stable before WE for host/cpu access BB/RFC REG"]
pub type BitPhywrSetupCntR = crate::FieldReader;
#[doc = "Field `BIT_PHYWR_SETUP_CNT` writer - the timing WDATA stable before WE for host/cpu access BB/RFC REG"]
pub type BitPhywrSetupCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PHYRE_CNT` reader - the timing after RE for host/cpu access BB/RFC REG"]
pub type BitPhyreCntR = crate::FieldReader;
#[doc = "Field `BIT_PHYRE_CNT` writer - the timing after RE for host/cpu access BB/RFC REG"]
pub type BitPhyreCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the timing RDATA stable after ADDR for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phyrd_wait_cnt(&self) -> BitPhyrdWaitCntR {
        BitPhyrdWaitCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the timing WDATA stable after WE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phywr_hold_cnt(&self) -> BitPhywrHoldCntR {
        BitPhywrHoldCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - the timing WDATA stable before WE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phywr_setup_cnt(&self) -> BitPhywrSetupCntR {
        BitPhywrSetupCntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - the timing after RE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phyre_cnt(&self) -> BitPhyreCntR {
        BitPhyreCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the timing RDATA stable after ADDR for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phyrd_wait_cnt(&mut self) -> BitPhyrdWaitCntW<'_, RegBbRfcAtrSpec> {
        BitPhyrdWaitCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - the timing WDATA stable after WE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phywr_hold_cnt(&mut self) -> BitPhywrHoldCntW<'_, RegBbRfcAtrSpec> {
        BitPhywrHoldCntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - the timing WDATA stable before WE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phywr_setup_cnt(&mut self) -> BitPhywrSetupCntW<'_, RegBbRfcAtrSpec> {
        BitPhywrSetupCntW::new(self, 16)
    }
    #[doc = "Bits 24:31 - the timing after RE for host/cpu access BB/RFC REG"]
    #[inline(always)]
    pub fn bit_phyre_cnt(&mut self) -> BitPhyreCntW<'_, RegBbRfcAtrSpec> {
        BitPhyreCntW::new(self, 24)
    }
}
#[doc = "BB/RFC REG Access Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bb_rfc_atr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bb_rfc_atr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBbRfcAtrSpec;
impl crate::RegisterSpec for RegBbRfcAtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bb_rfc_atr::R`](R) reader structure"]
impl crate::Readable for RegBbRfcAtrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bb_rfc_atr::W`](W) writer structure"]
impl crate::Writable for RegBbRfcAtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BB_RFC_ATR to value 0x0705_0406"]
impl crate::Resettable for RegBbRfcAtrSpec {
    const RESET_VALUE: u32 = 0x0705_0406;
}
