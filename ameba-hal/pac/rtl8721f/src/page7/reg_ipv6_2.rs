#[doc = "Register `REG_IPV6_2` reader"]
pub type R = crate::R<RegIpv6_2Spec>;
#[doc = "Register `REG_IPV6_2` writer"]
pub type W = crate::W<RegIpv6_2Spec>;
#[doc = "Field `BIT_IPV6_MYIPAD_2` reader - "]
pub type BitIpv6Myipad2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_IPV6_MYIPAD_2` writer - "]
pub type BitIpv6Myipad2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_ipv6_myipad_2(&self) -> BitIpv6Myipad2R {
        BitIpv6Myipad2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_ipv6_myipad_2(&mut self) -> BitIpv6Myipad2W<'_, RegIpv6_2Spec> {
        BitIpv6Myipad2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ipv6_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ipv6_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIpv6_2Spec;
impl crate::RegisterSpec for RegIpv6_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ipv6_2::R`](R) reader structure"]
impl crate::Readable for RegIpv6_2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ipv6_2::W`](W) writer structure"]
impl crate::Writable for RegIpv6_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_IPV6_2 to value 0"]
impl crate::Resettable for RegIpv6_2Spec {}
