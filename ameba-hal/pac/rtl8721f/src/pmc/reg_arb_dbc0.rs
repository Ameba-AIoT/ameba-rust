#[doc = "Register `REG_ARB_DBC0` reader"]
pub type R = crate::R<RegArbDbc0Spec>;
#[doc = "Register `REG_ARB_DBC0` writer"]
pub type W = crate::W<RegArbDbc0Spec>;
#[doc = "Field `dbc_en_regu_sts` reader - regu sts arbit debounce enable 0:disable 1:enable"]
pub type DbcEnReguStsR = crate::BitReader;
#[doc = "Field `dbc_en_regu_sts` writer - regu sts arbit debounce enable 0:disable 1:enable"]
pub type DbcEnReguStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbc_cnt_regu_sts` reader - regu sts debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
pub type DbcCntReguStsR = crate::FieldReader<u16>;
#[doc = "Field `dbc_cnt_regu_sts` writer - regu sts debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
pub type DbcCntReguStsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `dbc_en_xmd` reader - xtal mode arbit debounce enable 0:disable 1:enable"]
pub type DbcEnXmdR = crate::BitReader;
#[doc = "Field `dbc_en_xmd` writer - xtal mode arbit debounce enable 0:disable 1:enable"]
pub type DbcEnXmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbc_cnt_xmd` reader - xmd debounce counter , cycle based in OSC4M"]
pub type DbcCntXmdR = crate::FieldReader<u16>;
#[doc = "Field `dbc_cnt_xmd` writer - xmd debounce counter , cycle based in OSC4M"]
pub type DbcCntXmdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - regu sts arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_regu_sts(&self) -> DbcEnReguStsR {
        DbcEnReguStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:12 - regu sts debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
    #[inline(always)]
    pub fn dbc_cnt_regu_sts(&self) -> DbcCntReguStsR {
        DbcCntReguStsR::new(((self.bits >> 1) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - xtal mode arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_xmd(&self) -> DbcEnXmdR {
        DbcEnXmdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - xmd debounce counter , cycle based in OSC4M"]
    #[inline(always)]
    pub fn dbc_cnt_xmd(&self) -> DbcCntXmdR {
        DbcCntXmdR::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - regu sts arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_regu_sts(&mut self) -> DbcEnReguStsW<'_, RegArbDbc0Spec> {
        DbcEnReguStsW::new(self, 0)
    }
    #[doc = "Bits 1:12 - regu sts debounce counter , cycle based in OSC4M for 4M sync to 100K, it must wait 40us for async handshake"]
    #[inline(always)]
    pub fn dbc_cnt_regu_sts(&mut self) -> DbcCntReguStsW<'_, RegArbDbc0Spec> {
        DbcCntReguStsW::new(self, 1)
    }
    #[doc = "Bit 16 - xtal mode arbit debounce enable 0:disable 1:enable"]
    #[inline(always)]
    pub fn dbc_en_xmd(&mut self) -> DbcEnXmdW<'_, RegArbDbc0Spec> {
        DbcEnXmdW::new(self, 16)
    }
    #[doc = "Bits 17:28 - xmd debounce counter , cycle based in OSC4M"]
    #[inline(always)]
    pub fn dbc_cnt_xmd(&mut self) -> DbcCntXmdW<'_, RegArbDbc0Spec> {
        DbcCntXmdW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_arb_dbc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_arb_dbc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegArbDbc0Spec;
impl crate::RegisterSpec for RegArbDbc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_arb_dbc0::R`](R) reader structure"]
impl crate::Readable for RegArbDbc0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_arb_dbc0::W`](W) writer structure"]
impl crate::Writable for RegArbDbc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ARB_DBC0 to value 0x0015_018f"]
impl crate::Resettable for RegArbDbc0Spec {
    const RESET_VALUE: u32 = 0x0015_018f;
}
