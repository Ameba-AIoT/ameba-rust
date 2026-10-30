#[doc = "Register `REG_NOACMD_SEL_EDCA_RANDOM_GEN` reader"]
pub type R = crate::R<RegNoacmdSelEdcaRandomGenSpec>;
#[doc = "Register `REG_NOACMD_SEL_EDCA_RANDOM_GEN` writer"]
pub type W = crate::W<RegNoacmdSelEdcaRandomGenSpec>;
#[doc = "Field `BIT_RANDOM_GEN` reader - Random number generator polynomial"]
pub type BitRandomGenR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RANDOM_GEN` writer - Random number generator polynomial"]
pub type BitRandomGenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TXCMD_SEG_SEL` reader - HW Debug Info\\]To select which one TXCMD Segment. For example, fill 0x5CF\\[3:0\\] with 0 or 6 to read 0x5F8\\[7:0\\] : Tx Agg 0x5F9\\[7:0\\] : Tx Rate"]
pub type BitTxcmdSegSelR = crate::FieldReader;
#[doc = "Field `BIT_TXCMD_SEG_SEL` writer - HW Debug Info\\]To select which one TXCMD Segment. For example, fill 0x5CF\\[3:0\\] with 0 or 6 to read 0x5F8\\[7:0\\] : Tx Agg 0x5F9\\[7:0\\] : Tx Rate"]
pub type BitTxcmdSegSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_NOA_SEL` reader - TX NoA parameter select (noa_unit0~2,parameter0~5) 0: NoA0 , 1:NoA1, 2:NoA2, 3:NoA3, 4:NoA4, default: NoA5 To control the register setting for which NOA. (register 0x05E0\\[103:0\\])"]
pub type BitNoaSelR = crate::FieldReader;
#[doc = "Field `BIT_NOA_SEL` writer - TX NoA parameter select (noa_unit0~2,parameter0~5) 0: NoA0 , 1:NoA1, 2:NoA2, 3:NoA3, 4:NoA4, default: NoA5 To control the register setting for which NOA. (register 0x05E0\\[103:0\\])"]
pub type BitNoaSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:23 - Random number generator polynomial"]
    #[inline(always)]
    pub fn bit_random_gen(&self) -> BitRandomGenR {
        BitRandomGenR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - HW Debug Info\\]To select which one TXCMD Segment. For example, fill 0x5CF\\[3:0\\] with 0 or 6 to read 0x5F8\\[7:0\\] : Tx Agg 0x5F9\\[7:0\\] : Tx Rate"]
    #[inline(always)]
    pub fn bit_txcmd_seg_sel(&self) -> BitTxcmdSegSelR {
        BitTxcmdSegSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - TX NoA parameter select (noa_unit0~2,parameter0~5) 0: NoA0 , 1:NoA1, 2:NoA2, 3:NoA3, 4:NoA4, default: NoA5 To control the register setting for which NOA. (register 0x05E0\\[103:0\\])"]
    #[inline(always)]
    pub fn bit_noa_sel(&self) -> BitNoaSelR {
        BitNoaSelR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Random number generator polynomial"]
    #[inline(always)]
    pub fn bit_random_gen(&mut self) -> BitRandomGenW<'_, RegNoacmdSelEdcaRandomGenSpec> {
        BitRandomGenW::new(self, 0)
    }
    #[doc = "Bits 24:27 - HW Debug Info\\]To select which one TXCMD Segment. For example, fill 0x5CF\\[3:0\\] with 0 or 6 to read 0x5F8\\[7:0\\] : Tx Agg 0x5F9\\[7:0\\] : Tx Rate"]
    #[inline(always)]
    pub fn bit_txcmd_seg_sel(&mut self) -> BitTxcmdSegSelW<'_, RegNoacmdSelEdcaRandomGenSpec> {
        BitTxcmdSegSelW::new(self, 24)
    }
    #[doc = "Bits 28:30 - TX NoA parameter select (noa_unit0~2,parameter0~5) 0: NoA0 , 1:NoA1, 2:NoA2, 3:NoA3, 4:NoA4, default: NoA5 To control the register setting for which NOA. (register 0x05E0\\[103:0\\])"]
    #[inline(always)]
    pub fn bit_noa_sel(&mut self) -> BitNoaSelW<'_, RegNoacmdSelEdcaRandomGenSpec> {
        BitNoaSelW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noacmd_sel_edca_random_gen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noacmd_sel_edca_random_gen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoacmdSelEdcaRandomGenSpec;
impl crate::RegisterSpec for RegNoacmdSelEdcaRandomGenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noacmd_sel_edca_random_gen::R`](R) reader structure"]
impl crate::Readable for RegNoacmdSelEdcaRandomGenSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_noacmd_sel_edca_random_gen::W`](W) writer structure"]
impl crate::Writable for RegNoacmdSelEdcaRandomGenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOACMD_SEL_EDCA_RANDOM_GEN to value 0x01e2_1051"]
impl crate::Resettable for RegNoacmdSelEdcaRandomGenSpec {
    const RESET_VALUE: u32 = 0x01e2_1051;
}
