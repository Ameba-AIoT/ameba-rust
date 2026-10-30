#[doc = "Register `PKE_Control` reader"]
pub type R = crate::R<PkeControlSpec>;
#[doc = "Register `PKE_Control` writer"]
pub type W = crate::W<PkeControlSpec>;
#[doc = "Field `PKE_Control_CAL_PRECOMPUTE` reader - When using RSA/ECC (bit 27 == 0), if this bit is set, PKE will calculate R ^ 2 mod N, n\\[63:0\\] ^ (-1) mod r only"]
pub type PkeControlCalPrecomputeR = crate::BitReader;
#[doc = "Field `PKE_Control_CAL_PRECOMPUTE` writer - When using RSA/ECC (bit 27 == 0), if this bit is set, PKE will calculate R ^ 2 mod N, n\\[63:0\\] ^ (-1) mod r only"]
pub type PkeControlCalPrecomputeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_IMEM_READ` reader - 1: Enable IMEM read (or else, the IMEM can only be read when bit\\[30\\] = 1) 0: IMEM can't be read"]
pub type PkeControlImemReadR = crate::BitReader;
#[doc = "Field `PKE_Control_IMEM_READ` writer - 1: Enable IMEM read (or else, the IMEM can only be read when bit\\[30\\] = 1) 0: IMEM can't be read"]
pub type PkeControlImemReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_DPA_DEFENSE` reader - 1: DPA defense enable 0: DPA defense disable"]
pub type PkeControlDpaDefenseR = crate::BitReader;
#[doc = "Field `PKE_Control_DPA_DEFENSE` writer - 1: DPA defense enable 0: DPA defense disable"]
pub type PkeControlDpaDefenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_TIMING_ATTACK` reader - 1: Timing attack defense enable 0: Timing attack defense disable"]
pub type PkeControlTimingAttackR = crate::BitReader;
#[doc = "Field `PKE_Control_TIMING_ATTACK` writer - 1: Timing attack defense enable 0: Timing attack defense disable"]
pub type PkeControlTimingAttackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_USE_LFSR` reader - 1: use internal LFSR0, and do not use internal LFSR. If user does not use internal LFSR, user needs to initial a 32-bit random number (can't be 0) to MMEM offset 0x600 (RSA3072), or MMEM offser 0x800 (RSA4096) or MMEM offset 0x380 (ECC). And if configured to ECC, please also clear the upper MMEM offset 0x381 to 0x3bf to 0."]
pub type PkeControlUseLfsrR = crate::BitReader;
#[doc = "Field `PKE_Control_USE_LFSR` writer - 1: use internal LFSR0, and do not use internal LFSR. If user does not use internal LFSR, user needs to initial a 32-bit random number (can't be 0) to MMEM offset 0x600 (RSA3072), or MMEM offser 0x800 (RSA4096) or MMEM offset 0x380 (ECC). And if configured to ECC, please also clear the upper MMEM offset 0x381 to 0x3bf to 0."]
pub type PkeControlUseLfsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_MODE_CURVE` reader - 1'b0: Edward Curves 1'b1: Montgomery Curves"]
pub type PkeControlModeCurveR = crate::BitReader;
#[doc = "Field `PKE_Control_MODE_CURVE` writer - 1'b0: Edward Curves 1'b1: Montgomery Curves"]
pub type PkeControlModeCurveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_MODE_MAIN` reader - 2'b00: ECC Prime field 2'b01: ECC Binary field 2'b10: RSA 2'b11: Edwards or Montgomery Curves"]
pub type PkeControlModeMainR = crate::FieldReader;
#[doc = "Field `PKE_Control_MODE_MAIN` writer - 2'b00: ECC Prime field 2'b01: ECC Binary field 2'b10: RSA 2'b11: Edwards or Montgomery Curves"]
pub type PkeControlModeMainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PKE_Control_PRECISE_LENGTH` reader - Precise RSA/ECC length in bits Caution: when preforming ecc point operation under binary field, there are situation which width of modulo is larger than ecc length. For example, width of f(x) of sect163r2 is 164, larger than ECC length 163. In this situation, just fill in Precise ECC length 163."]
pub type PkeControlPreciseLengthR = crate::FieldReader<u16>;
#[doc = "Field `PKE_Control_PRECISE_LENGTH` writer - Precise RSA/ECC length in bits Caution: when preforming ecc point operation under binary field, there are situation which width of modulo is larger than ecc length. For example, width of f(x) of sect163r2 is 164, larger than ECC length 163. In this situation, just fill in Precise ECC length 163."]
pub type PkeControlPreciseLengthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PKE_Control_PRECOMPUTATION` reader - R^2 mod N ready (1 means not need to calculate the R^2 mod N, already stored by user) For RSA , R^2 mod N ready and DPA defense can't be enabled at the same time. Valid only when bit\\[27\\] == 0"]
pub type PkeControlPrecomputationR = crate::BitReader;
#[doc = "Field `PKE_Control_PRECOMPUTATION` writer - R^2 mod N ready (1 means not need to calculate the R^2 mod N, already stored by user) For RSA , R^2 mod N ready and DPA defense can't be enabled at the same time. Valid only when bit\\[27\\] == 0"]
pub type PkeControlPrecomputationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_BYTE_SWAP` reader - Endian byte swap enable (only supported in MMEM)"]
pub type PkeControlByteSwapR = crate::BitReader;
#[doc = "Field `PKE_Control_BYTE_SWAP` writer - Endian byte swap enable (only supported in MMEM)"]
pub type PkeControlByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_WORD_SWAP` reader - Endian word swap enable (only supported in MMEM)"]
pub type PkeControlWordSwapR = crate::BitReader;
#[doc = "Field `PKE_Control_WORD_SWAP` writer - Endian word swap enable (only supported in MMEM)"]
pub type PkeControlWordSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_OTHER_FUNCTION` reader - Other functions (not RSA/ECC) enable bit 1: enabled 0: disabled (RSA: modular exponentiation; ECC: scalar multiplication)"]
pub type PkeControlOtherFunctionR = crate::BitReader;
#[doc = "Field `PKE_Control_OTHER_FUNCTION` writer - Other functions (not RSA/ECC) enable bit 1: enabled 0: disabled (RSA: modular exponentiation; ECC: scalar multiplication)"]
pub type PkeControlOtherFunctionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_START` reader - Write 1 to start PKE (cleared automatically by PKE)"]
pub type PkeControlStartR = crate::BitReader;
#[doc = "Field `PKE_Control_START` writer - Write 1 to start PKE (cleared automatically by PKE)"]
pub type PkeControlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_Control_HASH_READY` reader - Write 1 to inform PKE that hash value used in SIMPLIFY ECDSA/EDDSA are ready"]
pub type PkeControlHashReadyR = crate::BitReader;
#[doc = "Field `PKE_Control_HASH_READY` writer - Write 1 to inform PKE that hash value used in SIMPLIFY ECDSA/EDDSA are ready"]
pub type PkeControlHashReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When using RSA/ECC (bit 27 == 0), if this bit is set, PKE will calculate R ^ 2 mod N, n\\[63:0\\] ^ (-1) mod r only"]
    #[inline(always)]
    pub fn pke_control_cal_precompute(&self) -> PkeControlCalPrecomputeR {
        PkeControlCalPrecomputeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable IMEM read (or else, the IMEM can only be read when bit\\[30\\] = 1) 0: IMEM can't be read"]
    #[inline(always)]
    pub fn pke_control_imem_read(&self) -> PkeControlImemReadR {
        PkeControlImemReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: DPA defense enable 0: DPA defense disable"]
    #[inline(always)]
    pub fn pke_control_dpa_defense(&self) -> PkeControlDpaDefenseR {
        PkeControlDpaDefenseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Timing attack defense enable 0: Timing attack defense disable"]
    #[inline(always)]
    pub fn pke_control_timing_attack(&self) -> PkeControlTimingAttackR {
        PkeControlTimingAttackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: use internal LFSR0, and do not use internal LFSR. If user does not use internal LFSR, user needs to initial a 32-bit random number (can't be 0) to MMEM offset 0x600 (RSA3072), or MMEM offser 0x800 (RSA4096) or MMEM offset 0x380 (ECC). And if configured to ECC, please also clear the upper MMEM offset 0x381 to 0x3bf to 0."]
    #[inline(always)]
    pub fn pke_control_use_lfsr(&self) -> PkeControlUseLfsrR {
        PkeControlUseLfsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1'b0: Edward Curves 1'b1: Montgomery Curves"]
    #[inline(always)]
    pub fn pke_control_mode_curve(&self) -> PkeControlModeCurveR {
        PkeControlModeCurveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 2'b00: ECC Prime field 2'b01: ECC Binary field 2'b10: RSA 2'b11: Edwards or Montgomery Curves"]
    #[inline(always)]
    pub fn pke_control_mode_main(&self) -> PkeControlModeMainR {
        PkeControlModeMainR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:23 - Precise RSA/ECC length in bits Caution: when preforming ecc point operation under binary field, there are situation which width of modulo is larger than ecc length. For example, width of f(x) of sect163r2 is 164, larger than ECC length 163. In this situation, just fill in Precise ECC length 163."]
    #[inline(always)]
    pub fn pke_control_precise_length(&self) -> PkeControlPreciseLengthR {
        PkeControlPreciseLengthR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - R^2 mod N ready (1 means not need to calculate the R^2 mod N, already stored by user) For RSA , R^2 mod N ready and DPA defense can't be enabled at the same time. Valid only when bit\\[27\\] == 0"]
    #[inline(always)]
    pub fn pke_control_precomputation(&self) -> PkeControlPrecomputationR {
        PkeControlPrecomputationR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Endian byte swap enable (only supported in MMEM)"]
    #[inline(always)]
    pub fn pke_control_byte_swap(&self) -> PkeControlByteSwapR {
        PkeControlByteSwapR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Endian word swap enable (only supported in MMEM)"]
    #[inline(always)]
    pub fn pke_control_word_swap(&self) -> PkeControlWordSwapR {
        PkeControlWordSwapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Other functions (not RSA/ECC) enable bit 1: enabled 0: disabled (RSA: modular exponentiation; ECC: scalar multiplication)"]
    #[inline(always)]
    pub fn pke_control_other_function(&self) -> PkeControlOtherFunctionR {
        PkeControlOtherFunctionR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1 to start PKE (cleared automatically by PKE)"]
    #[inline(always)]
    pub fn pke_control_start(&self) -> PkeControlStartR {
        PkeControlStartR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to inform PKE that hash value used in SIMPLIFY ECDSA/EDDSA are ready"]
    #[inline(always)]
    pub fn pke_control_hash_ready(&self) -> PkeControlHashReadyR {
        PkeControlHashReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When using RSA/ECC (bit 27 == 0), if this bit is set, PKE will calculate R ^ 2 mod N, n\\[63:0\\] ^ (-1) mod r only"]
    #[inline(always)]
    pub fn pke_control_cal_precompute(&mut self) -> PkeControlCalPrecomputeW<'_, PkeControlSpec> {
        PkeControlCalPrecomputeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable IMEM read (or else, the IMEM can only be read when bit\\[30\\] = 1) 0: IMEM can't be read"]
    #[inline(always)]
    pub fn pke_control_imem_read(&mut self) -> PkeControlImemReadW<'_, PkeControlSpec> {
        PkeControlImemReadW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: DPA defense enable 0: DPA defense disable"]
    #[inline(always)]
    pub fn pke_control_dpa_defense(&mut self) -> PkeControlDpaDefenseW<'_, PkeControlSpec> {
        PkeControlDpaDefenseW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Timing attack defense enable 0: Timing attack defense disable"]
    #[inline(always)]
    pub fn pke_control_timing_attack(&mut self) -> PkeControlTimingAttackW<'_, PkeControlSpec> {
        PkeControlTimingAttackW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: use internal LFSR0, and do not use internal LFSR. If user does not use internal LFSR, user needs to initial a 32-bit random number (can't be 0) to MMEM offset 0x600 (RSA3072), or MMEM offser 0x800 (RSA4096) or MMEM offset 0x380 (ECC). And if configured to ECC, please also clear the upper MMEM offset 0x381 to 0x3bf to 0."]
    #[inline(always)]
    pub fn pke_control_use_lfsr(&mut self) -> PkeControlUseLfsrW<'_, PkeControlSpec> {
        PkeControlUseLfsrW::new(self, 4)
    }
    #[doc = "Bit 5 - 1'b0: Edward Curves 1'b1: Montgomery Curves"]
    #[inline(always)]
    pub fn pke_control_mode_curve(&mut self) -> PkeControlModeCurveW<'_, PkeControlSpec> {
        PkeControlModeCurveW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 2'b00: ECC Prime field 2'b01: ECC Binary field 2'b10: RSA 2'b11: Edwards or Montgomery Curves"]
    #[inline(always)]
    pub fn pke_control_mode_main(&mut self) -> PkeControlModeMainW<'_, PkeControlSpec> {
        PkeControlModeMainW::new(self, 6)
    }
    #[doc = "Bits 8:23 - Precise RSA/ECC length in bits Caution: when preforming ecc point operation under binary field, there are situation which width of modulo is larger than ecc length. For example, width of f(x) of sect163r2 is 164, larger than ECC length 163. In this situation, just fill in Precise ECC length 163."]
    #[inline(always)]
    pub fn pke_control_precise_length(&mut self) -> PkeControlPreciseLengthW<'_, PkeControlSpec> {
        PkeControlPreciseLengthW::new(self, 8)
    }
    #[doc = "Bit 24 - R^2 mod N ready (1 means not need to calculate the R^2 mod N, already stored by user) For RSA , R^2 mod N ready and DPA defense can't be enabled at the same time. Valid only when bit\\[27\\] == 0"]
    #[inline(always)]
    pub fn pke_control_precomputation(&mut self) -> PkeControlPrecomputationW<'_, PkeControlSpec> {
        PkeControlPrecomputationW::new(self, 24)
    }
    #[doc = "Bit 25 - Endian byte swap enable (only supported in MMEM)"]
    #[inline(always)]
    pub fn pke_control_byte_swap(&mut self) -> PkeControlByteSwapW<'_, PkeControlSpec> {
        PkeControlByteSwapW::new(self, 25)
    }
    #[doc = "Bit 26 - Endian word swap enable (only supported in MMEM)"]
    #[inline(always)]
    pub fn pke_control_word_swap(&mut self) -> PkeControlWordSwapW<'_, PkeControlSpec> {
        PkeControlWordSwapW::new(self, 26)
    }
    #[doc = "Bit 27 - Other functions (not RSA/ECC) enable bit 1: enabled 0: disabled (RSA: modular exponentiation; ECC: scalar multiplication)"]
    #[inline(always)]
    pub fn pke_control_other_function(&mut self) -> PkeControlOtherFunctionW<'_, PkeControlSpec> {
        PkeControlOtherFunctionW::new(self, 27)
    }
    #[doc = "Bit 30 - Write 1 to start PKE (cleared automatically by PKE)"]
    #[inline(always)]
    pub fn pke_control_start(&mut self) -> PkeControlStartW<'_, PkeControlSpec> {
        PkeControlStartW::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to inform PKE that hash value used in SIMPLIFY ECDSA/EDDSA are ready"]
    #[inline(always)]
    pub fn pke_control_hash_ready(&mut self) -> PkeControlHashReadyW<'_, PkeControlSpec> {
        PkeControlHashReadyW::new(self, 31)
    }
}
#[doc = "PKE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeControlSpec;
impl crate::RegisterSpec for PkeControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_control::R`](R) reader structure"]
impl crate::Readable for PkeControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_control::W`](W) writer structure"]
impl crate::Writable for PkeControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_Control to value 0"]
impl crate::Resettable for PkeControlSpec {}
