#[doc = "Register `REG_PMC_PATCH_GRP0_1` reader"]
pub type R = crate::R<RegPmcPatchGrp0_1Spec>;
#[doc = "Register `REG_PMC_PATCH_GRP0_1` writer"]
pub type W = crate::W<RegPmcPatchGrp0_1Spec>;
#[doc = "Field `SEL_PMC_G0` reader - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG0R = crate::BitReader;
#[doc = "Field `SEL_PMC_G0` writer - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
pub type SelPmcG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_G0` reader - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG0R = crate::FieldReader<u16>;
#[doc = "Field `SADDR_G0` writer - Start address of the first instruction of AON PMC patch group0"]
pub type SaddrG0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INSTR0_G0` reader - Instruction0 of AON PMC patch group0"]
pub type Instr0G0R = crate::FieldReader<u32>;
#[doc = "Field `INSTR0_G0` writer - Instruction0 of AON PMC patch group0"]
pub type Instr0G0W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `INSTR1_G0_3_0` reader - Instruction1 of AON PMC patch group0"]
pub type Instr1G0_3_0R = crate::FieldReader;
#[doc = "Field `INSTR1_G0_3_0` writer - Instruction1 of AON PMC patch group0"]
pub type Instr1G0_3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g0(&self) -> SelPmcG0R {
        SelPmcG0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g0(&self) -> SaddrG0R {
        SaddrG0R::new(((self.bits >> 1) & 0x01ff) as u16)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group0"]
    #[inline(always)]
    pub fn instr0_g0(&self) -> Instr0G0R {
        Instr0G0R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group0"]
    #[inline(always)]
    pub fn instr1_g0_3_0(&self) -> Instr1G0_3_0R {
        Instr1G0_3_0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select PMC for this patch group 0:aon pmc 1:regu pmc"]
    #[inline(always)]
    pub fn sel_pmc_g0(&mut self) -> SelPmcG0W<'_, RegPmcPatchGrp0_1Spec> {
        SelPmcG0W::new(self, 0)
    }
    #[doc = "Bits 1:9 - Start address of the first instruction of AON PMC patch group0"]
    #[inline(always)]
    pub fn saddr_g0(&mut self) -> SaddrG0W<'_, RegPmcPatchGrp0_1Spec> {
        SaddrG0W::new(self, 1)
    }
    #[doc = "Bits 10:27 - Instruction0 of AON PMC patch group0"]
    #[inline(always)]
    pub fn instr0_g0(&mut self) -> Instr0G0W<'_, RegPmcPatchGrp0_1Spec> {
        Instr0G0W::new(self, 10)
    }
    #[doc = "Bits 28:31 - Instruction1 of AON PMC patch group0"]
    #[inline(always)]
    pub fn instr1_g0_3_0(&mut self) -> Instr1G0_3_0W<'_, RegPmcPatchGrp0_1Spec> {
        Instr1G0_3_0W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_patch_grp0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_patch_grp0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcPatchGrp0_1Spec;
impl crate::RegisterSpec for RegPmcPatchGrp0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_patch_grp0_1::R`](R) reader structure"]
impl crate::Readable for RegPmcPatchGrp0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_patch_grp0_1::W`](W) writer structure"]
impl crate::Writable for RegPmcPatchGrp0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_PATCH_GRP0_1 to value 0"]
impl crate::Resettable for RegPmcPatchGrp0_1Spec {}
