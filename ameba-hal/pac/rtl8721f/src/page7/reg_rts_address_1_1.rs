#[doc = "Register `REG_RTS_ADDRESS_1_1` reader"]
pub type R = crate::R<RegRtsAddress1_1Spec>;
#[doc = "Register `REG_RTS_ADDRESS_1_1` writer"]
pub type W = crate::W<RegRtsAddress1_1Spec>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR1_1` reader - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr1_1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR1_1` writer - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr1_1(&self) -> BitRWmacRtsAddr1_1R {
        BitRWmacRtsAddr1_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr1_1(&mut self) -> BitRWmacRtsAddr1_1W<'_, RegRtsAddress1_1Spec> {
        BitRWmacRtsAddr1_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegRtsAddress1_1Spec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rts_address_1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rts_address_1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtsAddress1_1Spec;
impl crate::RegisterSpec for RegRtsAddress1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rts_address_1_1::R`](R) reader structure"]
impl crate::Readable for RegRtsAddress1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rts_address_1_1::W`](W) writer structure"]
impl crate::Writable for RegRtsAddress1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTS_ADDRESS_1_1 to value 0"]
impl crate::Resettable for RegRtsAddress1_1Spec {}
