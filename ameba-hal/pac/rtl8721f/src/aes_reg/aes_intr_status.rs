#[doc = "Register `AES_Intr_status` reader"]
pub type R = crate::R<AesIntrStatusSpec>;
#[doc = "Field `chn0_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 0."]
pub type Chn0AesIntStatusR = crate::BitReader;
#[doc = "Field `chn1_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 1."]
pub type Chn1AesIntStatusR = crate::BitReader;
#[doc = "Field `chn2_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 2."]
pub type Chn2AesIntStatusR = crate::BitReader;
#[doc = "Field `chn3_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 3."]
pub type Chn3AesIntStatusR = crate::BitReader;
#[doc = "Field `chn4_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 4."]
pub type Chn4AesIntStatusR = crate::BitReader;
#[doc = "Field `chn5_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 5."]
pub type Chn5AesIntStatusR = crate::BitReader;
#[doc = "Field `chn6_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 6."]
pub type Chn6AesIntStatusR = crate::BitReader;
#[doc = "Field `chn7_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 7."]
pub type Chn7AesIntStatusR = crate::BitReader;
#[doc = "Field `chn8_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 8."]
pub type Chn8AesIntStatusR = crate::BitReader;
#[doc = "Field `chn9_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 9."]
pub type Chn9AesIntStatusR = crate::BitReader;
#[doc = "Field `chn10_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 10."]
pub type Chn10AesIntStatusR = crate::BitReader;
#[doc = "Field `chn11_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 11."]
pub type Chn11AesIntStatusR = crate::BitReader;
#[doc = "Field `chn12_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 12."]
pub type Chn12AesIntStatusR = crate::BitReader;
#[doc = "Field `chn13_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 13."]
pub type Chn13AesIntStatusR = crate::BitReader;
#[doc = "Field `chn14_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 14."]
pub type Chn14AesIntStatusR = crate::BitReader;
#[doc = "Field `chn15_aes_int_status` reader - Set to 1 when any AES interrupt occurs on channel 15."]
pub type Chn15AesIntStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set to 1 when any AES interrupt occurs on channel 0."]
    #[inline(always)]
    pub fn chn0_aes_int_status(&self) -> Chn0AesIntStatusR {
        Chn0AesIntStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 when any AES interrupt occurs on channel 1."]
    #[inline(always)]
    pub fn chn1_aes_int_status(&self) -> Chn1AesIntStatusR {
        Chn1AesIntStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 when any AES interrupt occurs on channel 2."]
    #[inline(always)]
    pub fn chn2_aes_int_status(&self) -> Chn2AesIntStatusR {
        Chn2AesIntStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 when any AES interrupt occurs on channel 3."]
    #[inline(always)]
    pub fn chn3_aes_int_status(&self) -> Chn3AesIntStatusR {
        Chn3AesIntStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to 1 when any AES interrupt occurs on channel 4."]
    #[inline(always)]
    pub fn chn4_aes_int_status(&self) -> Chn4AesIntStatusR {
        Chn4AesIntStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set to 1 when any AES interrupt occurs on channel 5."]
    #[inline(always)]
    pub fn chn5_aes_int_status(&self) -> Chn5AesIntStatusR {
        Chn5AesIntStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set to 1 when any AES interrupt occurs on channel 6."]
    #[inline(always)]
    pub fn chn6_aes_int_status(&self) -> Chn6AesIntStatusR {
        Chn6AesIntStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set to 1 when any AES interrupt occurs on channel 7."]
    #[inline(always)]
    pub fn chn7_aes_int_status(&self) -> Chn7AesIntStatusR {
        Chn7AesIntStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to 1 when any AES interrupt occurs on channel 8."]
    #[inline(always)]
    pub fn chn8_aes_int_status(&self) -> Chn8AesIntStatusR {
        Chn8AesIntStatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set to 1 when any AES interrupt occurs on channel 9."]
    #[inline(always)]
    pub fn chn9_aes_int_status(&self) -> Chn9AesIntStatusR {
        Chn9AesIntStatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set to 1 when any AES interrupt occurs on channel 10."]
    #[inline(always)]
    pub fn chn10_aes_int_status(&self) -> Chn10AesIntStatusR {
        Chn10AesIntStatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set to 1 when any AES interrupt occurs on channel 11."]
    #[inline(always)]
    pub fn chn11_aes_int_status(&self) -> Chn11AesIntStatusR {
        Chn11AesIntStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set to 1 when any AES interrupt occurs on channel 12."]
    #[inline(always)]
    pub fn chn12_aes_int_status(&self) -> Chn12AesIntStatusR {
        Chn12AesIntStatusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set to 1 when any AES interrupt occurs on channel 13."]
    #[inline(always)]
    pub fn chn13_aes_int_status(&self) -> Chn13AesIntStatusR {
        Chn13AesIntStatusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set to 1 when any AES interrupt occurs on channel 14."]
    #[inline(always)]
    pub fn chn14_aes_int_status(&self) -> Chn14AesIntStatusR {
        Chn14AesIntStatusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set to 1 when any AES interrupt occurs on channel 15."]
    #[inline(always)]
    pub fn chn15_aes_int_status(&self) -> Chn15AesIntStatusR {
        Chn15AesIntStatusR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_intr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIntrStatusSpec;
impl crate::RegisterSpec for AesIntrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_intr_status::R`](R) reader structure"]
impl crate::Readable for AesIntrStatusSpec {}
#[doc = "`reset()` method sets AES_Intr_status to value 0"]
impl crate::Resettable for AesIntrStatusSpec {}
