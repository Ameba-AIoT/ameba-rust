#[doc = "Register `REG_PMC_PATCH_GRP0_0` reader"]
pub type R = crate::R<RegPmcPatchGrp0_0Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP0_0` writer"]
pub type W = crate::W<RegPmcPatchGrp0_0Spec>;
#[doc = "Field `PATCH_GRP0_SADDR` reader - Start address of the first instruction of LSYSON PMC patch group0"]
pub type PatchGrp0SaddrR = crate::FieldReader;
#[doc = "Field `PATCH_GRP0_SADDR` writer - Start address of the first instruction of LSYSON PMC patch group0"]
pub type PatchGrp0SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PATCH_GRP0_INSTR0` reader - instruction0 of LSYSON PMC patch group0"]
pub type PatchGrp0Instr0R = crate::FieldReader<u32>;
#[doc = "Field `PATCH_GRP0_INSTR0` writer - instruction0 of LSYSON PMC patch group0"]
pub type PatchGrp0Instr0W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PATCH_GRP0_INSTR1_5_0` reader - instruction1 of LSYSON PMC patch group0,bit\\[5:0\\]"]
pub type PatchGrp0Instr1_5_0R = crate::FieldReader;
#[doc = "Field `PATCH_GRP0_INSTR1_5_0` writer - instruction1 of LSYSON PMC patch group0,bit\\[5:0\\]"]
pub type PatchGrp0Instr1_5_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Start address of the first instruction of LSYSON PMC patch group0"]
    #[inline(always)]
    pub fn patch_grp0_saddr(&self) -> PatchGrp0SaddrR {
        PatchGrp0SaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:25 - instruction0 of LSYSON PMC patch group0"]
    #[inline(always)]
    pub fn patch_grp0_instr0(&self) -> PatchGrp0Instr0R {
        PatchGrp0Instr0R::new((self.bits >> 8) & 0x0003_ffff)
    }
    #[doc = "Bits 26:31 - instruction1 of LSYSON PMC patch group0,bit\\[5:0\\]"]
    #[inline(always)]
    pub fn patch_grp0_instr1_5_0(&self) -> PatchGrp0Instr1_5_0R {
        PatchGrp0Instr1_5_0R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start address of the first instruction of LSYSON PMC patch group0"]
    #[inline(always)]
    pub fn patch_grp0_saddr(&mut self) -> PatchGrp0SaddrW<'_, RegPmcPatchGrp0_0Spec> {
        PatchGrp0SaddrW::new(self, 0)
    }
    #[doc = "Bits 8:25 - instruction0 of LSYSON PMC patch group0"]
    #[inline(always)]
    pub fn patch_grp0_instr0(&mut self) -> PatchGrp0Instr0W<'_, RegPmcPatchGrp0_0Spec> {
        PatchGrp0Instr0W::new(self, 8)
    }
    #[doc = "Bits 26:31 - instruction1 of LSYSON PMC patch group0,bit\\[5:0\\]"]
    #[inline(always)]
    pub fn patch_grp0_instr1_5_0(&mut self) -> PatchGrp0Instr1_5_0W<'_, RegPmcPatchGrp0_0Spec> {
        PatchGrp0Instr1_5_0W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp0_0Spec;
impl crate::RegisterSpec for RegPmcPatchGrp0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp0_0::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp0_0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp0_0::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp0_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP0_0 to value 0"]
impl crate::Resettable for RegPmcPatchGrp0_0Spec {}
