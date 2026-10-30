#[doc = "Register `IDAU_BA0` reader"]
pub type R = crate::R<IdauBa0Spec>;
#[doc = "Register `IDAU_BA0` writer"]
pub type W = crate::W<IdauBa0Spec>;
#[doc = "Field `IDAU_BA0` reader - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
pub type IdauBa0R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_BA0` writer - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
pub type IdauBa0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba0(&self) -> IdauBa0R {
        IdauBa0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_ba0(&mut self) -> IdauBa0W<'_, IdauBa0Spec> {
        IdauBa0W::new(self, 12)
    }
}
#[doc = "Base address register 0 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ba0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ba0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauBa0Spec;
impl crate::RegisterSpec for IdauBa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ba0::R`](R) reader structure"]
impl crate::Readable for IdauBa0Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_ba0::W`](W) writer structure"]
impl crate::Writable for IdauBa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_BA0 to value 0"]
impl crate::Resettable for IdauBa0Spec {}
