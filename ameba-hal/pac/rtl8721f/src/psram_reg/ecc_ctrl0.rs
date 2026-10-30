#[doc = "Register `ECC_CTRL0` reader"]
pub type R = crate::R<EccCtrl0Spec>;
#[doc = "Register `ECC_CTRL0` writer"]
pub type W = crate::W<EccCtrl0Spec>;
#[doc = "Field `PAGE_CMD` reader - NAND page command setting. Ex. READ: Page Data Read (13h) WRITE: Page Execute (10h) Exists: TAG >= 2.0.11"]
pub type PageCmdR = crate::FieldReader;
#[doc = "Field `PAGE_CMD` writer - NAND page command setting. Ex. READ: Page Data Read (13h) WRITE: Page Execute (10h) Exists: TAG >= 2.0.11"]
pub type PageCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAGE_ADDR` reader - NAND Page address setting. Exists: TAG >= 2.0.11"]
pub type PageAddrR = crate::FieldReader<u32>;
#[doc = "Field `PAGE_ADDR` writer - NAND Page address setting. Exists: TAG >= 2.0.11"]
pub type PageAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - NAND page command setting. Ex. READ: Page Data Read (13h) WRITE: Page Execute (10h) Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn page_cmd(&self) -> PageCmdR {
        PageCmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - NAND Page address setting. Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn page_addr(&self) -> PageAddrR {
        PageAddrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAND page command setting. Ex. READ: Page Data Read (13h) WRITE: Page Execute (10h) Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn page_cmd(&mut self) -> PageCmdW<'_, EccCtrl0Spec> {
        PageCmdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - NAND Page address setting. Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn page_addr(&mut self) -> PageAddrW<'_, EccCtrl0Spec> {
        PageAddrW::new(self, 8)
    }
}
#[doc = "NAND ECC control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtrl0Spec;
impl crate::RegisterSpec for EccCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctrl0::R`](R) reader structure"]
impl crate::Readable for EccCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl0::W`](W) writer structure"]
impl crate::Writable for EccCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_CTRL0 to value 0"]
impl crate::Resettable for EccCtrl0Spec {}
