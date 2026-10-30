#[doc = "Register `REG_HW_SEQ2_3` reader"]
pub type R = crate::R<RegHwSeq2_3Spec>;
#[doc = "Register `REG_HW_SEQ2_3` writer"]
pub type W = crate::W<RegHwSeq2_3Spec>;
#[doc = "Field `BIT_HW_SSN_SEQ2` reader - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HW_SSN_SEQ2` writer - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HW_SSN_SEQ3` reader - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq3R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HW_SSN_SEQ3` writer - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq2(&self) -> BitHwSsnSeq2R {
        BitHwSsnSeq2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq3(&self) -> BitHwSsnSeq3R {
        BitHwSsnSeq3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq2(&mut self) -> BitHwSsnSeq2W<'_, RegHwSeq2_3Spec> {
        BitHwSsnSeq2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq3(&mut self) -> BitHwSsnSeq3W<'_, RegHwSeq2_3Spec> {
        BitHwSsnSeq3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_seq2_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_seq2_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwSeq2_3Spec;
impl crate::RegisterSpec for RegHwSeq2_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_seq2_3::R`](R) reader structure"]
impl crate::Readable for RegHwSeq2_3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_seq2_3::W`](W) writer structure"]
impl crate::Writable for RegHwSeq2_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_SEQ2_3 to value 0"]
impl crate::Resettable for RegHwSeq2_3Spec {}
