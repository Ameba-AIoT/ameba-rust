#[doc = "Register `Dummy` reader"]
pub type R = crate::R<DummySpec>;
#[doc = "Register `Dummy` writer"]
pub type W = crate::W<DummySpec>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pg_rom_sel` reader - change all rom group power mode to slp"]
pub type PgRomSelR = crate::BitReader;
#[doc = "Field `pg_rom_sel` writer - change all rom group power mode to slp"]
pub type PgRomSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nopg_ls_sel` reader - change all nopg ram group power mode to ls"]
pub type NopgLsSelR = crate::BitReader;
#[doc = "Field `nopg_ls_sel` writer - change all nopg ram group power mode to ls"]
pub type NopgLsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pg_ret_sel` reader - change all pg ram group power mode to ret"]
pub type PgRetSelR = crate::BitReader;
#[doc = "Field `pg_ret_sel` writer - change all pg ram group power mode to ret"]
pub type PgRetSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - change all rom group power mode to slp"]
    #[inline(always)]
    pub fn pg_rom_sel(&self) -> PgRomSelR {
        PgRomSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - change all nopg ram group power mode to ls"]
    #[inline(always)]
    pub fn nopg_ls_sel(&self) -> NopgLsSelR {
        NopgLsSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - change all pg ram group power mode to ret"]
    #[inline(always)]
    pub fn pg_ret_sel(&self) -> PgRetSelR {
        PgRetSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, DummySpec> {
        DummyW::new(self, 0)
    }
    #[doc = "Bit 29 - change all rom group power mode to slp"]
    #[inline(always)]
    pub fn pg_rom_sel(&mut self) -> PgRomSelW<'_, DummySpec> {
        PgRomSelW::new(self, 29)
    }
    #[doc = "Bit 30 - change all nopg ram group power mode to ls"]
    #[inline(always)]
    pub fn nopg_ls_sel(&mut self) -> NopgLsSelW<'_, DummySpec> {
        NopgLsSelW::new(self, 30)
    }
    #[doc = "Bit 31 - change all pg ram group power mode to ret"]
    #[inline(always)]
    pub fn pg_ret_sel(&mut self) -> PgRetSelW<'_, DummySpec> {
        PgRetSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummySpec;
impl crate::RegisterSpec for DummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy::R`](R) reader structure"]
impl crate::Readable for DummySpec {}
#[doc = "`write(|w| ..)` method takes [`dummy::W`](W) writer structure"]
impl crate::Writable for DummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Dummy to value 0xff00"]
impl crate::Resettable for DummySpec {
    const RESET_VALUE: u32 = 0xff00;
}
