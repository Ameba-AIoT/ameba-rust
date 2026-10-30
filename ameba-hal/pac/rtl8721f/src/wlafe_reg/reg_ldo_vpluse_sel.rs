#[doc = "Register `REG_LDO_VPLUSE_SEL` reader"]
pub type R = crate::R<RegLdoVpluseSelSpec>;
#[doc = "Register `REG_LDO_VPLUSE_SEL` writer"]
pub type W = crate::W<RegLdoVpluseSelSpec>;
#[doc = "Field `vpluse_sel` reader - vpluse_ldo output decided by: 1: power on ldo 0: ctrl reg"]
pub type VpluseSelR = crate::BitReader;
#[doc = "Field `vpluse_sel` writer - vpluse_ldo output decided by: 1: power on ldo 0: ctrl reg"]
pub type VpluseSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vpluse_cnt` reader - vpluse_ldo high output delay cycle after pll_ldo set high,default:5(1.25us)"]
pub type VpluseCntR = crate::FieldReader<u16>;
#[doc = "Field `vpluse_cnt` writer - vpluse_ldo high output delay cycle after pll_ldo set high,default:5(1.25us)"]
pub type VpluseCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `vpluse_wait_mcycle` reader - 1: vpluse_ldo_dac and vpluse_ldo_adc pull low should wait more cycle after ldo pull high"]
pub type VpluseWaitMcycleR = crate::BitReader;
#[doc = "Field `vpluse_wait_mcycle` writer - 1: vpluse_ldo_dac and vpluse_ldo_adc pull low should wait more cycle after ldo pull high"]
pub type VpluseWaitMcycleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - vpluse_ldo output decided by: 1: power on ldo 0: ctrl reg"]
    #[inline(always)]
    pub fn vpluse_sel(&self) -> VpluseSelR {
        VpluseSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - vpluse_ldo high output delay cycle after pll_ldo set high,default:5(1.25us)"]
    #[inline(always)]
    pub fn vpluse_cnt(&self) -> VpluseCntR {
        VpluseCntR::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - 1: vpluse_ldo_dac and vpluse_ldo_adc pull low should wait more cycle after ldo pull high"]
    #[inline(always)]
    pub fn vpluse_wait_mcycle(&self) -> VpluseWaitMcycleR {
        VpluseWaitMcycleR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - vpluse_ldo output decided by: 1: power on ldo 0: ctrl reg"]
    #[inline(always)]
    pub fn vpluse_sel(&mut self) -> VpluseSelW<'_, RegLdoVpluseSelSpec> {
        VpluseSelW::new(self, 0)
    }
    #[doc = "Bits 1:16 - vpluse_ldo high output delay cycle after pll_ldo set high,default:5(1.25us)"]
    #[inline(always)]
    pub fn vpluse_cnt(&mut self) -> VpluseCntW<'_, RegLdoVpluseSelSpec> {
        VpluseCntW::new(self, 1)
    }
    #[doc = "Bit 17 - 1: vpluse_ldo_dac and vpluse_ldo_adc pull low should wait more cycle after ldo pull high"]
    #[inline(always)]
    pub fn vpluse_wait_mcycle(&mut self) -> VpluseWaitMcycleW<'_, RegLdoVpluseSelSpec> {
        VpluseWaitMcycleW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ldo_vpluse_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ldo_vpluse_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLdoVpluseSelSpec;
impl crate::RegisterSpec for RegLdoVpluseSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ldo_vpluse_sel::R`](R) reader structure"]
impl crate::Readable for RegLdoVpluseSelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ldo_vpluse_sel::W`](W) writer structure"]
impl crate::Writable for RegLdoVpluseSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LDO_VPLUSE_SEL to value 0x0002_0083"]
impl crate::Resettable for RegLdoVpluseSelSpec {
    const RESET_VALUE: u32 = 0x0002_0083;
}
