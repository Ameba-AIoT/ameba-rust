#[doc = "Register `WDG_MKEYR` reader"]
pub type R = crate::R<WdgMkeyrSpec>;
#[doc = "Register `WDG_MKEYR` writer"]
pub type W = crate::W<WdgMkeyrSpec>;
#[doc = "Field `MKEY` reader - 0x6969: enable access to register WDG_CR/WDG_RLR/WDG_WINR 0x5A5A: reload WDG counter 0x3C3C: enable WDG function"]
pub type MkeyR = crate::FieldReader<u16>;
#[doc = "Field `MKEY` writer - 0x6969: enable access to register WDG_CR/WDG_RLR/WDG_WINR 0x5A5A: reload WDG counter 0x3C3C: enable WDG function"]
pub type MkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 0x6969: enable access to register WDG_CR/WDG_RLR/WDG_WINR 0x5A5A: reload WDG counter 0x3C3C: enable WDG function"]
    #[inline(always)]
    pub fn mkey(&self) -> MkeyR {
        MkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0x6969: enable access to register WDG_CR/WDG_RLR/WDG_WINR 0x5A5A: reload WDG counter 0x3C3C: enable WDG function"]
    #[inline(always)]
    pub fn mkey(&mut self) -> MkeyW<'_, WdgMkeyrSpec> {
        MkeyW::new(self, 0)
    }
}
#[doc = "WDG Magic Key register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdg_mkeyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdg_mkeyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdgMkeyrSpec;
impl crate::RegisterSpec for WdgMkeyrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_mkeyr::R`](R) reader structure"]
impl crate::Readable for WdgMkeyrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdg_mkeyr::W`](W) writer structure"]
impl crate::Writable for WdgMkeyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDG_MKEYR to value 0"]
impl crate::Resettable for WdgMkeyrSpec {}
