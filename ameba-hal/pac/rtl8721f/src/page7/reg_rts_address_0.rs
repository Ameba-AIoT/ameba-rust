#[doc = "Register `REG_RTS_ADDRESS_0` reader"]
pub type R = crate::R<RegRtsAddress0Spec>;
#[doc = "Register `REG_RTS_ADDRESS_0` writer"]
pub type W = crate::W<RegRtsAddress0Spec>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR0` reader - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_WMAC_RTS_ADDR0` writer - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
pub type BitRWmacRtsAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr0(&self) -> BitRWmacRtsAddr0R {
        BitRWmacRtsAddr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - if 0x604\\[31\\]=1 & rts is multicast, use this value as Address1 of RTS"]
    #[inline(always)]
    pub fn bit_r_wmac_rts_addr0(&mut self) -> BitRWmacRtsAddr0W<'_, RegRtsAddress0Spec> {
        BitRWmacRtsAddr0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rts_address_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rts_address_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtsAddress0Spec;
impl crate::RegisterSpec for RegRtsAddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rts_address_0::R`](R) reader structure"]
impl crate::Readable for RegRtsAddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rts_address_0::W`](W) writer structure"]
impl crate::Writable for RegRtsAddress0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTS_ADDRESS_0 to value 0"]
impl crate::Resettable for RegRtsAddress0Spec {}
