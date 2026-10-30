#[doc = "Register `REG_ARB_DBC1` reader"]
pub type R = crate::R<RegArbDbc1Spec>;
#[doc = "Register `REG_ARB_DBC1` writer"]
pub type W = crate::W<RegArbDbc1Spec>;
#[doc = "Field `dbc_en_mldo` reader - mldo arbit debounce enable 0:disable 1:enable"]
pub type DbcEnMldoR = crate::BitReader;
#[doc = "Field `dbc_en_mldo` writer - mldo arbit debounce enable 0:disable 1:enable"]
pub type DbcEnMldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbc_cnt_mldo` reader - mldo debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
pub type DbcCntMldoR = crate::FieldReader<u16>;
#[doc = "Field `dbc_cnt_mldo` writer - mldo debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
pub type DbcCntMldoW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - mldo arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_mldo(&self) -> DbcEnMldoR {
        DbcEnMldoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:12 - mldo debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
    #[inline(always)]
    pub fn dbc_cnt_mldo(&self) -> DbcCntMldoR {
        DbcCntMldoR::new(((self.bits >> 1) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - mldo arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_mldo(&mut self) -> DbcEnMldoW<'_, RegArbDbc1Spec> {
        DbcEnMldoW::new(self, 0)
    }
    #[doc = "Bits 1:12 - mldo debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
    #[inline(always)]
    pub fn dbc_cnt_mldo(&mut self) -> DbcCntMldoW<'_, RegArbDbc1Spec> {
        DbcCntMldoW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_arb_dbc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_arb_dbc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegArbDbc1Spec;
impl crate::RegisterSpec for RegArbDbc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_arb_dbc1::R`](R) reader structure"]
impl crate::Readable for RegArbDbc1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_arb_dbc1::W`](W) writer structure"]
impl crate::Writable for RegArbDbc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ARB_DBC1 to value 0x018f"]
impl crate::Resettable for RegArbDbc1Spec {
    const RESET_VALUE: u32 = 0x018f;
}
