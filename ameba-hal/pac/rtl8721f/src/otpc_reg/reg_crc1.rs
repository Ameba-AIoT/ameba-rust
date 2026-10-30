#[doc = "Register `REG_CRC1` reader"]
pub type R = crate::R<RegCrc1Spec>;
#[doc = "Field `crc_entry2` reader - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
pub type CrcEntry2R = crate::FieldReader<u16>;
#[doc = "Field `crc_entry3` reader - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
pub type CrcEntry3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
    #[inline(always)]
    pub fn crc_entry2(&self) -> CrcEntry2R {
        CrcEntry2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - valid crc Note : this field will be autoloaded twice time, and sticked to L."]
    #[inline(always)]
    pub fn crc_entry3(&self) -> CrcEntry3R {
        CrcEntry3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_crc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCrc1Spec;
impl crate::RegisterSpec for RegCrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_crc1::R`](R) reader structure"]
impl crate::Readable for RegCrc1Spec {}
#[doc = "`reset()` method sets REG_CRC1 to value 0xffff_ffff"]
impl crate::Resettable for RegCrc1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
