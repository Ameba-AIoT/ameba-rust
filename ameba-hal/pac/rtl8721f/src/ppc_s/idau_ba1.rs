#[doc = "Register `IDAU_BA1` reader"]
pub type R = crate::R<IdauBa1Spec>;
#[doc = "Register `IDAU_BA1` writer"]
pub type W = crate::W<IdauBa1Spec>;
#[doc = "Field `IDAU_BA1` reader - It's the base address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 1 \\[31:28\\]: reserved"]
pub type IdauBa1R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA1` writer - It's the base address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 1 \\[31:28\\]: reserved"]
pub type IdauBa1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 1 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba1(&self) -> IdauBa1R {
        IdauBa1R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 1 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba1(&mut self) -> IdauBa1W<'_, IdauBa1Spec> {
        IdauBa1W::new(self, 12)
    }
}
#[doc = "Base address register 1 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa1Spec;
impl crate::RegisterSpec for IdauBa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba1::R`](R) reader structure"]
impl crate::Readable for IdauBa1Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba1::W`](W) writer structure"]
impl crate::Writable for IdauBa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA1 to value 0"]
impl crate::Resettable for IdauBa1Spec {}
