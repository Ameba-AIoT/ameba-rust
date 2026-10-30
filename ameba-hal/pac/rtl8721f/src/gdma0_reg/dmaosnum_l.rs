#[doc = "Register `DMAOSNUM_L` reader"]
pub type R = crate::R<DmaosnumLSpec>;
#[doc = "Register `DMAOSNUM_L` writer"]
pub type W = crate::W<DmaosnumLSpec>;
#[doc = "Field `DMAOSNUM_L_OSR` reader - If the OSR number of DMAC is programmable, this register is writable. And DMAC will not issue new read command until OSR of DMAC is less than this value. If the OSR number is not programmable, this value is read as \"channel number\". And DMAC does not limit the OSR number. If user defines OSR_CNT, b = log2(OSR_CNT +1); otherwise, b = log2(DMAH_NUM_CHANNELS +1) bits 7: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new read commands."]
pub type DmaosnumLOsrR = crate::FieldReader;
#[doc = "Field `DMAOSNUM_L_OSR` writer - If the OSR number of DMAC is programmable, this register is writable. And DMAC will not issue new read command until OSR of DMAC is less than this value. If the OSR number is not programmable, this value is read as \"channel number\". And DMAC does not limit the OSR number. If user defines OSR_CNT, b = log2(OSR_CNT +1); otherwise, b = log2(DMAH_NUM_CHANNELS +1) bits 7: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new read commands."]
pub type DmaosnumLOsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAOSNUM_L_OSW` reader - If the OSW number of DMAC is programmable, this register is writable. And DMAC will not issue new write command until OSW of DMAC is less than this value. If the OSW number is not programmable, this value is read as \"4\". And DMAC's OSW is fixed to 4. If user defines OSW_CNT, b = log2(OSW_CNT +1) +7; otherwise, b = log2(4+1) +7. bits 15: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new write commands."]
pub type DmaosnumLOswR = crate::FieldReader;
#[doc = "Field `DMAOSNUM_L_OSW` writer - If the OSW number of DMAC is programmable, this register is writable. And DMAC will not issue new write command until OSW of DMAC is less than this value. If the OSW number is not programmable, this value is read as \"4\". And DMAC's OSW is fixed to 4. If user defines OSW_CNT, b = log2(OSW_CNT +1) +7; otherwise, b = log2(4+1) +7. bits 15: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new write commands."]
pub type DmaosnumLOswW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - If the OSR number of DMAC is programmable, this register is writable. And DMAC will not issue new read command until OSR of DMAC is less than this value. If the OSR number is not programmable, this value is read as \"channel number\". And DMAC does not limit the OSR number. If user defines OSR_CNT, b = log2(OSR_CNT +1); otherwise, b = log2(DMAH_NUM_CHANNELS +1) bits 7: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new read commands."]
    #[inline(always)]
    pub fn dmaosnum_l_osr(&self) -> DmaosnumLOsrR {
        DmaosnumLOsrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - If the OSW number of DMAC is programmable, this register is writable. And DMAC will not issue new write command until OSW of DMAC is less than this value. If the OSW number is not programmable, this value is read as \"4\". And DMAC's OSW is fixed to 4. If user defines OSW_CNT, b = log2(OSW_CNT +1) +7; otherwise, b = log2(4+1) +7. bits 15: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new write commands."]
    #[inline(always)]
    pub fn dmaosnum_l_osw(&self) -> DmaosnumLOswR {
        DmaosnumLOswR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - If the OSR number of DMAC is programmable, this register is writable. And DMAC will not issue new read command until OSR of DMAC is less than this value. If the OSR number is not programmable, this value is read as \"channel number\". And DMAC does not limit the OSR number. If user defines OSR_CNT, b = log2(OSR_CNT +1); otherwise, b = log2(DMAH_NUM_CHANNELS +1) bits 7: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new read commands."]
    #[inline(always)]
    pub fn dmaosnum_l_osr(&mut self) -> DmaosnumLOsrW<'_, DmaosnumLSpec> {
        DmaosnumLOsrW::new(self, 0)
    }
    #[doc = "Bits 8:10 - If the OSW number of DMAC is programmable, this register is writable. And DMAC will not issue new write command until OSW of DMAC is less than this value. If the OSW number is not programmable, this value is read as \"4\". And DMAC's OSW is fixed to 4. If user defines OSW_CNT, b = log2(OSW_CNT +1) +7; otherwise, b = log2(4+1) +7. bits 15: b+1 do not exist and read back as 0. Note: If write 0 to this register, DMAC can not issue any new write commands."]
    #[inline(always)]
    pub fn dmaosnum_l_osw(&mut self) -> DmaosnumLOswW<'_, DmaosnumLSpec> {
        DmaosnumLOswW::new(self, 8)
    }
}
#[doc = "DMA low-32 bits Outstanding Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaosnum_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaosnum_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaosnumLSpec;
impl crate::RegisterSpec for DmaosnumLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaosnum_l::R`](R) reader structure"]
impl crate::Readable for DmaosnumLSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaosnum_l::W`](W) writer structure"]
impl crate::Writable for DmaosnumLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAOSNUM_L to value 0x0408"]
impl crate::Resettable for DmaosnumLSpec {
    const RESET_VALUE: u32 = 0x0408;
}
