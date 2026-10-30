#[doc = "Register `REG_CHINFO_ADDR` reader"]
pub type R = crate::R<RegChinfoAddrSpec>;
#[doc = "Register `REG_CHINFO_ADDR` writer"]
pub type W = crate::W<RegChinfoAddrSpec>;
#[doc = "Field `r_CHINFO_TX_ADDR_l` reader - low 32bit tx address for trigger channel info"]
pub type RChinfoTxAddrLR = crate::FieldReader<u32>;
#[doc = "Field `r_CHINFO_TX_ADDR_l` writer - low 32bit tx address for trigger channel info"]
pub type RChinfoTxAddrLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bit tx address for trigger channel info"]
    #[inline(always)]
    pub fn r_chinfo_tx_addr_l(&self) -> RChinfoTxAddrLR {
        RChinfoTxAddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bit tx address for trigger channel info"]
    #[inline(always)]
    pub fn r_chinfo_tx_addr_l(&mut self) -> RChinfoTxAddrLW<'_, RegChinfoAddrSpec> {
        RChinfoTxAddrLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chinfo_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chinfo_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChinfoAddrSpec;
impl crate::RegisterSpec for RegChinfoAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chinfo_addr::R`](R) reader structure"]
impl crate::Readable for RegChinfoAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_chinfo_addr::W`](W) writer structure"]
impl crate::Writable for RegChinfoAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHINFO_ADDR to value 0"]
impl crate::Resettable for RegChinfoAddrSpec {}
