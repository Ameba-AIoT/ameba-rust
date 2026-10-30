#[doc = "Register `XTAL_DEBUG` reader"]
pub type R = crate::R<XtalDebugSpec>;
#[doc = "Register `XTAL_DEBUG` writer"]
pub type W = crate::W<XtalDebugSpec>;
#[doc = "Field `xtal_arb_manual_en` reader - Aribter manual mode enable"]
pub type XtalArbManualEnR = crate::BitReader;
#[doc = "Field `xtal_arb_manual_en` writer - Aribter manual mode enable"]
pub type XtalArbManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_auto_pdck` reader - Auto PDCK enable signal. If set this bit, arbiter will do PDCK before XTAL enter LPS mode. Once PDCK done , this bit will auto set 0"]
pub type XtalAutoPdckR = crate::BitReader;
#[doc = "Field `xtal_auto_pdck` writer - Auto PDCK enable signal. If set this bit, arbiter will do PDCK before XTAL enter LPS mode. Once PDCK done , this bit will auto set 0"]
pub type XtalAutoPdckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `manu_pow_xtal` reader - manual mode POW_XTAL"]
pub type ManuPowXtalR = crate::BitReader;
#[doc = "Field `manu_pow_xtal` writer - manual mode POW_XTAL"]
pub type ManuPowXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `manu_pow_xtal_lps` reader - manual mode POW_XTAL_LPS"]
pub type ManuPowXtalLpsR = crate::BitReader;
#[doc = "Field `manu_pow_xtal_lps` writer - manual mode POW_XTAL_LPS"]
pub type ManuPowXtalLpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `manu_xtal_mode` reader - manual mode XTAL_MODE"]
pub type ManuXtalModeR = crate::BitReader;
#[doc = "Field `manu_xtal_mode` writer - manual mode XTAL_MODE"]
pub type ManuXtalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `manu_pow_bg` reader - manual mode POW_BG"]
pub type ManuPowBgR = crate::BitReader;
#[doc = "Field `manu_pow_bg` writer - manual mode POW_BG"]
pub type ManuPowBgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPDCK_VREF_OUT` reader - PDCK digital calculate result for ANA comparator"]
pub type XpdckVrefOutR = crate::FieldReader;
#[doc = "Field `XPDCK_READY` reader - signal for xtal"]
pub type XpdckReadyR = crate::BitReader;
#[doc = "Field `XPDCK_BUSY` reader - signal for xtal"]
pub type XpdckBusyR = crate::BitReader;
#[doc = "Field `XTAL_GM_OUT` reader - AACK digital calculate result for ANA comparator"]
pub type XtalGmOutR = crate::FieldReader;
#[doc = "Field `XAAC_READY` reader - signal for xtal"]
pub type XaacReadyR = crate::BitReader;
#[doc = "Field `XAAC_PK_SEL` reader - XAAC_PK_SEL(AAC calibration address)"]
pub type XaacPkSelR = crate::FieldReader;
#[doc = "Field `xtal_tout` reader - xtal arbiter wait feedback signal timeout delay cycle, if this valua is max, arbiter will disable timeout function. Time unit is xtal_tutime"]
pub type XtalToutR = crate::FieldReader<u16>;
#[doc = "Field `xtal_tout` writer - xtal arbiter wait feedback signal timeout delay cycle, if this valua is max, arbiter will disable timeout function. Time unit is xtal_tutime"]
pub type XtalToutW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Aribter manual mode enable"]
    #[inline(always)]
    pub fn xtal_arb_manual_en(&self) -> XtalArbManualEnR {
        XtalArbManualEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto PDCK enable signal. If set this bit, arbiter will do PDCK before XTAL enter LPS mode. Once PDCK done , this bit will auto set 0"]
    #[inline(always)]
    pub fn xtal_auto_pdck(&self) -> XtalAutoPdckR {
        XtalAutoPdckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - manual mode POW_XTAL"]
    #[inline(always)]
    pub fn manu_pow_xtal(&self) -> ManuPowXtalR {
        ManuPowXtalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - manual mode POW_XTAL_LPS"]
    #[inline(always)]
    pub fn manu_pow_xtal_lps(&self) -> ManuPowXtalLpsR {
        ManuPowXtalLpsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - manual mode XTAL_MODE"]
    #[inline(always)]
    pub fn manu_xtal_mode(&self) -> ManuXtalModeR {
        ManuXtalModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - manual mode POW_BG"]
    #[inline(always)]
    pub fn manu_pow_bg(&self) -> ManuPowBgR {
        ManuPowBgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - PDCK digital calculate result for ANA comparator"]
    #[inline(always)]
    pub fn xpdck_vref_out(&self) -> XpdckVrefOutR {
        XpdckVrefOutR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - signal for xtal"]
    #[inline(always)]
    pub fn xpdck_ready(&self) -> XpdckReadyR {
        XpdckReadyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - signal for xtal"]
    #[inline(always)]
    pub fn xpdck_busy(&self) -> XpdckBusyR {
        XpdckBusyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - AACK digital calculate result for ANA comparator"]
    #[inline(always)]
    pub fn xtal_gm_out(&self) -> XtalGmOutR {
        XtalGmOutR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - signal for xtal"]
    #[inline(always)]
    pub fn xaac_ready(&self) -> XaacReadyR {
        XaacReadyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - XAAC_PK_SEL(AAC calibration address)"]
    #[inline(always)]
    pub fn xaac_pk_sel(&self) -> XaacPkSelR {
        XaacPkSelR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:31 - xtal arbiter wait feedback signal timeout delay cycle, if this valua is max, arbiter will disable timeout function. Time unit is xtal_tutime"]
    #[inline(always)]
    pub fn xtal_tout(&self) -> XtalToutR {
        XtalToutR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Aribter manual mode enable"]
    #[inline(always)]
    pub fn xtal_arb_manual_en(&mut self) -> XtalArbManualEnW<'_, XtalDebugSpec> {
        XtalArbManualEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto PDCK enable signal. If set this bit, arbiter will do PDCK before XTAL enter LPS mode. Once PDCK done , this bit will auto set 0"]
    #[inline(always)]
    pub fn xtal_auto_pdck(&mut self) -> XtalAutoPdckW<'_, XtalDebugSpec> {
        XtalAutoPdckW::new(self, 1)
    }
    #[doc = "Bit 2 - manual mode POW_XTAL"]
    #[inline(always)]
    pub fn manu_pow_xtal(&mut self) -> ManuPowXtalW<'_, XtalDebugSpec> {
        ManuPowXtalW::new(self, 2)
    }
    #[doc = "Bit 3 - manual mode POW_XTAL_LPS"]
    #[inline(always)]
    pub fn manu_pow_xtal_lps(&mut self) -> ManuPowXtalLpsW<'_, XtalDebugSpec> {
        ManuPowXtalLpsW::new(self, 3)
    }
    #[doc = "Bit 4 - manual mode XTAL_MODE"]
    #[inline(always)]
    pub fn manu_xtal_mode(&mut self) -> ManuXtalModeW<'_, XtalDebugSpec> {
        ManuXtalModeW::new(self, 4)
    }
    #[doc = "Bit 5 - manual mode POW_BG"]
    #[inline(always)]
    pub fn manu_pow_bg(&mut self) -> ManuPowBgW<'_, XtalDebugSpec> {
        ManuPowBgW::new(self, 5)
    }
    #[doc = "Bits 22:31 - xtal arbiter wait feedback signal timeout delay cycle, if this valua is max, arbiter will disable timeout function. Time unit is xtal_tutime"]
    #[inline(always)]
    pub fn xtal_tout(&mut self) -> XtalToutW<'_, XtalDebugSpec> {
        XtalToutW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalDebugSpec;
impl crate::RegisterSpec for XtalDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_debug::R`](R) reader structure"]
impl crate::Readable for XtalDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_debug::W`](W) writer structure"]
impl crate::Writable for XtalDebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_DEBUG to value 0x71c0_0002"]
impl crate::Resettable for XtalDebugSpec {
    const RESET_VALUE: u32 = 0x71c0_0002;
}
