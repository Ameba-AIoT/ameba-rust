#[doc = "Register `REG_PMC_PATCH_GRP1_1` reader"]
pub type R = crate::R<RegPmcPatchGrp1_1Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP1_1` writer"]
pub type W = crate::W<RegPmcPatchGrp1_1Spec>;
#[doc = "Field `PATCH_GRP1_INSTR1_17_6` reader - instruction1 of LSYSON PMC patch group1,bit\\[17:6\\]"]
pub type PatchGrp1Instr1_17_6R = crate::FieldReader<u16>;
#[doc = "Field `PATCH_GRP1_INSTR1_17_6` writer - instruction1 of LSYSON PMC patch group1,bit\\[17:6\\]"]
pub type PatchGrp1Instr1_17_6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PATCH_GRP1_INSTR2` reader - instruction2 of LSYSON PMC patch group1"]
pub type PatchGrp1Instr2R = crate::FieldReader<u32>;
#[doc = "Field `PATCH_GRP1_INSTR2` writer - instruction2 of LSYSON PMC patch group1"]
pub type PatchGrp1Instr2W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:11 - instruction1 of LSYSON PMC patch group1,bit\\[17:6\\]"]
    #[inline(always)]
    pub fn patch_grp1_instr1_17_6(&self) -> PatchGrp1Instr1_17_6R {
        PatchGrp1Instr1_17_6R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:29 - instruction2 of LSYSON PMC patch group1"]
    #[inline(always)]
    pub fn patch_grp1_instr2(&self) -> PatchGrp1Instr2R {
        PatchGrp1Instr2R::new((self.bits >> 12) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - instruction1 of LSYSON PMC patch group1,bit\\[17:6\\]"]
    #[inline(always)]
    pub fn patch_grp1_instr1_17_6(&mut self) -> PatchGrp1Instr1_17_6W<'_, RegPmcPatchGrp1_1Spec> {
        PatchGrp1Instr1_17_6W::new(self, 0)
    }
    #[doc = "Bits 12:29 - instruction2 of LSYSON PMC patch group1"]
    #[inline(always)]
    pub fn patch_grp1_instr2(&mut self) -> PatchGrp1Instr2W<'_, RegPmcPatchGrp1_1Spec> {
        PatchGrp1Instr2W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp1_1Spec;
impl crate::RegisterSpec for RegPmcPatchGrp1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp1_1::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp1_1::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP1_1 to value 0"]
impl crate::Resettable for RegPmcPatchGrp1_1Spec {}
