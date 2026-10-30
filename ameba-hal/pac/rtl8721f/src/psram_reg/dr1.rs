#[doc = "Register `DR1` reader"]
pub type R = crate::R<Dr1Spec>;
#[doc = "Register `DR1` writer"]
pub type W = crate::W<Dr1Spec>;
#[doc = "Field `DR1` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr1R = crate::FieldReader<u32>;
#[doc = "Field `DR1` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr1(&self) -> Dr1R {
        Dr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr1(&mut self) -> Dr1W<'_, Dr1Spec> {
        Dr1W::new(self, 0)
    }
}
#[doc = "Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr1Spec;
impl crate::RegisterSpec for Dr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr1::R`](R) reader structure"]
impl crate::Readable for Dr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dr1::W`](W) writer structure"]
impl crate::Writable for Dr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for Dr1Spec {}
