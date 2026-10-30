#[doc = "Register `SDIO_DAT_CMD` reader"]
pub type R = crate::R<SdioDatCmdSpec>;
#[doc = "Field `sdio_cmd_dly_disable` reader - sdio_cmd_dly_disable"]
pub type SdioCmdDlyDisableR = crate::FieldReader;
#[doc = "Field `sdio_cmd_duty_en` reader - sdio_cmd_duty_en"]
pub type SdioCmdDutyEnR = crate::BitReader;
#[doc = "Field `sdio_dat_dly_disable` reader - sdio_dat_dly_disable"]
pub type SdioDatDlyDisableR = crate::FieldReader;
#[doc = "Field `sdio_dat_duty_en` reader - sdio_dat_duty_en"]
pub type SdioDatDutyEnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - sdio_cmd_dly_disable"]
    #[inline(always)]
    pub fn sdio_cmd_dly_disable(&self) -> SdioCmdDlyDisableR {
        SdioCmdDlyDisableR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - sdio_cmd_duty_en"]
    #[inline(always)]
    pub fn sdio_cmd_duty_en(&self) -> SdioCmdDutyEnR {
        SdioCmdDutyEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - sdio_dat_dly_disable"]
    #[inline(always)]
    pub fn sdio_dat_dly_disable(&self) -> SdioDatDlyDisableR {
        SdioDatDlyDisableR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - sdio_dat_duty_en"]
    #[inline(always)]
    pub fn sdio_dat_duty_en(&self) -> SdioDatDutyEnR {
        SdioDatDutyEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_dat_cmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioDatCmdSpec;
impl crate::RegisterSpec for SdioDatCmdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdio_dat_cmd::R`](R) reader structure"]
impl crate::Readable for SdioDatCmdSpec {}
#[doc = "`reset()` method sets SDIO_DAT_CMD to value 0"]
impl crate::Resettable for SdioDatCmdSpec {}
