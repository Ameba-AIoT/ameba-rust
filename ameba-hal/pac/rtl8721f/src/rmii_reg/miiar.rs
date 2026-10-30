#[doc = "Register `MIIAR` reader"]
pub type R = crate::R<MiiarSpec>;
#[doc = "Register `MIIAR` writer"]
pub type W = crate::W<MiiarSpec>;
#[doc = "Field `Data15_0` reader - Defines 16 bit MII resgister data."]
pub type Data15_0R = crate::FieldReader<u16>;
#[doc = "Field `Data15_0` writer - Defines 16 bit MII resgister data."]
pub type Data15_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RegAddr4_0` reader - Defines 5-bit MII register address."]
pub type RegAddr4_0R = crate::FieldReader;
#[doc = "Field `RegAddr4_0` writer - Defines 5-bit MII register address."]
pub type RegAddr4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `polling_eee_reg` reader - polling PCS EEE advertisement register (7.60). In rl6166(formerly rle0315) and before: this bit is not supported. In rle0390: In in-direct method, gmac had to access PCS register by multiples R/W of reg13 and 14. It is OK for PCS to be aborted in the middle of indirect method. In rle0371: HW will complete current indirect access to PCS then stop in-direct method. Please refer to EEE_CR1_reg."]
pub type PollingEeeRegR = crate::BitReader;
#[doc = "Field `polling_eee_reg` writer - polling PCS EEE advertisement register (7.60). In rl6166(formerly rle0315) and before: this bit is not supported. In rle0390: In in-direct method, gmac had to access PCS register by multiples R/W of reg13 and 14. It is OK for PCS to be aborted in the middle of indirect method. In rle0371: HW will complete current indirect access to PCS then stop in-direct method. Please refer to EEE_CR1_reg."]
pub type PollingEeeRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Disable_auto_polling` reader - Disable auto polling feature of mdio operation. 0: HW auto polling PCS status. 1: HW does not auto polling PCS status."]
pub type DisableAutoPollingR = crate::BitReader;
#[doc = "Field `Disable_auto_polling` writer - Disable auto polling feature of mdio operation. 0: HW auto polling PCS status. 1: HW does not auto polling PCS status."]
pub type DisableAutoPollingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `En_eeep_auto_write` reader - HW auto writes PCS.page4.reg20 and reg27 to adjust analog parameters based on eeep_lpi state. SW should change page to page4 then set MIIA_REG.en_eeep_auto_write. HW will write eeep_abig_on/off and eeep_ldvbias_on/off (in EEEP_CR6_REG and EEEP_CR7_REG) to PCS.page4.reg20 and reg27, respectively, based on eeep_lpi state. If SW wants to access another page of PCS, SW should disable MIIA_REG.en_eeep_auto_write."]
pub type EnEeepAutoWriteR = crate::BitReader;
#[doc = "Field `En_eeep_auto_write` writer - HW auto writes PCS.page4.reg20 and reg27 to adjust analog parameters based on eeep_lpi state. SW should change page to page4 then set MIIA_REG.en_eeep_auto_write. HW will write eeep_abig_on/off and eeep_ldvbias_on/off (in EEEP_CR6_REG and EEEP_CR7_REG) to PCS.page4.reg20 and reg27, respectively, based on eeep_lpi state. If SW wants to access another page of PCS, SW should disable MIIA_REG.en_eeep_auto_write."]
pub type EnEeepAutoWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mdio_busy` reader - MDIO is busy"]
pub type MdioBusyR = crate::BitReader;
#[doc = "Field `PHYAddress` reader - Defines the Phy address for the MII."]
pub type PhyaddressR = crate::FieldReader;
#[doc = "Field `PHYAddress` writer - Defines the Phy address for the MII."]
pub type PhyaddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Flag` reader - Flag bit, used to identify access to MII register: 1: Write data to MII register. Turns to 0 automatically upon completion of MAC writing to the specified MII register. 0: Read data from MII register. Turns to 1 automatically upon completion of MAC reading the specified MII register. Read write turn around time I s about 64 us."]
pub type FlagR = crate::BitReader;
#[doc = "Field `Flag` writer - Flag bit, used to identify access to MII register: 1: Write data to MII register. Turns to 0 automatically upon completion of MAC writing to the specified MII register. 0: Read data from MII register. Turns to 1 automatically upon completion of MAC reading the specified MII register. Read write turn around time I s about 64 us."]
pub type FlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Defines 16 bit MII resgister data."]
    #[inline(always)]
    pub fn data15_0(&self) -> Data15_0R {
        Data15_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Defines 5-bit MII register address."]
    #[inline(always)]
    pub fn reg_addr4_0(&self) -> RegAddr4_0R {
        RegAddr4_0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - polling PCS EEE advertisement register (7.60). In rl6166(formerly rle0315) and before: this bit is not supported. In rle0390: In in-direct method, gmac had to access PCS register by multiples R/W of reg13 and 14. It is OK for PCS to be aborted in the middle of indirect method. In rle0371: HW will complete current indirect access to PCS then stop in-direct method. Please refer to EEE_CR1_reg."]
    #[inline(always)]
    pub fn polling_eee_reg(&self) -> PollingEeeRegR {
        PollingEeeRegR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable auto polling feature of mdio operation. 0: HW auto polling PCS status. 1: HW does not auto polling PCS status."]
    #[inline(always)]
    pub fn disable_auto_polling(&self) -> DisableAutoPollingR {
        DisableAutoPollingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - HW auto writes PCS.page4.reg20 and reg27 to adjust analog parameters based on eeep_lpi state. SW should change page to page4 then set MIIA_REG.en_eeep_auto_write. HW will write eeep_abig_on/off and eeep_ldvbias_on/off (in EEEP_CR6_REG and EEEP_CR7_REG) to PCS.page4.reg20 and reg27, respectively, based on eeep_lpi state. If SW wants to access another page of PCS, SW should disable MIIA_REG.en_eeep_auto_write."]
    #[inline(always)]
    pub fn en_eeep_auto_write(&self) -> EnEeepAutoWriteR {
        EnEeepAutoWriteR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - MDIO is busy"]
    #[inline(always)]
    pub fn mdio_busy(&self) -> MdioBusyR {
        MdioBusyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Defines the Phy address for the MII."]
    #[inline(always)]
    pub fn phyaddress(&self) -> PhyaddressR {
        PhyaddressR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Flag bit, used to identify access to MII register: 1: Write data to MII register. Turns to 0 automatically upon completion of MAC writing to the specified MII register. 0: Read data from MII register. Turns to 1 automatically upon completion of MAC reading the specified MII register. Read write turn around time I s about 64 us."]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines 16 bit MII resgister data."]
    #[inline(always)]
    pub fn data15_0(&mut self) -> Data15_0W<'_, MiiarSpec> {
        Data15_0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Defines 5-bit MII register address."]
    #[inline(always)]
    pub fn reg_addr4_0(&mut self) -> RegAddr4_0W<'_, MiiarSpec> {
        RegAddr4_0W::new(self, 16)
    }
    #[doc = "Bit 21 - polling PCS EEE advertisement register (7.60). In rl6166(formerly rle0315) and before: this bit is not supported. In rle0390: In in-direct method, gmac had to access PCS register by multiples R/W of reg13 and 14. It is OK for PCS to be aborted in the middle of indirect method. In rle0371: HW will complete current indirect access to PCS then stop in-direct method. Please refer to EEE_CR1_reg."]
    #[inline(always)]
    pub fn polling_eee_reg(&mut self) -> PollingEeeRegW<'_, MiiarSpec> {
        PollingEeeRegW::new(self, 21)
    }
    #[doc = "Bit 22 - Disable auto polling feature of mdio operation. 0: HW auto polling PCS status. 1: HW does not auto polling PCS status."]
    #[inline(always)]
    pub fn disable_auto_polling(&mut self) -> DisableAutoPollingW<'_, MiiarSpec> {
        DisableAutoPollingW::new(self, 22)
    }
    #[doc = "Bit 23 - HW auto writes PCS.page4.reg20 and reg27 to adjust analog parameters based on eeep_lpi state. SW should change page to page4 then set MIIA_REG.en_eeep_auto_write. HW will write eeep_abig_on/off and eeep_ldvbias_on/off (in EEEP_CR6_REG and EEEP_CR7_REG) to PCS.page4.reg20 and reg27, respectively, based on eeep_lpi state. If SW wants to access another page of PCS, SW should disable MIIA_REG.en_eeep_auto_write."]
    #[inline(always)]
    pub fn en_eeep_auto_write(&mut self) -> EnEeepAutoWriteW<'_, MiiarSpec> {
        EnEeepAutoWriteW::new(self, 23)
    }
    #[doc = "Bits 26:30 - Defines the Phy address for the MII."]
    #[inline(always)]
    pub fn phyaddress(&mut self) -> PhyaddressW<'_, MiiarSpec> {
        PhyaddressW::new(self, 26)
    }
    #[doc = "Bit 31 - Flag bit, used to identify access to MII register: 1: Write data to MII register. Turns to 0 automatically upon completion of MAC writing to the specified MII register. 0: Read data from MII register. Turns to 1 automatically upon completion of MAC reading the specified MII register. Read write turn around time I s about 64 us."]
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, MiiarSpec> {
        FlagW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`miiar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miiar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiarSpec;
impl crate::RegisterSpec for MiiarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miiar::R`](R) reader structure"]
impl crate::Readable for MiiarSpec {}
#[doc = "`write(|w| ..)` method takes [`miiar::W`](W) writer structure"]
impl crate::Writable for MiiarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIIAR to value 0"]
impl crate::Resettable for MiiarSpec {}
