#[doc = "Register `REG_CRC0` reader"]
pub type R = crate::R<RegCrc0Spec>;
#[doc = "Field `crc_entry0` reader - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
pub type CrcEntry0R = crate::FieldReader<u16>;
#[doc = "Field `crc_entry1` reader - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
pub type CrcEntry1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
    #[inline(always)]
    pub fn crc_entry0(&self) -> CrcEntry0R {
        CrcEntry0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
    #[inline(always)]
    pub fn crc_entry1(&self) -> CrcEntry1R {
        CrcEntry1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_crc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCrc0Spec;
impl crate::RegisterSpec for RegCrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_crc0::R`](R) reader structure"]
impl crate::Readable for RegCrc0Spec {}
#[doc = "`reset()` method sets REG_CRC0 to value 0xffff_ffff"]
impl crate::Resettable for RegCrc0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
