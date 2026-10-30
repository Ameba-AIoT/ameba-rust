#[doc = "Register `REG_PMC_DBG_CTRL2_V1` reader"]
pub type R = crate::R<RegPmcDbgCtrl2V1Spec>;
#[doc = "Register `REG_PMC_DBG_CTRL2_V1` writer"]
pub type W = crate::W<RegPmcDbgCtrl2V1Spec>;
#[doc = "Field `BIT_SYSON_REG_ARB` reader - HW PMC Access Register Time Out Control(no use)"]
pub type BitSysonRegArbR = crate::FieldReader;
#[doc = "Field `BIT_SYSON_REG_ARB` writer - HW PMC Access Register Time Out Control(no use)"]
pub type BitSysonRegArbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - HW PMC Access Register Time Out Control(no use)"]
    #[inline(always)]
    pub fn bit_syson_reg_arb(&self) -> BitSysonRegArbR {
        BitSysonRegArbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - HW PMC Access Register Time Out Control(no use)"]
    #[inline(always)]
    pub fn bit_syson_reg_arb(&mut self) -> BitSysonRegArbW<'_, RegPmcDbgCtrl2V1Spec> {
        BitSysonRegArbW::new(self, 0)
    }
    #[doc = "Bits 2:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegPmcDbgCtrl2V1Spec> {
        RsvdW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_dbg_ctrl2_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_dbg_ctrl2_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcDbgCtrl2V1Spec;
impl crate::RegisterSpec for RegPmcDbgCtrl2V1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_dbg_ctrl2_v1::R`](R) reader structure"]
impl crate::Readable for RegPmcDbgCtrl2V1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_dbg_ctrl2_v1::W`](W) writer structure"]
impl crate::Writable for RegPmcDbgCtrl2V1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_DBG_CTRL2_V1 to value 0"]
impl crate::Resettable for RegPmcDbgCtrl2V1Spec {}
