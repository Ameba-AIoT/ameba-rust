#[doc = "Register `REG_WMAC_DBG_SEL` reader"]
pub type R = crate::R<RegWmacDbgSelSpec>;
#[doc = "Register `REG_WMAC_DBG_SEL` writer"]
pub type W = crate::W<RegWmacDbgSelSpec>;
#[doc = "Field `BIT_TXCMD_DBG_SEL` reader - "]
pub type BitTxcmdDbgSelR = crate::FieldReader;
#[doc = "Field `BIT_TXCMD_DBG_SEL` writer - "]
pub type BitTxcmdDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXCMD_DBG_SHIFT` reader - "]
pub type BitTxcmdDbgShiftR = crate::FieldReader;
#[doc = "Field `BIT_TXCMD_DBG_SHIFT` writer - "]
pub type BitTxcmdDbgShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn bit_txcmd_dbg_sel(&self) -> BitTxcmdDbgSelR {
        BitTxcmdDbgSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn bit_txcmd_dbg_shift(&self) -> BitTxcmdDbgShiftR {
        BitTxcmdDbgShiftR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn bit_txcmd_dbg_sel(&mut self) -> BitTxcmdDbgSelW<'_, RegWmacDbgSelSpec> {
        BitTxcmdDbgSelW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn bit_txcmd_dbg_shift(&mut self) -> BitTxcmdDbgShiftW<'_, RegWmacDbgSelSpec> {
        BitTxcmdDbgShiftW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacDbgSelSpec;
impl crate::RegisterSpec for RegWmacDbgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_dbg_sel::R`](R) reader structure"]
impl crate::Readable for RegWmacDbgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_dbg_sel::W`](W) writer structure"]
impl crate::Writable for RegWmacDbgSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_DBG_SEL to value 0"]
impl crate::Resettable for RegWmacDbgSelSpec {}
