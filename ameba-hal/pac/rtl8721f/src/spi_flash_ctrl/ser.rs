#[doc = "Register `SER` reader"]
pub type R = crate::R<SerSpec>;
#[doc = "Register `SER` writer"]
pub type W = crate::W<SerSpec>;
#[doc = "Field `SER` reader - Each bit in the register corresponds to one SPI Flash. In user mode user program the register to select target flash. 0: Not selected 1: Selected"]
pub type SerR = crate::BitReader;
#[doc = "Field `SER` writer - Each bit in the register corresponds to one SPI Flash. In user mode user program the register to select target flash. 0: Not selected 1: Selected"]
pub type SerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSN_SW` reader - Set to determine the switch of csn0 and csn1~3. 0: no switch 1: csn0 switch with cs1 2: csn0 switch with cs2 3: csn0 switch with cs3 Exists: TAG >= 2.0.8."]
pub type CsnSwR = crate::FieldReader;
#[doc = "Field `CSN_SW` writer - Set to determine the switch of csn0 and csn1~3. 0: no switch 1: csn0 switch with cs1 2: csn0 switch with cs2 3: csn0 switch with cs3 Exists: TAG >= 2.0.8."]
pub type CsnSwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Each bit in the register corresponds to one SPI Flash. In user mode user program the register to select target flash. 0: Not selected 1: Selected"]
    #[inline(always)]
    pub fn ser(&self) -> SerR {
        SerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 30:31 - Set to determine the switch of csn0 and csn1~3. 0: no switch 1: csn0 switch with cs1 2: csn0 switch with cs2 3: csn0 switch with cs3 Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn csn_sw(&self) -> CsnSwR {
        CsnSwR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Each bit in the register corresponds to one SPI Flash. In user mode user program the register to select target flash. 0: Not selected 1: Selected"]
    #[inline(always)]
    pub fn ser(&mut self) -> SerW<'_, SerSpec> {
        SerW::new(self, 0)
    }
    #[doc = "Bits 30:31 - Set to determine the switch of csn0 and csn1~3. 0: no switch 1: csn0 switch with cs1 2: csn0 switch with cs2 3: csn0 switch with cs3 Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn csn_sw(&mut self) -> CsnSwW<'_, SerSpec> {
        CsnSwW::new(self, 30)
    }
}
#[doc = "Slave Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerSpec;
impl crate::RegisterSpec for SerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser::R`](R) reader structure"]
impl crate::Readable for SerSpec {}
#[doc = "`write(|w| ..)` method takes [`ser::W`](W) writer structure"]
impl crate::Writable for SerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SER to value 0x01"]
impl crate::Resettable for SerSpec {
    const RESET_VALUE: u32 = 0x01;
}
