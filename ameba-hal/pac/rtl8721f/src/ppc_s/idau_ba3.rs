#[doc = "Register `IDAU_BA3` reader"]
pub type R = crate::R<IdauBa3Spec>;
#[doc = "Register `IDAU_BA3` writer"]
pub type W = crate::W<IdauBa3Spec>;
#[doc = "Field `IDAU_BA3` reader - It's the base address of region 3 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 3 \\[31:28\\]: reserved"]
pub type IdauBa3R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA3` writer - It's the base address of region 3 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 3 \\[31:28\\]: reserved"]
pub type IdauBa3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 3 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 3 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba3(&self) -> IdauBa3R {
        IdauBa3R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 3 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 3 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba3(&mut self) -> IdauBa3W<'_, IdauBa3Spec> {
        IdauBa3W::new(self, 12)
    }
}
#[doc = "Base address register 3 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa3Spec;
impl crate::RegisterSpec for IdauBa3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba3::R`](R) reader structure"]
impl crate::Readable for IdauBa3Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba3::W`](W) writer structure"]
impl crate::Writable for IdauBa3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA3 to value 0"]
impl crate::Resettable for IdauBa3Spec {}
