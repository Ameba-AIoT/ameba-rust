#[doc = "Register `REG_MBIDCAM1_ENTRY14` reader"]
pub type R = crate::R<RegMbidcam1Entry14Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY14` writer"]
pub type W = crate::W<RegMbidcam1Entry14Spec>;
#[doc = "Field `MBIDCAM1_ENTRY14_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry14BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY14_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry14BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY14_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry14DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY14_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry14DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY14_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry14LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY14_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry14LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY14_VALID` reader - "]
pub type Mbidcam1Entry14ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY14_VALID` writer - "]
pub type Mbidcam1Entry14ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry14_bssid_macid1(&self) -> Mbidcam1Entry14BssidMacid1R {
        Mbidcam1Entry14BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry14_dua_cts_en(&self) -> Mbidcam1Entry14DuaCtsEnR {
        Mbidcam1Entry14DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry14_lsig_txop_en(&self) -> Mbidcam1Entry14LsigTxopEnR {
        Mbidcam1Entry14LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry14_valid(&self) -> Mbidcam1Entry14ValidR {
        Mbidcam1Entry14ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry14_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry14BssidMacid1W<'_, RegMbidcam1Entry14Spec> {
        Mbidcam1Entry14BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry14_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry14DuaCtsEnW<'_, RegMbidcam1Entry14Spec> {
        Mbidcam1Entry14DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry14_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry14LsigTxopEnW<'_, RegMbidcam1Entry14Spec> {
        Mbidcam1Entry14LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry14_valid(&mut self) -> Mbidcam1Entry14ValidW<'_, RegMbidcam1Entry14Spec> {
        Mbidcam1Entry14ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry14Spec;
impl crate::RegisterSpec for RegMbidcam1Entry14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry14::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry14::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY14 to value 0"]
impl crate::Resettable for RegMbidcam1Entry14Spec {}
