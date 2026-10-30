#[doc = "Register `REG_PMC_ADDR01` reader"]
pub type R = crate::R<RegPmcAddr01Spec>;
#[doc = "Register `REG_PMC_ADDR01` writer"]
pub type W = crate::W<RegPmcAddr01Spec>;
#[doc = "Field `pmc_patch_addr0` reader - HW used only , these are for pmc patch Note: these addrs are shared by aon_pmc and regu_pmc"]
pub type PmcPatchAddr0R = crate::FieldReader<u16>;
#[doc = "Field `pmc_patch_addr0` writer - HW used only , these are for pmc patch Note: these addrs are shared by aon_pmc and regu_pmc"]
pub type PmcPatchAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `pmc_patch_addr1` reader - HW used only , these are for pmc patch"]
pub type PmcPatchAddr1R = crate::FieldReader<u16>;
#[doc = "Field `pmc_patch_addr1` writer - HW used only , these are for pmc patch"]
pub type PmcPatchAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HW used only , these are for pmc patch Note: these addrs are shared by aon_pmc and regu_pmc"]
    #[inline(always)]
    pub fn pmc_patch_addr0(&self) -> PmcPatchAddr0R {
        PmcPatchAddr0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr1(&self) -> PmcPatchAddr1R {
        PmcPatchAddr1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HW used only , these are for pmc patch Note: these addrs are shared by aon_pmc and regu_pmc"]
    #[inline(always)]
    pub fn pmc_patch_addr0(&mut self) -> PmcPatchAddr0W<'_, RegPmcAddr01Spec> {
        PmcPatchAddr0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr1(&mut self) -> PmcPatchAddr1W<'_, RegPmcAddr01Spec> {
        PmcPatchAddr1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_addr01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_addr01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcAddr01Spec;
impl crate::RegisterSpec for RegPmcAddr01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_addr01::R`](R) reader structure"]
impl crate::Readable for RegPmcAddr01Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_addr01::W`](W) writer structure"]
impl crate::Writable for RegPmcAddr01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_ADDR01 to value 0x00f0_00f0"]
impl crate::Resettable for RegPmcAddr01Spec {
    const RESET_VALUE: u32 = 0x00f0_00f0;
}
