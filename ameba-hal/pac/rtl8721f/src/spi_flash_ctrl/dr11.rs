#[doc = "Register `DR11` reader"]
pub type R = crate::R<Dr11Spec>;
#[doc = "Register `DR11` writer"]
pub type W = crate::W<Dr11Spec>;
#[doc = "Field `DR11` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr11R = crate::FieldReader<u32>;
#[doc = "Field `DR11` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr11(&self) -> Dr11R {
        Dr11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr11(&mut self) -> Dr11W<'_, Dr11Spec> {
        Dr11W::new(self, 0)
    }
}
#[doc = "Data Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr11Spec;
impl crate::RegisterSpec for Dr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr11::R`](R) reader structure"]
impl crate::Readable for Dr11Spec {}
#[doc = "`write(|w| ..)` method takes [`dr11::W`](W) writer structure"]
impl crate::Writable for Dr11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR11 to value 0"]
impl crate::Resettable for Dr11Spec {}
