#[doc = "Register `REG_PMC_CTRL` reader"]
pub type R = crate::R<RegPmcCtrlSpec>;
#[doc = "Register `REG_PMC_CTRL` writer"]
pub type W = crate::W<RegPmcCtrlSpec>;
#[doc = "Field `PMC_STL_SWR` reader - wait time for SWR stable 0: 64*T (T = 10us) 1: 128*T 2: 256*T 3: 512*T"]
pub type PmcStlSwrR = crate::FieldReader;
#[doc = "Field `PMC_STL_SWR` writer - wait time for SWR stable 0: 64*T (T = 10us) 1: 128*T 2: 256*T 3: 512*T"]
pub type PmcStlSwrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PMC_STL_CLDO` reader - wait time for core LDO stable 0: 8*T (T = 10us) 1: 16*T 2: 32*T 3: 64*T"]
pub type PmcStlCldoR = crate::FieldReader;
#[doc = "Field `PMC_STL_CLDO` writer - wait time for core LDO stable 0: 8*T (T = 10us) 1: 16*T 2: 32*T 3: 64*T"]
pub type PmcStlCldoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - wait time for SWR stable 0: 64*T (T = 10us) 1: 128*T 2: 256*T 3: 512*T"]
    #[inline(always)]
    pub fn pmc_stl_swr(&self) -> PmcStlSwrR {
        PmcStlSwrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - wait time for core LDO stable 0: 8*T (T = 10us) 1: 16*T 2: 32*T 3: 64*T"]
    #[inline(always)]
    pub fn pmc_stl_cldo(&self) -> PmcStlCldoR {
        PmcStlCldoR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - wait time for SWR stable 0: 64*T (T = 10us) 1: 128*T 2: 256*T 3: 512*T"]
    #[inline(always)]
    pub fn pmc_stl_swr(&mut self) -> PmcStlSwrW<'_, RegPmcCtrlSpec> {
        PmcStlSwrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - wait time for core LDO stable 0: 8*T (T = 10us) 1: 16*T 2: 32*T 3: 64*T"]
    #[inline(always)]
    pub fn pmc_stl_cldo(&mut self) -> PmcStlCldoW<'_, RegPmcCtrlSpec> {
        PmcStlCldoW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcCtrlSpec;
impl crate::RegisterSpec for RegPmcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPmcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPmcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_CTRL to value 0x28"]
impl crate::Resettable for RegPmcCtrlSpec {
    const RESET_VALUE: u32 = 0x28;
}
