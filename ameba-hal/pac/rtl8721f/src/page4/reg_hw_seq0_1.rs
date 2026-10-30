#[doc = "Register `REG_HW_SEQ0_1` reader"]
pub type R = crate::R<RegHwSeq0_1Spec>;
#[doc = "Register `REG_HW_SEQ0_1` writer"]
pub type W = crate::W<RegHwSeq0_1Spec>;
#[doc = "Field `BIT_HW_SSN_SEQ0` reader - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HW_SSN_SEQ0` writer - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HW_SSN_SEQ1` reader - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HW_SSN_SEQ1` writer - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
pub type BitHwSsnSeq1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq0(&self) -> BitHwSsnSeq0R {
        BitHwSsnSeq0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq1(&self) -> BitHwSsnSeq1R {
        BitHwSsnSeq1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq0(&mut self) -> BitHwSsnSeq0W<'_, RegHwSeq0_1Spec> {
        BitHwSsnSeq0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - When TXDESC HWSEQEN is set, MAC will overwrite SEQ control by this register. Before DRV hand over the SEQ control to HW, DRV may set the current SEQ to this register. When DRV will take back the SEQ control from HW, DRV may get the current sequence from this register."]
    #[inline(always)]
    pub fn bit_hw_ssn_seq1(&mut self) -> BitHwSsnSeq1W<'_, RegHwSeq0_1Spec> {
        BitHwSsnSeq1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_seq0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_seq0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwSeq0_1Spec;
impl crate::RegisterSpec for RegHwSeq0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_seq0_1::R`](R) reader structure"]
impl crate::Readable for RegHwSeq0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_seq0_1::W`](W) writer structure"]
impl crate::Writable for RegHwSeq0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_SEQ0_1 to value 0"]
impl crate::Resettable for RegHwSeq0_1Spec {}
