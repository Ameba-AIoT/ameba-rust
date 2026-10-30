#[doc = "Register `OSC131K_POW` reader"]
pub type R = crate::R<Osc131kPowSpec>;
#[doc = "Register `OSC131K_POW` writer"]
pub type W = crate::W<Osc131kPowSpec>;
#[doc = "Field `POW_32KOSC` reader - // \\[RF\\]\\[32KOSC\\] 1'b1 : enable"]
pub type Pow32koscR = crate::BitReader;
#[doc = "Field `POW_32KOSC` writer - // \\[RF\\]\\[32KOSC\\] 1'b1 : enable"]
pub type Pow32koscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_1` reader - "]
pub type Dmy1R = crate::FieldReader<u16>;
#[doc = "Field `dmy_1` writer - "]
pub type Dmy1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `GATED_STUP_OK_D` reader - // \\[RF\\]\\[32KOSC\\] dummy"]
pub type GatedStupOkDR = crate::BitReader;
#[doc = "Field `GATED_STUP_OK_D` writer - // \\[RF\\]\\[32KOSC\\] dummy"]
pub type GatedStupOkDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_32KOSC_DUMMY` reader - // \\[RF\\]\\[32KOSC\\] dummy"]
pub type Reg32koscDummyR = crate::FieldReader;
#[doc = "Field `REG_32KOSC_DUMMY` writer - // \\[RF\\]\\[32KOSC\\] dummy"]
pub type Reg32koscDummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - // \\[RF\\]\\[32KOSC\\] 1'b1 : enable"]
    #[inline(always)]
    pub fn pow_32kosc(&self) -> Pow32koscR {
        Pow32koscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn dmy_1(&self) -> Dmy1R {
        Dmy1R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - // \\[RF\\]\\[32KOSC\\] dummy"]
    #[inline(always)]
    pub fn gated_stup_ok_d(&self) -> GatedStupOkDR {
        GatedStupOkDR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - // \\[RF\\]\\[32KOSC\\] dummy"]
    #[inline(always)]
    pub fn reg_32kosc_dummy(&self) -> Reg32koscDummyR {
        Reg32koscDummyR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - // \\[RF\\]\\[32KOSC\\] 1'b1 : enable"]
    #[inline(always)]
    pub fn pow_32kosc(&mut self) -> Pow32koscW<'_, Osc131kPowSpec> {
        Pow32koscW::new(self, 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn dmy_1(&mut self) -> Dmy1W<'_, Osc131kPowSpec> {
        Dmy1W::new(self, 1)
    }
    #[doc = "Bit 16 - // \\[RF\\]\\[32KOSC\\] dummy"]
    #[inline(always)]
    pub fn gated_stup_ok_d(&mut self) -> GatedStupOkDW<'_, Osc131kPowSpec> {
        GatedStupOkDW::new(self, 16)
    }
    #[doc = "Bits 17:18 - // \\[RF\\]\\[32KOSC\\] dummy"]
    #[inline(always)]
    pub fn reg_32kosc_dummy(&mut self) -> Reg32koscDummyW<'_, Osc131kPowSpec> {
        Reg32koscDummyW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`osc131k_pow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc131k_pow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc131kPowSpec;
impl crate::RegisterSpec for Osc131kPowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc131k_pow::R`](R) reader structure"]
impl crate::Readable for Osc131kPowSpec {}
#[doc = "`write(|w| ..)` method takes [`osc131k_pow::W`](W) writer structure"]
impl crate::Writable for Osc131kPowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC131K_POW to value 0x01"]
impl crate::Resettable for Osc131kPowSpec {
    const RESET_VALUE: u32 = 0x01;
}
