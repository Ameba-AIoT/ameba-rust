#[doc = "Register `IR_VERSION` reader"]
pub type R = crate::R<IrVersionSpec>;
#[doc = "Field `IR_VERSION` reader - IR IP version"]
pub type IrVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IR IP version"]
    #[inline(always)]
    pub fn ir_version(&self) -> IrVersionR {
        IrVersionR::new(self.bits)
    }
}
#[doc = "IR IP version register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrVersionSpec;
impl crate::RegisterSpec for IrVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_version::R`](R) reader structure"]
impl crate::Readable for IrVersionSpec {}
#[doc = "`reset()` method sets IR_VERSION to value 0x1410_150a"]
impl crate::Resettable for IrVersionSpec {
    const RESET_VALUE: u32 = 0x1410_150a;
}
