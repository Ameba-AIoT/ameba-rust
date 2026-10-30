#[doc = "Register `REG_TRAN_BSSID1_H` reader"]
pub type R = crate::R<RegTranBssid1HSpec>;
#[doc = "Register `REG_TRAN_BSSID1_H` writer"]
pub type W = crate::W<RegTranBssid1HSpec>;
#[doc = "Field `r_wmac_transbssid1_h` reader - higher 16 bits of transmitted bssid1"]
pub type RWmacTransbssid1HR = crate::FieldReader<u16>;
#[doc = "Field `r_wmac_transbssid1_h` writer - higher 16 bits of transmitted bssid1"]
pub type RWmacTransbssid1HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_bss_color1` reader - bss color of current BSS"]
pub type RBssColor1R = crate::FieldReader;
#[doc = "Field `r_bss_color1` writer - bss color of current BSS"]
pub type RBssColor1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_bss_color2` reader - bss color of port2"]
pub type RBssColor2R = crate::FieldReader;
#[doc = "Field `r_bss_color2` writer - bss color of port2"]
pub type RBssColor2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - higher 16 bits of transmitted bssid1"]
    #[inline(always)]
    pub fn r_wmac_transbssid1_h(&self) -> RWmacTransbssid1HR {
        RWmacTransbssid1HR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - bss color of current BSS"]
    #[inline(always)]
    pub fn r_bss_color1(&self) -> RBssColor1R {
        RBssColor1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - bss color of port2"]
    #[inline(always)]
    pub fn r_bss_color2(&self) -> RBssColor2R {
        RBssColor2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - higher 16 bits of transmitted bssid1"]
    #[inline(always)]
    pub fn r_wmac_transbssid1_h(&mut self) -> RWmacTransbssid1HW<'_, RegTranBssid1HSpec> {
        RWmacTransbssid1HW::new(self, 0)
    }
    #[doc = "Bits 16:21 - bss color of current BSS"]
    #[inline(always)]
    pub fn r_bss_color1(&mut self) -> RBssColor1W<'_, RegTranBssid1HSpec> {
        RBssColor1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - bss color of port2"]
    #[inline(always)]
    pub fn r_bss_color2(&mut self) -> RBssColor2W<'_, RegTranBssid1HSpec> {
        RBssColor2W::new(self, 24)
    }
}
#[doc = "trasmitted bssid register1_h\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tran_bssid1_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tran_bssid1_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTranBssid1HSpec;
impl crate::RegisterSpec for RegTranBssid1HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tran_bssid1_h::R`](R) reader structure"]
impl crate::Readable for RegTranBssid1HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tran_bssid1_h::W`](W) writer structure"]
impl crate::Writable for RegTranBssid1HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRAN_BSSID1_H to value 0"]
impl crate::Resettable for RegTranBssid1HSpec {}
