#[doc = "Register `DR8` reader"]
pub type R = crate::R<Dr8Spec>;
#[doc = "Register `DR8` writer"]
pub type W = crate::W<Dr8Spec>;
#[doc = "Field `DR8` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr8R = crate::FieldReader<u32>;
#[doc = "Field `DR8` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr8(&self) -> Dr8R {
        Dr8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr8(&mut self) -> Dr8W<'_, Dr8Spec> {
        Dr8W::new(self, 0)
    }
}
#[doc = "Data Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr8Spec;
impl crate::RegisterSpec for Dr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::Readable for Dr8Spec {}
#[doc = "`write(|w| ..)` method takes [`dr8::W`](W) writer structure"]
impl crate::Writable for Dr8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR8 to value 0"]
impl crate::Resettable for Dr8Spec {}
