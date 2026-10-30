#[doc = "Register `ECC_CTRL1` reader"]
pub type R = crate::R<EccCtrl1Spec>;
#[doc = "Register `ECC_CTRL1` writer"]
pub type W = crate::W<EccCtrl1Spec>;
#[doc = "Field `DATA_CMD` reader - NAND data command setting. Ex. READ: Read data (03h) WRITE: Load Program Data (02h) Exists: TAG >= 2.0.11"]
pub type DataCmdR = crate::FieldReader;
#[doc = "Field `DATA_CMD` writer - NAND data command setting. Ex. READ: Read data (03h) WRITE: Load Program Data (02h) Exists: TAG >= 2.0.11"]
pub type DataCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NAND data command setting. Ex. READ: Read data (03h) WRITE: Load Program Data (02h) Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn data_cmd(&self) -> DataCmdR {
        DataCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAND data command setting. Ex. READ: Read data (03h) WRITE: Load Program Data (02h) Exists: TAG >= 2.0.11"]
    #[inline(always)]
    pub fn data_cmd(&mut self) -> DataCmdW<'_, EccCtrl1Spec> {
        DataCmdW::new(self, 0)
    }
}
#[doc = "NAND ECC control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtrl1Spec;
impl crate::RegisterSpec for EccCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctrl1::R`](R) reader structure"]
impl crate::Readable for EccCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl1::W`](W) writer structure"]
impl crate::Writable for EccCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_CTRL1 to value 0"]
impl crate::Resettable for EccCtrl1Spec {}
