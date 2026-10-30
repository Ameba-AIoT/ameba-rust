#[doc = "Register `IDAU2_BA2` reader"]
pub type R = crate::R<Idau2Ba2Spec>;
#[doc = "Register `IDAU2_BA2` writer"]
pub type W = crate::W<Idau2Ba2Spec>;
#[doc = "Field `IDAU2_BA2` reader - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
pub type Idau2Ba2R = crate::FieldReader<u16>;
#[doc = "Field `IDAU2_BA2` writer - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
pub type Idau2Ba2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_ba2(&self) -> Idau2Ba2R {
        Idau2Ba2R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 2 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 2 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_ba2(&mut self) -> Idau2Ba2W<'_, Idau2Ba2Spec> {
        Idau2Ba2W::new(self, 12)
    }
}
#[doc = "Base address register 2 in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_ba2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_ba2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2Ba2Spec;
impl crate::RegisterSpec for Idau2Ba2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_ba2::R`](R) reader structure"]
impl crate::Readable for Idau2Ba2Spec {}
#[doc = "`write(|w| ..)` method takes [`idau2_ba2::W`](W) writer structure"]
impl crate::Writable for Idau2Ba2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_BA2 to value 0"]
impl crate::Resettable for Idau2Ba2Spec {}
