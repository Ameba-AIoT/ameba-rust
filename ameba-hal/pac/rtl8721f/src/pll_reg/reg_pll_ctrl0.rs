#[doc = "Register `REG_PLL_CTRL0` reader"]
pub type R = crate::R<RegPllCtrl0Spec>;
#[doc = "Register `REG_PLL_CTRL0` writer"]
pub type W = crate::W<RegPllCtrl0Spec>;
#[doc = "Field `REG_PREDIV` reader - Pre-divides the PLL input reference frequency ckin: 00:ckin/1 01:ckin/2 10:ckin/4 11:ckin/8"]
pub type RegPredivR = crate::FieldReader;
#[doc = "Field `REG_PREDIV` writer - Pre-divides the PLL input reference frequency ckin: 00:ckin/1 01:ckin/2 10:ckin/4 11:ckin/8"]
pub type RegPredivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_CMU_EN` reader - PLL enable signal"]
pub type RegCmuEnR = crate::BitReader;
#[doc = "Field `REG_CMU_EN` writer - PLL enable signal"]
pub type RegCmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CCO_SEL` reader - Selects CCO mode or VCO mode 0: vco mode 1: cco mode"]
pub type RegCcoSelR = crate::BitReader;
#[doc = "Field `REG_CCO_SEL` writer - Selects CCO mode or VCO mode 0: vco mode 1: cco mode"]
pub type RegCcoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_BIG_CCO` reader - Controls the KVCO curve slope"]
pub type RegBigCcoR = crate::BitReader;
#[doc = "Field `REG_BIG_CCO` writer - Controls the KVCO curve slope"]
pub type RegBigCcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ICP` reader - CP current ICP selection: 00:ICP=5u 01:ICP=10u 10:ICP=15u 11:ICP=20u"]
pub type RegIcpR = crate::FieldReader;
#[doc = "Field `REG_ICP` writer - CP current ICP selection: 00:ICP=5u 01:ICP=10u 10:ICP=15u 11:ICP=20u"]
pub type RegIcpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_LPF_CP` reader - LPF small capacitor CS selection: 0:CS=1p 1:CS=2p"]
pub type RegLpfCpR = crate::BitReader;
#[doc = "Field `REG_LPF_CP` writer - LPF small capacitor CS selection: 0:CS=1p 1:CS=2p"]
pub type RegLpfCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LF_R` reader - LPF resistor RS selection: 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
pub type RegLfRR = crate::FieldReader;
#[doc = "Field `REG_LF_R` writer - LPF resistor RS selection: 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
pub type RegLfRW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_CMU_PI_ISEL` reader - Selects the PI current ISEL magnitude: 0000:ICP=3.125u 1000:ICP=28.125u 0001:ICP=6.25u 1001:ICP=31.25u 0010:ICP=9.375u 1010:ICP=34.375u 0011:ICP=12.5u 1011:ICP=37.5u 0100:ICP=15.625u 1100:ICP=40.625u 0101:ICP=18.75u 1101:ICP=43.75u 0110:ICP=21.875u 1110:ICP=46.875u 0111:ICP=25u 1111:ICP=50u"]
pub type RegCmuPiIselR = crate::FieldReader;
#[doc = "Field `REG_CMU_PI_ISEL` writer - Selects the PI current ISEL magnitude: 0000:ICP=3.125u 1000:ICP=28.125u 0001:ICP=6.25u 1001:ICP=31.25u 0010:ICP=9.375u 1010:ICP=34.375u 0011:ICP=12.5u 1011:ICP=37.5u 0100:ICP=15.625u 1100:ICP=40.625u 0101:ICP=18.75u 1101:ICP=43.75u 0110:ICP=21.875u 1110:ICP=46.875u 0111:ICP=25u 1111:ICP=50u"]
pub type RegCmuPiIselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_TEST_EN` reader - Debug signal"]
pub type RegTestEnR = crate::BitReader;
#[doc = "Field `REG_TEST_EN` writer - Debug signal"]
pub type RegTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMU_DBG` reader - "]
pub type CmuDbgR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Pre-divides the PLL input reference frequency ckin: 00:ckin/1 01:ckin/2 10:ckin/4 11:ckin/8"]
    #[inline(always)]
    pub fn reg_prediv(&self) -> RegPredivR {
        RegPredivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - PLL enable signal"]
    #[inline(always)]
    pub fn reg_cmu_en(&self) -> RegCmuEnR {
        RegCmuEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects CCO mode or VCO mode 0: vco mode 1: cco mode"]
    #[inline(always)]
    pub fn reg_cco_sel(&self) -> RegCcoSelR {
        RegCcoSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the KVCO curve slope"]
    #[inline(always)]
    pub fn reg_big_cco(&self) -> RegBigCcoR {
        RegBigCcoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CP current ICP selection: 00:ICP=5u 01:ICP=10u 10:ICP=15u 11:ICP=20u"]
    #[inline(always)]
    pub fn reg_icp(&self) -> RegIcpR {
        RegIcpR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - LPF small capacitor CS selection: 0:CS=1p 1:CS=2p"]
    #[inline(always)]
    pub fn reg_lpf_cp(&self) -> RegLpfCpR {
        RegLpfCpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - LPF resistor RS selection: 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
    #[inline(always)]
    pub fn reg_lf_r(&self) -> RegLfRR {
        RegLfRR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Selects the PI current ISEL magnitude: 0000:ICP=3.125u 1000:ICP=28.125u 0001:ICP=6.25u 1001:ICP=31.25u 0010:ICP=9.375u 1010:ICP=34.375u 0011:ICP=12.5u 1011:ICP=37.5u 0100:ICP=15.625u 1100:ICP=40.625u 0101:ICP=18.75u 1101:ICP=43.75u 0110:ICP=21.875u 1110:ICP=46.875u 0111:ICP=25u 1111:ICP=50u"]
    #[inline(always)]
    pub fn reg_cmu_pi_isel(&self) -> RegCmuPiIselR {
        RegCmuPiIselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Debug signal"]
    #[inline(always)]
    pub fn reg_test_en(&self) -> RegTestEnR {
        RegTestEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn cmu_dbg(&self) -> CmuDbgR {
        CmuDbgR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pre-divides the PLL input reference frequency ckin: 00:ckin/1 01:ckin/2 10:ckin/4 11:ckin/8"]
    #[inline(always)]
    pub fn reg_prediv(&mut self) -> RegPredivW<'_, RegPllCtrl0Spec> {
        RegPredivW::new(self, 0)
    }
    #[doc = "Bit 2 - PLL enable signal"]
    #[inline(always)]
    pub fn reg_cmu_en(&mut self) -> RegCmuEnW<'_, RegPllCtrl0Spec> {
        RegCmuEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects CCO mode or VCO mode 0: vco mode 1: cco mode"]
    #[inline(always)]
    pub fn reg_cco_sel(&mut self) -> RegCcoSelW<'_, RegPllCtrl0Spec> {
        RegCcoSelW::new(self, 3)
    }
    #[doc = "Bit 4 - Controls the KVCO curve slope"]
    #[inline(always)]
    pub fn reg_big_cco(&mut self) -> RegBigCcoW<'_, RegPllCtrl0Spec> {
        RegBigCcoW::new(self, 4)
    }
    #[doc = "Bits 5:6 - CP current ICP selection: 00:ICP=5u 01:ICP=10u 10:ICP=15u 11:ICP=20u"]
    #[inline(always)]
    pub fn reg_icp(&mut self) -> RegIcpW<'_, RegPllCtrl0Spec> {
        RegIcpW::new(self, 5)
    }
    #[doc = "Bit 7 - LPF small capacitor CS selection: 0:CS=1p 1:CS=2p"]
    #[inline(always)]
    pub fn reg_lpf_cp(&mut self) -> RegLpfCpW<'_, RegPllCtrl0Spec> {
        RegLpfCpW::new(self, 7)
    }
    #[doc = "Bits 8:10 - LPF resistor RS selection: 000: 3.1KΩ , 001: 6.1KΩ , 010: 9.2KΩ , 011: 12.2KΩ, 100: 15.2KΩ, 101: 18.2KΩ, 110: 21.3KΩ, 111: 24.3KΩ"]
    #[inline(always)]
    pub fn reg_lf_r(&mut self) -> RegLfRW<'_, RegPllCtrl0Spec> {
        RegLfRW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Selects the PI current ISEL magnitude: 0000:ICP=3.125u 1000:ICP=28.125u 0001:ICP=6.25u 1001:ICP=31.25u 0010:ICP=9.375u 1010:ICP=34.375u 0011:ICP=12.5u 1011:ICP=37.5u 0100:ICP=15.625u 1100:ICP=40.625u 0101:ICP=18.75u 1101:ICP=43.75u 0110:ICP=21.875u 1110:ICP=46.875u 0111:ICP=25u 1111:ICP=50u"]
    #[inline(always)]
    pub fn reg_cmu_pi_isel(&mut self) -> RegCmuPiIselW<'_, RegPllCtrl0Spec> {
        RegCmuPiIselW::new(self, 12)
    }
    #[doc = "Bit 16 - Debug signal"]
    #[inline(always)]
    pub fn reg_test_en(&mut self) -> RegTestEnW<'_, RegPllCtrl0Spec> {
        RegTestEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pll_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pll_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPllCtrl0Spec;
impl crate::RegisterSpec for RegPllCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pll_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegPllCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pll_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegPllCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PLL_CTRL0 to value 0x3428"]
impl crate::Resettable for RegPllCtrl0Spec {
    const RESET_VALUE: u32 = 0x3428;
}
