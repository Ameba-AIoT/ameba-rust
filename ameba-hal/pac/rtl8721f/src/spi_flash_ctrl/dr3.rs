#[doc = "Register `DR3` reader"]
pub type R = crate::R<Dr3Spec>;
#[doc = "Register `DR3` writer"]
pub type W = crate::W<Dr3Spec>;
#[doc = "Field `DR3` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr3R = crate::FieldReader<u32>;
#[doc = "Field `DR3` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr3(&self) -> Dr3R {
        Dr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr3(&mut self) -> Dr3W<'_, Dr3Spec> {
        Dr3W::new(self, 0)
    }
}
#[doc = "Data Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr3Spec;
impl crate::RegisterSpec for Dr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for Dr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dr3::W`](W) writer structure"]
impl crate::Writable for Dr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for Dr3Spec {}
