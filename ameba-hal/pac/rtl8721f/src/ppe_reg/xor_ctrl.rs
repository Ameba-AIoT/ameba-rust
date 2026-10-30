#[doc = "Register `XOR_CTRL` reader"]
pub type R = crate::R<XorCtrlSpec>;
#[doc = "Register `XOR_CTRL` writer"]
pub type W = crate::W<XorCtrlSpec>;
#[doc = "Field `XOR` reader - 0x1: Enable XOR function 0x0: Disable XOR function"]
pub type XorR = crate::BitReader;
#[doc = "Field `XOR` writer - 0x1: Enable XOR function 0x0: Disable XOR function"]
pub type XorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1: Enable XOR function 0x0: Disable XOR function"]
    #[inline(always)]
    pub fn xor(&self) -> XorR {
        XorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1: Enable XOR function 0x0: Disable XOR function"]
    #[inline(always)]
    pub fn xor(&mut self) -> XorW<'_, XorCtrlSpec> {
        XorW::new(self, 0)
    }
}
#[doc = "XOR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`xor_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xor_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XorCtrlSpec;
impl crate::RegisterSpec for XorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xor_ctrl::R`](R) reader structure"]
impl crate::Readable for XorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xor_ctrl::W`](W) writer structure"]
impl crate::Writable for XorCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOR_CTRL to value 0x01"]
impl crate::Resettable for XorCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
