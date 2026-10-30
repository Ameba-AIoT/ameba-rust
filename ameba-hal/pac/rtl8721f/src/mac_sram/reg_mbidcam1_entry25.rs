#[doc = "Register `REG_MBIDCAM1_ENTRY25` reader"]
pub type R = crate::R<RegMbidcam1Entry25Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY25` writer"]
pub type W = crate::W<RegMbidcam1Entry25Spec>;
#[doc = "Field `MBIDCAM1_ENTRY25_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry25BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY25_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry25BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY25_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry25DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY25_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry25DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY25_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry25LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY25_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry25LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY25_VALID` reader - "]
pub type Mbidcam1Entry25ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY25_VALID` writer - "]
pub type Mbidcam1Entry25ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry25_bssid_macid1(&self) -> Mbidcam1Entry25BssidMacid1R {
        Mbidcam1Entry25BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry25_dua_cts_en(&self) -> Mbidcam1Entry25DuaCtsEnR {
        Mbidcam1Entry25DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry25_lsig_txop_en(&self) -> Mbidcam1Entry25LsigTxopEnR {
        Mbidcam1Entry25LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry25_valid(&self) -> Mbidcam1Entry25ValidR {
        Mbidcam1Entry25ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry25_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry25BssidMacid1W<'_, RegMbidcam1Entry25Spec> {
        Mbidcam1Entry25BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry25_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry25DuaCtsEnW<'_, RegMbidcam1Entry25Spec> {
        Mbidcam1Entry25DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry25_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry25LsigTxopEnW<'_, RegMbidcam1Entry25Spec> {
        Mbidcam1Entry25LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry25_valid(&mut self) -> Mbidcam1Entry25ValidW<'_, RegMbidcam1Entry25Spec> {
        Mbidcam1Entry25ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry25Spec;
impl crate::RegisterSpec for RegMbidcam1Entry25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry25::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry25::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY25 to value 0"]
impl crate::Resettable for RegMbidcam1Entry25Spec {}
