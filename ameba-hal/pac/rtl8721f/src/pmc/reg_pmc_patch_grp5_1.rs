#[doc = "Register `REG_PMC_PATCH_GRP5_1` reader"]
pub type R = crate::R<RegPmcPatchGrp5_1Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP5_1` writer"]
pub type W = crate::W<RegPmcPatchGrp5_1Spec>;
#[doc = "Field `PATCH_GRP5_INSTR1_17_6` reader - instruction1 of LSYSON PMC patch group5,bit\\[17:6\\]"]
pub type PatchGrp5Instr1_17_6R = crate::FieldReader<u16>;
#[doc = "Field `PATCH_GRP5_INSTR1_17_6` writer - instruction1 of LSYSON PMC patch group5,bit\\[17:6\\]"]
pub type PatchGrp5Instr1_17_6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PATCH_GRP5_INSTR2` reader - instruction2 of LSYSON PMC patch group5"]
pub type PatchGrp5Instr2R = crate::FieldReader<u32>;
#[doc = "Field `PATCH_GRP5_INSTR2` writer - instruction2 of LSYSON PMC patch group5"]
pub type PatchGrp5Instr2W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:11 - instruction1 of LSYSON PMC patch group5,bit\\[17:6\\]"]
    #[inline(always)]
    pub fn patch_grp5_instr1_17_6(&self) -> PatchGrp5Instr1_17_6R {
        PatchGrp5Instr1_17_6R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:29 - instruction2 of LSYSON PMC patch group5"]
    #[inline(always)]
    pub fn patch_grp5_instr2(&self) -> PatchGrp5Instr2R {
        PatchGrp5Instr2R::new((self.bits >> 12) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - instruction1 of LSYSON PMC patch group5,bit\\[17:6\\]"]
    #[inline(always)]
    pub fn patch_grp5_instr1_17_6(&mut self) -> PatchGrp5Instr1_17_6W<'_, RegPmcPatchGrp5_1Spec> {
        PatchGrp5Instr1_17_6W::new(self, 0)
    }
    #[doc = "Bits 12:29 - instruction2 of LSYSON PMC patch group5"]
    #[inline(always)]
    pub fn patch_grp5_instr2(&mut self) -> PatchGrp5Instr2W<'_, RegPmcPatchGrp5_1Spec> {
        PatchGrp5Instr2W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp5_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp5_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp5_1Spec;
impl crate::RegisterSpec for RegPmcPatchGrp5_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp5_1::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp5_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp5_1::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp5_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP5_1 to value 0"]
impl crate::Resettable for RegPmcPatchGrp5_1Spec {}
