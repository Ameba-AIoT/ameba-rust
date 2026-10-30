#[doc = "Register `REG_ISO` reader"]
pub type R = crate::R<RegIsoSpec>;
#[doc = "Register `REG_ISO` writer"]
pub type W = crate::W<RegIsoSpec>;
#[doc = "Field `iso_soc_req` reader - isolate signals from PD_SOC to other power domain. It is arb result"]
pub type IsoSocReqR = crate::BitReader;
#[doc = "Field `iso_bt` reader - 1: isolate signals from PD_BT to other power domain 0: release iso"]
pub type IsoBtR = crate::BitReader;
#[doc = "Field `iso_bt` writer - 1: isolate signals from PD_BT to other power domain 0: release iso"]
pub type IsoBtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_shrm` reader - 1: isolate signals from PD_SHRM to other power domain 0: release iso"]
pub type IsoShrmR = crate::BitReader;
#[doc = "Field `iso_shrm` writer - 1: isolate signals from PD_SHRM to other power domain 0: release iso"]
pub type IsoShrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - isolate signals from PD_SOC to other power domain. It is arb result"]
    #[inline(always)]
    pub fn iso_soc_req(&self) -> IsoSocReqR {
        IsoSocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: isolate signals from PD_BT to other power domain 0: release iso"]
    #[inline(always)]
    pub fn iso_bt(&self) -> IsoBtR {
        IsoBtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: isolate signals from PD_SHRM to other power domain 0: release iso"]
    #[inline(always)]
    pub fn iso_shrm(&self) -> IsoShrmR {
        IsoShrmR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1: isolate signals from PD_BT to other power domain 0: release iso"]
    #[inline(always)]
    pub fn iso_bt(&mut self) -> IsoBtW<'_, RegIsoSpec> {
        IsoBtW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: isolate signals from PD_SHRM to other power domain 0: release iso"]
    #[inline(always)]
    pub fn iso_shrm(&mut self) -> IsoShrmW<'_, RegIsoSpec> {
        IsoShrmW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_iso::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_iso::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIsoSpec;
impl crate::RegisterSpec for RegIsoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_iso::R`](R) reader structure"]
impl crate::Readable for RegIsoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_iso::W`](W) writer structure"]
impl crate::Writable for RegIsoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ISO to value 0x06"]
impl crate::Resettable for RegIsoSpec {
    const RESET_VALUE: u16 = 0x06;
}
