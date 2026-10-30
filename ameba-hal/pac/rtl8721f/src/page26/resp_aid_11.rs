#[doc = "Register `RESP_AID_11` reader"]
pub type R = crate::R<RespAid11Spec>;
#[doc = "Register `RESP_AID_11` writer"]
pub type W = crate::W<RespAid11Spec>;
#[doc = "Field `aid11` reader - specified the aid of the sta that mactx responding to"]
pub type Aid11R = crate::FieldReader<u16>;
#[doc = "Field `aid11` writer - specified the aid of the sta that mactx responding to"]
pub type Aid11W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - specified the aid of the sta that mactx responding to"]
    #[inline(always)]
    pub fn aid11(&self) -> Aid11R {
        Aid11R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - specified the aid of the sta that mactx responding to"]
    #[inline(always)]
    pub fn aid11(&mut self) -> Aid11W<'_, RespAid11Spec> {
        Aid11W::new(self, 0)
    }
}
#[doc = "aid 11 for resp\n\nYou can [`read`](crate::Reg::read) this register and get [`resp_aid_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp_aid_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespAid11Spec;
impl crate::RegisterSpec for RespAid11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp_aid_11::R`](R) reader structure"]
impl crate::Readable for RespAid11Spec {}
#[doc = "`write(|w| ..)` method takes [`resp_aid_11::W`](W) writer structure"]
impl crate::Writable for RespAid11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESP_AID_11 to value 0"]
impl crate::Resettable for RespAid11Spec {}
