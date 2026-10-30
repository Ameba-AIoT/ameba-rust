#[doc = "Register `clock_control_3` reader"]
pub type R = crate::R<ClockControl3Spec>;
#[doc = "Register `clock_control_3` writer"]
pub type W = crate::W<ClockControl3Spec>;
#[doc = "Field `dmic1_clk_sel` reader - Set clock of digital microphone 3'b000: 5MHz 3'b001: 2.5MHz 3'b010: 1.25MHz 3'b011: 625kHz 3'b100: 312.5kHz 3'b101: Reserved 3'b110: Reserved 3'b111: 769.2kHz"]
pub type Dmic1ClkSelR = crate::FieldReader;
#[doc = "Field `dmic1_clk_sel` writer - Set clock of digital microphone 3'b000: 5MHz 3'b001: 2.5MHz 3'b010: 1.25MHz 3'b011: 625kHz 3'b100: 312.5kHz 3'b101: Reserved 3'b110: Reserved 3'b111: 769.2kHz"]
pub type Dmic1ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dmic1_clk_en` reader - Digital microphone clock enable"]
pub type Dmic1ClkEnR = crate::BitReader;
#[doc = "Field `dmic1_clk_en` writer - Digital microphone clock enable"]
pub type Dmic1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Set clock of digital microphone 3'b000: 5MHz 3'b001: 2.5MHz 3'b010: 1.25MHz 3'b011: 625kHz 3'b100: 312.5kHz 3'b101: Reserved 3'b110: Reserved 3'b111: 769.2kHz"]
    #[inline(always)]
    pub fn dmic1_clk_sel(&self) -> Dmic1ClkSelR {
        Dmic1ClkSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Digital microphone clock enable"]
    #[inline(always)]
    pub fn dmic1_clk_en(&self) -> Dmic1ClkEnR {
        Dmic1ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set clock of digital microphone 3'b000: 5MHz 3'b001: 2.5MHz 3'b010: 1.25MHz 3'b011: 625kHz 3'b100: 312.5kHz 3'b101: Reserved 3'b110: Reserved 3'b111: 769.2kHz"]
    #[inline(always)]
    pub fn dmic1_clk_sel(&mut self) -> Dmic1ClkSelW<'_, ClockControl3Spec> {
        Dmic1ClkSelW::new(self, 0)
    }
    #[doc = "Bit 3 - Digital microphone clock enable"]
    #[inline(always)]
    pub fn dmic1_clk_en(&mut self) -> Dmic1ClkEnW<'_, ClockControl3Spec> {
        Dmic1ClkEnW::new(self, 3)
    }
}
#[doc = "Clock Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl3Spec;
impl crate::RegisterSpec for ClockControl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_3::R`](R) reader structure"]
impl crate::Readable for ClockControl3Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_3::W`](W) writer structure"]
impl crate::Writable for ClockControl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_3 to value 0x01"]
impl crate::Resettable for ClockControl3Spec {
    const RESET_VALUE: u32 = 0x01;
}
