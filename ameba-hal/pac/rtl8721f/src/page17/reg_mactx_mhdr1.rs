#[doc = "Register `REG_MACTX_MHDR1` reader"]
pub type R = crate::R<RegMactxMhdr1Spec>;
#[doc = "Register `REG_MACTX_MHDR1` writer"]
pub type W = crate::W<RegMactxMhdr1Spec>;
#[doc = "Field `r_WMAC_NHDR_HTC` reader - HTC field of tb_ppdu, which contain UPH a-control(4)"]
pub type RWmacNhdrHtcR = crate::FieldReader<u32>;
#[doc = "Field `r_WMAC_NHDR_HTC` writer - HTC field of tb_ppdu, which contain UPH a-control(4)"]
pub type RWmacNhdrHtcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HTC field of tb_ppdu, which contain UPH a-control(4)"]
    #[inline(always)]
    pub fn r_wmac_nhdr_htc(&self) -> RWmacNhdrHtcR {
        RWmacNhdrHtcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HTC field of tb_ppdu, which contain UPH a-control(4)"]
    #[inline(always)]
    pub fn r_wmac_nhdr_htc(&mut self) -> RWmacNhdrHtcW<'_, RegMactxMhdr1Spec> {
        RWmacNhdrHtcW::new(self, 0)
    }
}
#[doc = "MAC header HTC\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mactx_mhdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mactx_mhdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMactxMhdr1Spec;
impl crate::RegisterSpec for RegMactxMhdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mactx_mhdr1::R`](R) reader structure"]
impl crate::Readable for RegMactxMhdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mactx_mhdr1::W`](W) writer structure"]
impl crate::Writable for RegMactxMhdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACTX_MHDR1 to value 0"]
impl crate::Resettable for RegMactxMhdr1Spec {}
