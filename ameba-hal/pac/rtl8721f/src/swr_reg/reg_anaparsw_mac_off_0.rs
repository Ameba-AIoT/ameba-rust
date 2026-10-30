#[doc = "Register `REG_ANAPARSW_MAC_OFF_0` reader"]
pub type R = crate::R<RegAnaparswMacOff0Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_OFF_0` writer"]
pub type W = crate::W<RegAnaparswMacOff0Spec>;
#[doc = "Field `null5` reader - "]
pub type Null5R = crate::FieldReader;
#[doc = "Field `null5` writer - "]
pub type Null5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2_L1` reader - 00 => 13f 01 => 26f 10 => 39f 11 => 41f"]
pub type C2L1R = crate::FieldReader;
#[doc = "Field `C2_L1` writer - 00 => 13f 01 => 26f 10 => 39f 11 => 41f"]
pub type C2L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `null6` reader - "]
pub type Null6R = crate::FieldReader;
#[doc = "Field `null6` writer - "]
pub type Null6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R1_L1` reader - 00 => 900k 01 => 720k 10 => 900k 11 => 1080k"]
pub type R1L1R = crate::FieldReader;
#[doc = "Field `R1_L1` writer - 00 => 900k 01 => 720k 10 => 900k 11 => 1080k"]
pub type R1L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `null7` reader - "]
pub type Null7R = crate::FieldReader;
#[doc = "Field `null7` writer - "]
pub type Null7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R3_L1` reader - 00 => 36k 01 => 30k 10 => 24k 11 => 24k"]
pub type R3L1R = crate::FieldReader;
#[doc = "Field `R3_L1` writer - 00 => 36k 01 => 30k 10 => 24k 11 => 24k"]
pub type R3L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_type_L` reader - 1:type3 0:type2"]
pub type RegTypeLR = crate::BitReader;
#[doc = "Field `reg_type_L` writer - 1:type3 0:type2"]
pub type RegTypeLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_clk_sel` reader - 00:out of phase CKSS 01:out of phase PWM_CTRIL 10:ckout 11:HG"]
pub type RegClkSelR = crate::FieldReader;
#[doc = "Field `reg_clk_sel` writer - 00:out of phase CKSS 01:out of phase PWM_CTRIL 10:ckout 11:HG"]
pub type RegClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENCOT_L` reader - 1:PFM COT"]
pub type EncotLR = crate::BitReader;
#[doc = "Field `ENCOT_L` writer - 1:PFM COT"]
pub type EncotLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_ON_END_L` reader - 1:EN min OFF time (PFM COT)"]
pub type EnOnEndLR = crate::BitReader;
#[doc = "Field `EN_ON_END_L` writer - 1:EN min OFF time (PFM COT)"]
pub type EnOnEndLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_OFFTIME_L` reader - 0:from min OFF time(PFM COT) 1:always on"]
pub type NoOfftimeLR = crate::BitReader;
#[doc = "Field `NO_OFFTIME_L` writer - 0:from min OFF time(PFM COT) 1:always on"]
pub type NoOfftimeLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARENB_H` reader - 1:off 0:on"]
pub type ArenbHR = crate::BitReader;
#[doc = "Field `ARENB_H` writer - 1:off 0:on"]
pub type ArenbHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POW_AUTO_L` reader - 1:auto PFM/PWM 0:register control"]
pub type PowAutoLR = crate::BitReader;
#[doc = "Field `POW_AUTO_L` writer - 1:auto PFM/PWM 0:register control"]
pub type PowAutoLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINON_LIQ` reader - 0:Min. on circuit always on 1:Min. on circuit pulse on"]
pub type MinonLiqR = crate::BitReader;
#[doc = "Field `MINON_LIQ` writer - 0:Min. on circuit always on 1:Min. on circuit pulse on"]
pub type MinonLiqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINOFF_LIQ` reader - 0:Min. off circuit always on 1:Min. off circuit pulse on"]
pub type MinoffLiqR = crate::BitReader;
#[doc = "Field `MINOFF_LIQ` writer - 0:Min. off circuit always on 1:Min. off circuit pulse on"]
pub type MinoffLiqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDO_OC_clamp` reader - 0:OC clamp with P/NMOS 1:OC clamp with PMOS"]
pub type LdoOcClampR = crate::BitReader;
#[doc = "Field `LDO_OC_clamp` writer - 0:OC clamp with P/NMOS 1:OC clamp with PMOS"]
pub type LdoOcClampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VO_DISCHG` reader - 1:VO with 1mA discharge"]
pub type VoDischgR = crate::BitReader;
#[doc = "Field `VO_DISCHG` writer - 1:VO with 1mA discharge"]
pub type VoDischgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Force_LDOS` reader - 1:LDOS force on"]
pub type ForceLdosR = crate::BitReader;
#[doc = "Field `Force_LDOS` writer - 1:LDOS force on"]
pub type ForceLdosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENOCPMUX_L` reader - 1:OCP signal control by MUX 0:OCP signal control by I"]
pub type EnocpmuxLR = crate::BitReader;
#[doc = "Field `ENOCPMUX_L` writer - 1:OCP signal control by MUX 0:OCP signal control by I"]
pub type EnocpmuxLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBOX_L1` reader - TBOX_L1<1:0> output 00 CKOVER/RAMPOVER 01 NI/PI 10 CKSS/SSOVER 11 i/NIOFF_H"]
pub type TboxL1R = crate::FieldReader;
#[doc = "Field `TBOX_L1` writer - TBOX_L1<1:0> output 00 CKOVER/RAMPOVER 01 NI/PI 10 CKSS/SSOVER 11 i/NIOFF_H"]
pub type TboxL1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_vofb_sel` reader - 0:vo15 1:LX_SPS"]
pub type RegVofbSelR = crate::BitReader;
#[doc = "Field `reg_vofb_sel` writer - 0:vo15 1:LX_SPS"]
pub type RegVofbSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_autozcd_L` reader - 0:register 1:10000"]
pub type RegAutozcdLR = crate::BitReader;
#[doc = "Field `reg_autozcd_L` writer - 0:register 1:10000"]
pub type RegAutozcdLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDZN_L` reader - Nonoverlap delay cell for NMOS"]
pub type SdznLR = crate::FieldReader;
#[doc = "Field `SDZN_L` writer - Nonoverlap delay cell for NMOS"]
pub type SdznLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn null5(&self) -> Null5R {
        Null5R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00 => 13f 01 => 26f 10 => 39f 11 => 41f"]
    #[inline(always)]
    pub fn c2_l1(&self) -> C2L1R {
        C2L1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn null6(&self) -> Null6R {
        Null6R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 00 => 900k 01 => 720k 10 => 900k 11 => 1080k"]
    #[inline(always)]
    pub fn r1_l1(&self) -> R1L1R {
        R1L1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn null7(&self) -> Null7R {
        Null7R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 00 => 36k 01 => 30k 10 => 24k 11 => 24k"]
    #[inline(always)]
    pub fn r3_l1(&self) -> R3L1R {
        R3L1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 1:type3 0:type2"]
    #[inline(always)]
    pub fn reg_type_l(&self) -> RegTypeLR {
        RegTypeLR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 00:out of phase CKSS 01:out of phase PWM_CTRIL 10:ckout 11:HG"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> RegClkSelR {
        RegClkSelR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 1:PFM COT"]
    #[inline(always)]
    pub fn encot_l(&self) -> EncotLR {
        EncotLR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1:EN min OFF time (PFM COT)"]
    #[inline(always)]
    pub fn en_on_end_l(&self) -> EnOnEndLR {
        EnOnEndLR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0:from min OFF time(PFM COT) 1:always on"]
    #[inline(always)]
    pub fn no_offtime_l(&self) -> NoOfftimeLR {
        NoOfftimeLR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1:off 0:on"]
    #[inline(always)]
    pub fn arenb_h(&self) -> ArenbHR {
        ArenbHR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1:auto PFM/PWM 0:register control"]
    #[inline(always)]
    pub fn pow_auto_l(&self) -> PowAutoLR {
        PowAutoLR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 0:Min. on circuit always on 1:Min. on circuit pulse on"]
    #[inline(always)]
    pub fn minon_liq(&self) -> MinonLiqR {
        MinonLiqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 0:Min. off circuit always on 1:Min. off circuit pulse on"]
    #[inline(always)]
    pub fn minoff_liq(&self) -> MinoffLiqR {
        MinoffLiqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 0:OC clamp with P/NMOS 1:OC clamp with PMOS"]
    #[inline(always)]
    pub fn ldo_oc_clamp(&self) -> LdoOcClampR {
        LdoOcClampR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1:VO with 1mA discharge"]
    #[inline(always)]
    pub fn vo_dischg(&self) -> VoDischgR {
        VoDischgR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1:LDOS force on"]
    #[inline(always)]
    pub fn force_ldos(&self) -> ForceLdosR {
        ForceLdosR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1:OCP signal control by MUX 0:OCP signal control by I"]
    #[inline(always)]
    pub fn enocpmux_l(&self) -> EnocpmuxLR {
        EnocpmuxLR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - TBOX_L1<1:0> output 00 CKOVER/RAMPOVER 01 NI/PI 10 CKSS/SSOVER 11 i/NIOFF_H"]
    #[inline(always)]
    pub fn tbox_l1(&self) -> TboxL1R {
        TboxL1R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 0:vo15 1:LX_SPS"]
    #[inline(always)]
    pub fn reg_vofb_sel(&self) -> RegVofbSelR {
        RegVofbSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0:register 1:10000"]
    #[inline(always)]
    pub fn reg_autozcd_l(&self) -> RegAutozcdLR {
        RegAutozcdLR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Nonoverlap delay cell for NMOS"]
    #[inline(always)]
    pub fn sdzn_l(&self) -> SdznLR {
        SdznLR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn null5(&mut self) -> Null5W<'_, RegAnaparswMacOff0Spec> {
        Null5W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 00 => 13f 01 => 26f 10 => 39f 11 => 41f"]
    #[inline(always)]
    pub fn c2_l1(&mut self) -> C2L1W<'_, RegAnaparswMacOff0Spec> {
        C2L1W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn null6(&mut self) -> Null6W<'_, RegAnaparswMacOff0Spec> {
        Null6W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 00 => 900k 01 => 720k 10 => 900k 11 => 1080k"]
    #[inline(always)]
    pub fn r1_l1(&mut self) -> R1L1W<'_, RegAnaparswMacOff0Spec> {
        R1L1W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn null7(&mut self) -> Null7W<'_, RegAnaparswMacOff0Spec> {
        Null7W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 00 => 36k 01 => 30k 10 => 24k 11 => 24k"]
    #[inline(always)]
    pub fn r3_l1(&mut self) -> R3L1W<'_, RegAnaparswMacOff0Spec> {
        R3L1W::new(self, 10)
    }
    #[doc = "Bit 12 - 1:type3 0:type2"]
    #[inline(always)]
    pub fn reg_type_l(&mut self) -> RegTypeLW<'_, RegAnaparswMacOff0Spec> {
        RegTypeLW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 00:out of phase CKSS 01:out of phase PWM_CTRIL 10:ckout 11:HG"]
    #[inline(always)]
    pub fn reg_clk_sel(&mut self) -> RegClkSelW<'_, RegAnaparswMacOff0Spec> {
        RegClkSelW::new(self, 13)
    }
    #[doc = "Bit 15 - 1:PFM COT"]
    #[inline(always)]
    pub fn encot_l(&mut self) -> EncotLW<'_, RegAnaparswMacOff0Spec> {
        EncotLW::new(self, 15)
    }
    #[doc = "Bit 16 - 1:EN min OFF time (PFM COT)"]
    #[inline(always)]
    pub fn en_on_end_l(&mut self) -> EnOnEndLW<'_, RegAnaparswMacOff0Spec> {
        EnOnEndLW::new(self, 16)
    }
    #[doc = "Bit 17 - 0:from min OFF time(PFM COT) 1:always on"]
    #[inline(always)]
    pub fn no_offtime_l(&mut self) -> NoOfftimeLW<'_, RegAnaparswMacOff0Spec> {
        NoOfftimeLW::new(self, 17)
    }
    #[doc = "Bit 18 - 1:off 0:on"]
    #[inline(always)]
    pub fn arenb_h(&mut self) -> ArenbHW<'_, RegAnaparswMacOff0Spec> {
        ArenbHW::new(self, 18)
    }
    #[doc = "Bit 19 - 1:auto PFM/PWM 0:register control"]
    #[inline(always)]
    pub fn pow_auto_l(&mut self) -> PowAutoLW<'_, RegAnaparswMacOff0Spec> {
        PowAutoLW::new(self, 19)
    }
    #[doc = "Bit 20 - 0:Min. on circuit always on 1:Min. on circuit pulse on"]
    #[inline(always)]
    pub fn minon_liq(&mut self) -> MinonLiqW<'_, RegAnaparswMacOff0Spec> {
        MinonLiqW::new(self, 20)
    }
    #[doc = "Bit 21 - 0:Min. off circuit always on 1:Min. off circuit pulse on"]
    #[inline(always)]
    pub fn minoff_liq(&mut self) -> MinoffLiqW<'_, RegAnaparswMacOff0Spec> {
        MinoffLiqW::new(self, 21)
    }
    #[doc = "Bit 22 - 0:OC clamp with P/NMOS 1:OC clamp with PMOS"]
    #[inline(always)]
    pub fn ldo_oc_clamp(&mut self) -> LdoOcClampW<'_, RegAnaparswMacOff0Spec> {
        LdoOcClampW::new(self, 22)
    }
    #[doc = "Bit 23 - 1:VO with 1mA discharge"]
    #[inline(always)]
    pub fn vo_dischg(&mut self) -> VoDischgW<'_, RegAnaparswMacOff0Spec> {
        VoDischgW::new(self, 23)
    }
    #[doc = "Bit 24 - 1:LDOS force on"]
    #[inline(always)]
    pub fn force_ldos(&mut self) -> ForceLdosW<'_, RegAnaparswMacOff0Spec> {
        ForceLdosW::new(self, 24)
    }
    #[doc = "Bit 25 - 1:OCP signal control by MUX 0:OCP signal control by I"]
    #[inline(always)]
    pub fn enocpmux_l(&mut self) -> EnocpmuxLW<'_, RegAnaparswMacOff0Spec> {
        EnocpmuxLW::new(self, 25)
    }
    #[doc = "Bits 26:27 - TBOX_L1<1:0> output 00 CKOVER/RAMPOVER 01 NI/PI 10 CKSS/SSOVER 11 i/NIOFF_H"]
    #[inline(always)]
    pub fn tbox_l1(&mut self) -> TboxL1W<'_, RegAnaparswMacOff0Spec> {
        TboxL1W::new(self, 26)
    }
    #[doc = "Bit 28 - 0:vo15 1:LX_SPS"]
    #[inline(always)]
    pub fn reg_vofb_sel(&mut self) -> RegVofbSelW<'_, RegAnaparswMacOff0Spec> {
        RegVofbSelW::new(self, 28)
    }
    #[doc = "Bit 29 - 0:register 1:10000"]
    #[inline(always)]
    pub fn reg_autozcd_l(&mut self) -> RegAutozcdLW<'_, RegAnaparswMacOff0Spec> {
        RegAutozcdLW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Nonoverlap delay cell for NMOS"]
    #[inline(always)]
    pub fn sdzn_l(&mut self) -> SdznLW<'_, RegAnaparswMacOff0Spec> {
        SdznLW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_off_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_off_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOff0Spec;
impl crate::RegisterSpec for RegAnaparswMacOff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_off_0::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOff0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_off_0::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOff0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_OFF_0 to value 0x4274_faaa"]
impl crate::Resettable for RegAnaparswMacOff0Spec {
    const RESET_VALUE: u32 = 0x4274_faaa;
}
