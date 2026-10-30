#[doc = "Register `REG_RXCMD_0` reader"]
pub type R = crate::R<RegRxcmd0Spec>;
#[doc = "Register `REG_RXCMD_0` writer"]
pub type W = crate::W<RegRxcmd0Spec>;
#[doc = "Field `BIT_RXCMD_INFO` reader - The Information is send to rxpktbuffer for host SW"]
pub type BitRxcmdInfoR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXCMD_INFO` writer - The Information is send to rxpktbuffer for host SW"]
pub type BitRxcmdInfoW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `BIT_RXCMD_EN` reader - RXCMD function enable"]
pub type BitRxcmdEnR = crate::BitReader;
#[doc = "Field `BIT_RXCMD_EN` writer - RXCMD function enable"]
pub type BitRxcmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - The Information is send to rxpktbuffer for host SW"]
    #[inline(always)]
    pub fn bit_rxcmd_info(&self) -> BitRxcmdInfoR {
        BitRxcmdInfoR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - RXCMD function enable"]
    #[inline(always)]
    pub fn bit_rxcmd_en(&self) -> BitRxcmdEnR {
        BitRxcmdEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - The Information is send to rxpktbuffer for host SW"]
    #[inline(always)]
    pub fn bit_rxcmd_info(&mut self) -> BitRxcmdInfoW<'_, RegRxcmd0Spec> {
        BitRxcmdInfoW::new(self, 0)
    }
    #[doc = "Bit 31 - RXCMD function enable"]
    #[inline(always)]
    pub fn bit_rxcmd_en(&mut self) -> BitRxcmdEnW<'_, RegRxcmd0Spec> {
        BitRxcmdEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxcmd_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxcmd_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxcmd0Spec;
impl crate::RegisterSpec for RegRxcmd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxcmd_0::R`](R) reader structure"]
impl crate::Readable for RegRxcmd0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxcmd_0::W`](W) writer structure"]
impl crate::Writable for RegRxcmd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXCMD_0 to value 0"]
impl crate::Resettable for RegRxcmd0Spec {}
