#[doc = "Register `IC_TAR` reader"]
pub type R = crate::R<IcTarSpec>;
#[doc = "Register `IC_TAR` writer"]
pub type W = crate::W<IcTarSpec>;
#[doc = "Field `IC_TAR` reader - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits."]
pub type IcTarR = crate::FieldReader<u16>;
#[doc = "Field `IC_TAR` writer - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits."]
pub type IcTarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `GC_OR_START` reader - If SPECIAL is set to 1, then this bit indicates whether a General Call or START BYTE command is to be performed by I2C. (ic_clk domain) * 0: General Call - after issuing a General Call, only write may be performed * 1: START BYTE"]
pub type GcOrStartR = crate::BitReader;
#[doc = "Field `GC_OR_START` writer - If SPECIAL is set to 1, then this bit indicates whether a General Call or START BYTE command is to be performed by I2C. (ic_clk domain) * 0: General Call - after issuing a General Call, only write may be performed * 1: START BYTE"]
pub type GcOrStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a General Call or START BYTE command. * 0: ignore bit 10 GC_OR_START and use IC_TAR normally * 1: perform special I2C command as specified in GC_OR_START bit"]
pub type SpecialR = crate::BitReader;
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a General Call or START BYTE command. * 0: ignore bit 10 GC_OR_START and use IC_TAR normally * 1: perform special I2C command as specified in GC_OR_START bit"]
pub type SpecialW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_10BITADDR_MASTER` reader - Control whether I2C starts its transfers in 7- or 10-bit addressing mode when acting as a master. * 0: 7-bit addressing * 1: 10-bit addressing"]
pub type Ic10bitaddrMasterR = crate::BitReader;
#[doc = "Field `IC_10BITADDR_MASTER` writer - Control whether I2C starts its transfers in 7- or 10-bit addressing mode when acting as a master. * 0: 7-bit addressing * 1: 10-bit addressing"]
pub type Ic10bitaddrMasterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits."]
    #[inline(always)]
    pub fn ic_tar(&self) -> IcTarR {
        IcTarR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If SPECIAL is set to 1, then this bit indicates whether a General Call or START BYTE command is to be performed by I2C. (ic_clk domain) * 0: General Call - after issuing a General Call, only write may be performed * 1: START BYTE"]
    #[inline(always)]
    pub fn gc_or_start(&self) -> GcOrStartR {
        GcOrStartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE command. * 0: ignore bit 10 GC_OR_START and use IC_TAR normally * 1: perform special I2C command as specified in GC_OR_START bit"]
    #[inline(always)]
    pub fn special(&self) -> SpecialR {
        SpecialR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control whether I2C starts its transfers in 7- or 10-bit addressing mode when acting as a master. * 0: 7-bit addressing * 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> Ic10bitaddrMasterR {
        Ic10bitaddrMasterR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits."]
    #[inline(always)]
    pub fn ic_tar(&mut self) -> IcTarW<'_, IcTarSpec> {
        IcTarW::new(self, 0)
    }
    #[doc = "Bit 10 - If SPECIAL is set to 1, then this bit indicates whether a General Call or START BYTE command is to be performed by I2C. (ic_clk domain) * 0: General Call - after issuing a General Call, only write may be performed * 1: START BYTE"]
    #[inline(always)]
    pub fn gc_or_start(&mut self) -> GcOrStartW<'_, IcTarSpec> {
        GcOrStartW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE command. * 0: ignore bit 10 GC_OR_START and use IC_TAR normally * 1: perform special I2C command as specified in GC_OR_START bit"]
    #[inline(always)]
    pub fn special(&mut self) -> SpecialW<'_, IcTarSpec> {
        SpecialW::new(self, 11)
    }
    #[doc = "Bit 12 - Control whether I2C starts its transfers in 7- or 10-bit addressing mode when acting as a master. * 0: 7-bit addressing * 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_master(&mut self) -> Ic10bitaddrMasterW<'_, IcTarSpec> {
        Ic10bitaddrMasterW::new(self, 12)
    }
}
#[doc = "I2C Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcTarSpec;
impl crate::RegisterSpec for IcTarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_tar::R`](R) reader structure"]
impl crate::Readable for IcTarSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_tar::W`](W) writer structure"]
impl crate::Writable for IcTarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_TAR to value 0x10"]
impl crate::Resettable for IcTarSpec {
    const RESET_VALUE: u32 = 0x10;
}
