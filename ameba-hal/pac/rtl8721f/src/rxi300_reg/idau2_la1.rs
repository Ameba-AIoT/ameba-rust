#[doc = "Register `IDAU2_LA1` reader"]
pub type R = crate::R<Idau2La1Spec>;
#[doc = "Register `IDAU2_LA1` writer"]
pub type W = crate::W<Idau2La1Spec>;
#[doc = "Field `IDAU2_LA1` reader - It's the top address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 1 \\[31:28\\]: reserved"]
pub type Idau2La1R = crate::FieldReader<u16>;
#[doc = "Field `IDAU2_LA1` writer - It's the top address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 1 \\[31:28\\]: reserved"]
pub type Idau2La1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the top address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 1 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_la1(&self) -> Idau2La1R {
        Idau2La1R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the top address of region 1 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: top address of region 1 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau2_la1(&mut self) -> Idau2La1W<'_, Idau2La1Spec> {
        Idau2La1W::new(self, 12)
    }
}
#[doc = "Top address register 1 in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_la1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_la1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2La1Spec;
impl crate::RegisterSpec for Idau2La1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_la1::R`](R) reader structure"]
impl crate::Readable for Idau2La1Spec {}
#[doc = "`write(|w| ..)` method takes [`idau2_la1::W`](W) writer structure"]
impl crate::Writable for Idau2La1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_LA1 to value 0x0fff_f000"]
impl crate::Resettable for Idau2La1Spec {
    const RESET_VALUE: u32 = 0x0fff_f000;
}
