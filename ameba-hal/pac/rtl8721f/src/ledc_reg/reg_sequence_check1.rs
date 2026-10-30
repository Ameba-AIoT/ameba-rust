#[doc = "Register `REG_SEQUENCE_CHECK1` reader"]
pub type R = crate::R<RegSequenceCheck1Spec>;
#[doc = "Register `REG_SEQUENCE_CHECK1` writer"]
pub type W = crate::W<RegSequenceCheck1Spec>;
#[doc = "Field `r_seq_chk1` reader - register protect sequence check, when this field = 0xa5a5a5a5, enable access protect register"]
pub type RSeqChk1R = crate::FieldReader<u32>;
#[doc = "Field `r_seq_chk1` writer - register protect sequence check, when this field = 0xa5a5a5a5, enable access protect register"]
pub type RSeqChk1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register protect sequence check, when this field = 0xa5a5a5a5, enable access protect register"]
    #[inline(always)]
    pub fn r_seq_chk1(&self) -> RSeqChk1R {
        RSeqChk1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - register protect sequence check, when this field = 0xa5a5a5a5, enable access protect register"]
    #[inline(always)]
    pub fn r_seq_chk1(&mut self) -> RSeqChk1W<'_, RegSequenceCheck1Spec> {
        RSeqChk1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sequence_check1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sequence_check1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSequenceCheck1Spec;
impl crate::RegisterSpec for RegSequenceCheck1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sequence_check1::R`](R) reader structure"]
impl crate::Readable for RegSequenceCheck1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sequence_check1::W`](W) writer structure"]
impl crate::Writable for RegSequenceCheck1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEQUENCE_CHECK1 to value 0"]
impl crate::Resettable for RegSequenceCheck1Spec {}
