#[doc = "Register `gmac_len_1` reader"]
pub type R = crate::R<GmacLen1Spec>;
#[doc = "Register `gmac_len_1` writer"]
pub type W = crate::W<GmacLen1Spec>;
#[doc = "Field `gmac_len_1` reader - AAD length register bit\\[32*1+31:32*1\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
pub type GmacLen1R = crate::FieldReader<u32>;
#[doc = "Field `gmac_len_1` writer - AAD length register bit\\[32*1+31:32*1\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
pub type GmacLen1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AAD length register bit\\[32*1+31:32*1\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn gmac_len_1(&self) -> GmacLen1R {
        GmacLen1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AAD length register bit\\[32*1+31:32*1\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn gmac_len_1(&mut self) -> GmacLen1W<'_, GmacLen1Spec> {
        GmacLen1W::new(self, 0)
    }
}
#[doc = "AAD length register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_len_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_len_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacLen1Spec;
impl crate::RegisterSpec for GmacLen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_len_1::R`](R) reader structure"]
impl crate::Readable for GmacLen1Spec {}
#[doc = "`write(|w| ..)` method takes [`gmac_len_1::W`](W) writer structure"]
impl crate::Writable for GmacLen1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gmac_len_1 to value 0"]
impl crate::Resettable for GmacLen1Spec {}
