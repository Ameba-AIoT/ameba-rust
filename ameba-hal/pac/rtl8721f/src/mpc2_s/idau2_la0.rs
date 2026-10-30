#[doc = "Register `IDAU2_LA0` reader"]
pub type R = crate::R<Idau2La0Spec>;
#[doc = "Register `IDAU2_LA0` writer"]
pub type W = crate::W<Idau2La0Spec>;
#[doc = "Field `IDAU2_LA0` reader - It's the top address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 0 \\[31:28\\]: reserved"]
pub type Idau2La0R = crate::FieldReader<u16>;
#[doc = "Field `IDAU2_LA0` writer - It's the top address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 0 \\[31:28\\]: reserved"]
pub type Idau2La0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the top address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_la0(&self) -> Idau2La0R {
        Idau2La0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the top address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_la0(&mut self) -> Idau2La0W<'_, Idau2La0Spec> {
        Idau2La0W::new(self, 12)
    }
}
#[doc = "Top address register 0 in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_la0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_la0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2La0Spec;
impl crate::RegisterSpec for Idau2La0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_la0::R`](R) reader structure"]
impl crate::Readable for Idau2La0Spec {}
#[doc = "`write(|w| ..)` method takes [`idau2_la0::W`](W) writer structure"]
impl crate::Writable for Idau2La0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_LA0 to value 0x0fff_f000"]
impl crate::Resettable for Idau2La0Spec {
    const RESET_VALUE: u32 = 0x0fff_f000;
}
