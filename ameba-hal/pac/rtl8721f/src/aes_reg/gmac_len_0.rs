#[doc = "Register `gmac_len_0` reader"]
pub type R = crate::R<GmacLen0Spec>;
#[doc = "Register `gmac_len_0` writer"]
pub type W = crate::W<GmacLen0Spec>;
#[doc = "Field `gmac_len_0` reader - AAD length register bit\\[32*0+31:32*0\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
pub type GmacLen0R = crate::FieldReader<u32>;
#[doc = "Field `gmac_len_0` writer - AAD length register bit\\[32*0+31:32*0\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
pub type GmacLen0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AAD length register bit\\[32*0+31:32*0\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn gmac_len_0(&self) -> GmacLen0R {
        GmacLen0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AAD length register bit\\[32*0+31:32*0\\] in GMAC and GCM mode In the GCM or GMAC mode, the user needs to inform the length of the Additional Authenticated Data (AAD). The length of the one AAD needs to be accumulated. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn gmac_len_0(&mut self) -> GmacLen0W<'_, GmacLen0Spec> {
        GmacLen0W::new(self, 0)
    }
}
#[doc = "AAD length register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_len_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_len_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacLen0Spec;
impl crate::RegisterSpec for GmacLen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_len_0::R`](R) reader structure"]
impl crate::Readable for GmacLen0Spec {}
#[doc = "`write(|w| ..)` method takes [`gmac_len_0::W`](W) writer structure"]
impl crate::Writable for GmacLen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gmac_len_0 to value 0"]
impl crate::Resettable for GmacLen0Spec {}
