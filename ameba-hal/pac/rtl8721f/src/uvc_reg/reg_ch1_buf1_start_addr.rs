#[doc = "Register `REG_CH1_BUF1_START_ADDR` reader"]
pub type R = crate::R<RegCh1Buf1StartAddrSpec>;
#[doc = "Register `REG_CH1_BUF1_START_ADDR` writer"]
pub type W = crate::W<RegCh1Buf1StartAddrSpec>;
#[doc = "Field `ch1_buf1_start_addr` reader - "]
pub type Ch1Buf1StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `ch1_buf1_start_addr` writer - "]
pub type Ch1Buf1StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_buf1_start_addr(&self) -> Ch1Buf1StartAddrR {
        Ch1Buf1StartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_buf1_start_addr(&mut self) -> Ch1Buf1StartAddrW<'_, RegCh1Buf1StartAddrSpec> {
        Ch1Buf1StartAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_buf1_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch1_buf1_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Buf1StartAddrSpec;
impl crate::RegisterSpec for RegCh1Buf1StartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_buf1_start_addr::R`](R) reader structure"]
impl crate::Readable for RegCh1Buf1StartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch1_buf1_start_addr::W`](W) writer structure"]
impl crate::Writable for RegCh1Buf1StartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH1_BUF1_START_ADDR to value 0"]
impl crate::Resettable for RegCh1Buf1StartAddrSpec {}
