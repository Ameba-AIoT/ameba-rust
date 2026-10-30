#[doc = "Register `REG_CH0_BUF1_END_ADDR` reader"]
pub type R = crate::R<RegCh0Buf1EndAddrSpec>;
#[doc = "Register `REG_CH0_BUF1_END_ADDR` writer"]
pub type W = crate::W<RegCh0Buf1EndAddrSpec>;
#[doc = "Field `ch0_buf1_end_addr` reader - "]
pub type Ch0Buf1EndAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch0_buf1_end_addr` writer - "]
pub type Ch0Buf1EndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf1_end_addr(&self) -> Ch0Buf1EndAddrR {
        Ch0Buf1EndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf1_end_addr(&mut self) -> Ch0Buf1EndAddrW<'_, RegCh0Buf1EndAddrSpec> {
        Ch0Buf1EndAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf1_end_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_buf1_end_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf1EndAddrSpec;
impl crate::RegisterSpec for RegCh0Buf1EndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf1_end_addr::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf1EndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_buf1_end_addr::W`](W) writer structure"]
impl crate::Writable for RegCh0Buf1EndAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_BUF1_END_ADDR to value 0"]
impl crate::Resettable for RegCh0Buf1EndAddrSpec {}
