#[doc = "Register `IDAU_BA4` reader"]
pub type R = crate::R<IdauBa4Spec>;
#[doc = "Register `IDAU_BA4` writer"]
pub type W = crate::W<IdauBa4Spec>;
#[doc = "Field `IDAU_BA4` reader - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
pub type IdauBa4R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA4` writer - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
pub type IdauBa4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba4(&self) -> IdauBa4R {
        IdauBa4R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba4(&mut self) -> IdauBa4W<'_, IdauBa4Spec> {
        IdauBa4W::new(self, 12)
    }
}
#[doc = "Base address register 4 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa4Spec;
impl crate::RegisterSpec for IdauBa4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba4::R`](R) reader structure"]
impl crate::Readable for IdauBa4Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba4::W`](W) writer structure"]
impl crate::Writable for IdauBa4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA4 to value 0"]
impl crate::Resettable for IdauBa4Spec {}
