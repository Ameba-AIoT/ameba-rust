#[doc = "Register `REG_IPV6_3` reader"]
pub type R = crate::R<RegIpv6_3Spec>;
#[doc = "Register `REG_IPV6_3` writer"]
pub type W = crate::W<RegIpv6_3Spec>;
#[doc = "Field `BIT_IPV6_MYIPAD_3` reader - "]
pub type BitIpv6Myipad3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_IPV6_MYIPAD_3` writer - "]
pub type BitIpv6Myipad3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_ipv6_myipad_3(&self) -> BitIpv6Myipad3R {
        BitIpv6Myipad3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_ipv6_myipad_3(&mut self) -> BitIpv6Myipad3W<'_, RegIpv6_3Spec> {
        BitIpv6Myipad3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ipv6_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ipv6_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIpv6_3Spec;
impl crate::RegisterSpec for RegIpv6_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ipv6_3::R`](R) reader structure"]
impl crate::Readable for RegIpv6_3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ipv6_3::W`](W) writer structure"]
impl crate::Writable for RegIpv6_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_IPV6_3 to value 0"]
impl crate::Resettable for RegIpv6_3Spec {}
