#[doc = "Register `REG_ANTTRN_CTR_V1` reader"]
pub type R = crate::R<RegAnttrnCtrV1Spec>;
#[doc = "Register `REG_ANTTRN_CTR_V1` writer"]
pub type W = crate::W<RegAnttrnCtrV1Spec>;
#[doc = "Field `BIT_TRAIN_STA_ADDR1` reader - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
pub type BitTrainStaAddr1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_TRAIN_STA_ADDR1` writer - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
pub type BitTrainStaAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
    #[inline(always)]
    pub fn bit_train_sta_addr1(&self) -> BitTrainStaAddr1R {
        BitTrainStaAddr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
    #[inline(always)]
    pub fn bit_train_sta_addr1(&mut self) -> BitTrainStaAddr1W<'_, RegAnttrnCtrV1Spec> {
        BitTrainStaAddr1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anttrn_ctr_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anttrn_ctr_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnttrnCtrV1Spec;
impl crate::RegisterSpec for RegAnttrnCtrV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anttrn_ctr_v1::R`](R) reader structure"]
impl crate::Readable for RegAnttrnCtrV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anttrn_ctr_v1::W`](W) writer structure"]
impl crate::Writable for RegAnttrnCtrV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANTTRN_CTR_V1 to value 0"]
impl crate::Resettable for RegAnttrnCtrV1Spec {}
