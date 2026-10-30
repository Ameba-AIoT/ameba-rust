#[doc = "Register `REG_SDIO_CTRL` reader"]
pub type R = crate::R<RegSdioCtrlSpec>;
#[doc = "Register `REG_SDIO_CTRL` writer"]
pub type W = crate::W<RegSdioCtrlSpec>;
#[doc = "Field `sdd_pmux_fen` reader - after sdio pinmux is configured, set this bit to 1"]
pub type SddPmuxFenR = crate::BitReader;
#[doc = "Field `sdd_pmux_fen` writer - after sdio pinmux is configured, set this bit to 1"]
pub type SddPmuxFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdd_ahbm_nonsec` reader - sdio wifi ahb mster 0: sec, 1: non-sec,"]
pub type SddAhbmNonsecR = crate::BitReader;
#[doc = "Field `sdd_ahbm_nonsec` writer - sdio wifi ahb mster 0: sec, 1: non-sec,"]
pub type SddAhbmNonsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdd_update_fw` reader - enable updating firmware via sdio"]
pub type SddUpdateFwR = crate::BitReader;
#[doc = "Field `sdd_update_fw` writer - enable updating firmware via sdio"]
pub type SddUpdateFwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_bt_en` reader - enable sdio bt function(fn2)"]
pub type SdioBtEnR = crate::BitReader;
#[doc = "Field `sdio_bt_en` writer - enable sdio bt function(fn2)"]
pub type SdioBtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - after sdio pinmux is configured, set this bit to 1"]
    #[inline(always)]
    pub fn sdd_pmux_fen(&self) -> SddPmuxFenR {
        SddPmuxFenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - sdio wifi ahb mster 0: sec, 1: non-sec,"]
    #[inline(always)]
    pub fn sdd_ahbm_nonsec(&self) -> SddAhbmNonsecR {
        SddAhbmNonsecR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable updating firmware via sdio"]
    #[inline(always)]
    pub fn sdd_update_fw(&self) -> SddUpdateFwR {
        SddUpdateFwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable sdio bt function(fn2)"]
    #[inline(always)]
    pub fn sdio_bt_en(&self) -> SdioBtEnR {
        SdioBtEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - after sdio pinmux is configured, set this bit to 1"]
    #[inline(always)]
    pub fn sdd_pmux_fen(&mut self) -> SddPmuxFenW<'_, RegSdioCtrlSpec> {
        SddPmuxFenW::new(self, 0)
    }
    #[doc = "Bit 6 - sdio wifi ahb mster 0: sec, 1: non-sec,"]
    #[inline(always)]
    pub fn sdd_ahbm_nonsec(&mut self) -> SddAhbmNonsecW<'_, RegSdioCtrlSpec> {
        SddAhbmNonsecW::new(self, 6)
    }
    #[doc = "Bit 7 - enable updating firmware via sdio"]
    #[inline(always)]
    pub fn sdd_update_fw(&mut self) -> SddUpdateFwW<'_, RegSdioCtrlSpec> {
        SddUpdateFwW::new(self, 7)
    }
    #[doc = "Bit 8 - enable sdio bt function(fn2)"]
    #[inline(always)]
    pub fn sdio_bt_en(&mut self) -> SdioBtEnW<'_, RegSdioCtrlSpec> {
        SdioBtEnW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sdio_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sdio_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSdioCtrlSpec;
impl crate::RegisterSpec for RegSdioCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sdio_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSdioCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sdio_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSdioCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SDIO_CTRL to value 0x0180"]
impl crate::Resettable for RegSdioCtrlSpec {
    const RESET_VALUE: u32 = 0x0180;
}
