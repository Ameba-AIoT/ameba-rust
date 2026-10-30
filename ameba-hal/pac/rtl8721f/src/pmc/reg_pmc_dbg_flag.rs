#[doc = "Register `REG_PMC_DBG_FLAG` reader"]
pub type R = crate::R<RegPmcDbgFlagSpec>;
#[doc = "Register `REG_PMC_DBG_FLAG` writer"]
pub type W = crate::W<RegPmcDbgFlagSpec>;
#[doc = "Field `lpon_pmc_flag_pc` reader - PMC will raise a debug flag if hw pmc pc value hit this value"]
pub type LponPmcFlagPcR = crate::FieldReader;
#[doc = "Field `lpon_pmc_flag_pc` writer - PMC will raise a debug flag if hw pmc pc value hit this value"]
pub type LponPmcFlagPcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PMC will raise a debug flag if hw pmc pc value hit this value"]
    #[inline(always)]
    pub fn lpon_pmc_flag_pc(&self) -> LponPmcFlagPcR {
        LponPmcFlagPcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PMC will raise a debug flag if hw pmc pc value hit this value"]
    #[inline(always)]
    pub fn lpon_pmc_flag_pc(&mut self) -> LponPmcFlagPcW<'_, RegPmcDbgFlagSpec> {
        LponPmcFlagPcW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_dbg_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_dbg_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcDbgFlagSpec;
impl crate::RegisterSpec for RegPmcDbgFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_dbg_flag::R`](R) reader structure"]
impl crate::Readable for RegPmcDbgFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_dbg_flag::W`](W) writer structure"]
impl crate::Writable for RegPmcDbgFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_DBG_FLAG to value 0"]
impl crate::Resettable for RegPmcDbgFlagSpec {}
