#[doc = "Register `REG_PMC_ADDR67` reader"]
pub type R = crate::R<RegPmcAddr67Spec>;
#[doc = "Register `REG_PMC_ADDR67` writer"]
pub type W = crate::W<RegPmcAddr67Spec>;
#[doc = "Field `pmc_patch_addr2` reader - HW used only , these are for pmc patch"]
pub type PmcPatchAddr2R = crate::FieldReader<u16>;
#[doc = "Field `pmc_patch_addr2` writer - HW used only , these are for pmc patch"]
pub type PmcPatchAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `pmc_patch_addr3` reader - HW used only , these are for pmc patch"]
pub type PmcPatchAddr3R = crate::FieldReader<u16>;
#[doc = "Field `pmc_patch_addr3` writer - HW used only , these are for pmc patch"]
pub type PmcPatchAddr3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr2(&self) -> PmcPatchAddr2R {
        PmcPatchAddr2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr3(&self) -> PmcPatchAddr3R {
        PmcPatchAddr3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr2(&mut self) -> PmcPatchAddr2W<'_, RegPmcAddr67Spec> {
        PmcPatchAddr2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - HW used only , these are for pmc patch"]
    #[inline(always)]
    pub fn pmc_patch_addr3(&mut self) -> PmcPatchAddr3W<'_, RegPmcAddr67Spec> {
        PmcPatchAddr3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_addr67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_addr67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcAddr67Spec;
impl crate::RegisterSpec for RegPmcAddr67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_addr67::R`](R) reader structure"]
impl crate::Readable for RegPmcAddr67Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_addr67::W`](W) writer structure"]
impl crate::Writable for RegPmcAddr67Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_ADDR67 to value 0x00f0_00f0"]
impl crate::Resettable for RegPmcAddr67Spec {
    const RESET_VALUE: u32 = 0x00f0_00f0;
}
