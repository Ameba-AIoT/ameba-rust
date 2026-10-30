#[doc = "Register `DUM_BYTE` reader"]
pub type R = crate::R<DumByteSpec>;
#[doc = "Register `DUM_BYTE` writer"]
pub type W = crate::W<DumByteSpec>;
#[doc = "Field `DUM_BYTE_VAL` reader - Push dummy byte value."]
pub type DumByteValR = crate::FieldReader;
#[doc = "Field `DUM_BYTE_VAL` writer - Push dummy byte value."]
pub type DumByteValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Push dummy byte value."]
    #[inline(always)]
    pub fn dum_byte_val(&self) -> DumByteValR {
        DumByteValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Push dummy byte value."]
    #[inline(always)]
    pub fn dum_byte_val(&mut self) -> DumByteValW<'_, DumByteSpec> {
        DumByteValW::new(self, 0)
    }
}
#[doc = "Dummy byte value\n\nYou can [`read`](crate::Reg::read) this register and get [`dum_byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dum_byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DumByteSpec;
impl crate::RegisterSpec for DumByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dum_byte::R`](R) reader structure"]
impl crate::Readable for DumByteSpec {}
#[doc = "`write(|w| ..)` method takes [`dum_byte::W`](W) writer structure"]
impl crate::Writable for DumByteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUM_BYTE to value 0"]
impl crate::Resettable for DumByteSpec {}
