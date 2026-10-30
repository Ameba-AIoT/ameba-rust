#[doc = "Register `IDAU1_BA0` reader"]
pub type R = crate::R<Idau1Ba0Spec>;
#[doc = "Register `IDAU1_BA0` writer"]
pub type W = crate::W<Idau1Ba0Spec>;
#[doc = "Field `IDAU1_BA0` reader - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
pub type Idau1Ba0R = crate::FieldReader<u16>;
#[doc = "Field `IDAU1_BA0` writer - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
pub type Idau1Ba0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau1_ba0(&self) -> Idau1Ba0R {
        Idau1Ba0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - It's the base address of region 0 which can be set as non-secure. \\[11:0\\]: reserved \\[27:12\\]: base address of region 0 \\[31:28\\]: reserved"]
    #[inline(always)]
    pub fn idau1_ba0(&mut self) -> Idau1Ba0W<'_, Idau1Ba0Spec> {
        Idau1Ba0W::new(self, 12)
    }
}
#[doc = "Base address register 0 in IDAU1\n\nYou can [`read`](crate::Reg::read) this register and get [`idau1_ba0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau1_ba0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau1Ba0Spec;
impl crate::RegisterSpec for Idau1Ba0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau1_ba0::R`](R) reader structure"]
impl crate::Readable for Idau1Ba0Spec {}
#[doc = "`write(|w| ..)` method takes [`idau1_ba0::W`](W) writer structure"]
impl crate::Writable for Idau1Ba0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU1_BA0 to value 0"]
impl crate::Resettable for Idau1Ba0Spec {}
