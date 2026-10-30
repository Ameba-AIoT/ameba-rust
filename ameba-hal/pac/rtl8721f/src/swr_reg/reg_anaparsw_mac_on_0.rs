#[doc = "Register `REG_ANAPARSW_MAC_ON_0` reader"]
pub type R = crate::R<RegAnaparswMacOn0Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_0` writer"]
pub type W = crate::W<RegAnaparswMacOn0Spec>;
#[doc = "Field `VOL_L1` reader - VREF 0.6 0.635 0.835 0.865 Code VOUT VOUT VOUT VOUT 1111 1.028571 1.088571 1.431429 1.482857 1110 1 1.058333 1.391667 1.441667 1101 0.972973 1.02973 1.354054 1.402703 1100 0.947368 1.002632 1.318421 1.365789 1011 0.935065 0.98961 1.301299 1.348052 1010 0.923077 0.976923 1.284615 1.330769 1001 0.911392 0.964557 1.268354 1.313924 1000 0.9 0.9525 1.2525 1.2975 0111 0.888889 0.940741 1.237037 1.281481 0110 0.878049 0.929268 1.221951 1.265854 0101 0.86747 0.918072 1.207229 1.250602 0100 0.847059 0.896471 1.178824 1.221176 0011 0.827586 0.875862 1.151724 1.193103 0010 0.808989 0.85618 1.125843 1.166292 0001 0.791209 0.837363 1.101099 1.140659 0000 0.774194 0.819355 1.077419 1.116129"]
pub type VolL1R = crate::FieldReader;
#[doc = "Field `VOL_L1` writer - VREF 0.6 0.635 0.835 0.865 Code VOUT VOUT VOUT VOUT 1111 1.028571 1.088571 1.431429 1.482857 1110 1 1.058333 1.391667 1.441667 1101 0.972973 1.02973 1.354054 1.402703 1100 0.947368 1.002632 1.318421 1.365789 1011 0.935065 0.98961 1.301299 1.348052 1010 0.923077 0.976923 1.284615 1.330769 1001 0.911392 0.964557 1.268354 1.313924 1000 0.9 0.9525 1.2525 1.2975 0111 0.888889 0.940741 1.237037 1.281481 0110 0.878049 0.929268 1.221951 1.265854 0101 0.86747 0.918072 1.207229 1.250602 0100 0.847059 0.896471 1.178824 1.221176 0011 0.827586 0.875862 1.151724 1.193103 0010 0.808989 0.85618 1.125843 1.166292 0001 0.791209 0.837363 1.101099 1.140659 0000 0.774194 0.819355 1.077419 1.116129"]
pub type VolL1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STD_L1` reader - STD_L1<1:0> time 750u 1.5m 10 3m 11 6m"]
pub type StdL1R = crate::FieldReader;
#[doc = "Field `STD_L1` writer - STD_L1<1:0> time 750u 1.5m 10 3m 11 6m"]
pub type StdL1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FPWM_L1` reader - FPWM_L1 = 1 => force PWM FPWM_L1 = 0 => force PFM"]
pub type FpwmL1R = crate::BitReader;
#[doc = "Field `FPWM_L1` writer - FPWM_L1 = 1 => force PWM FPWM_L1 = 0 => force PFM"]
pub type FpwmL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bypass_L` reader - 1:bypass SSR latch"]
pub type RegBypassLR = crate::BitReader;
#[doc = "Field `reg_bypass_L` writer - 1:bypass SSR latch"]
pub type RegBypassLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAW_FREQ_L` reader - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLR = crate::FieldReader;
#[doc = "Field `POWOCP_L1` reader - POWOCP_L1=1=> enable over current protection"]
pub type PowocpL1R = crate::BitReader;
#[doc = "Field `POWOCP_L1` writer - POWOCP_L1=1=> enable over current protection"]
pub type PowocpL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCP_L1` reader - PWM current (A) PFM current (A) mP 000 0.533 0.105 001 0.580 0.145 010 0.630 0.185 011 0.680 0.223 100 0.730 0.263 101 0.780 0.303 110 0.830 0.342 111 0.890 0.290(VIN/10)"]
pub type OcpL1R = crate::FieldReader;
#[doc = "Field `OCP_L1` writer - PWM current (A) PFM current (A) mP 000 0.533 0.105 001 0.580 0.145 010 0.630 0.185 011 0.680 0.223 100 0.730 0.263 101 0.780 0.303 110 0.830 0.342 111 0.890 0.290(VIN/10)"]
pub type OcpL1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POW_ZCD_L` reader - Enable ZCD"]
pub type PowZcdLR = crate::BitReader;
#[doc = "Field `POW_ZCD_L` writer - Enable ZCD"]
pub type PowZcdLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_zcdc_H` reader - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHR = crate::FieldReader;
#[doc = "Field `ZCD_SDZ_L` reader - Increase ZCD delay cell"]
pub type ZcdSdzLR = crate::FieldReader;
#[doc = "Field `ZCD_SDZ_L` writer - Increase ZCD delay cell"]
pub type ZcdSdzLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Force_ZCD_Bias` reader - 0:ZCD bias on after SS 1:ZCD bias on when ZCD enable"]
pub type ForceZcdBiasR = crate::BitReader;
#[doc = "Field `Force_ZCD_Bias` writer - 0:ZCD bias on after SS 1:ZCD bias on when ZCD enable"]
pub type ForceZcdBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFPFM_L` reader - SWR/LDO SEL <93>=0 <93>=1 0000 0.495 0.458 0001 0.654 0.610 0010 0.707 0.662 0011 0.760 0.712 0100 0.795 0.746 0101 0.848 0.797 0110 0.883 0.831 0111 0.954 0.899 1000 1.042 0.984 1001 1.131 1.069 1010 1.219 1.153 1011 1.255 1.188 1100 1.308 1.238 1101 1.343 1.272 1110 1.414 1.340 1111 1.503 1.425"]
pub type VrefpfmLR = crate::FieldReader;
#[doc = "Field `VREFPFM_L` writer - SWR/LDO SEL <93>=0 <93>=1 0000 0.495 0.458 0001 0.654 0.610 0010 0.707 0.662 0011 0.760 0.712 0100 0.795 0.746 0101 0.848 0.797 0110 0.883 0.831 0111 0.954 0.899 1000 1.042 0.984 1001 1.131 1.069 1010 1.219 1.153 1011 1.255 1.188 1100 1.308 1.238 1101 1.343 1.272 1110 1.414 1.340 1111 1.503 1.425"]
pub type VrefpfmLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COT_I_L` reader - COT ON Current 00:150nA 01:150nA+100nA 10:150nA+150nA 11:150nA+250nA"]
pub type CotILR = crate::FieldReader;
#[doc = "Field `COT_I_L` writer - COT ON Current 00:150nA 01:150nA+100nA 10:150nA+150nA 11:150nA+250nA"]
pub type CotILW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POW_MINOFF_L` reader - 0:normal current 1:increase OCP cmp current"]
pub type PowMinoffLR = crate::BitReader;
#[doc = "Field `POW_MINOFF_L` writer - 0:normal current 1:increase OCP cmp current"]
pub type PowMinoffLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF_END_SEL` reader - COT off time controlled by, 0: ZCD 1: min off time"]
pub type OffEndSelR = crate::BitReader;
#[doc = "Field `OFF_END_SEL` writer - COT off time controlled by, 0: ZCD 1: min off time"]
pub type OffEndSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_HLOAD` reader - Enable Heavy load counter"]
pub type EnHloadR = crate::BitReader;
#[doc = "Field `EN_HLOAD` writer - Enable Heavy load counter"]
pub type EnHloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFMCMP_IQ` reader - PFM comparator bias, 0: 50nA 1:250nA"]
pub type PfmcmpIqR = crate::BitReader;
#[doc = "Field `PFMCMP_IQ` writer - PFM comparator bias, 0: 50nA 1:250nA"]
pub type PfmcmpIqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - VREF 0.6 0.635 0.835 0.865 Code VOUT VOUT VOUT VOUT 1111 1.028571 1.088571 1.431429 1.482857 1110 1 1.058333 1.391667 1.441667 1101 0.972973 1.02973 1.354054 1.402703 1100 0.947368 1.002632 1.318421 1.365789 1011 0.935065 0.98961 1.301299 1.348052 1010 0.923077 0.976923 1.284615 1.330769 1001 0.911392 0.964557 1.268354 1.313924 1000 0.9 0.9525 1.2525 1.2975 0111 0.888889 0.940741 1.237037 1.281481 0110 0.878049 0.929268 1.221951 1.265854 0101 0.86747 0.918072 1.207229 1.250602 0100 0.847059 0.896471 1.178824 1.221176 0011 0.827586 0.875862 1.151724 1.193103 0010 0.808989 0.85618 1.125843 1.166292 0001 0.791209 0.837363 1.101099 1.140659 0000 0.774194 0.819355 1.077419 1.116129"]
    #[inline(always)]
    pub fn vol_l1(&self) -> VolL1R {
        VolL1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - STD_L1<1:0> time 750u 1.5m 10 3m 11 6m"]
    #[inline(always)]
    pub fn std_l1(&self) -> StdL1R {
        StdL1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FPWM_L1 = 1 => force PWM FPWM_L1 = 0 => force PFM"]
    #[inline(always)]
    pub fn fpwm_l1(&self) -> FpwmL1R {
        FpwmL1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1:bypass SSR latch"]
    #[inline(always)]
    pub fn reg_bypass_l(&self) -> RegBypassLR {
        RegBypassLR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l(&self) -> SawFreqLR {
        SawFreqLR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - POWOCP_L1=1=> enable over current protection"]
    #[inline(always)]
    pub fn powocp_l1(&self) -> PowocpL1R {
        PowocpL1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - PWM current (A) PFM current (A) mP 000 0.533 0.105 001 0.580 0.145 010 0.630 0.185 011 0.680 0.223 100 0.730 0.263 101 0.780 0.303 110 0.830 0.342 111 0.890 0.290(VIN/10)"]
    #[inline(always)]
    pub fn ocp_l1(&self) -> OcpL1R {
        OcpL1R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable ZCD"]
    #[inline(always)]
    pub fn pow_zcd_l(&self) -> PowZcdLR {
        PowZcdLR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h(&self) -> RegZcdcHR {
        RegZcdcHR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Increase ZCD delay cell"]
    #[inline(always)]
    pub fn zcd_sdz_l(&self) -> ZcdSdzLR {
        ZcdSdzLR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 0:ZCD bias on after SS 1:ZCD bias on when ZCD enable"]
    #[inline(always)]
    pub fn force_zcd_bias(&self) -> ForceZcdBiasR {
        ForceZcdBiasR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - SWR/LDO SEL <93>=0 <93>=1 0000 0.495 0.458 0001 0.654 0.610 0010 0.707 0.662 0011 0.760 0.712 0100 0.795 0.746 0101 0.848 0.797 0110 0.883 0.831 0111 0.954 0.899 1000 1.042 0.984 1001 1.131 1.069 1010 1.219 1.153 1011 1.255 1.188 1100 1.308 1.238 1101 1.343 1.272 1110 1.414 1.340 1111 1.503 1.425"]
    #[inline(always)]
    pub fn vrefpfm_l(&self) -> VrefpfmLR {
        VrefpfmLR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - COT ON Current 00:150nA 01:150nA+100nA 10:150nA+150nA 11:150nA+250nA"]
    #[inline(always)]
    pub fn cot_i_l(&self) -> CotILR {
        CotILR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 0:normal current 1:increase OCP cmp current"]
    #[inline(always)]
    pub fn pow_minoff_l(&self) -> PowMinoffLR {
        PowMinoffLR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - COT off time controlled by, 0: ZCD 1: min off time"]
    #[inline(always)]
    pub fn off_end_sel(&self) -> OffEndSelR {
        OffEndSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Heavy load counter"]
    #[inline(always)]
    pub fn en_hload(&self) -> EnHloadR {
        EnHloadR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PFM comparator bias, 0: 50nA 1:250nA"]
    #[inline(always)]
    pub fn pfmcmp_iq(&self) -> PfmcmpIqR {
        PfmcmpIqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - VREF 0.6 0.635 0.835 0.865 Code VOUT VOUT VOUT VOUT 1111 1.028571 1.088571 1.431429 1.482857 1110 1 1.058333 1.391667 1.441667 1101 0.972973 1.02973 1.354054 1.402703 1100 0.947368 1.002632 1.318421 1.365789 1011 0.935065 0.98961 1.301299 1.348052 1010 0.923077 0.976923 1.284615 1.330769 1001 0.911392 0.964557 1.268354 1.313924 1000 0.9 0.9525 1.2525 1.2975 0111 0.888889 0.940741 1.237037 1.281481 0110 0.878049 0.929268 1.221951 1.265854 0101 0.86747 0.918072 1.207229 1.250602 0100 0.847059 0.896471 1.178824 1.221176 0011 0.827586 0.875862 1.151724 1.193103 0010 0.808989 0.85618 1.125843 1.166292 0001 0.791209 0.837363 1.101099 1.140659 0000 0.774194 0.819355 1.077419 1.116129"]
    #[inline(always)]
    pub fn vol_l1(&mut self) -> VolL1W<'_, RegAnaparswMacOn0Spec> {
        VolL1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - STD_L1<1:0> time 750u 1.5m 10 3m 11 6m"]
    #[inline(always)]
    pub fn std_l1(&mut self) -> StdL1W<'_, RegAnaparswMacOn0Spec> {
        StdL1W::new(self, 4)
    }
    #[doc = "Bit 6 - FPWM_L1 = 1 => force PWM FPWM_L1 = 0 => force PFM"]
    #[inline(always)]
    pub fn fpwm_l1(&mut self) -> FpwmL1W<'_, RegAnaparswMacOn0Spec> {
        FpwmL1W::new(self, 6)
    }
    #[doc = "Bit 7 - 1:bypass SSR latch"]
    #[inline(always)]
    pub fn reg_bypass_l(&mut self) -> RegBypassLW<'_, RegAnaparswMacOn0Spec> {
        RegBypassLW::new(self, 7)
    }
    #[doc = "Bit 12 - POWOCP_L1=1=> enable over current protection"]
    #[inline(always)]
    pub fn powocp_l1(&mut self) -> PowocpL1W<'_, RegAnaparswMacOn0Spec> {
        PowocpL1W::new(self, 12)
    }
    #[doc = "Bits 13:15 - PWM current (A) PFM current (A) mP 000 0.533 0.105 001 0.580 0.145 010 0.630 0.185 011 0.680 0.223 100 0.730 0.263 101 0.780 0.303 110 0.830 0.342 111 0.890 0.290(VIN/10)"]
    #[inline(always)]
    pub fn ocp_l1(&mut self) -> OcpL1W<'_, RegAnaparswMacOn0Spec> {
        OcpL1W::new(self, 13)
    }
    #[doc = "Bit 16 - Enable ZCD"]
    #[inline(always)]
    pub fn pow_zcd_l(&mut self) -> PowZcdLW<'_, RegAnaparswMacOn0Spec> {
        PowZcdLW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Increase ZCD delay cell"]
    #[inline(always)]
    pub fn zcd_sdz_l(&mut self) -> ZcdSdzLW<'_, RegAnaparswMacOn0Spec> {
        ZcdSdzLW::new(self, 19)
    }
    #[doc = "Bit 21 - 0:ZCD bias on after SS 1:ZCD bias on when ZCD enable"]
    #[inline(always)]
    pub fn force_zcd_bias(&mut self) -> ForceZcdBiasW<'_, RegAnaparswMacOn0Spec> {
        ForceZcdBiasW::new(self, 21)
    }
    #[doc = "Bits 22:25 - SWR/LDO SEL <93>=0 <93>=1 0000 0.495 0.458 0001 0.654 0.610 0010 0.707 0.662 0011 0.760 0.712 0100 0.795 0.746 0101 0.848 0.797 0110 0.883 0.831 0111 0.954 0.899 1000 1.042 0.984 1001 1.131 1.069 1010 1.219 1.153 1011 1.255 1.188 1100 1.308 1.238 1101 1.343 1.272 1110 1.414 1.340 1111 1.503 1.425"]
    #[inline(always)]
    pub fn vrefpfm_l(&mut self) -> VrefpfmLW<'_, RegAnaparswMacOn0Spec> {
        VrefpfmLW::new(self, 22)
    }
    #[doc = "Bits 26:27 - COT ON Current 00:150nA 01:150nA+100nA 10:150nA+150nA 11:150nA+250nA"]
    #[inline(always)]
    pub fn cot_i_l(&mut self) -> CotILW<'_, RegAnaparswMacOn0Spec> {
        CotILW::new(self, 26)
    }
    #[doc = "Bit 28 - 0:normal current 1:increase OCP cmp current"]
    #[inline(always)]
    pub fn pow_minoff_l(&mut self) -> PowMinoffLW<'_, RegAnaparswMacOn0Spec> {
        PowMinoffLW::new(self, 28)
    }
    #[doc = "Bit 29 - COT off time controlled by, 0: ZCD 1: min off time"]
    #[inline(always)]
    pub fn off_end_sel(&mut self) -> OffEndSelW<'_, RegAnaparswMacOn0Spec> {
        OffEndSelW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Heavy load counter"]
    #[inline(always)]
    pub fn en_hload(&mut self) -> EnHloadW<'_, RegAnaparswMacOn0Spec> {
        EnHloadW::new(self, 30)
    }
    #[doc = "Bit 31 - PFM comparator bias, 0: 50nA 1:250nA"]
    #[inline(always)]
    pub fn pfmcmp_iq(&mut self) -> PfmcmpIqW<'_, RegAnaparswMacOn0Spec> {
        PfmcmpIqW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn0Spec;
impl crate::RegisterSpec for RegAnaparswMacOn0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_0::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_0::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_0 to value 0xd4ed_98d5"]
impl crate::Resettable for RegAnaparswMacOn0Spec {
    const RESET_VALUE: u32 = 0xd4ed_98d5;
}
