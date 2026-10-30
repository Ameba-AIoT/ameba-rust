#[doc = "Register `REG_TRAN_BSSID0_H` reader"]
pub type R = crate::R<RegTranBssid0HSpec>;
#[doc = "Register `REG_TRAN_BSSID0_H` writer"]
pub type W = crate::W<RegTranBssid0HSpec>;
#[doc = "Field `r_wmac_transbssid0_h` reader - higher 16 bits of transmitted bssid0"]
pub type RWmacTransbssid0HR = crate::FieldReader<u16>;
#[doc = "Field `r_wmac_transbssid0_h` writer - higher 16 bits of transmitted bssid0"]
pub type RWmacTransbssid0HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_bss_color0` reader - bss color of current BSS"]
pub type RBssColor0R = crate::FieldReader;
#[doc = "Field `r_bss_color0` writer - bss color of current BSS"]
pub type RBssColor0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_en_mbssid` reader - enable multibssid function"]
pub type REnMbssidR = crate::BitReader;
#[doc = "Field `r_en_mbssid` writer - enable multibssid function"]
pub type REnMbssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_en_partaid` reader - enable inter ppdu decision according to partial aid for VHT ppdu"]
pub type REnPartaidR = crate::BitReader;
#[doc = "Field `r_en_partaid` writer - enable inter ppdu decision according to partial aid for VHT ppdu"]
pub type REnPartaidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_mbssid_ctlinfo_srch_en` reader - when set, rx trigger and vht/he ndpa, will search control info using assocaiated bssid"]
pub type RMbssidCtlinfoSrchEnR = crate::BitReader;
#[doc = "Field `r_mbssid_ctlinfo_srch_en` writer - when set, rx trigger and vht/he ndpa, will search control info using assocaiated bssid"]
pub type RMbssidCtlinfoSrchEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - higher 16 bits of transmitted bssid0"]
    #[inline(always)]
    pub fn r_wmac_transbssid0_h(&self) -> RWmacTransbssid0HR {
        RWmacTransbssid0HR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - bss color of current BSS"]
    #[inline(always)]
    pub fn r_bss_color0(&self) -> RBssColor0R {
        RBssColor0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - enable multibssid function"]
    #[inline(always)]
    pub fn r_en_mbssid(&self) -> REnMbssidR {
        REnMbssidR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - enable inter ppdu decision according to partial aid for VHT ppdu"]
    #[inline(always)]
    pub fn r_en_partaid(&self) -> REnPartaidR {
        REnPartaidR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - when set, rx trigger and vht/he ndpa, will search control info using assocaiated bssid"]
    #[inline(always)]
    pub fn r_mbssid_ctlinfo_srch_en(&self) -> RMbssidCtlinfoSrchEnR {
        RMbssidCtlinfoSrchEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - higher 16 bits of transmitted bssid0"]
    #[inline(always)]
    pub fn r_wmac_transbssid0_h(&mut self) -> RWmacTransbssid0HW<'_, RegTranBssid0HSpec> {
        RWmacTransbssid0HW::new(self, 0)
    }
    #[doc = "Bits 16:21 - bss color of current BSS"]
    #[inline(always)]
    pub fn r_bss_color0(&mut self) -> RBssColor0W<'_, RegTranBssid0HSpec> {
        RBssColor0W::new(self, 16)
    }
    #[doc = "Bit 22 - enable multibssid function"]
    #[inline(always)]
    pub fn r_en_mbssid(&mut self) -> REnMbssidW<'_, RegTranBssid0HSpec> {
        REnMbssidW::new(self, 22)
    }
    #[doc = "Bit 23 - enable inter ppdu decision according to partial aid for VHT ppdu"]
    #[inline(always)]
    pub fn r_en_partaid(&mut self) -> REnPartaidW<'_, RegTranBssid0HSpec> {
        REnPartaidW::new(self, 23)
    }
    #[doc = "Bit 24 - when set, rx trigger and vht/he ndpa, will search control info using assocaiated bssid"]
    #[inline(always)]
    pub fn r_mbssid_ctlinfo_srch_en(&mut self) -> RMbssidCtlinfoSrchEnW<'_, RegTranBssid0HSpec> {
        RMbssidCtlinfoSrchEnW::new(self, 24)
    }
}
#[doc = "trasmitted bssid register0_h\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tran_bssid0_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tran_bssid0_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTranBssid0HSpec;
impl crate::RegisterSpec for RegTranBssid0HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tran_bssid0_h::R`](R) reader structure"]
impl crate::Readable for RegTranBssid0HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tran_bssid0_h::W`](W) writer structure"]
impl crate::Writable for RegTranBssid0HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRAN_BSSID0_H to value 0"]
impl crate::Resettable for RegTranBssid0HSpec {}
