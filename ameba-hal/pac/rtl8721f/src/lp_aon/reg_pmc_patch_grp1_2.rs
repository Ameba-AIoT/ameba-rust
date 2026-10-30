#[doc = "Register `REG_PMC_PATCH_GRP1_2` reader"]
pub type R = crate::R<RegPmcPatchGrp1_2Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP1_2` writer"]
pub type W = crate::W<RegPmcPatchGrp1_2Spec>;
#[doc = "Field `INSTR1_G1_17_4` reader - Instruction1 of AON PMC patch group1"]
pub type Instr1G1_17_4R = crate::FieldReader<u16>;
#[doc = "Field `INSTR1_G1_17_4` writer - Instruction1 of AON PMC patch group1"]
pub type Instr1G1_17_4W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `INSTR2_G1_17_0` reader - Instruction2 of AON PMC patch group1"]
pub type Instr2G1_17_0R = crate::FieldReader<u32>;
#[doc = "Field `INSTR2_G1_17_0` writer - Instruction2 of AON PMC patch group1"]
pub type Instr2G1_17_0W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:13 - Instruction1 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr1_g1_17_4(&self) -> Instr1G1_17_4R {
        Instr1G1_17_4R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Instruction2 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr2_g1_17_0(&self) -> Instr2G1_17_0R {
        Instr2G1_17_0R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Instruction1 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr1_g1_17_4(&mut self) -> Instr1G1_17_4W<'_, RegPmcPatchGrp1_2Spec> {
        Instr1G1_17_4W::new(self, 0)
    }
    #[doc = "Bits 14:31 - Instruction2 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr2_g1_17_0(&mut self) -> Instr2G1_17_0W<'_, RegPmcPatchGrp1_2Spec> {
        Instr2G1_17_0W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp1_2Spec;
impl crate::RegisterSpec for RegPmcPatchGrp1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp1_2::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp1_2::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP1_2 to value 0"]
impl crate::Resettable for RegPmcPatchGrp1_2Spec {}
