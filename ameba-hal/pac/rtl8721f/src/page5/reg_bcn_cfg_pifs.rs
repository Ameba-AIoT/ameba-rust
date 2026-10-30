#[doc = "Register `REG_BCN_CFG_PIFS` reader"]
pub type R = crate::R<RegBcnCfgPifsSpec>;
#[doc = "Register `REG_BCN_CFG_PIFS` writer"]
pub type W = crate::W<RegBcnCfgPifsSpec>;
#[doc = "Field `BIT_BCNIFS` reader - Beacon Interframe Space In units of microseconds, this register defines the medium idle time required before transmitting beacon (AP mode) or entering random back off window."]
pub type BitBcnifsR = crate::FieldReader;
#[doc = "Field `BIT_BCNIFS` writer - Beacon Interframe Space In units of microseconds, this register defines the medium idle time required before transmitting beacon (AP mode) or entering random back off window."]
pub type BitBcnifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNCW_MIN` reader - Beacon Contention Window Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
pub type BitBcncwMinR = crate::FieldReader;
#[doc = "Field `BIT_BCNCW_MIN` writer - Beacon Contention Window Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
pub type BitBcncwMinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_BCNCW_MAX` reader - Beacon Contention Window MAX Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
pub type BitBcncwMaxR = crate::FieldReader;
#[doc = "Field `BIT_BCNCW_MAX` writer - Beacon Contention Window MAX Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
pub type BitBcncwMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_PIFS` reader - PIFS time. It is only used in AP mode and in units of us."]
pub type BitPifsR = crate::FieldReader;
#[doc = "Field `BIT_PIFS` writer - PIFS time. It is only used in AP mode and in units of us."]
pub type BitPifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RDG_PIFS` reader - PIFS time. It is only used in RDG mode and in units of us."]
pub type BitRdgPifsR = crate::FieldReader;
#[doc = "Field `BIT_RDG_PIFS` writer - PIFS time. It is only used in RDG mode and in units of us."]
pub type BitRdgPifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Beacon Interframe Space In units of microseconds, this register defines the medium idle time required before transmitting beacon (AP mode) or entering random back off window."]
    #[inline(always)]
    pub fn bit_bcnifs(&self) -> BitBcnifsR {
        BitBcnifsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Beacon Contention Window Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
    #[inline(always)]
    pub fn bit_bcncw_min(&self) -> BitBcncwMinR {
        BitBcncwMinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Beacon Contention Window MAX Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
    #[inline(always)]
    pub fn bit_bcncw_max(&self) -> BitBcncwMaxR {
        BitBcncwMaxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - PIFS time. It is only used in AP mode and in units of us."]
    #[inline(always)]
    pub fn bit_pifs(&self) -> BitPifsR {
        BitPifsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PIFS time. It is only used in RDG mode and in units of us."]
    #[inline(always)]
    pub fn bit_rdg_pifs(&self) -> BitRdgPifsR {
        BitRdgPifsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Beacon Interframe Space In units of microseconds, this register defines the medium idle time required before transmitting beacon (AP mode) or entering random back off window."]
    #[inline(always)]
    pub fn bit_bcnifs(&mut self) -> BitBcnifsW<'_, RegBcnCfgPifsSpec> {
        BitBcnifsW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Beacon Contention Window Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
    #[inline(always)]
    pub fn bit_bcncw_min(&mut self) -> BitBcncwMinW<'_, RegBcnCfgPifsSpec> {
        BitBcncwMinW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Beacon Contention Window MAX Defines the contention window value for back off before transmitting beacon frame. This field is set to 0 in AP mode to indicate no back off window. In ad hoc mode, its default value is 6. 26-1 = 63 time slots."]
    #[inline(always)]
    pub fn bit_bcncw_max(&mut self) -> BitBcncwMaxW<'_, RegBcnCfgPifsSpec> {
        BitBcncwMaxW::new(self, 12)
    }
    #[doc = "Bits 16:23 - PIFS time. It is only used in AP mode and in units of us."]
    #[inline(always)]
    pub fn bit_pifs(&mut self) -> BitPifsW<'_, RegBcnCfgPifsSpec> {
        BitPifsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - PIFS time. It is only used in RDG mode and in units of us."]
    #[inline(always)]
    pub fn bit_rdg_pifs(&mut self) -> BitRdgPifsW<'_, RegBcnCfgPifsSpec> {
        BitRdgPifsW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_cfg_pifs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_cfg_pifs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnCfgPifsSpec;
impl crate::RegisterSpec for RegBcnCfgPifsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_cfg_pifs::R`](R) reader structure"]
impl crate::Readable for RegBcnCfgPifsSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_cfg_pifs::W`](W) writer structure"]
impl crate::Writable for RegBcnCfgPifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_CFG_PIFS to value 0x281c_4413"]
impl crate::Resettable for RegBcnCfgPifsSpec {
    const RESET_VALUE: u32 = 0x281c_4413;
}
