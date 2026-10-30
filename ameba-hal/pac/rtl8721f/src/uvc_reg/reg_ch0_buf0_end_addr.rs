#[doc = "Register `REG_CH0_BUF0_END_ADDR` reader"]
pub type R = crate::R<RegCh0Buf0EndAddrSpec>;
#[doc = "Register `REG_CH0_BUF0_END_ADDR` writer"]
pub type W = crate::W<RegCh0Buf0EndAddrSpec>;
#[doc = "Field `ch0_buf0_end_addr` reader - "]
pub type Ch0Buf0EndAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch0_buf0_end_addr` writer - "]
pub type Ch0Buf0EndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_end_addr(&self) -> Ch0Buf0EndAddrR {
        Ch0Buf0EndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_end_addr(&mut self) -> Ch0Buf0EndAddrW<'_, RegCh0Buf0EndAddrSpec> {
        Ch0Buf0EndAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_end_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_buf0_end_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0EndAddrSpec;
impl crate::RegisterSpec for RegCh0Buf0EndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_end_addr::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0EndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_buf0_end_addr::W`](W) writer structure"]
impl crate::Writable for RegCh0Buf0EndAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_BUF0_END_ADDR to value 0"]
impl crate::Resettable for RegCh0Buf0EndAddrSpec {}
