#[doc = "Register `REG_GENERAL_OPTION` reader"]
pub type R = crate::R<RegGeneralOptionSpec>;
#[doc = "Register `REG_GENERAL_OPTION` writer"]
pub type W = crate::W<RegGeneralOptionSpec>;
#[doc = "Field `BIT_TXSERV_FIELD_SEL` reader - "]
pub type BitTxservFieldSelR = crate::BitReader;
#[doc = "Field `BIT_TXSERV_FIELD_SEL` writer - "]
pub type BitTxservFieldSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EXT_DBGSEL` reader - enable extend dbg port sel"]
pub type BitExtDbgselR = crate::BitReader;
#[doc = "Field `BIT_EXT_DBGSEL` writer - enable extend dbg port sel"]
pub type BitExtDbgselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_txserv_field_sel(&self) -> BitTxservFieldSelR {
        BitTxservFieldSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - enable extend dbg port sel"]
    #[inline(always)]
    pub fn bit_ext_dbgsel(&self) -> BitExtDbgselR {
        BitExtDbgselR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_txserv_field_sel(&mut self) -> BitTxservFieldSelW<'_, RegGeneralOptionSpec> {
        BitTxservFieldSelW::new(self, 2)
    }
    #[doc = "Bit 6 - enable extend dbg port sel"]
    #[inline(always)]
    pub fn bit_ext_dbgsel(&mut self) -> BitExtDbgselW<'_, RegGeneralOptionSpec> {
        BitExtDbgselW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_general_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_general_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGeneralOptionSpec;
impl crate::RegisterSpec for RegGeneralOptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_general_option::R`](R) reader structure"]
impl crate::Readable for RegGeneralOptionSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_general_option::W`](W) writer structure"]
impl crate::Writable for RegGeneralOptionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GENERAL_OPTION to value 0"]
impl crate::Resettable for RegGeneralOptionSpec {}
