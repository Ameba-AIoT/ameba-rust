#[doc = "Register `IDAU2_BA4` reader"]
pub type R = crate::R<Idau2Ba4Spec>;
#[doc = "Register `IDAU2_BA4` writer"]
pub type W = crate::W<Idau2Ba4Spec>;
#[doc = "Field `IDAU2_BA4` reader - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
pub type Idau2Ba4R = crate::FieldReader<u16>;
#[doc = "Field `IDAU2_BA4` writer - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
pub type Idau2Ba4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_ba4(&self) -> Idau2Ba4R {
        Idau2Ba4R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_ba4(&mut self) -> Idau2Ba4W<'_, Idau2Ba4Spec> {
        Idau2Ba4W::new(self, 12)
    }
}
#[doc = "Base address register 4 in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_ba4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_ba4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2Ba4Spec;
impl crate::RegisterSpec for Idau2Ba4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_ba4::R`](R) reader structure"]
impl crate::Readable for Idau2Ba4Spec {}
#[doc = "`write(|w| ..)` method takes [`idau2_ba4::W`](W) writer structure"]
impl crate::Writable for Idau2Ba4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_BA4 to value 0"]
impl crate::Resettable for Idau2Ba4Spec {}
