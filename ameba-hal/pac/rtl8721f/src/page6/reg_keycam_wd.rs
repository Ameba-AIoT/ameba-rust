#[doc = "Register `REG_KEYCAM_WD` reader"]
pub type R = crate::R<RegKeycamWdSpec>;
#[doc = "Register `REG_KEYCAM_WD` writer"]
pub type W = crate::W<RegKeycamWdSpec>;
#[doc = "Field `BIT_KEYCAM_WDATA` reader - Security CAM Write Content"]
pub type BitKeycamWdataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_KEYCAM_WDATA` writer - Security CAM Write Content"]
pub type BitKeycamWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Security CAM Write Content"]
    #[inline(always)]
    pub fn bit_keycam_wdata(&self) -> BitKeycamWdataR {
        BitKeycamWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Security CAM Write Content"]
    #[inline(always)]
    pub fn bit_keycam_wdata(&mut self) -> BitKeycamWdataW<'_, RegKeycamWdSpec> {
        BitKeycamWdataW::new(self, 0)
    }
}
#[doc = "Security CAM Content Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam_wd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam_wd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycamWdSpec;
impl crate::RegisterSpec for RegKeycamWdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam_wd::R`](R) reader structure"]
impl crate::Readable for RegKeycamWdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam_wd::W`](W) writer structure"]
impl crate::Writable for RegKeycamWdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM_WD to value 0"]
impl crate::Resettable for RegKeycamWdSpec {}
