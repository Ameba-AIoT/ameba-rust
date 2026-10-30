#[doc = "Register `REG_DUMMY` reader"]
pub type R = crate::R<RegDummySpec>;
#[doc = "Register `REG_DUMMY` writer"]
pub type W = crate::W<RegDummySpec>;
#[doc = "Field `opt_ign_mode` reader - option for ignore regu mode_l wait when pfm -> pwm. default not wait as usual"]
pub type OptIgnModeR = crate::BitReader;
#[doc = "Field `opt_ign_mode` writer - option for ignore regu mode_l wait when pfm -> pwm. default not wait as usual"]
pub type OptIgnModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `opt_en_wak_dummy` reader - option for enable swr dummy load to speed up when pfm -> pwm. if opt_wait_mode is 0 , this bit has no use"]
pub type OptEnWakDummyR = crate::BitReader;
#[doc = "Field `opt_en_wak_dummy` writer - option for enable swr dummy load to speed up when pfm -> pwm. if opt_wait_mode is 0 , this bit has no use"]
pub type OptEnWakDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `opt_en_slp_dummy` reader - option for enable swr dummy load to speed up when pfm voltage falling. Default not enable , using a programmble delay to wait"]
pub type OptEnSlpDummyR = crate::BitReader;
#[doc = "Field `opt_en_slp_dummy` writer - option for enable swr dummy load to speed up when pfm voltage falling. Default not enable , using a programmble delay to wait"]
pub type OptEnSlpDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc4m_ign_volt` reader - Options if osc4m can auto require voltage , disable it may speedup reponse time when system never goto 0.7v 0. osc4m will send volt areq when osc4m enable or disable 1. osc4m never req volt change , make sure , system voltage will never goto 0.7v Note: RL6973 never goto 0.7v, fix 1"]
pub type Osc4mIgnVoltR = crate::BitReader;
#[doc = "Field `osc4m_ign_volt` writer - Options if osc4m can auto require voltage , disable it may speedup reponse time when system never goto 0.7v 0. osc4m will send volt areq when osc4m enable or disable 1. osc4m never req volt change , make sure , system voltage will never goto 0.7v Note: RL6973 never goto 0.7v, fix 1"]
pub type Osc4mIgnVoltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tmode_mldo` reader - 3318 LDO enter testmode , and register setting will not controlled by pmc"]
pub type TmodeMldoR = crate::BitReader;
#[doc = "Field `tmode_mldo` writer - 3318 LDO enter testmode , and register setting will not controlled by pmc"]
pub type TmodeMldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cldo_force_dml_off` reader - Force core ldo dummy load off , used when pc mode 1: force CLDO dummy load off , regu pmc will wirte this when enter pc mode 0: release"]
pub type CldoForceDmlOffR = crate::BitReader;
#[doc = "Field `cldo_force_dml_off` writer - Force core ldo dummy load off , used when pc mode 1: force CLDO dummy load off , regu pmc will wirte this when enter pc mode 0: release"]
pub type CldoForceDmlOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mldo_stl` reader - mem LDO power on stable wait time 00: 200us 01: 500us 10: 1ms 11: 2ms"]
pub type MldoStlR = crate::FieldReader;
#[doc = "Field `mldo_stl` writer - mem LDO power on stable wait time 00: 200us 01: 500us 10: 1ms 11: 2ms"]
pub type MldoStlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pmc_prg_dly` reader - regu_pmc programmable delay value 0: 100us 1: 500us 2: 1ms 3: 2ms 4: 10ms 5: 50ms 6: 100ms 7: 200ms"]
pub type PmcPrgDlyR = crate::FieldReader;
#[doc = "Field `pmc_prg_dly` writer - regu_pmc programmable delay value 0: 100us 1: 500us 2: 1ms 3: 2ms 4: 10ms 5: 50ms 6: 100ms 7: 200ms"]
pub type PmcPrgDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - option for ignore regu mode_l wait when pfm -> pwm. default not wait as usual"]
    #[inline(always)]
    pub fn opt_ign_mode(&self) -> OptIgnModeR {
        OptIgnModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - option for enable swr dummy load to speed up when pfm -> pwm. if opt_wait_mode is 0 , this bit has no use"]
    #[inline(always)]
    pub fn opt_en_wak_dummy(&self) -> OptEnWakDummyR {
        OptEnWakDummyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - option for enable swr dummy load to speed up when pfm voltage falling. Default not enable , using a programmble delay to wait"]
    #[inline(always)]
    pub fn opt_en_slp_dummy(&self) -> OptEnSlpDummyR {
        OptEnSlpDummyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Options if osc4m can auto require voltage , disable it may speedup reponse time when system never goto 0.7v 0. osc4m will send volt areq when osc4m enable or disable 1. osc4m never req volt change , make sure , system voltage will never goto 0.7v Note: RL6973 never goto 0.7v, fix 1"]
    #[inline(always)]
    pub fn osc4m_ign_volt(&self) -> Osc4mIgnVoltR {
        Osc4mIgnVoltR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 3318 LDO enter testmode , and register setting will not controlled by pmc"]
    #[inline(always)]
    pub fn tmode_mldo(&self) -> TmodeMldoR {
        TmodeMldoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force core ldo dummy load off , used when pc mode 1: force CLDO dummy load off , regu pmc will wirte this when enter pc mode 0: release"]
    #[inline(always)]
    pub fn cldo_force_dml_off(&self) -> CldoForceDmlOffR {
        CldoForceDmlOffR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - mem LDO power on stable wait time 00: 200us 01: 500us 10: 1ms 11: 2ms"]
    #[inline(always)]
    pub fn mldo_stl(&self) -> MldoStlR {
        MldoStlR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - regu_pmc programmable delay value 0: 100us 1: 500us 2: 1ms 3: 2ms 4: 10ms 5: 50ms 6: 100ms 7: 200ms"]
    #[inline(always)]
    pub fn pmc_prg_dly(&self) -> PmcPrgDlyR {
        PmcPrgDlyR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - option for ignore regu mode_l wait when pfm -> pwm. default not wait as usual"]
    #[inline(always)]
    pub fn opt_ign_mode(&mut self) -> OptIgnModeW<'_, RegDummySpec> {
        OptIgnModeW::new(self, 0)
    }
    #[doc = "Bit 1 - option for enable swr dummy load to speed up when pfm -> pwm. if opt_wait_mode is 0 , this bit has no use"]
    #[inline(always)]
    pub fn opt_en_wak_dummy(&mut self) -> OptEnWakDummyW<'_, RegDummySpec> {
        OptEnWakDummyW::new(self, 1)
    }
    #[doc = "Bit 2 - option for enable swr dummy load to speed up when pfm voltage falling. Default not enable , using a programmble delay to wait"]
    #[inline(always)]
    pub fn opt_en_slp_dummy(&mut self) -> OptEnSlpDummyW<'_, RegDummySpec> {
        OptEnSlpDummyW::new(self, 2)
    }
    #[doc = "Bit 3 - Options if osc4m can auto require voltage , disable it may speedup reponse time when system never goto 0.7v 0. osc4m will send volt areq when osc4m enable or disable 1. osc4m never req volt change , make sure , system voltage will never goto 0.7v Note: RL6973 never goto 0.7v, fix 1"]
    #[inline(always)]
    pub fn osc4m_ign_volt(&mut self) -> Osc4mIgnVoltW<'_, RegDummySpec> {
        Osc4mIgnVoltW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDummySpec> {
        DummyW::new(self, 4)
    }
    #[doc = "Bit 8 - 3318 LDO enter testmode , and register setting will not controlled by pmc"]
    #[inline(always)]
    pub fn tmode_mldo(&mut self) -> TmodeMldoW<'_, RegDummySpec> {
        TmodeMldoW::new(self, 8)
    }
    #[doc = "Bit 9 - Force core ldo dummy load off , used when pc mode 1: force CLDO dummy load off , regu pmc will wirte this when enter pc mode 0: release"]
    #[inline(always)]
    pub fn cldo_force_dml_off(&mut self) -> CldoForceDmlOffW<'_, RegDummySpec> {
        CldoForceDmlOffW::new(self, 9)
    }
    #[doc = "Bits 10:11 - mem LDO power on stable wait time 00: 200us 01: 500us 10: 1ms 11: 2ms"]
    #[inline(always)]
    pub fn mldo_stl(&mut self) -> MldoStlW<'_, RegDummySpec> {
        MldoStlW::new(self, 10)
    }
    #[doc = "Bits 12:14 - regu_pmc programmable delay value 0: 100us 1: 500us 2: 1ms 3: 2ms 4: 10ms 5: 50ms 6: 100ms 7: 200ms"]
    #[inline(always)]
    pub fn pmc_prg_dly(&mut self) -> PmcPrgDlyW<'_, RegDummySpec> {
        PmcPrgDlyW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummySpec;
impl crate::RegisterSpec for RegDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy::R`](R) reader structure"]
impl crate::Readable for RegDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy::W`](W) writer structure"]
impl crate::Writable for RegDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY to value 0x6873"]
impl crate::Resettable for RegDummySpec {
    const RESET_VALUE: u32 = 0x6873;
}
