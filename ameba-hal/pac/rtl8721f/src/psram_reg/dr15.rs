#[doc = "Register `DR15` reader"]
pub type R = crate::R<Dr15Spec>;
#[doc = "Register `DR15` writer"]
pub type W = crate::W<Dr15Spec>;
#[doc = "Field `DR15` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr15R = crate::FieldReader<u32>;
#[doc = "Field `DR15` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr15(&self) -> Dr15R {
        Dr15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr15(&mut self) -> Dr15W<'_, Dr15Spec> {
        Dr15W::new(self, 0)
    }
}
#[doc = "Data Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr15Spec;
impl crate::RegisterSpec for Dr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr15::R`](R) reader structure"]
impl crate::Readable for Dr15Spec {}
#[doc = "`write(|w| ..)` method takes [`dr15::W`](W) writer structure"]
impl crate::Writable for Dr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR15 to value 0"]
impl crate::Resettable for Dr15Spec {}
