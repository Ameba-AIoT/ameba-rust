#[doc = "Register `IDAU_BA5` reader"]
pub type R = crate::R<IdauBa5Spec>;
#[doc = "Register `IDAU_BA5` writer"]
pub type W = crate::W<IdauBa5Spec>;
#[doc = "Field `IDAU_BA5` reader - It's the base address of region 5 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 5 \\[31:28\\]: reserved"]
pub type IdauBa5R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA5` writer - It's the base address of region 5 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 5 \\[31:28\\]: reserved"]
pub type IdauBa5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 5 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 5 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba5(&self) -> IdauBa5R {
        IdauBa5R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 5 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 5 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba5(&mut self) -> IdauBa5W<'_, IdauBa5Spec> {
        IdauBa5W::new(self, 12)
    }
}
#[doc = "Base address register 5 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa5Spec;
impl crate::RegisterSpec for IdauBa5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba5::R`](R) reader structure"]
impl crate::Readable for IdauBa5Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba5::W`](W) writer structure"]
impl crate::Writable for IdauBa5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA5 to value 0"]
impl crate::Resettable for IdauBa5Spec {}
