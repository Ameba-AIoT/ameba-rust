#[doc = "Register `REG_CLK_CALC` reader"]
pub type R = crate::R<RegClkCalcSpec>;
#[doc = "Register `REG_CLK_CALC` writer"]
pub type W = crate::W<RegClkCalcSpec>;
#[doc = "Field `ck_cal_rpt` reader - Clock calibration result"]
pub type CkCalRptR = crate::FieldReader<u16>;
#[doc = "Field `ck_cal_sel` reader - Calibration clock selection . The refrence clock is xtal clock. 0: OSC4M 1: AON100K 2: AON128K for SDM"]
pub type CkCalSelR = crate::FieldReader;
#[doc = "Field `ck_cal_sel` writer - Calibration clock selection . The refrence clock is xtal clock. 0: OSC4M 1: AON100K 2: AON128K for SDM"]
pub type CkCalSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ck_cal_start` reader - Start clk calibration . This bit will auto-set to 0 when cal done ."]
pub type CkCalStartR = crate::BitReader;
#[doc = "Field `ck_cal_start` writer - Start clk calibration . This bit will auto-set to 0 when cal done ."]
pub type CkCalStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Clock calibration result"]
    #[inline(always)]
    pub fn ck_cal_rpt(&self) -> CkCalRptR {
        CkCalRptR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Calibration clock selection . The refrence clock is xtal clock. 0: OSC4M 1: AON100K 2: AON128K for SDM"]
    #[inline(always)]
    pub fn ck_cal_sel(&self) -> CkCalSelR {
        CkCalSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Start clk calibration . This bit will auto-set to 0 when cal done ."]
    #[inline(always)]
    pub fn ck_cal_start(&self) -> CkCalStartR {
        CkCalStartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Calibration clock selection . The refrence clock is xtal clock. 0: OSC4M 1: AON100K 2: AON128K for SDM"]
    #[inline(always)]
    pub fn ck_cal_sel(&mut self) -> CkCalSelW<'_, RegClkCalcSpec> {
        CkCalSelW::new(self, 16)
    }
    #[doc = "Bit 31 - Start clk calibration . This bit will auto-set to 0 when cal done ."]
    #[inline(always)]
    pub fn ck_cal_start(&mut self) -> CkCalStartW<'_, RegClkCalcSpec> {
        CkCalStartW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_calc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_calc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkCalcSpec;
impl crate::RegisterSpec for RegClkCalcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clk_calc::R`](R) reader structure"]
impl crate::Readable for RegClkCalcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clk_calc::W`](W) writer structure"]
impl crate::Writable for RegClkCalcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLK_CALC to value 0"]
impl crate::Resettable for RegClkCalcSpec {}
