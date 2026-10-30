#[doc = "Register `REG_RTS_ADDRESS_0_1` reader"]
pub type R = crate::R<RegRtsAddress0_1Spec>;
#[doc = "Register `REG_RTS_ADDRESS_0_1` writer"]
pub type W = crate::W<RegRtsAddress0_1Spec>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR0_1` reader - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr0_1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR0_1` writer - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr0_1(&self) -> BitRWmacRtsAddr0_1R {
        BitRWmacRtsAddr0_1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr0_1(&mut self) -> BitRWmacRtsAddr0_1W<'_, RegRtsAddress0_1Spec> {
        BitRWmacRtsAddr0_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rts_address_0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rts_address_0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtsAddress0_1Spec;
impl crate::RegisterSpec for RegRtsAddress0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rts_address_0_1::R`](R) reader structure"]
impl crate::Readable for RegRtsAddress0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rts_address_0_1::W`](W) writer structure"]
impl crate::Writable for RegRtsAddress0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTS_ADDRESS_0_1 to value 0"]
impl crate::Resettable for RegRtsAddress0_1Spec {}
