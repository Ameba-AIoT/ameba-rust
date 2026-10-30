#[doc = "Register `REG_REGU_PMC_SP_UP` reader"]
pub type R = crate::R<RegReguPmcSpUpSpec>;
#[doc = "Register `REG_REGU_PMC_SP_UP` writer"]
pub type W = crate::W<RegReguPmcSpUpSpec>;
#[doc = "Field `apmc_sp0` reader - Start Point for ST3/ST4 up (ST3 not support)"]
pub type ApmcSp0R = crate::FieldReader;
#[doc = "Field `apmc_sp0` writer - Start Point for ST3/ST4 up (ST3 not support)"]
pub type ApmcSp0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `apmc_sp1` reader - Start Point for ST5 up"]
pub type ApmcSp1R = crate::FieldReader;
#[doc = "Field `apmc_sp1` writer - Start Point for ST5 up"]
pub type ApmcSp1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `apmc_sp2` reader - Start Point for ST6 up"]
pub type ApmcSp2R = crate::FieldReader;
#[doc = "Field `apmc_sp2` writer - Start Point for ST6 up"]
pub type ApmcSp2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Start Point for ST3/ST4 up (ST3 not support)"]
    #[inline(always)]
    pub fn apmc_sp0(&self) -> ApmcSp0R {
        ApmcSp0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Start Point for ST5 up"]
    #[inline(always)]
    pub fn apmc_sp1(&self) -> ApmcSp1R {
        ApmcSp1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Start Point for ST6 up"]
    #[inline(always)]
    pub fn apmc_sp2(&self) -> ApmcSp2R {
        ApmcSp2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Start Point for ST3/ST4 up (ST3 not support)"]
    #[inline(always)]
    pub fn apmc_sp0(&mut self) -> ApmcSp0W<'_, RegReguPmcSpUpSpec> {
        ApmcSp0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Start Point for ST5 up"]
    #[inline(always)]
    pub fn apmc_sp1(&mut self) -> ApmcSp1W<'_, RegReguPmcSpUpSpec> {
        ApmcSp1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Start Point for ST6 up"]
    #[inline(always)]
    pub fn apmc_sp2(&mut self) -> ApmcSp2W<'_, RegReguPmcSpUpSpec> {
        ApmcSp2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_regu_pmc_sp_up::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_regu_pmc_sp_up::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegReguPmcSpUpSpec;
impl crate::RegisterSpec for RegReguPmcSpUpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_regu_pmc_sp_up::R`](R) reader structure"]
impl crate::Readable for RegReguPmcSpUpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_regu_pmc_sp_up::W`](W) writer structure"]
impl crate::Writable for RegReguPmcSpUpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REGU_PMC_SP_UP to value 0x003b_2f2d"]
impl crate::Resettable for RegReguPmcSpUpSpec {
    const RESET_VALUE: u32 = 0x003b_2f2d;
}
