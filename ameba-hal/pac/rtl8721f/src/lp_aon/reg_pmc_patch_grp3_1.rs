#[doc = "Register `REG_PMC_PATCH_GRP3_1` reader"]
pub type R = crate::R<RegPmcPatchGrp3_1Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP3_1` writer"]
pub type W = crate::W<RegPmcPatchGrp3_1Spec>;
#[doc = "Field `SEL_PMC_G3` reader - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG3R = crate::BitReader;
#[doc = "Field `SEL_PMC_G3` writer - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_G3` reader - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG3R = crate::FieldReader<u16>;
#[doc = "Field `SADDR_G3` writer - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INSTR0_G3` reader - Instruction0 of AON PMC patch group3"]
pub type Instr0G3R = crate::FieldReader<u32>;
#[doc = "Field `INSTR0_G3` writer - Instruction0 of AON PMC patch group3"]
pub type Instr0G3W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `INSTR1_G3_3_0` reader - Instruction1 of AON PMC patch group3"]
pub type Instr1G3_3_0R = crate::FieldReader;
#[doc = "Field `INSTR1_G3_3_0` writer - Instruction1 of AON PMC patch group3"]
pub type Instr1G3_3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g3(&self) -> SelPmcG3R {
        SelPmcG3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g3(&self) -> SaddrG3R {
        SaddrG3R::new(((self.bits >> 1) & 0x01ff) as u16)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group3"]
    #[inline(always)]
    pub fn instr0_g3(&self) -> Instr0G3R {
        Instr0G3R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group3"]
    #[inline(always)]
    pub fn instr1_g3_3_0(&self) -> Instr1G3_3_0R {
        Instr1G3_3_0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g3(&mut self) -> SelPmcG3W<'_, RegPmcPatchGrp3_1Spec> {
        SelPmcG3W::new(self, 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g3(&mut self) -> SaddrG3W<'_, RegPmcPatchGrp3_1Spec> {
        SaddrG3W::new(self, 1)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group3"]
    #[inline(always)]
    pub fn instr0_g3(&mut self) -> Instr0G3W<'_, RegPmcPatchGrp3_1Spec> {
        Instr0G3W::new(self, 10)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group3"]
    #[inline(always)]
    pub fn instr1_g3_3_0(&mut self) -> Instr1G3_3_0W<'_, RegPmcPatchGrp3_1Spec> {
        Instr1G3_3_0W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp3_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp3_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp3_1Spec;
impl crate::RegisterSpec for RegPmcPatchGrp3_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp3_1::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp3_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp3_1::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp3_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP3_1 to value 0"]
impl crate::Resettable for RegPmcPatchGrp3_1Spec {}
