#[doc = "Register `IMR0_REG` reader"]
pub type R = crate::R<Imr0RegSpec>;
#[doc = "Register `IMR0_REG` writer"]
pub type W = crate::W<Imr0RegSpec>;
#[doc = "Field `IMR0_ROK1` reader - This bit is IMR.rok"]
pub type Imr0Rok1R = crate::BitReader;
#[doc = "Field `IMR0_ROK2` reader - "]
pub type Imr0Rok2R = crate::BitReader;
#[doc = "Field `IMR0_ROK2` writer - "]
pub type Imr0Rok2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_ROK3` reader - "]
pub type Imr0Rok3R = crate::BitReader;
#[doc = "Field `IMR0_ROK3` writer - "]
pub type Imr0Rok3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_ROK4` reader - "]
pub type Imr0Rok4R = crate::BitReader;
#[doc = "Field `IMR0_ROK4` writer - "]
pub type Imr0Rok4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_ROK5` reader - "]
pub type Imr0Rok5R = crate::BitReader;
#[doc = "Field `IMR0_ROK5` writer - "]
pub type Imr0Rok5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_ROK6` reader - "]
pub type Imr0Rok6R = crate::BitReader;
#[doc = "Field `IMR0_ROK6` writer - "]
pub type Imr0Rok6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TOK1` reader - This bit is IMR.tok"]
pub type Imr0Tok1R = crate::BitReader;
#[doc = "Field `IMR0_TOK2` reader - "]
pub type Imr0Tok2R = crate::BitReader;
#[doc = "Field `IMR0_TOK2` writer - "]
pub type Imr0Tok2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TOK3` reader - "]
pub type Imr0Tok3R = crate::BitReader;
#[doc = "Field `IMR0_TOK3` writer - "]
pub type Imr0Tok3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TOK4` reader - "]
pub type Imr0Tok4R = crate::BitReader;
#[doc = "Field `IMR0_TOK4` writer - "]
pub type Imr0Tok4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TOK5` reader - "]
pub type Imr0Tok5R = crate::BitReader;
#[doc = "Field `IMR0_TOK5` writer - "]
pub type Imr0Tok5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TDU1` reader - This bit is IMR.tdu"]
pub type Imr0Tdu1R = crate::BitReader;
#[doc = "Field `IMR0_TDU2` reader - "]
pub type Imr0Tdu2R = crate::BitReader;
#[doc = "Field `IMR0_TDU2` writer - "]
pub type Imr0Tdu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TDU3` reader - "]
pub type Imr0Tdu3R = crate::BitReader;
#[doc = "Field `IMR0_TDU3` writer - "]
pub type Imr0Tdu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TDU4` reader - "]
pub type Imr0Tdu4R = crate::BitReader;
#[doc = "Field `IMR0_TDU4` writer - "]
pub type Imr0Tdu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR0_TDU5` reader - "]
pub type Imr0Tdu5R = crate::BitReader;
#[doc = "Field `IMR0_TDU5` writer - "]
pub type Imr0Tdu5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is IMR.rok"]
    #[inline(always)]
    pub fn imr0_rok1(&self) -> Imr0Rok1R {
        Imr0Rok1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn imr0_rok2(&self) -> Imr0Rok2R {
        Imr0Rok2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn imr0_rok3(&self) -> Imr0Rok3R {
        Imr0Rok3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn imr0_rok4(&self) -> Imr0Rok4R {
        Imr0Rok4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn imr0_rok5(&self) -> Imr0Rok5R {
        Imr0Rok5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn imr0_rok6(&self) -> Imr0Rok6R {
        Imr0Rok6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is IMR.tok"]
    #[inline(always)]
    pub fn imr0_tok1(&self) -> Imr0Tok1R {
        Imr0Tok1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn imr0_tok2(&self) -> Imr0Tok2R {
        Imr0Tok2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn imr0_tok3(&self) -> Imr0Tok3R {
        Imr0Tok3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn imr0_tok4(&self) -> Imr0Tok4R {
        Imr0Tok4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn imr0_tok5(&self) -> Imr0Tok5R {
        Imr0Tok5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is IMR.tdu"]
    #[inline(always)]
    pub fn imr0_tdu1(&self) -> Imr0Tdu1R {
        Imr0Tdu1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn imr0_tdu2(&self) -> Imr0Tdu2R {
        Imr0Tdu2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn imr0_tdu3(&self) -> Imr0Tdu3R {
        Imr0Tdu3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn imr0_tdu4(&self) -> Imr0Tdu4R {
        Imr0Tdu4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn imr0_tdu5(&self) -> Imr0Tdu5R {
        Imr0Tdu5R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn imr0_rok2(&mut self) -> Imr0Rok2W<'_, Imr0RegSpec> {
        Imr0Rok2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn imr0_rok3(&mut self) -> Imr0Rok3W<'_, Imr0RegSpec> {
        Imr0Rok3W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn imr0_rok4(&mut self) -> Imr0Rok4W<'_, Imr0RegSpec> {
        Imr0Rok4W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn imr0_rok5(&mut self) -> Imr0Rok5W<'_, Imr0RegSpec> {
        Imr0Rok5W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn imr0_rok6(&mut self) -> Imr0Rok6W<'_, Imr0RegSpec> {
        Imr0Rok6W::new(self, 5)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn imr0_tok2(&mut self) -> Imr0Tok2W<'_, Imr0RegSpec> {
        Imr0Tok2W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn imr0_tok3(&mut self) -> Imr0Tok3W<'_, Imr0RegSpec> {
        Imr0Tok3W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn imr0_tok4(&mut self) -> Imr0Tok4W<'_, Imr0RegSpec> {
        Imr0Tok4W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn imr0_tok5(&mut self) -> Imr0Tok5W<'_, Imr0RegSpec> {
        Imr0Tok5W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn imr0_tdu2(&mut self) -> Imr0Tdu2W<'_, Imr0RegSpec> {
        Imr0Tdu2W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn imr0_tdu3(&mut self) -> Imr0Tdu3W<'_, Imr0RegSpec> {
        Imr0Tdu3W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn imr0_tdu4(&mut self) -> Imr0Tdu4W<'_, Imr0RegSpec> {
        Imr0Tdu4W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn imr0_tdu5(&mut self) -> Imr0Tdu5W<'_, Imr0RegSpec> {
        Imr0Tdu5W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`imr0_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr0_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr0RegSpec;
impl crate::RegisterSpec for Imr0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr0_reg::R`](R) reader structure"]
impl crate::Readable for Imr0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`imr0_reg::W`](W) writer structure"]
impl crate::Writable for Imr0RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR0_REG to value 0"]
impl crate::Resettable for Imr0RegSpec {}
