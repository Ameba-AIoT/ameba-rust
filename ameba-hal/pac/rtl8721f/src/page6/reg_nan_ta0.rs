#[doc = "Register `REG_NAN_TA0` reader"]
pub type R = crate::R<RegNanTa0Spec>;
#[doc = "Register `REG_NAN_TA0` writer"]
pub type W = crate::W<RegNanTa0Spec>;
#[doc = "Field `BIT_WMAC_NAN_TA_L` reader - Low 32bit tx address for NAN"]
pub type BitWmacNanTaLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_WMAC_NAN_TA_L` writer - Low 32bit tx address for NAN"]
pub type BitWmacNanTaLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32bit tx address for NAN"]
    #[inline(always)]
    pub fn bit_wmac_nan_ta_l(&self) -> BitWmacNanTaLR {
        BitWmacNanTaLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32bit tx address for NAN"]
    #[inline(always)]
    pub fn bit_wmac_nan_ta_l(&mut self) -> BitWmacNanTaLW<'_, RegNanTa0Spec> {
        BitWmacNanTaLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_ta0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_ta0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanTa0Spec;
impl crate::RegisterSpec for RegNanTa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_ta0::R`](R) reader structure"]
impl crate::Readable for RegNanTa0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_ta0::W`](W) writer structure"]
impl crate::Writable for RegNanTa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_TA0 to value 0"]
impl crate::Resettable for RegNanTa0Spec {}
