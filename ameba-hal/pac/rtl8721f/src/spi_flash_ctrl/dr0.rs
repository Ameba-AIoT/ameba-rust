#[doc = "Register `DR0` reader"]
pub type R = crate::R<Dr0Spec>;
#[doc = "Register `DR0` writer"]
pub type W = crate::W<Dr0Spec>;
#[doc = "Field `DR0` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr0R = crate::FieldReader<u32>;
#[doc = "Field `DR0` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr0(&self) -> Dr0R {
        Dr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr0(&mut self) -> Dr0W<'_, Dr0Spec> {
        Dr0W::new(self, 0)
    }
}
#[doc = "Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr0Spec;
impl crate::RegisterSpec for Dr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for Dr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dr0::W`](W) writer structure"]
impl crate::Writable for Dr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for Dr0Spec {}
