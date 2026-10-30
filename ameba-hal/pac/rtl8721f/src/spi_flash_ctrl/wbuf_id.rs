#[doc = "Register `WBUF_ID` reader"]
pub type R = crate::R<WbufIdSpec>;
#[doc = "Register `WBUF_ID` writer"]
pub type W = crate::W<WbufIdSpec>;
#[doc = "Field `WBID_ORI` reader - Indicate the awid prior mask are able to push to WBUF in write buffer function. Example: If awid = 0x80 ~ 0x8f are able to push to WBUF. Set WBID_ORI = 0x80 Set WBID_MSK = 0xfff0"]
pub type WbidOriR = crate::FieldReader<u16>;
#[doc = "Field `WBID_ORI` writer - Indicate the awid prior mask are able to push to WBUF in write buffer function. Example: If awid = 0x80 ~ 0x8f are able to push to WBUF. Set WBID_ORI = 0x80 Set WBID_MSK = 0xfff0"]
pub type WbidOriW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WBID_MSK` reader - WBID_ORI bit mask. 0: Interrupt bit is masked. 1: Interrupt bit enable."]
pub type WbidMskR = crate::FieldReader<u16>;
#[doc = "Field `WBID_MSK` writer - WBID_ORI bit mask. 0: Interrupt bit is masked. 1: Interrupt bit enable."]
pub type WbidMskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicate the awid prior mask are able to push to WBUF in write buffer function. Example: If awid = 0x80 ~ 0x8f are able to push to WBUF. Set WBID_ORI = 0x80 Set WBID_MSK = 0xfff0"]
    #[inline(always)]
    pub fn wbid_ori(&self) -> WbidOriR {
        WbidOriR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WBID_ORI bit mask. 0: Interrupt bit is masked. 1: Interrupt bit enable."]
    #[inline(always)]
    pub fn wbid_msk(&self) -> WbidMskR {
        WbidMskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicate the awid prior mask are able to push to WBUF in write buffer function. Example: If awid = 0x80 ~ 0x8f are able to push to WBUF. Set WBID_ORI = 0x80 Set WBID_MSK = 0xfff0"]
    #[inline(always)]
    pub fn wbid_ori(&mut self) -> WbidOriW<'_, WbufIdSpec> {
        WbidOriW::new(self, 0)
    }
    #[doc = "Bits 16:31 - WBID_ORI bit mask. 0: Interrupt bit is masked. 1: Interrupt bit enable."]
    #[inline(always)]
    pub fn wbid_msk(&mut self) -> WbidMskW<'_, WbufIdSpec> {
        WbidMskW::new(self, 16)
    }
}
#[doc = "WBUF ID control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wbuf_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbuf_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbufIdSpec;
impl crate::RegisterSpec for WbufIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wbuf_id::R`](R) reader structure"]
impl crate::Readable for WbufIdSpec {}
#[doc = "`write(|w| ..)` method takes [`wbuf_id::W`](W) writer structure"]
impl crate::Writable for WbufIdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WBUF_ID to value 0"]
impl crate::Resettable for WbufIdSpec {}
