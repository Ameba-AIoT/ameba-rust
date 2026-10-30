#[doc = "Register `REG_RESP_TXINFO_CFG` reader"]
pub type R = crate::R<RegRespTxinfoCfgSpec>;
#[doc = "Register `REG_RESP_TXINFO_CFG` writer"]
pub type W = crate::W<RegRespTxinfoCfgSpec>;
#[doc = "Field `BIT_RESP_TXANT` reader - tx antenna information"]
pub type BitRespTxantR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RESP_TXANT` writer - tx antenna information"]
pub type BitRespTxantW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `BIT_RESP_TXPOWER` reader - tx power setting when send response packet"]
pub type BitRespTxpowerR = crate::FieldReader;
#[doc = "Field `BIT_RESP_TXPOWER` writer - tx power setting when send response packet"]
pub type BitRespTxpowerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_ANTSEL_SEL` reader - antenna select 0: use rx phystatus antsel information; 1: use tx report antsel information; other: use register antsel information."]
pub type BitAntselSelR = crate::FieldReader;
#[doc = "Field `BIT_ANTSEL_SEL` writer - antenna select 0: use rx phystatus antsel information; 1: use tx report antsel information; other: use register antsel information."]
pub type BitAntselSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_ANTINF_SEL` reader - Antsel information for response frame in register or ctrl info 00: Antsel_en always disable, 01:Antsel_en field follow ctrl info 1X:Antsel_en always enable"]
pub type BitAntinfSelR = crate::FieldReader;
#[doc = "Field `BIT_ANTINF_SEL` writer - Antsel information for response frame in register or ctrl info 00: Antsel_en always disable, 01:Antsel_en field follow ctrl info 1X:Antsel_en always enable"]
pub type BitAntinfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RESP_MFB` reader - MFB information in register for response packet"]
pub type BitRespMfbR = crate::FieldReader;
#[doc = "Field `BIT_RESP_MFB` writer - MFB information in register for response packet"]
pub type BitRespMfbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:17 - tx antenna information"]
    #[inline(always)]
    pub fn bit_resp_txant(&self) -> BitRespTxantR {
        BitRespTxantR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:20 - tx power setting when send response packet"]
    #[inline(always)]
    pub fn bit_resp_txpower(&self) -> BitRespTxpowerR {
        BitRespTxpowerR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - antenna select 0: use rx phystatus antsel information; 1: use tx report antsel information; other: use register antsel information."]
    #[inline(always)]
    pub fn bit_antsel_sel(&self) -> BitAntselSelR {
        BitAntselSelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Antsel information for response frame in register or ctrl info 00: Antsel_en always disable, 01:Antsel_en field follow ctrl info 1X:Antsel_en always enable"]
    #[inline(always)]
    pub fn bit_antinf_sel(&self) -> BitAntinfSelR {
        BitAntinfSelR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MFB information in register for response packet"]
    #[inline(always)]
    pub fn bit_resp_mfb(&self) -> BitRespMfbR {
        BitRespMfbR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - tx antenna information"]
    #[inline(always)]
    pub fn bit_resp_txant(&mut self) -> BitRespTxantW<'_, RegRespTxinfoCfgSpec> {
        BitRespTxantW::new(self, 0)
    }
    #[doc = "Bits 18:20 - tx power setting when send response packet"]
    #[inline(always)]
    pub fn bit_resp_txpower(&mut self) -> BitRespTxpowerW<'_, RegRespTxinfoCfgSpec> {
        BitRespTxpowerW::new(self, 18)
    }
    #[doc = "Bits 21:22 - antenna select 0: use rx phystatus antsel information; 1: use tx report antsel information; other: use register antsel information."]
    #[inline(always)]
    pub fn bit_antsel_sel(&mut self) -> BitAntselSelW<'_, RegRespTxinfoCfgSpec> {
        BitAntselSelW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Antsel information for response frame in register or ctrl info 00: Antsel_en always disable, 01:Antsel_en field follow ctrl info 1X:Antsel_en always enable"]
    #[inline(always)]
    pub fn bit_antinf_sel(&mut self) -> BitAntinfSelW<'_, RegRespTxinfoCfgSpec> {
        BitAntinfSelW::new(self, 23)
    }
    #[doc = "Bits 25:31 - MFB information in register for response packet"]
    #[inline(always)]
    pub fn bit_resp_mfb(&mut self) -> BitRespMfbW<'_, RegRespTxinfoCfgSpec> {
        BitRespMfbW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_resp_txinfo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_resp_txinfo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRespTxinfoCfgSpec;
impl crate::RegisterSpec for RegRespTxinfoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_resp_txinfo_cfg::R`](R) reader structure"]
impl crate::Readable for RegRespTxinfoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_resp_txinfo_cfg::W`](W) writer structure"]
impl crate::Writable for RegRespTxinfoCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RESP_TXINFO_CFG to value 0x3f"]
impl crate::Resettable for RegRespTxinfoCfgSpec {
    const RESET_VALUE: u32 = 0x3f;
}
