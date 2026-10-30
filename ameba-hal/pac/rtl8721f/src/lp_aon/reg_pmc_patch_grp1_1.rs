#[doc = "Register `REG_PMC_PATCH_GRP1_1` reader"]
pub type R = crate::R<RegPmcPatchGrp1_1Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP1_1` writer"]
pub type W = crate::W<RegPmcPatchGrp1_1Spec>;
#[doc = "Field `SEL_PMC_G1` reader - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG1R = crate::BitReader;
#[doc = "Field `SEL_PMC_G1` writer - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_G1` reader - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG1R = crate::FieldReader<u16>;
#[doc = "Field `SADDR_G1` writer - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INSTR0_G1` reader - Instruction0 of AON PMC patch group1"]
pub type Instr0G1R = crate::FieldReader<u32>;
#[doc = "Field `INSTR0_G1` writer - Instruction0 of AON PMC patch group1"]
pub type Instr0G1W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `INSTR1_G1_3_0` reader - Instruction1 of AON PMC patch group1"]
pub type Instr1G1_3_0R = crate::FieldReader;
#[doc = "Field `INSTR1_G1_3_0` writer - Instruction1 of AON PMC patch group1"]
pub type Instr1G1_3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g1(&self) -> SelPmcG1R {
        SelPmcG1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g1(&self) -> SaddrG1R {
        SaddrG1R::new(((self.bits >> 1) & 0x01ff) as u16)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr0_g1(&self) -> Instr0G1R {
        Instr0G1R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr1_g1_3_0(&self) -> Instr1G1_3_0R {
        Instr1G1_3_0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g1(&mut self) -> SelPmcG1W<'_, RegPmcPatchGrp1_1Spec> {
        SelPmcG1W::new(self, 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g1(&mut self) -> SaddrG1W<'_, RegPmcPatchGrp1_1Spec> {
        SaddrG1W::new(self, 1)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr0_g1(&mut self) -> Instr0G1W<'_, RegPmcPatchGrp1_1Spec> {
        Instr0G1W::new(self, 10)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group1"]
    #[inline(always)]
    pub fn instr1_g1_3_0(&mut self) -> Instr1G1_3_0W<'_, RegPmcPatchGrp1_1Spec> {
        Instr1G1_3_0W::new(self, 28)
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
