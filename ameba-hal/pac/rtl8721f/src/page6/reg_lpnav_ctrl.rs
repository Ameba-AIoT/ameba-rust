#[doc = "Register `REG_LPNAV_CTRL` reader"]
pub type R = crate::R<RegLpnavCtrlSpec>;
#[doc = "Register `REG_LPNAV_CTRL` writer"]
pub type W = crate::W<RegLpnavCtrlSpec>;
#[doc = "Field `BIT_LPNAV_THR` reader - When protected NAV in the received frame is greater than this threshold value, then MAC enters into LPNAV mode. The time unit is 16us."]
pub type BitLpnavThrR = crate::FieldReader<u16>;
#[doc = "Field `BIT_LPNAV_THR` writer - When protected NAV in the received frame is greater than this threshold value, then MAC enters into LPNAV mode. The time unit is 16us."]
pub type BitLpnavThrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_LPNAV_EARLY` reader - When in LPNAV mode, MAC will recall BB and RF into RX idle mode. The unit is us."]
pub type BitLpnavEarlyR = crate::FieldReader<u16>;
#[doc = "Field `BIT_LPNAV_EARLY` writer - When in LPNAV mode, MAC will recall BB and RF into RX idle mode. The unit is us."]
pub type BitLpnavEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `BIT_LPNAV_EN` reader - Low Power NAV Mode Enable. When this bit is set, enable MAC low power NAV mode"]
pub type BitLpnavEnR = crate::BitReader;
#[doc = "Field `BIT_LPNAV_EN` writer - Low Power NAV Mode Enable. When this bit is set, enable MAC low power NAV mode"]
pub type BitLpnavEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - When protected NAV in the received frame is greater than this threshold value, then MAC enters into LPNAV mode. The time unit is 16us."]
    #[inline(always)]
    pub fn bit_lpnav_thr(&self) -> BitLpnavThrR {
        BitLpnavThrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - When in LPNAV mode, MAC will recall BB and RF into RX idle mode. The unit is us."]
    #[inline(always)]
    pub fn bit_lpnav_early(&self) -> BitLpnavEarlyR {
        BitLpnavEarlyR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Low Power NAV Mode Enable. When this bit is set, enable MAC low power NAV mode"]
    #[inline(always)]
    pub fn bit_lpnav_en(&self) -> BitLpnavEnR {
        BitLpnavEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - When protected NAV in the received frame is greater than this threshold value, then MAC enters into LPNAV mode. The time unit is 16us."]
    #[inline(always)]
    pub fn bit_lpnav_thr(&mut self) -> BitLpnavThrW<'_, RegLpnavCtrlSpec> {
        BitLpnavThrW::new(self, 0)
    }
    #[doc = "Bits 16:30 - When in LPNAV mode, MAC will recall BB and RF into RX idle mode. The unit is us."]
    #[inline(always)]
    pub fn bit_lpnav_early(&mut self) -> BitLpnavEarlyW<'_, RegLpnavCtrlSpec> {
        BitLpnavEarlyW::new(self, 16)
    }
    #[doc = "Bit 31 - Low Power NAV Mode Enable. When this bit is set, enable MAC low power NAV mode"]
    #[inline(always)]
    pub fn bit_lpnav_en(&mut self) -> BitLpnavEnW<'_, RegLpnavCtrlSpec> {
        BitLpnavEnW::new(self, 31)
    }
}
#[doc = "Low Power NAV Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpnav_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpnav_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpnavCtrlSpec;
impl crate::RegisterSpec for RegLpnavCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpnav_ctrl::R`](R) reader structure"]
impl crate::Readable for RegLpnavCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpnav_ctrl::W`](W) writer structure"]
impl crate::Writable for RegLpnavCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPNAV_CTRL to value 0x000a_0200"]
impl crate::Resettable for RegLpnavCtrlSpec {
    const RESET_VALUE: u32 = 0x000a_0200;
}
