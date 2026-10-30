#[doc = "Register `REG_RXCMD_1` reader"]
pub type R = crate::R<RegRxcmd1Spec>;
#[doc = "Register `REG_RXCMD_1` writer"]
pub type W = crate::W<RegRxcmd1Spec>;
#[doc = "Field `BIT_RXCMD_PRD` reader - Define the period unit in us to send the RXCMD_INFO to rxpktbuffer for host SW"]
pub type BitRxcmdPrdR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXCMD_PRD` writer - Define the period unit in us to send the RXCMD_INFO to rxpktbuffer for host SW"]
pub type BitRxcmdPrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Define the period unit in us to send the RXCMD_INFO to rxpktbuffer for host SW"]
    #[inline(always)]
    pub fn bit_rxcmd_prd(&self) -> BitRxcmdPrdR {
        BitRxcmdPrdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Define the period unit in us to send the RXCMD_INFO to rxpktbuffer for host SW"]
    #[inline(always)]
    pub fn bit_rxcmd_prd(&mut self) -> BitRxcmdPrdW<'_, RegRxcmd1Spec> {
        BitRxcmdPrdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxcmd_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxcmd_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxcmd1Spec;
impl crate::RegisterSpec for RegRxcmd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxcmd_1::R`](R) reader structure"]
impl crate::Readable for RegRxcmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxcmd_1::W`](W) writer structure"]
impl crate::Writable for RegRxcmd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXCMD_1 to value 0"]
impl crate::Resettable for RegRxcmd1Spec {}
