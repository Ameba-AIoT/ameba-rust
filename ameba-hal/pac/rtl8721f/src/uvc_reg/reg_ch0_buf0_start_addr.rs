#[doc = "Register `REG_CH0_BUF0_START_ADDR` reader"]
pub type R = crate::R<RegCh0Buf0StartAddrSpec>;
#[doc = "Register `REG_CH0_BUF0_START_ADDR` writer"]
pub type W = crate::W<RegCh0Buf0StartAddrSpec>;
#[doc = "Field `ch0_buf0_start_addr` reader - "]
pub type Ch0Buf0StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch0_buf0_start_addr` writer - "]
pub type Ch0Buf0StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_start_addr(&self) -> Ch0Buf0StartAddrR {
        Ch0Buf0StartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_start_addr(&mut self) -> Ch0Buf0StartAddrW<'_, RegCh0Buf0StartAddrSpec> {
        Ch0Buf0StartAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_buf0_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0StartAddrSpec;
impl crate::RegisterSpec for RegCh0Buf0StartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_start_addr::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0StartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_buf0_start_addr::W`](W) writer structure"]
impl crate::Writable for RegCh0Buf0StartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_BUF0_START_ADDR to value 0"]
impl crate::Resettable for RegCh0Buf0StartAddrSpec {}
