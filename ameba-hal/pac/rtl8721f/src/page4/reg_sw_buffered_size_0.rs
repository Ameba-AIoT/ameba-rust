#[doc = "Register `REG_SW_BUFFERED_SIZE_0` reader"]
pub type R = crate::R<RegSwBufferedSize0Spec>;
#[doc = "Register `REG_SW_BUFFERED_SIZE_0` writer"]
pub type W = crate::W<RegSwBufferedSize0Spec>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_VO` reader - SW update. Buffered packet size of VOQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzVoR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_VO` writer - SW update. Buffered packet size of VOQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzVoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_VI` reader - SW update. Buffered packet size of VIQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzViR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_VI` writer - SW update. Buffered packet size of VIQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzViW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SW update. Buffered packet size of VOQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_vo(&self) -> BitSwBufferedSzVoR {
        BitSwBufferedSzVoR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SW update. Buffered packet size of VIQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_vi(&self) -> BitSwBufferedSzViR {
        BitSwBufferedSzViR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SW update. Buffered packet size of VOQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_vo(&mut self) -> BitSwBufferedSzVoW<'_, RegSwBufferedSize0Spec> {
        BitSwBufferedSzVoW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SW update. Buffered packet size of VIQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_vi(&mut self) -> BitSwBufferedSzViW<'_, RegSwBufferedSize0Spec> {
        BitSwBufferedSzViW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_buffered_size_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_buffered_size_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwBufferedSize0Spec;
impl crate::RegisterSpec for RegSwBufferedSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_buffered_size_0::R`](R) reader structure"]
impl crate::Readable for RegSwBufferedSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_buffered_size_0::W`](W) writer structure"]
impl crate::Writable for RegSwBufferedSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_BUFFERED_SIZE_0 to value 0"]
impl crate::Resettable for RegSwBufferedSize0Spec {}
