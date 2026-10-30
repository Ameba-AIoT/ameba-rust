#[doc = "Register `IDAU_BA2` reader"]
pub type R = crate::R<IdauBa2Spec>;
#[doc = "Register `IDAU_BA2` writer"]
pub type W = crate::W<IdauBa2Spec>;
#[doc = "Field `IDAU_BA2` reader - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
pub type IdauBa2R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA2` writer - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
pub type IdauBa2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba2(&self) -> IdauBa2R {
        IdauBa2R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba2(&mut self) -> IdauBa2W<'_, IdauBa2Spec> {
        IdauBa2W::new(self, 12)
    }
}
#[doc = "Base address register 2 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa2Spec;
impl crate::RegisterSpec for IdauBa2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba2::R`](R) reader structure"]
impl crate::Readable for IdauBa2Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba2::W`](W) writer structure"]
impl crate::Writable for IdauBa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA2 to value 0"]
impl crate::Resettable for IdauBa2Spec {}
