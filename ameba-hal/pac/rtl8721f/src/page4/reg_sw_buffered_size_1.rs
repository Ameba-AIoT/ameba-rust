#[doc = "Register `REG_SW_BUFFERED_SIZE_1` reader"]
pub type R = crate::R<RegSwBufferedSize1Spec>;
#[doc = "Register `REG_SW_BUFFERED_SIZE_1` writer"]
pub type W = crate::W<RegSwBufferedSize1Spec>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_BE` reader - SW update. Buffered packet size of BEQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzBeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_BE` writer - SW update. Buffered packet size of BEQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzBeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_BK` reader - SW update. Buffered packet size of BKQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzBkR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SW_BUFFERED_SZ_BK` writer - SW update. Buffered packet size of BKQ, used to update BSR. Unit: byte"]
pub type BitSwBufferedSzBkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SW update. Buffered packet size of BEQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_be(&self) -> BitSwBufferedSzBeR {
        BitSwBufferedSzBeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SW update. Buffered packet size of BKQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_bk(&self) -> BitSwBufferedSzBkR {
        BitSwBufferedSzBkR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SW update. Buffered packet size of BEQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_be(&mut self) -> BitSwBufferedSzBeW<'_, RegSwBufferedSize1Spec> {
        BitSwBufferedSzBeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SW update. Buffered packet size of BKQ, used to update BSR. Unit: byte"]
    #[inline(always)]
    pub fn bit_sw_buffered_sz_bk(&mut self) -> BitSwBufferedSzBkW<'_, RegSwBufferedSize1Spec> {
        BitSwBufferedSzBkW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_buffered_size_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_buffered_size_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwBufferedSize1Spec;
impl crate::RegisterSpec for RegSwBufferedSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_buffered_size_1::R`](R) reader structure"]
impl crate::Readable for RegSwBufferedSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_buffered_size_1::W`](W) writer structure"]
impl crate::Writable for RegSwBufferedSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_BUFFERED_SIZE_1 to value 0"]
impl crate::Resettable for RegSwBufferedSize1Spec {}
