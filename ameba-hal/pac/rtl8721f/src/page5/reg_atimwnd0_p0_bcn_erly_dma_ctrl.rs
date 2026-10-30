#[doc = "Register `REG_ATIMWND0_P0_BCN_ERLY_DMA_CTRL` reader"]
pub type R = crate::R<RegAtimwnd0P0BcnErlyDmaCtrlSpec>;
#[doc = "Register `REG_ATIMWND0_P0_BCN_ERLY_DMA_CTRL` writer"]
pub type W = crate::W<RegAtimwnd0P0BcnErlyDmaCtrlSpec>;
#[doc = "Field `BIT_DRVERLYITV_P0` reader - Time to send interrupt to notify driver to change beacon content. Time unit is TU. The register also is shared by P2P-CTW and P2P-NoA (from NoA-Absent to NoA-On). It is used to wake up early at DriverEarlyTime"]
pub type BitDrverlyitvP0R = crate::FieldReader;
#[doc = "Field `BIT_DRVERLYITV_P0` writer - Time to send interrupt to notify driver to change beacon content. Time unit is TU. The register also is shared by P2P-CTW and P2P-NoA (from NoA-Absent to NoA-On). It is used to wake up early at DriverEarlyTime"]
pub type BitDrverlyitvP0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNDMATIM_P0` reader - Indicates the time before TBTT to perform beacon queue DMA. This register is shared by the time to assert ATIM window end interrupt. Measured in units of \"TU\" before TBTT and before ATIM window end."]
pub type BitBcndmatimP0R = crate::FieldReader;
#[doc = "Field `BIT_BCNDMATIM_P0` writer - Indicates the time before TBTT to perform beacon queue DMA. This register is shared by the time to assert ATIM window end interrupt. Measured in units of \"TU\" before TBTT and before ATIM window end."]
pub type BitBcndmatimP0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIMWND0` reader - ATIM window width of root VAP in units of TU."]
pub type BitAtimwnd0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_ATIMWND0` writer - ATIM window width of root VAP in units of TU."]
pub type BitAtimwnd0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. The register also is shared by P2P-CTW and P2P-NoA (from NoA-Absent to NoA-On). It is used to wake up early at DriverEarlyTime"]
    #[inline(always)]
    pub fn bit_drverlyitv_p0(&self) -> BitDrverlyitvP0R {
        BitDrverlyitvP0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates the time before TBTT to perform beacon queue DMA. This register is shared by the time to assert ATIM window end interrupt. Measured in units of \"TU\" before TBTT and before ATIM window end."]
    #[inline(always)]
    pub fn bit_bcndmatim_p0(&self) -> BitBcndmatimP0R {
        BitBcndmatimP0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - ATIM window width of root VAP in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd0(&self) -> BitAtimwnd0R {
        BitAtimwnd0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. The register also is shared by P2P-CTW and P2P-NoA (from NoA-Absent to NoA-On). It is used to wake up early at DriverEarlyTime"]
    #[inline(always)]
    pub fn bit_drverlyitv_p0(&mut self) -> BitDrverlyitvP0W<'_, RegAtimwnd0P0BcnErlyDmaCtrlSpec> {
        BitDrverlyitvP0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates the time before TBTT to perform beacon queue DMA. This register is shared by the time to assert ATIM window end interrupt. Measured in units of \"TU\" before TBTT and before ATIM window end."]
    #[inline(always)]
    pub fn bit_bcndmatim_p0(&mut self) -> BitBcndmatimP0W<'_, RegAtimwnd0P0BcnErlyDmaCtrlSpec> {
        BitBcndmatimP0W::new(self, 8)
    }
    #[doc = "Bits 16:31 - ATIM window width of root VAP in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd0(&mut self) -> BitAtimwnd0W<'_, RegAtimwnd0P0BcnErlyDmaCtrlSpec> {
        BitAtimwnd0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_atimwnd0_p0_bcn_erly_dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_atimwnd0_p0_bcn_erly_dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAtimwnd0P0BcnErlyDmaCtrlSpec;
impl crate::RegisterSpec for RegAtimwnd0P0BcnErlyDmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_atimwnd0_p0_bcn_erly_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAtimwnd0P0BcnErlyDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_atimwnd0_p0_bcn_erly_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAtimwnd0P0BcnErlyDmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ATIMWND0_P0_BCN_ERLY_DMA_CTRL to value 0x0002_0305"]
impl crate::Resettable for RegAtimwnd0P0BcnErlyDmaCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_0305;
}
