#[doc = "Register `OPS_PSR_RPT` reader"]
pub type R = crate::R<OpsPsrRptSpec>;
#[doc = "Register `OPS_PSR_RPT` writer"]
pub type W = crate::W<OpsPsrRptSpec>;
#[doc = "Field `ops_duration_rpt` reader - ops frame duration field rpt"]
pub type OpsDurationRptR = crate::FieldReader;
#[doc = "Field `ops_duration_rpt` writer - ops frame duration field rpt"]
pub type OpsDurationRptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mbssid_n_mem` reader - multi-bssid n value"]
pub type MbssidNMemR = crate::FieldReader;
#[doc = "Field `mbssid_n_mem` writer - multi-bssid n value"]
pub type MbssidNMemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ops frame duration field rpt"]
    #[inline(always)]
    pub fn ops_duration_rpt(&self) -> OpsDurationRptR {
        OpsDurationRptR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - multi-bssid n value"]
    #[inline(always)]
    pub fn mbssid_n_mem(&self) -> MbssidNMemR {
        MbssidNMemR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ops frame duration field rpt"]
    #[inline(always)]
    pub fn ops_duration_rpt(&mut self) -> OpsDurationRptW<'_, OpsPsrRptSpec> {
        OpsDurationRptW::new(self, 0)
    }
    #[doc = "Bits 8:15 - multi-bssid n value"]
    #[inline(always)]
    pub fn mbssid_n_mem(&mut self) -> MbssidNMemW<'_, OpsPsrRptSpec> {
        MbssidNMemW::new(self, 8)
    }
}
#[doc = "OPS DURATION\n\nYou can [`read`](crate::Reg::read) this register and get [`ops_psr_rpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ops_psr_rpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpsPsrRptSpec;
impl crate::RegisterSpec for OpsPsrRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ops_psr_rpt::R`](R) reader structure"]
impl crate::Readable for OpsPsrRptSpec {}
#[doc = "`write(|w| ..)` method takes [`ops_psr_rpt::W`](W) writer structure"]
impl crate::Writable for OpsPsrRptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPS_PSR_RPT to value 0"]
impl crate::Resettable for OpsPsrRptSpec {}
