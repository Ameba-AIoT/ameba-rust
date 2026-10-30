#[doc = "Register `REG_UPLL_CTRL1` reader"]
pub type R = crate::R<RegUpllCtrl1Spec>;
#[doc = "Register `REG_UPLL_CTRL1` writer"]
pub type W = crate::W<RegUpllCtrl1Spec>;
#[doc = "Field `upll_n_pll_top` reader - (n+2) divider's divisor"]
pub type UpllNPllTopR = crate::FieldReader;
#[doc = "Field `upll_n_pll_top` writer - (n+2) divider's divisor"]
pub type UpllNPllTopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `upll_cmu_prediv` reader - Division-ratio of pre-divider for PLL clock in 11: fPLLCKIN pre-divided-by-8 10: fPLLCKIN pre-divided-by-4 01: fPLLCKIN pre-divided-by-2 00: fPLLCKIN pre-divided-by-1 fPLLCKIN: frequency of PLLCKIN"]
pub type UpllCmuPredivR = crate::FieldReader;
#[doc = "Field `upll_cmu_prediv` writer - Division-ratio of pre-divider for PLL clock in 11: fPLLCKIN pre-divided-by-8 10: fPLLCKIN pre-divided-by-4 01: fPLLCKIN pre-divided-by-2 00: fPLLCKIN pre-divided-by-1 fPLLCKIN: frequency of PLLCKIN"]
pub type UpllCmuPredivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `upll_cmu_cco_sel` reader - 0: VCO mode 1: CCO mode"]
pub type UpllCmuCcoSelR = crate::BitReader;
#[doc = "Field `upll_cmu_cco_sel` writer - 0: VCO mode 1: CCO mode"]
pub type UpllCmuCcoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upll_cmu_div_cyc` reader - Pulse width of divider output clock, adjust for PI monotonic phase transfer 0: 3/480e6 cycle is logic 1 1: 6/480e6 cycle is logic 1"]
pub type UpllCmuDivCycR = crate::BitReader;
#[doc = "Field `upll_cmu_div_cyc` writer - Pulse width of divider output clock, adjust for PI monotonic phase transfer 0: 3/480e6 cycle is logic 1 1: 6/480e6 cycle is logic 1"]
pub type UpllCmuDivCycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upll_cmu_ckrdy_bypass` reader - Bypass the time delay of CKRDY 1: Bypass CKRDY 0: PLL480M clock output after ∆T ∆T: delay time, refer to REG_CMU_CKRDY"]
pub type UpllCmuCkrdyBypassR = crate::BitReader;
#[doc = "Field `upll_cmu_ckrdy_bypass` writer - Bypass the time delay of CKRDY 1: Bypass CKRDY 0: PLL480M clock output after ∆T ∆T: delay time, refer to REG_CMU_CKRDY"]
pub type UpllCmuCkrdyBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upll_cmu_ckrdy` reader - PLL480M clock output after ∆T for waiting 480MHz clock ready, ∆T: 000: 21*TPLLCKIN, 001: 22*TPLLCKIN 010: 25*TPLLCKIN, 011: 27*TPLLCKIN 100: 210*TPLLCKIN, 101: 212*TPLLCKIN 110: 216*TPLLCKIN, 111: 3*216*TPLLCKIN TPLLCKIN: period of PLLCKIN"]
pub type UpllCmuCkrdyR = crate::FieldReader;
#[doc = "Field `upll_cmu_ckrdy` writer - PLL480M clock output after ∆T for waiting 480MHz clock ready, ∆T: 000: 21*TPLLCKIN, 001: 22*TPLLCKIN 010: 25*TPLLCKIN, 011: 27*TPLLCKIN 100: 210*TPLLCKIN, 101: 212*TPLLCKIN 110: 216*TPLLCKIN, 111: 3*216*TPLLCKIN TPLLCKIN: period of PLLCKIN"]
pub type UpllCmuCkrdyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `upll_cmu_big_kvco` reader - Control Kvco of CCO, adjust for bandwidth & jitter 0: Smaller KVCO 1: Larger KVCO Kvco(Hz/V): the frequency change per 1V of CCO"]
pub type UpllCmuBigKvcoR = crate::BitReader;
#[doc = "Field `upll_cmu_big_kvco` writer - Control Kvco of CCO, adjust for bandwidth & jitter 0: Smaller KVCO 1: Larger KVCO Kvco(Hz/V): the frequency change per 1V of CCO"]
pub type UpllCmuBigKvcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upll_cmu_icp` reader - Charge-Pump current, adjust for PLL bandwidth 00: 5uA 01: 10uA 10: 15uA 11: 20uA"]
pub type UpllCmuIcpR = crate::FieldReader;
#[doc = "Field `upll_cmu_icp` writer - Charge-Pump current, adjust for PLL bandwidth 00: 5uA 01: 10uA 10: 15uA 11: 20uA"]
pub type UpllCmuIcpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `upll_cmu_lf_r` reader - Resistance of loop filter, adjust for PLL bandwidth 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
pub type UpllCmuLfRR = crate::FieldReader;
#[doc = "Field `upll_cmu_lf_r` writer - Resistance of loop filter, adjust for PLL bandwidth 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
pub type UpllCmuLfRW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `upll_ckref_sel` reader - ECO, REG_CKREF_SEL for usbphy, reset default 1"]
pub type UpllCkrefSelR = crate::BitReader;
#[doc = "Field `upll_ckref_sel` writer - ECO, REG_CKREF_SEL for usbphy, reset default 1"]
pub type UpllCkrefSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - (n+2) divider's divisor"]
    #[inline(always)]
    pub fn upll_n_pll_top(&self) -> UpllNPllTopR {
        UpllNPllTopR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Division-ratio of pre-divider for PLL clock in 11: fPLLCKIN pre-divided-by-8 10: fPLLCKIN pre-divided-by-4 01: fPLLCKIN pre-divided-by-2 00: fPLLCKIN pre-divided-by-1 fPLLCKIN: frequency of PLLCKIN"]
    #[inline(always)]
    pub fn upll_cmu_prediv(&self) -> UpllCmuPredivR {
        UpllCmuPredivR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 0: VCO mode 1: CCO mode"]
    #[inline(always)]
    pub fn upll_cmu_cco_sel(&self) -> UpllCmuCcoSelR {
        UpllCmuCcoSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pulse width of divider output clock, adjust for PI monotonic phase transfer 0: 3/480e6 cycle is logic 1 1: 6/480e6 cycle is logic 1"]
    #[inline(always)]
    pub fn upll_cmu_div_cyc(&self) -> UpllCmuDivCycR {
        UpllCmuDivCycR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bypass the time delay of CKRDY 1: Bypass CKRDY 0: PLL480M clock output after ∆T ∆T: delay time, refer to REG_CMU_CKRDY"]
    #[inline(always)]
    pub fn upll_cmu_ckrdy_bypass(&self) -> UpllCmuCkrdyBypassR {
        UpllCmuCkrdyBypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - PLL480M clock output after ∆T for waiting 480MHz clock ready, ∆T: 000: 21*TPLLCKIN, 001: 22*TPLLCKIN 010: 25*TPLLCKIN, 011: 27*TPLLCKIN 100: 210*TPLLCKIN, 101: 212*TPLLCKIN 110: 216*TPLLCKIN, 111: 3*216*TPLLCKIN TPLLCKIN: period of PLLCKIN"]
    #[inline(always)]
    pub fn upll_cmu_ckrdy(&self) -> UpllCmuCkrdyR {
        UpllCmuCkrdyR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Control Kvco of CCO, adjust for bandwidth & jitter 0: Smaller KVCO 1: Larger KVCO Kvco(Hz/V): the frequency change per 1V of CCO"]
    #[inline(always)]
    pub fn upll_cmu_big_kvco(&self) -> UpllCmuBigKvcoR {
        UpllCmuBigKvcoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Charge-Pump current, adjust for PLL bandwidth 00: 5uA 01: 10uA 10: 15uA 11: 20uA"]
    #[inline(always)]
    pub fn upll_cmu_icp(&self) -> UpllCmuIcpR {
        UpllCmuIcpR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - Resistance of loop filter, adjust for PLL bandwidth 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
    #[inline(always)]
    pub fn upll_cmu_lf_r(&self) -> UpllCmuLfRR {
        UpllCmuLfRR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 31 - ECO, REG_CKREF_SEL for usbphy, reset default 1"]
    #[inline(always)]
    pub fn upll_ckref_sel(&self) -> UpllCkrefSelR {
        UpllCkrefSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - (n+2) divider's divisor"]
    #[inline(always)]
    pub fn upll_n_pll_top(&mut self) -> UpllNPllTopW<'_, RegUpllCtrl1Spec> {
        UpllNPllTopW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Division-ratio of pre-divider for PLL clock in 11: fPLLCKIN pre-divided-by-8 10: fPLLCKIN pre-divided-by-4 01: fPLLCKIN pre-divided-by-2 00: fPLLCKIN pre-divided-by-1 fPLLCKIN: frequency of PLLCKIN"]
    #[inline(always)]
    pub fn upll_cmu_prediv(&mut self) -> UpllCmuPredivW<'_, RegUpllCtrl1Spec> {
        UpllCmuPredivW::new(self, 8)
    }
    #[doc = "Bit 10 - 0: VCO mode 1: CCO mode"]
    #[inline(always)]
    pub fn upll_cmu_cco_sel(&mut self) -> UpllCmuCcoSelW<'_, RegUpllCtrl1Spec> {
        UpllCmuCcoSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Pulse width of divider output clock, adjust for PI monotonic phase transfer 0: 3/480e6 cycle is logic 1 1: 6/480e6 cycle is logic 1"]
    #[inline(always)]
    pub fn upll_cmu_div_cyc(&mut self) -> UpllCmuDivCycW<'_, RegUpllCtrl1Spec> {
        UpllCmuDivCycW::new(self, 11)
    }
    #[doc = "Bit 12 - Bypass the time delay of CKRDY 1: Bypass CKRDY 0: PLL480M clock output after ∆T ∆T: delay time, refer to REG_CMU_CKRDY"]
    #[inline(always)]
    pub fn upll_cmu_ckrdy_bypass(&mut self) -> UpllCmuCkrdyBypassW<'_, RegUpllCtrl1Spec> {
        UpllCmuCkrdyBypassW::new(self, 12)
    }
    #[doc = "Bits 13:15 - PLL480M clock output after ∆T for waiting 480MHz clock ready, ∆T: 000: 21*TPLLCKIN, 001: 22*TPLLCKIN 010: 25*TPLLCKIN, 011: 27*TPLLCKIN 100: 210*TPLLCKIN, 101: 212*TPLLCKIN 110: 216*TPLLCKIN, 111: 3*216*TPLLCKIN TPLLCKIN: period of PLLCKIN"]
    #[inline(always)]
    pub fn upll_cmu_ckrdy(&mut self) -> UpllCmuCkrdyW<'_, RegUpllCtrl1Spec> {
        UpllCmuCkrdyW::new(self, 13)
    }
    #[doc = "Bit 16 - Control Kvco of CCO, adjust for bandwidth & jitter 0: Smaller KVCO 1: Larger KVCO Kvco(Hz/V): the frequency change per 1V of CCO"]
    #[inline(always)]
    pub fn upll_cmu_big_kvco(&mut self) -> UpllCmuBigKvcoW<'_, RegUpllCtrl1Spec> {
        UpllCmuBigKvcoW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Charge-Pump current, adjust for PLL bandwidth 00: 5uA 01: 10uA 10: 15uA 11: 20uA"]
    #[inline(always)]
    pub fn upll_cmu_icp(&mut self) -> UpllCmuIcpW<'_, RegUpllCtrl1Spec> {
        UpllCmuIcpW::new(self, 17)
    }
    #[doc = "Bits 19:21 - Resistance of loop filter, adjust for PLL bandwidth 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
    #[inline(always)]
    pub fn upll_cmu_lf_r(&mut self) -> UpllCmuLfRW<'_, RegUpllCtrl1Spec> {
        UpllCmuLfRW::new(self, 19)
    }
    #[doc = "Bit 31 - ECO, REG_CKREF_SEL for usbphy, reset default 1"]
    #[inline(always)]
    pub fn upll_ckref_sel(&mut self) -> UpllCkrefSelW<'_, RegUpllCtrl1Spec> {
        UpllCkrefSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_upll_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_upll_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUpllCtrl1Spec;
impl crate::RegisterSpec for RegUpllCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_upll_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegUpllCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_upll_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegUpllCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UPLL_CTRL1 to value 0x801a_ac16"]
impl crate::Resettable for RegUpllCtrl1Spec {
    const RESET_VALUE: u32 = 0x801a_ac16;
}
