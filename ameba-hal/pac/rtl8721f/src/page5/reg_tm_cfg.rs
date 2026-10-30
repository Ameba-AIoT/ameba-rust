#[doc = "Register `REG_TM_CFG` reader"]
pub type R = crate::R<RegTmCfgSpec>;
#[doc = "Register `REG_TM_CFG` writer"]
pub type W = crate::W<RegTmCfgSpec>;
#[doc = "Field `BIT_TM_FUNC_EN` reader - 1: Enable Timing Measurement function"]
pub type BitTmFuncEnR = crate::BitReader;
#[doc = "Field `BIT_TM_FUNC_EN` writer - 1: Enable Timing Measurement function"]
pub type BitTmFuncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXTMREQ_EN` reader - 1: HW will parsing rx packet, to find TM request frame"]
pub type BitRxtmreqEnR = crate::BitReader;
#[doc = "Field `BIT_RXTMREQ_EN` writer - 1: HW will parsing rx packet, to find TM request frame"]
pub type BitRxtmreqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXTM_EN` reader - 1: HW will parsing rx packet, to find TM frame"]
pub type BitRxtmEnR = crate::BitReader;
#[doc = "Field `BIT_RXTM_EN` writer - 1: HW will parsing rx packet, to find TM frame"]
pub type BitRxtmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TM_TSFT_PORT_SEL` reader - selected TM timer port 0: port0; 1: port1; 2: port2; 3: freerun"]
pub type BitTmTsftPortSelR = crate::FieldReader;
#[doc = "Field `BIT_TM_TSFT_PORT_SEL` writer - selected TM timer port 0: port0; 1: port1; 2: port2; 3: freerun"]
pub type BitTmTsftPortSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 1: Enable Timing Measurement function"]
    #[inline(always)]
    pub fn bit_tm_func_en(&self) -> BitTmFuncEnR {
        BitTmFuncEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: HW will parsing rx packet, to find TM request frame"]
    #[inline(always)]
    pub fn bit_rxtmreq_en(&self) -> BitRxtmreqEnR {
        BitRxtmreqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: HW will parsing rx packet, to find TM frame"]
    #[inline(always)]
    pub fn bit_rxtm_en(&self) -> BitRxtmEnR {
        BitRxtmEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - selected TM timer port 0: port0; 1: port1; 2: port2; 3: freerun"]
    #[inline(always)]
    pub fn bit_tm_tsft_port_sel(&self) -> BitTmTsftPortSelR {
        BitTmTsftPortSelR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable Timing Measurement function"]
    #[inline(always)]
    pub fn bit_tm_func_en(&mut self) -> BitTmFuncEnW<'_, RegTmCfgSpec> {
        BitTmFuncEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: HW will parsing rx packet, to find TM request frame"]
    #[inline(always)]
    pub fn bit_rxtmreq_en(&mut self) -> BitRxtmreqEnW<'_, RegTmCfgSpec> {
        BitRxtmreqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: HW will parsing rx packet, to find TM frame"]
    #[inline(always)]
    pub fn bit_rxtm_en(&mut self) -> BitRxtmEnW<'_, RegTmCfgSpec> {
        BitRxtmEnW::new(self, 2)
    }
    #[doc = "Bits 3:4 - selected TM timer port 0: port0; 1: port1; 2: port2; 3: freerun"]
    #[inline(always)]
    pub fn bit_tm_tsft_port_sel(&mut self) -> BitTmTsftPortSelW<'_, RegTmCfgSpec> {
        BitTmTsftPortSelW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tm_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmCfgSpec;
impl crate::RegisterSpec for RegTmCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_cfg::R`](R) reader structure"]
impl crate::Readable for RegTmCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tm_cfg::W`](W) writer structure"]
impl crate::Writable for RegTmCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TM_CFG to value 0"]
impl crate::Resettable for RegTmCfgSpec {}
