#[doc = "Register `REG_SSC_CTRL4` reader"]
pub type R = crate::R<RegSscCtrl4Spec>;
#[doc = "Register `REG_SSC_CTRL4` writer"]
pub type W = crate::W<RegSscCtrl4Spec>;
#[doc = "Field `REG_TIME_RDY_CKOUT` reader - output signal CLKRDY delay control"]
pub type RegTimeRdyCkoutR = crate::FieldReader;
#[doc = "Field `REG_TIME_RDY_CKOUT` writer - output signal CLKRDY delay control"]
pub type RegTimeRdyCkoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TIME2_RST_WIDTH` reader - PLL watchdog signal pulse width: 2'b00: watchdog reset pulse width =2 CKREF period 2'b01: watchdog reset pulse width =4 CKREF period 2'b01: watchdog reset pulse width =8 CKREF period 2'b01: watchdog reset pulse width =20 CKREF period"]
pub type RegTime2RstWidthR = crate::FieldReader;
#[doc = "Field `REG_TIME2_RST_WIDTH` writer - PLL watchdog signal pulse width: 2'b00: watchdog reset pulse width =2 CKREF period 2'b01: watchdog reset pulse width =4 CKREF period 2'b01: watchdog reset pulse width =8 CKREF period 2'b01: watchdog reset pulse width =20 CKREF period"]
pub type RegTime2RstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMOD` reader - "]
pub type FmodR = crate::BitReader;
#[doc = "Field `OC_DONE` reader - "]
pub type OcDoneR = crate::BitReader;
#[doc = "Field `CLKRDY` reader - "]
pub type ClkrdyR = crate::BitReader;
#[doc = "Field `NCODE_DEBUG` reader - "]
pub type NcodeDebugR = crate::FieldReader;
#[doc = "Field `FCODE_DEBUG` reader - "]
pub type FcodeDebugR = crate::FieldReader<u16>;
#[doc = "Field `fen_pll` reader - SSC global reset, at same time reset PLL also when pll on , set it to 1 after PWC is enable when pll off , set it to 0 at last"]
pub type FenPllR = crate::BitReader;
#[doc = "Field `fen_pll` writer - SSC global reset, at same time reset PLL also when pll on , set it to 1 after PWC is enable when pll off , set it to 0 at last"]
pub type FenPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC_UALV_PLL` reader - keep UA10PC_EN for PLL"]
pub type PwcUalvPllR = crate::BitReader;
#[doc = "Field `PWC_UALV_PLL` writer - keep UA10PC_EN for PLL"]
pub type PwcUalvPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC_UAHV_PLL` reader - keep UA33PC_EN for PLL"]
pub type PwcUahvPllR = crate::BitReader;
#[doc = "Field `PWC_UAHV_PLL` writer - keep UA33PC_EN for PLL"]
pub type PwcUahvPllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - output signal CLKRDY delay control"]
    #[inline(always)]
    pub fn reg_time_rdy_ckout(&self) -> RegTimeRdyCkoutR {
        RegTimeRdyCkoutR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL watchdog signal pulse width: 2'b00: watchdog reset pulse width =2 CKREF period 2'b01: watchdog reset pulse width =4 CKREF period 2'b01: watchdog reset pulse width =8 CKREF period 2'b01: watchdog reset pulse width =20 CKREF period"]
    #[inline(always)]
    pub fn reg_time2_rst_width(&self) -> RegTime2RstWidthR {
        RegTime2RstWidthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fmod(&self) -> FmodR {
        FmodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn oc_done(&self) -> OcDoneR {
        OcDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn ncode_debug(&self) -> NcodeDebugR {
        NcodeDebugR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:26"]
    #[inline(always)]
    pub fn fcode_debug(&self) -> FcodeDebugR {
        FcodeDebugR::new(((self.bits >> 13) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - SSC global reset, at same time reset PLL also when pll on , set it to 1 after PWC is enable when pll off , set it to 0 at last"]
    #[inline(always)]
    pub fn fen_pll(&self) -> FenPllR {
        FenPllR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - keep UA10PC_EN for PLL"]
    #[inline(always)]
    pub fn pwc_ualv_pll(&self) -> PwcUalvPllR {
        PwcUalvPllR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - keep UA33PC_EN for PLL"]
    #[inline(always)]
    pub fn pwc_uahv_pll(&self) -> PwcUahvPllR {
        PwcUahvPllR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - output signal CLKRDY delay control"]
    #[inline(always)]
    pub fn reg_time_rdy_ckout(&mut self) -> RegTimeRdyCkoutW<'_, RegSscCtrl4Spec> {
        RegTimeRdyCkoutW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PLL watchdog signal pulse width: 2'b00: watchdog reset pulse width =2 CKREF period 2'b01: watchdog reset pulse width =4 CKREF period 2'b01: watchdog reset pulse width =8 CKREF period 2'b01: watchdog reset pulse width =20 CKREF period"]
    #[inline(always)]
    pub fn reg_time2_rst_width(&mut self) -> RegTime2RstWidthW<'_, RegSscCtrl4Spec> {
        RegTime2RstWidthW::new(self, 2)
    }
    #[doc = "Bit 29 - SSC global reset, at same time reset PLL also when pll on , set it to 1 after PWC is enable when pll off , set it to 0 at last"]
    #[inline(always)]
    pub fn fen_pll(&mut self) -> FenPllW<'_, RegSscCtrl4Spec> {
        FenPllW::new(self, 29)
    }
    #[doc = "Bit 30 - keep UA10PC_EN for PLL"]
    #[inline(always)]
    pub fn pwc_ualv_pll(&mut self) -> PwcUalvPllW<'_, RegSscCtrl4Spec> {
        PwcUalvPllW::new(self, 30)
    }
    #[doc = "Bit 31 - keep UA33PC_EN for PLL"]
    #[inline(always)]
    pub fn pwc_uahv_pll(&mut self) -> PwcUahvPllW<'_, RegSscCtrl4Spec> {
        PwcUahvPllW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ssc_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ssc_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSscCtrl4Spec;
impl crate::RegisterSpec for RegSscCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ssc_ctrl4::R`](R) reader structure"]
impl crate::Readable for RegSscCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ssc_ctrl4::W`](W) writer structure"]
impl crate::Writable for RegSscCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SSC_CTRL4 to value 0x02"]
impl crate::Resettable for RegSscCtrl4Spec {
    const RESET_VALUE: u32 = 0x02;
}
