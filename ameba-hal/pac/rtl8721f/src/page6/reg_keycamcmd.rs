#[doc = "Register `REG_KEYCAMCMD` reader"]
pub type R = crate::R<RegKeycamcmdSpec>;
#[doc = "Register `REG_KEYCAMCMD` writer"]
pub type W = crate::W<RegKeycamcmdSpec>;
#[doc = "Field `BIT_KEYCAM_AD` reader - Security CAM Address Offset CAM address is organized into 8 double-words/entry."]
pub type BitKeycamAdR = crate::FieldReader<u16>;
#[doc = "Field `BIT_KEYCAM_AD` writer - Security CAM Address Offset CAM address is organized into 8 double-words/entry."]
pub type BitKeycamAdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BIT_SECCAM_WE` reader - Security CAM Write Enable"]
pub type BitSeccamWeR = crate::BitReader;
#[doc = "Field `BIT_SECCAM_WE` writer - Security CAM Write Enable"]
pub type BitSeccamWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_KEYCAM_DA_EN` reader - Security CAM Direct Acess Enable"]
pub type BitKeycamDaEnR = crate::BitReader;
#[doc = "Field `BIT_KEYCAM_DA_EN` writer - Security CAM Direct Acess Enable"]
pub type BitKeycamDaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MFBCAM_CLEAR` reader - Write 1 to clear all MFB value in CAM to 7'h7F. After reset will self clear to 0"]
pub type BitMfbcamClearR = crate::BitReader;
#[doc = "Field `BIT_KEYCAM_CLEAR` reader - Set to 1 to clear all valid bits in CAM. After reset will self clear to 0."]
pub type BitKeycamClearR = crate::BitReader;
#[doc = "Field `BIT_KEYCAM_POLLING` reader - Security CAM Polling"]
pub type BitKeycamPollingR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Security CAM Address Offset CAM address is organized into 8 double-words/entry."]
    #[inline(always)]
    pub fn bit_keycam_ad(&self) -> BitKeycamAdR {
        BitKeycamAdR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Security CAM Write Enable"]
    #[inline(always)]
    pub fn bit_seccam_we(&self) -> BitSeccamWeR {
        BitSeccamWeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security CAM Direct Acess Enable"]
    #[inline(always)]
    pub fn bit_keycam_da_en(&self) -> BitKeycamDaEnR {
        BitKeycamDaEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Write 1 to clear all MFB value in CAM to 7'h7F. After reset will self clear to 0"]
    #[inline(always)]
    pub fn bit_mfbcam_clear(&self) -> BitMfbcamClearR {
        BitMfbcamClearR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set to 1 to clear all valid bits in CAM. After reset will self clear to 0."]
    #[inline(always)]
    pub fn bit_keycam_clear(&self) -> BitKeycamClearR {
        BitKeycamClearR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security CAM Polling"]
    #[inline(always)]
    pub fn bit_keycam_polling(&self) -> BitKeycamPollingR {
        BitKeycamPollingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Security CAM Address Offset CAM address is organized into 8 double-words/entry."]
    #[inline(always)]
    pub fn bit_keycam_ad(&mut self) -> BitKeycamAdW<'_, RegKeycamcmdSpec> {
        BitKeycamAdW::new(self, 0)
    }
    #[doc = "Bit 16 - Security CAM Write Enable"]
    #[inline(always)]
    pub fn bit_seccam_we(&mut self) -> BitSeccamWeW<'_, RegKeycamcmdSpec> {
        BitSeccamWeW::new(self, 16)
    }
    #[doc = "Bit 17 - Security CAM Direct Acess Enable"]
    #[inline(always)]
    pub fn bit_keycam_da_en(&mut self) -> BitKeycamDaEnW<'_, RegKeycamcmdSpec> {
        BitKeycamDaEnW::new(self, 17)
    }
}
#[doc = "CAM Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycamcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycamcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycamcmdSpec;
impl crate::RegisterSpec for RegKeycamcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycamcmd::R`](R) reader structure"]
impl crate::Readable for RegKeycamcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycamcmd::W`](W) writer structure"]
impl crate::Writable for RegKeycamcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAMCMD to value 0x0002_0000"]
impl crate::Resettable for RegKeycamcmdSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
