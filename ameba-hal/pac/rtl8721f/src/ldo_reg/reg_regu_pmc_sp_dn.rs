#[doc = "Register `REG_REGU_PMC_SP_DN` reader"]
pub type R = crate::R<RegReguPmcSpDnSpec>;
#[doc = "Register `REG_REGU_PMC_SP_DN` writer"]
pub type W = crate::W<RegReguPmcSpDnSpec>;
#[doc = "Field `apmc_sp3` reader - Start Point for ST5 dn"]
pub type ApmcSp3R = crate::FieldReader;
#[doc = "Field `apmc_sp3` writer - Start Point for ST5 dn"]
pub type ApmcSp3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `apmc_sp4` reader - Start Point for ST4/ST3 dn"]
pub type ApmcSp4R = crate::FieldReader;
#[doc = "Field `apmc_sp4` writer - Start Point for ST4/ST3 dn"]
pub type ApmcSp4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `apmc_sp5` reader - Start Point for ST2 dn , noused"]
pub type ApmcSp5R = crate::FieldReader;
#[doc = "Field `apmc_sp5` writer - Start Point for ST2 dn , noused"]
pub type ApmcSp5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Start Point for ST5 dn"]
    #[inline(always)]
    pub fn apmc_sp3(&self) -> ApmcSp3R {
        ApmcSp3R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Start Point for ST4/ST3 dn"]
    #[inline(always)]
    pub fn apmc_sp4(&self) -> ApmcSp4R {
        ApmcSp4R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Start Point for ST2 dn , noused"]
    #[inline(always)]
    pub fn apmc_sp5(&self) -> ApmcSp5R {
        ApmcSp5R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Start Point for ST5 dn"]
    #[inline(always)]
    pub fn apmc_sp3(&mut self) -> ApmcSp3W<'_, RegReguPmcSpDnSpec> {
        ApmcSp3W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Start Point for ST4/ST3 dn"]
    #[inline(always)]
    pub fn apmc_sp4(&mut self) -> ApmcSp4W<'_, RegReguPmcSpDnSpec> {
        ApmcSp4W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Start Point for ST2 dn , noused"]
    #[inline(always)]
    pub fn apmc_sp5(&mut self) -> ApmcSp5W<'_, RegReguPmcSpDnSpec> {
        ApmcSp5W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_regu_pmc_sp_dn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_regu_pmc_sp_dn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegReguPmcSpDnSpec;
impl crate::RegisterSpec for RegReguPmcSpDnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_regu_pmc_sp_dn::R`](R) reader structure"]
impl crate::Readable for RegReguPmcSpDnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_regu_pmc_sp_dn::W`](W) writer structure"]
impl crate::Writable for RegReguPmcSpDnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REGU_PMC_SP_DN to value 0x0051_4f4c"]
impl crate::Resettable for RegReguPmcSpDnSpec {
    const RESET_VALUE: u32 = 0x0051_4f4c;
}
