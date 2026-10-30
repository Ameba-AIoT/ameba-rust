#[doc = "Register `REG_PS_TIMER_TWT_CRTL` reader"]
pub type R = crate::R<RegPsTimerTwtCrtlSpec>;
#[doc = "Register `REG_PS_TIMER_TWT_CRTL` writer"]
pub type W = crate::W<RegPsTimerTwtCrtlSpec>;
#[doc = "Field `BIT_RXBCN_TIME_PORT` reader - select port for report the TSFT 0: port0 1: port1 default is port0."]
pub type BitRxbcnTimePortR = crate::BitReader;
#[doc = "Field `BIT_RXBCN_TIME_PORT` writer - select port for report the TSFT 0: port0 1: port1 default is port0."]
pub type BitRxbcnTimePortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GET_TSFT_OFFSET_EN` reader - 1: Enable Enable get TSFT offset function"]
pub type BitGetTsftOffsetEnR = crate::BitReader;
#[doc = "Field `BIT_GET_TSFT_OFFSET_EN` writer - 1: Enable Enable get TSFT offset function"]
pub type BitGetTsftOffsetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFT_DIFF_P0_EN` reader - "]
pub type BitTsftDiffP0EnR = crate::BitReader;
#[doc = "Field `BIT_TSFT_DIFF_P0_EN` writer - "]
pub type BitTsftDiffP0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PS_TIMER_E_TSF_SEL` reader - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerETsfSelR = crate::FieldReader;
#[doc = "Field `BIT_PS_TIMER_E_TSF_SEL` writer - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerETsfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_PS_TIMER_E_EN` reader - Enable PS_TIMER_E"]
pub type BitPsTimerEEnR = crate::BitReader;
#[doc = "Field `BIT_PS_TIMER_E_EN` writer - Enable PS_TIMER_E"]
pub type BitPsTimerEEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PS_TIMER_D_TSF_SEL` reader - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerDTsfSelR = crate::FieldReader;
#[doc = "Field `BIT_PS_TIMER_D_TSF_SEL` writer - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerDTsfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_PS_TIMER_D_EN` reader - Enable PS_TIMER_D"]
pub type BitPsTimerDEnR = crate::BitReader;
#[doc = "Field `BIT_PS_TIMER_D_EN` writer - Enable PS_TIMER_D"]
pub type BitPsTimerDEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PS_TIMER_C_TSF_SEL` reader - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerCTsfSelR = crate::FieldReader;
#[doc = "Field `BIT_PS_TIMER_C_TSF_SEL` writer - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerCTsfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_PS_TIMER_C_EN` reader - Enable PS_TIMER_C"]
pub type BitPsTimerCEnR = crate::BitReader;
#[doc = "Field `BIT_PS_TIMER_C_EN` writer - Enable PS_TIMER_C"]
pub type BitPsTimerCEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PS_TIMER_B_TSF_SEL` reader - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerBTsfSelR = crate::FieldReader;
#[doc = "Field `BIT_PS_TIMER_B_TSF_SEL` writer - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerBTsfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_PS_TIMER_B_EN` reader - Enable PS_TIMER_A"]
pub type BitPsTimerBEnR = crate::BitReader;
#[doc = "Field `BIT_PS_TIMER_B_EN` writer - Enable PS_TIMER_A"]
pub type BitPsTimerBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PS_TIMER_A_TSF_SEL` reader - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerATsfSelR = crate::FieldReader;
#[doc = "Field `BIT_PS_TIMER_A_TSF_SEL` writer - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
pub type BitPsTimerATsfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_PS_TIMER_A_EN` reader - Enable PS_TIMER_B"]
pub type BitPsTimerAEnR = crate::BitReader;
#[doc = "Field `BIT_PS_TIMER_A_EN` writer - Enable PS_TIMER_B"]
pub type BitPsTimerAEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - select port for report the TSFT 0: port0 1: port1 default is port0."]
    #[inline(always)]
    pub fn bit_rxbcn_time_port(&self) -> BitRxbcnTimePortR {
        BitRxbcnTimePortR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable Enable get TSFT offset function"]
    #[inline(always)]
    pub fn bit_get_tsft_offset_en(&self) -> BitGetTsftOffsetEnR {
        BitGetTsftOffsetEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_tsft_diff_p0_en(&self) -> BitTsftDiffP0EnR {
        BitTsftDiffP0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_e_tsf_sel(&self) -> BitPsTimerETsfSelR {
        BitPsTimerETsfSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Enable PS_TIMER_E"]
    #[inline(always)]
    pub fn bit_ps_timer_e_en(&self) -> BitPsTimerEEnR {
        BitPsTimerEEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_d_tsf_sel(&self) -> BitPsTimerDTsfSelR {
        BitPsTimerDTsfSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Enable PS_TIMER_D"]
    #[inline(always)]
    pub fn bit_ps_timer_d_en(&self) -> BitPsTimerDEnR {
        BitPsTimerDEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_c_tsf_sel(&self) -> BitPsTimerCTsfSelR {
        BitPsTimerCTsfSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Enable PS_TIMER_C"]
    #[inline(always)]
    pub fn bit_ps_timer_c_en(&self) -> BitPsTimerCEnR {
        BitPsTimerCEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_b_tsf_sel(&self) -> BitPsTimerBTsfSelR {
        BitPsTimerBTsfSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - Enable PS_TIMER_A"]
    #[inline(always)]
    pub fn bit_ps_timer_b_en(&self) -> BitPsTimerBEnR {
        BitPsTimerBEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_a_tsf_sel(&self) -> BitPsTimerATsfSelR {
        BitPsTimerATsfSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable PS_TIMER_B"]
    #[inline(always)]
    pub fn bit_ps_timer_a_en(&self) -> BitPsTimerAEnR {
        BitPsTimerAEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - select port for report the TSFT 0: port0 1: port1 default is port0."]
    #[inline(always)]
    pub fn bit_rxbcn_time_port(&mut self) -> BitRxbcnTimePortW<'_, RegPsTimerTwtCrtlSpec> {
        BitRxbcnTimePortW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable Enable get TSFT offset function"]
    #[inline(always)]
    pub fn bit_get_tsft_offset_en(&mut self) -> BitGetTsftOffsetEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitGetTsftOffsetEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_tsft_diff_p0_en(&mut self) -> BitTsftDiffP0EnW<'_, RegPsTimerTwtCrtlSpec> {
        BitTsftDiffP0EnW::new(self, 2)
    }
    #[doc = "Bits 12:13 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_e_tsf_sel(&mut self) -> BitPsTimerETsfSelW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerETsfSelW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable PS_TIMER_E"]
    #[inline(always)]
    pub fn bit_ps_timer_e_en(&mut self) -> BitPsTimerEEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerEEnW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_d_tsf_sel(&mut self) -> BitPsTimerDTsfSelW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerDTsfSelW::new(self, 16)
    }
    #[doc = "Bit 19 - Enable PS_TIMER_D"]
    #[inline(always)]
    pub fn bit_ps_timer_d_en(&mut self) -> BitPsTimerDEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerDEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_c_tsf_sel(&mut self) -> BitPsTimerCTsfSelW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerCTsfSelW::new(self, 20)
    }
    #[doc = "Bit 23 - Enable PS_TIMER_C"]
    #[inline(always)]
    pub fn bit_ps_timer_c_en(&mut self) -> BitPsTimerCEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerCEnW::new(self, 23)
    }
    #[doc = "Bits 24:25 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_b_tsf_sel(&mut self) -> BitPsTimerBTsfSelW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerBTsfSelW::new(self, 24)
    }
    #[doc = "Bit 27 - Enable PS_TIMER_A"]
    #[inline(always)]
    pub fn bit_ps_timer_b_en(&mut self) -> BitPsTimerBEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerBEnW::new(self, 27)
    }
    #[doc = "Bits 28:29 - 0 for port 0, 1 for port1, 2 for port2, 3 for freerun"]
    #[inline(always)]
    pub fn bit_ps_timer_a_tsf_sel(&mut self) -> BitPsTimerATsfSelW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerATsfSelW::new(self, 28)
    }
    #[doc = "Bit 31 - Enable PS_TIMER_B"]
    #[inline(always)]
    pub fn bit_ps_timer_a_en(&mut self) -> BitPsTimerAEnW<'_, RegPsTimerTwtCrtlSpec> {
        BitPsTimerAEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_twt_crtl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_twt_crtl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerTwtCrtlSpec;
impl crate::RegisterSpec for RegPsTimerTwtCrtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_twt_crtl::R`](R) reader structure"]
impl crate::Readable for RegPsTimerTwtCrtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_twt_crtl::W`](W) writer structure"]
impl crate::Writable for RegPsTimerTwtCrtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_TWT_CRTL to value 0"]
impl crate::Resettable for RegPsTimerTwtCrtlSpec {}
