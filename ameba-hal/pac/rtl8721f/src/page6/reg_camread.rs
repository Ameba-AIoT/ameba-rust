#[doc = "Register `REG_CAMREAD` reader"]
pub type R = crate::R<RegCamreadSpec>;
#[doc = "Register `REG_CAMREAD` writer"]
pub type W = crate::W<RegCamreadSpec>;
#[doc = "Field `BIT_CAMR_DATA` reader - Security CAM Read Content"]
pub type BitCamrDataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_CAMR_DATA` writer - Security CAM Read Content"]
pub type BitCamrDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Security CAM Read Content"]
    #[inline(always)]
    pub fn bit_camr_data(&self) -> BitCamrDataR {
        BitCamrDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Security CAM Read Content"]
    #[inline(always)]
    pub fn bit_camr_data(&mut self) -> BitCamrDataW<'_, RegCamreadSpec> {
        BitCamrDataW::new(self, 0)
    }
}
#[doc = "CAM Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_camread::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_camread::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamreadSpec;
impl crate::RegisterSpec for RegCamreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_camread::R`](R) reader structure"]
impl crate::Readable for RegCamreadSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_camread::W`](W) writer structure"]
impl crate::Writable for RegCamreadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAMREAD to value 0"]
impl crate::Resettable for RegCamreadSpec {}
