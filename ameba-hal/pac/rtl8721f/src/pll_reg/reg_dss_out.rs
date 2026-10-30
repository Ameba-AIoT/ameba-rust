#[doc = "Register `REG_DSS_OUT` reader"]
pub type R = crate::R<RegDssOutSpec>;
#[doc = "Register `REG_DSS_OUT` writer"]
pub type W = crate::W<RegDssOutSpec>;
#[doc = "Field `dss_data_out` reader - "]
pub type DssDataOutR = crate::FieldReader<u32>;
#[doc = "Field `dss_ready` reader - "]
pub type DssReadyR = crate::BitReader;
#[doc = "Field `dss_wsort_go` reader - "]
pub type DssWsortGoR = crate::BitReader;
#[doc = "Field `dss_out_sel` reader - 0: select lvt dss 1: select uhvt dss"]
pub type DssOutSelR = crate::BitReader;
#[doc = "Field `dss_out_sel` writer - 0: select lvt dss 1: select uhvt dss"]
pub type DssOutSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dss_data_out(&self) -> DssDataOutR {
        DssDataOutR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dss_ready(&self) -> DssReadyR {
        DssReadyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dss_wsort_go(&self) -> DssWsortGoR {
        DssWsortGoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: select lvt dss 1: select uhvt dss"]
    #[inline(always)]
    pub fn dss_out_sel(&self) -> DssOutSelR {
        DssOutSelR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - 0: select lvt dss 1: select uhvt dss"]
    #[inline(always)]
    pub fn dss_out_sel(&mut self) -> DssOutSelW<'_, RegDssOutSpec> {
        DssOutSelW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dss_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dss_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDssOutSpec;
impl crate::RegisterSpec for RegDssOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dss_out::R`](R) reader structure"]
impl crate::Readable for RegDssOutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dss_out::W`](W) writer structure"]
impl crate::Writable for RegDssOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DSS_OUT to value 0"]
impl crate::Resettable for RegDssOutSpec {}
