#[doc = "Register `REG_SIFS` reader"]
pub type R = crate::R<RegSifsSpec>;
#[doc = "Register `REG_SIFS` writer"]
pub type W = crate::W<RegSifsSpec>;
#[doc = "Field `BIT_SIFS_CCK_CTX` reader - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
pub type BitSifsCckCtxR = crate::FieldReader;
#[doc = "Field `BIT_SIFS_CCK_CTX` writer - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
pub type BitSifsCckCtxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SIFS_OFDM_CTX` reader - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
pub type BitSifsOfdmCtxR = crate::FieldReader;
#[doc = "Field `BIT_SIFS_OFDM_CTX` writer - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
pub type BitSifsOfdmCtxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SIFS_CCK_TRX` reader - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
pub type BitSifsCckTrxR = crate::FieldReader;
#[doc = "Field `BIT_SIFS_CCK_TRX` writer - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
pub type BitSifsCckTrxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SIFS_OFDM_TRX` reader - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
pub type BitSifsOfdmTrxR = crate::FieldReader;
#[doc = "Field `BIT_SIFS_OFDM_TRX` writer - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
pub type BitSifsOfdmTrxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_cck_ctx(&self) -> BitSifsCckCtxR {
        BitSifsCckCtxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_ofdm_ctx(&self) -> BitSifsOfdmCtxR {
        BitSifsOfdmCtxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_cck_trx(&self) -> BitSifsCckTrxR {
        BitSifsCckTrxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_ofdm_trx(&self) -> BitSifsOfdmTrxR {
        BitSifsOfdmTrxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_cck_ctx(&mut self) -> BitSifsCckCtxW<'_, RegSifsSpec> {
        BitSifsCckCtxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register sets the SIFS time required for consecutive TX events for example CTS2S-Data, or broadcast bursting. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_ofdm_ctx(&mut self) -> BitSifsOfdmCtxW<'_, RegSifsSpec> {
        BitSifsOfdmCtxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_cck_trx(&mut self) -> BitSifsCckTrxW<'_, RegSifsSpec> {
        BitSifsCckTrxW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This register sets the SIFS time required in NAV protection period after receiving any immediate response. for example Ack-Data or RTS/CTS-Data. It is in unit of us."]
    #[inline(always)]
    pub fn bit_sifs_ofdm_trx(&mut self) -> BitSifsOfdmTrxW<'_, RegSifsSpec> {
        BitSifsOfdmTrxW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sifs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sifs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSifsSpec;
impl crate::RegisterSpec for RegSifsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sifs::R`](R) reader structure"]
impl crate::Readable for RegSifsSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sifs::W`](W) writer structure"]
impl crate::Writable for RegSifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SIFS to value 0x0e0a_0e0a"]
impl crate::Resettable for RegSifsSpec {
    const RESET_VALUE: u32 = 0x0e0a_0e0a;
}
