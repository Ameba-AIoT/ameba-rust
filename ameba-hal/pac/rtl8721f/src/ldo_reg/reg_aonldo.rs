#[doc = "Register `REG_AONLDO` reader"]
pub type R = crate::R<RegAonldoSpec>;
#[doc = "Register `REG_AONLDO` writer"]
pub type W = crate::W<RegAonldoSpec>;
#[doc = "Field `AONLDO_VREF_SEL` reader - AONLDO VREF_SEL 0 (default) VREF = 0P600V 1 VREF = 0P575V"]
pub type AonldoVrefSelR = crate::BitReader;
#[doc = "Field `AONLDO_VREF_SEL` writer - AONLDO VREF_SEL 0 (default) VREF = 0P600V 1 VREF = 0P575V"]
pub type AonldoVrefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LV_POR_VREF_SEL` reader - LV_POR VREF 0 (default) VREF = 0P600V 1 VREF = 0P450V"]
pub type LvPorVrefSelR = crate::BitReader;
#[doc = "Field `LV_POR_VREF_SEL` writer - LV_POR VREF 0 (default) VREF = 0P600V 1 VREF = 0P450V"]
pub type LvPorVrefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_5` reader - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy5R = crate::FieldReader;
#[doc = "Field `dmy_5` writer - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AONLDO_VOSEL_H` reader - AONLDO09 voltage select 0000: 0.528V 0.464V 0001: 0.630V 0.554V 0010: 0.659V 0.579V 0011: 0.690V 0.606V 0100: 0.724V 0.636V 0101: 0.742V 0.652V 0110: 0.761V 0.669V 0111: 0.782V 0.687V 1000: 0.803V 0.706V 1001: 0.826V 0.725V 1010: 0.850V 0.746V 1011: 0.875V 0.769V 1100: 0.902V 0.792V 1101: 0.930V 0.817V 1110: 0.960V 0.844V 1111: 0.993V 0.872V"]
pub type AonldoVoselHR = crate::FieldReader;
#[doc = "Field `AONLDO_VOSEL_H` writer - AONLDO09 voltage select 0000: 0.528V 0.464V 0001: 0.630V 0.554V 0010: 0.659V 0.579V 0011: 0.690V 0.606V 0100: 0.724V 0.636V 0101: 0.742V 0.652V 0110: 0.761V 0.669V 0111: 0.782V 0.687V 1000: 0.803V 0.706V 1001: 0.826V 0.725V 1010: 0.850V 0.746V 1011: 0.875V 0.769V 1100: 0.902V 0.792V 1101: 0.930V 0.817V 1110: 0.960V 0.844V 1111: 0.993V 0.872V"]
pub type AonldoVoselHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en_sleep_l` reader - Enable LDO09H sleep mode 0 : disable (default) 1 : enable"]
pub type EnSleepLR = crate::BitReader;
#[doc = "Field `en_sleep_l` writer - Enable LDO09H sleep mode 0 : disable (default) 1 : enable"]
pub type EnSleepLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_4` reader - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy4R = crate::BitReader;
#[doc = "Field `dmy_4` writer - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_3` reader - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy3R = crate::FieldReader;
#[doc = "Field `dmy_3` writer - // \\[REGU\\]\\[AONLDO\\] dummy"]
pub type Dmy3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - AONLDO VREF_SEL 0 (default) VREF = 0P600V 1 VREF = 0P575V"]
    #[inline(always)]
    pub fn aonldo_vref_sel(&self) -> AonldoVrefSelR {
        AonldoVrefSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LV_POR VREF 0 (default) VREF = 0P600V 1 VREF = 0P450V"]
    #[inline(always)]
    pub fn lv_por_vref_sel(&self) -> LvPorVrefSelR {
        LvPorVrefSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_5(&self) -> Dmy5R {
        Dmy5R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AONLDO09 voltage select 0000: 0.528V 0.464V 0001: 0.630V 0.554V 0010: 0.659V 0.579V 0011: 0.690V 0.606V 0100: 0.724V 0.636V 0101: 0.742V 0.652V 0110: 0.761V 0.669V 0111: 0.782V 0.687V 1000: 0.803V 0.706V 1001: 0.826V 0.725V 1010: 0.850V 0.746V 1011: 0.875V 0.769V 1100: 0.902V 0.792V 1101: 0.930V 0.817V 1110: 0.960V 0.844V 1111: 0.993V 0.872V"]
    #[inline(always)]
    pub fn aonldo_vosel_h(&self) -> AonldoVoselHR {
        AonldoVoselHR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable LDO09H sleep mode 0 : disable (default) 1 : enable"]
    #[inline(always)]
    pub fn en_sleep_l(&self) -> EnSleepLR {
        EnSleepLR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_4(&self) -> Dmy4R {
        Dmy4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_3(&self) -> Dmy3R {
        Dmy3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AONLDO VREF_SEL 0 (default) VREF = 0P600V 1 VREF = 0P575V"]
    #[inline(always)]
    pub fn aonldo_vref_sel(&mut self) -> AonldoVrefSelW<'_, RegAonldoSpec> {
        AonldoVrefSelW::new(self, 0)
    }
    #[doc = "Bit 1 - LV_POR VREF 0 (default) VREF = 0P600V 1 VREF = 0P450V"]
    #[inline(always)]
    pub fn lv_por_vref_sel(&mut self) -> LvPorVrefSelW<'_, RegAonldoSpec> {
        LvPorVrefSelW::new(self, 1)
    }
    #[doc = "Bits 2:3 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_5(&mut self) -> Dmy5W<'_, RegAonldoSpec> {
        Dmy5W::new(self, 2)
    }
    #[doc = "Bits 4:7 - AONLDO09 voltage select 0000: 0.528V 0.464V 0001: 0.630V 0.554V 0010: 0.659V 0.579V 0011: 0.690V 0.606V 0100: 0.724V 0.636V 0101: 0.742V 0.652V 0110: 0.761V 0.669V 0111: 0.782V 0.687V 1000: 0.803V 0.706V 1001: 0.826V 0.725V 1010: 0.850V 0.746V 1011: 0.875V 0.769V 1100: 0.902V 0.792V 1101: 0.930V 0.817V 1110: 0.960V 0.844V 1111: 0.993V 0.872V"]
    #[inline(always)]
    pub fn aonldo_vosel_h(&mut self) -> AonldoVoselHW<'_, RegAonldoSpec> {
        AonldoVoselHW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable LDO09H sleep mode 0 : disable (default) 1 : enable"]
    #[inline(always)]
    pub fn en_sleep_l(&mut self) -> EnSleepLW<'_, RegAonldoSpec> {
        EnSleepLW::new(self, 8)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_4(&mut self) -> Dmy4W<'_, RegAonldoSpec> {
        Dmy4W::new(self, 9)
    }
    #[doc = "Bits 10:15 - // \\[REGU\\]\\[AONLDO\\] dummy"]
    #[inline(always)]
    pub fn dmy_3(&mut self) -> Dmy3W<'_, RegAonldoSpec> {
        Dmy3W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aonldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aonldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonldoSpec;
impl crate::RegisterSpec for RegAonldoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aonldo::R`](R) reader structure"]
impl crate::Readable for RegAonldoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aonldo::W`](W) writer structure"]
impl crate::Writable for RegAonldoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AONLDO to value 0xa0"]
impl crate::Resettable for RegAonldoSpec {
    const RESET_VALUE: u32 = 0xa0;
}
