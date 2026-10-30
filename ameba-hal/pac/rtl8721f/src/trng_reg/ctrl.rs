#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `NOISE_COLLECTOR_OUTPUT_DISABLE` reader - Noise collector output disable"]
pub type NoiseCollectorOutputDisableR = crate::BitReader;
#[doc = "Field `NOISE_COLLECTOR_OUTPUT_DISABLE` writer - Noise collector output disable"]
pub type NoiseCollectorOutputDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBIAS_CORRECTOR_BYPASS` reader - Debias corrector bypass enable"]
pub type DebiasCorrectorBypassR = crate::BitReader;
#[doc = "Field `DEBIAS_CORRECTOR_BYPASS` writer - Debias corrector bypass enable"]
pub type DebiasCorrectorBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSR_BYPASS` reader - LFSR bypass enable"]
pub type LfsrBypassR = crate::BitReader;
#[doc = "Field `LFSR_BYPASS` writer - LFSR bypass enable"]
pub type LfsrBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LWC_cond_rate` reader - Compression rate of lightweight-cipher conditioning component. Set this register to 0 will bypass lightweight-cipher conditioning component. Set this register larger than 0 will enable extra trng conditioning. This register is available only when LWC_COND config is 1."]
pub type LwcCondRateR = crate::FieldReader;
#[doc = "Field `LWC_cond_rate` writer - Compression rate of lightweight-cipher conditioning component. Set this register to 0 will bypass lightweight-cipher conditioning component. Set this register larger than 0 will enable extra trng conditioning. This register is available only when LWC_COND config is 1."]
pub type LwcCondRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECURITY_CONTROL` reader - If this field is equal to 0xA, the TRNG becomes a non-secure slave, and all the registers can be accessed by non-secure transaction. If this field is not equal to 0xA, the non-seucre transaction can only access address 0xC~0x30. This register is available only when NONSEC_RNG_BUFFER config is 1 that support TrustZone and dual buffer."]
pub type SecurityControlR = crate::FieldReader;
#[doc = "Field `SECURITY_CONTROL` writer - If this field is equal to 0xA, the TRNG becomes a non-secure slave, and all the registers can be accessed by non-secure transaction. If this field is not equal to 0xA, the non-seucre transaction can only access address 0xC~0x30. This register is available only when NONSEC_RNG_BUFFER config is 1 that support TrustZone and dual buffer."]
pub type SecurityControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Noise collector output disable"]
    #[inline(always)]
    pub fn noise_collector_output_disable(&self) -> NoiseCollectorOutputDisableR {
        NoiseCollectorOutputDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debias corrector bypass enable"]
    #[inline(always)]
    pub fn debias_corrector_bypass(&self) -> DebiasCorrectorBypassR {
        DebiasCorrectorBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFSR bypass enable"]
    #[inline(always)]
    pub fn lfsr_bypass(&self) -> LfsrBypassR {
        LfsrBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Compression rate of lightweight-cipher conditioning component. Set this register to 0 will bypass lightweight-cipher conditioning component. Set this register larger than 0 will enable extra trng conditioning. This register is available only when LWC_COND config is 1."]
    #[inline(always)]
    pub fn lwc_cond_rate(&self) -> LwcCondRateR {
        LwcCondRateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - If this field is equal to 0xA, the TRNG becomes a non-secure slave, and all the registers can be accessed by non-secure transaction. If this field is not equal to 0xA, the non-seucre transaction can only access address 0xC~0x30. This register is available only when NONSEC_RNG_BUFFER config is 1 that support TrustZone and dual buffer."]
    #[inline(always)]
    pub fn security_control(&self) -> SecurityControlR {
        SecurityControlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Noise collector output disable"]
    #[inline(always)]
    pub fn noise_collector_output_disable(&mut self) -> NoiseCollectorOutputDisableW<'_, CtrlSpec> {
        NoiseCollectorOutputDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - Debias corrector bypass enable"]
    #[inline(always)]
    pub fn debias_corrector_bypass(&mut self) -> DebiasCorrectorBypassW<'_, CtrlSpec> {
        DebiasCorrectorBypassW::new(self, 1)
    }
    #[doc = "Bit 2 - LFSR bypass enable"]
    #[inline(always)]
    pub fn lfsr_bypass(&mut self) -> LfsrBypassW<'_, CtrlSpec> {
        LfsrBypassW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Compression rate of lightweight-cipher conditioning component. Set this register to 0 will bypass lightweight-cipher conditioning component. Set this register larger than 0 will enable extra trng conditioning. This register is available only when LWC_COND config is 1."]
    #[inline(always)]
    pub fn lwc_cond_rate(&mut self) -> LwcCondRateW<'_, CtrlSpec> {
        LwcCondRateW::new(self, 4)
    }
    #[doc = "Bits 16:19 - If this field is equal to 0xA, the TRNG becomes a non-secure slave, and all the registers can be accessed by non-secure transaction. If this field is not equal to 0xA, the non-seucre transaction can only access address 0xC~0x30. This register is available only when NONSEC_RNG_BUFFER config is 1 that support TrustZone and dual buffer."]
    #[inline(always)]
    pub fn security_control(&mut self) -> SecurityControlW<'_, CtrlSpec> {
        SecurityControlW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
