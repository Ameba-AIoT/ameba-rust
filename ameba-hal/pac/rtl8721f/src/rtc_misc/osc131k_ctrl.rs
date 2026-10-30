#[doc = "Register `OSC131K_CTRL` reader"]
pub type R = crate::R<Osc131kCtrlSpec>;
#[doc = "Register `OSC131K_CTRL` writer"]
pub type W = crate::W<Osc131kCtrlSpec>;
#[doc = "Field `RCAL` reader - // \\[RF\\]\\[32KOSC\\] REG_CTRL // osc frequency is highier as RCAL's larger"]
pub type RcalR = crate::FieldReader;
#[doc = "Field `RCAL` writer - // \\[RF\\]\\[32KOSC\\] REG_CTRL // osc frequency is highier as RCAL's larger"]
pub type RcalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `dmy_2` reader - // \\[RF\\]\\[32KOSC\\] REG_CTRL // dummy"]
pub type Dmy2R = crate::FieldReader;
#[doc = "Field `dmy_2` writer - // \\[RF\\]\\[32KOSC\\] REG_CTRL // dummy"]
pub type Dmy2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GATED_STUP_OK` reader - //1'b1 : Close Startup standby current ; //1'b0 : Startup standby current"]
pub type GatedStupOkR = crate::BitReader;
#[doc = "Field `GATED_STUP_OK` writer - //1'b1 : Close Startup standby current ; //1'b0 : Startup standby current"]
pub type GatedStupOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_LDO_VREF` reader - // \\[RF\\]\\[32KOSC\\] REG_CTRL // 0: LDO low valtage setting 1:high voltage"]
pub type SelLdoVrefR = crate::BitReader;
#[doc = "Field `SEL_LDO_VREF` writer - // \\[RF\\]\\[32KOSC\\] REG_CTRL // 0: LDO low valtage setting 1:high voltage"]
pub type SelLdoVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_rtc` reader - 1: Enable RTC function 0: Disable"]
pub type FenRtcR = crate::BitReader;
#[doc = "Field `fen_rtc` writer - 1: Enable RTC function 0: Disable"]
pub type FenRtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // osc frequency is highier as RCAL's larger"]
    #[inline(always)]
    pub fn rcal(&self) -> RcalR {
        RcalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // dummy"]
    #[inline(always)]
    pub fn dmy_2(&self) -> Dmy2R {
        Dmy2R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 14 - //1'b1 : Close Startup standby current ; //1'b0 : Startup standby current"]
    #[inline(always)]
    pub fn gated_stup_ok(&self) -> GatedStupOkR {
        GatedStupOkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // 0: LDO low valtage setting 1:high voltage"]
    #[inline(always)]
    pub fn sel_ldo_vref(&self) -> SelLdoVrefR {
        SelLdoVrefR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Enable RTC function 0: Disable"]
    #[inline(always)]
    pub fn fen_rtc(&self) -> FenRtcR {
        FenRtcR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // osc frequency is highier as RCAL's larger"]
    #[inline(always)]
    pub fn rcal(&mut self) -> RcalW<'_, Osc131kCtrlSpec> {
        RcalW::new(self, 0)
    }
    #[doc = "Bits 6:13 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // dummy"]
    #[inline(always)]
    pub fn dmy_2(&mut self) -> Dmy2W<'_, Osc131kCtrlSpec> {
        Dmy2W::new(self, 6)
    }
    #[doc = "Bit 14 - //1'b1 : Close Startup standby current ; //1'b0 : Startup standby current"]
    #[inline(always)]
    pub fn gated_stup_ok(&mut self) -> GatedStupOkW<'_, Osc131kCtrlSpec> {
        GatedStupOkW::new(self, 14)
    }
    #[doc = "Bit 15 - // \\[RF\\]\\[32KOSC\\] REG_CTRL // 0: LDO low valtage setting 1:high voltage"]
    #[inline(always)]
    pub fn sel_ldo_vref(&mut self) -> SelLdoVrefW<'_, Osc131kCtrlSpec> {
        SelLdoVrefW::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Enable RTC function 0: Disable"]
    #[inline(always)]
    pub fn fen_rtc(&mut self) -> FenRtcW<'_, Osc131kCtrlSpec> {
        FenRtcW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`osc131k_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc131k_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc131kCtrlSpec;
impl crate::RegisterSpec for Osc131kCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc131k_ctrl::R`](R) reader structure"]
impl crate::Readable for Osc131kCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`osc131k_ctrl::W`](W) writer structure"]
impl crate::Writable for Osc131kCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC131K_CTRL to value 0x20"]
impl crate::Resettable for Osc131kCtrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
