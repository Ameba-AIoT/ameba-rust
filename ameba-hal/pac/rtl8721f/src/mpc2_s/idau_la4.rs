#[doc = "Register `IDAU_LA4` reader"]
pub type R = crate::R<IdauLa4Spec>;
#[doc = "Register `IDAU_LA4` writer"]
pub type W = crate::W<IdauLa4Spec>;
#[doc = "Field `IDAU_LA4` reader - It's the top address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 4 \\[31:28\\]: reserved"]
pub type IdauLa4R = crate::FieldReader<u16>;
#[doc = "Field `IDAU_LA4` writer - It's the top address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 4 \\[31:28\\]: reserved"]
pub type IdauLa4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the top address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_la4(&self) -> IdauLa4R {
        IdauLa4R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the top address of region 4 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 4 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau_la4(&mut self) -> IdauLa4W<'_, IdauLa4Spec> {
        IdauLa4W::new(self, 12)
    }
}
#[doc = "Top address register 4 in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_la4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_la4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauLa4Spec;
impl crate::RegisterSpec for IdauLa4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_la4::R`](R) reader structure"]
impl crate::Readable for IdauLa4Spec {}
#[doc = "`write(|w| ..)` method takes [`idau_la4::W`](W) writer structure"]
impl crate::Writable for IdauLa4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_LA4 to value 0x0fff_f000"]
impl crate::Resettable for IdauLa4Spec {
    const RESET_VALUE: u32 = 0x0fff_f000;
}
