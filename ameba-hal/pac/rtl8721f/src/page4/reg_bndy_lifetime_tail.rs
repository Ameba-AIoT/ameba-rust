#[doc = "Register `REG_BNDY_LIFETIME_TAIL` reader"]
pub type R = crate::R<RegBndyLifetimeTailSpec>;
#[doc = "Register `REG_BNDY_LIFETIME_TAIL` writer"]
pub type W = crate::W<RegBndyLifetimeTailSpec>;
#[doc = "Field `BIT_BCNQ_PGBNDY` reader - TXDMA will pull BCNQ Pkt in this address."]
pub type BitBcnqPgbndyR = crate::FieldReader;
#[doc = "Field `BIT_BCNQ_PGBNDY` writer - TXDMA will pull BCNQ Pkt in this address."]
pub type BitBcnqPgbndyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_MGQ_PGBNDY` reader - TXDMA will pull CPU MGQ Pkt in this address."]
pub type BitMgqPgbndyR = crate::FieldReader;
#[doc = "Field `BIT_MGQ_PGBNDY` writer - TXDMA will pull CPU MGQ Pkt in this address."]
pub type BitMgqPgbndyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_LIFETIME_VO_EN` reader - Enable VO queue lifetime function"]
pub type BitLifetimeVoEnR = crate::BitReader;
#[doc = "Field `BIT_LIFETIME_VO_EN` writer - Enable VO queue lifetime function"]
pub type BitLifetimeVoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LIFETIME_VI_EN` reader - Enable VI queue lifetime function"]
pub type BitLifetimeViEnR = crate::BitReader;
#[doc = "Field `BIT_LIFETIME_VI_EN` writer - Enable VI queue lifetime function"]
pub type BitLifetimeViEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LIFETIME_BE_EN` reader - Enable BE queue lifetime function"]
pub type BitLifetimeBeEnR = crate::BitReader;
#[doc = "Field `BIT_LIFETIME_BE_EN` writer - Enable BE queue lifetime function"]
pub type BitLifetimeBeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LIFETIME_BK_EN` reader - Enable BK queue lifetime function"]
pub type BitLifetimeBkEnR = crate::BitReader;
#[doc = "Field `BIT_LIFETIME_BK_EN` writer - Enable BK queue lifetime function"]
pub type BitLifetimeBkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RTYBIT_CTRL_EN` reader - 1: Enable Retry Count 0: Disable"]
pub type BitRtybitCtrlEnR = crate::BitReader;
#[doc = "Field `BIT_RTYBIT_CTRL_EN` writer - 1: Enable Retry Count 0: Disable"]
pub type BitRtybitCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HW_SSN_EN_EVTQ` reader - HW update the SSN \\[8\\] for eventq"]
pub type BitHwSsnEnEvtqR = crate::BitReader;
#[doc = "Field `BIT_HW_SSN_EN_EVTQ` writer - HW update the SSN \\[8\\] for eventq"]
pub type BitHwSsnEnEvtqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BT_INT_TYPE` reader - BT Interrupt type 01: PTA 10: CPU"]
pub type BitBtIntTypeR = crate::FieldReader;
#[doc = "Field `BIT_BT_INT_TYPE` writer - BT Interrupt type 01: PTA 10: CPU"]
pub type BitBtIntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_FW_Free_Tail` reader - Free Tail Page Number"]
pub type BitFwFreeTailR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TXDMA will pull BCNQ Pkt in this address."]
    #[inline(always)]
    pub fn bit_bcnq_pgbndy(&self) -> BitBcnqPgbndyR {
        BitBcnqPgbndyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TXDMA will pull CPU MGQ Pkt in this address."]
    #[inline(always)]
    pub fn bit_mgq_pgbndy(&self) -> BitMgqPgbndyR {
        BitMgqPgbndyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable VO queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_vo_en(&self) -> BitLifetimeVoEnR {
        BitLifetimeVoEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable VI queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_vi_en(&self) -> BitLifetimeViEnR {
        BitLifetimeViEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable BE queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_be_en(&self) -> BitLifetimeBeEnR {
        BitLifetimeBeEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable BK queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_bk_en(&self) -> BitLifetimeBkEnR {
        BitLifetimeBkEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable Retry Count 0: Disable"]
    #[inline(always)]
    pub fn bit_rtybit_ctrl_en(&self) -> BitRtybitCtrlEnR {
        BitRtybitCtrlEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HW update the SSN \\[8\\] for eventq"]
    #[inline(always)]
    pub fn bit_hw_ssn_en_evtq(&self) -> BitHwSsnEnEvtqR {
        BitHwSsnEnEvtqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - BT Interrupt type 01: PTA 10: CPU"]
    #[inline(always)]
    pub fn bit_bt_int_type(&self) -> BitBtIntTypeR {
        BitBtIntTypeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Free Tail Page Number"]
    #[inline(always)]
    pub fn bit_fw_free_tail(&self) -> BitFwFreeTailR {
        BitFwFreeTailR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXDMA will pull BCNQ Pkt in this address."]
    #[inline(always)]
    pub fn bit_bcnq_pgbndy(&mut self) -> BitBcnqPgbndyW<'_, RegBndyLifetimeTailSpec> {
        BitBcnqPgbndyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TXDMA will pull CPU MGQ Pkt in this address."]
    #[inline(always)]
    pub fn bit_mgq_pgbndy(&mut self) -> BitMgqPgbndyW<'_, RegBndyLifetimeTailSpec> {
        BitMgqPgbndyW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable VO queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_vo_en(&mut self) -> BitLifetimeVoEnW<'_, RegBndyLifetimeTailSpec> {
        BitLifetimeVoEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable VI queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_vi_en(&mut self) -> BitLifetimeViEnW<'_, RegBndyLifetimeTailSpec> {
        BitLifetimeViEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable BE queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_be_en(&mut self) -> BitLifetimeBeEnW<'_, RegBndyLifetimeTailSpec> {
        BitLifetimeBeEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable BK queue lifetime function"]
    #[inline(always)]
    pub fn bit_lifetime_bk_en(&mut self) -> BitLifetimeBkEnW<'_, RegBndyLifetimeTailSpec> {
        BitLifetimeBkEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Enable Retry Count 0: Disable"]
    #[inline(always)]
    pub fn bit_rtybit_ctrl_en(&mut self) -> BitRtybitCtrlEnW<'_, RegBndyLifetimeTailSpec> {
        BitRtybitCtrlEnW::new(self, 20)
    }
    #[doc = "Bit 21 - HW update the SSN \\[8\\] for eventq"]
    #[inline(always)]
    pub fn bit_hw_ssn_en_evtq(&mut self) -> BitHwSsnEnEvtqW<'_, RegBndyLifetimeTailSpec> {
        BitHwSsnEnEvtqW::new(self, 21)
    }
    #[doc = "Bits 22:23 - BT Interrupt type 01: PTA 10: CPU"]
    #[inline(always)]
    pub fn bit_bt_int_type(&mut self) -> BitBtIntTypeW<'_, RegBndyLifetimeTailSpec> {
        BitBtIntTypeW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bndy_lifetime_tail::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bndy_lifetime_tail::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBndyLifetimeTailSpec;
impl crate::RegisterSpec for RegBndyLifetimeTailSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bndy_lifetime_tail::R`](R) reader structure"]
impl crate::Readable for RegBndyLifetimeTailSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bndy_lifetime_tail::W`](W) writer structure"]
impl crate::Writable for RegBndyLifetimeTailSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BNDY_LIFETIME_TAIL to value 0xf510_f6f6"]
impl crate::Resettable for RegBndyLifetimeTailSpec {
    const RESET_VALUE: u32 = 0xf510_f6f6;
}
