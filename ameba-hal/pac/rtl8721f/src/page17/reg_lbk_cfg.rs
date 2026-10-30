#[doc = "Register `REG_LBK_CFG` reader"]
pub type R = crate::R<RegLbkCfgSpec>;
#[doc = "Register `REG_LBK_CFG` writer"]
pub type W = crate::W<RegLbkCfgSpec>;
#[doc = "Field `r_maclbk_rdy_period` reader - tdrdy/rdrdy interval count"]
pub type RMaclbkRdyPeriodR = crate::FieldReader<u16>;
#[doc = "Field `r_maclbk_rdy_period` writer - tdrdy/rdrdy interval count"]
pub type RMaclbkRdyPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - tdrdy/rdrdy interval count"]
    #[inline(always)]
    pub fn r_maclbk_rdy_period(&self) -> RMaclbkRdyPeriodR {
        RMaclbkRdyPeriodR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - tdrdy/rdrdy interval count"]
    #[inline(always)]
    pub fn r_maclbk_rdy_period(&mut self) -> RMaclbkRdyPeriodW<'_, RegLbkCfgSpec> {
        RMaclbkRdyPeriodW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lbk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lbk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLbkCfgSpec;
impl crate::RegisterSpec for RegLbkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lbk_cfg::R`](R) reader structure"]
impl crate::Readable for RegLbkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lbk_cfg::W`](W) writer structure"]
impl crate::Writable for RegLbkCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LBK_CFG to value 0x12"]
impl crate::Resettable for RegLbkCfgSpec {
    const RESET_VALUE: u32 = 0x12;
}
