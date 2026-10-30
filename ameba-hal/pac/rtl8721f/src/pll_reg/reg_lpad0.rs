#[doc = "Register `REG_LPAD0` reader"]
pub type R = crate::R<RegLpad0Spec>;
#[doc = "Register `REG_LPAD0` writer"]
pub type W = crate::W<RegLpad0Spec>;
#[doc = "Field `pow_ldo` reader - pow ldo"]
pub type PowLdoR = crate::BitReader;
#[doc = "Field `pow_ldo` writer - pow ldo"]
pub type PowLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow` reader - pow adc"]
pub type PowR = crate::BitReader;
#[doc = "Field `pow` writer - pow adc"]
pub type PowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_ref` reader - pow adc refgen"]
pub type PowRefR = crate::BitReader;
#[doc = "Field `pow_ref` writer - pow adc refgen"]
pub type PowRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ck` reader - 1: clk from external"]
pub type RegCkR = crate::BitReader;
#[doc = "Field `reg_ck` writer - 1: clk from external"]
pub type RegCkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpad_dmy1` reader - dummy"]
pub type LpadDmy1R = crate::BitReader;
#[doc = "Field `lpad_dmy1` writer - dummy"]
pub type LpadDmy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ldo_sel` reader - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
pub type RegLdoSelR = crate::FieldReader;
#[doc = "Field `reg_ldo_sel` writer - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
pub type RegLdoSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vref_sel` reader - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
pub type VrefSelR = crate::FieldReader;
#[doc = "Field `vref_sel` writer - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_rev_clk` reader - 1: rever ck_data"]
pub type RegRevClkR = crate::BitReader;
#[doc = "Field `reg_rev_clk` writer - 1: rever ck_data"]
pub type RegRevClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cmpdec` reader - meta detect time 00: short"]
pub type RegCmpdecR = crate::FieldReader;
#[doc = "Field `reg_cmpdec` writer - meta detect time 00: short"]
pub type RegCmpdecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_meta` reader - 1: enable meta detect"]
pub type RegMetaR = crate::BitReader;
#[doc = "Field `reg_meta` writer - 1: enable meta detect"]
pub type RegMetaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_mode_18` reader - 1 : 1.8V mode(bypassen=0) 0 : 3.3V mode(bypassen=0)"]
pub type EnMode18R = crate::BitReader;
#[doc = "Field `en_mode_18` writer - 1 : 1.8V mode(bypassen=0) 0 : 3.3V mode(bypassen=0)"]
pub type EnMode18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypassen` reader - 1:bypass mode, 0: divide mode"]
pub type BypassenR = crate::FieldReader;
#[doc = "Field `bypassen` writer - 1:bypass mode, 0: divide mode"]
pub type BypassenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_enTG` reader - 1: enable transmission gate"]
pub type RegEnTgR = crate::BitReader;
#[doc = "Field `reg_enTG` writer - 1: enable transmission gate"]
pub type RegEnTgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpad_dmy0` reader - "]
pub type LpadDmy0R = crate::BitReader;
#[doc = "Field `lpad_dmy0` writer - "]
pub type LpadDmy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_vcmin` reader - 00:0.4625V 01:0.4500V 10:0.4375V 11:0.4250V"]
pub type RegVcminR = crate::FieldReader;
#[doc = "Field `reg_vcmin` writer - 00:0.4625V 01:0.4500V 10:0.4375V 11:0.4250V"]
pub type RegVcminW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sel_lna` reader - 1: comparator low noise for ALL bit"]
pub type SelLnaR = crate::BitReader;
#[doc = "Field `sel_lna` writer - 1: comparator low noise for ALL bit"]
pub type SelLnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_ln` reader - 1: comparator low noise for LSB 6bit"]
pub type SelLnR = crate::BitReader;
#[doc = "Field `sel_ln` writer - 1: comparator low noise for LSB 6bit"]
pub type SelLnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vset` reader - 11:1.89V 10:1.80V 01:1.71V 00:1.62V"]
pub type VsetR = crate::FieldReader;
#[doc = "Field `vset` writer - 11:1.89V 10:1.80V 01:1.71V 00:1.62V"]
pub type VsetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - pow ldo"]
    #[inline(always)]
    pub fn pow_ldo(&self) -> PowLdoR {
        PowLdoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pow adc"]
    #[inline(always)]
    pub fn pow(&self) -> PowR {
        PowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pow adc refgen"]
    #[inline(always)]
    pub fn pow_ref(&self) -> PowRefR {
        PowRefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: clk from external"]
    #[inline(always)]
    pub fn reg_ck(&self) -> RegCkR {
        RegCkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dummy"]
    #[inline(always)]
    pub fn lpad_dmy1(&self) -> LpadDmy1R {
        LpadDmy1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
    #[inline(always)]
    pub fn reg_ldo_sel(&self) -> RegLdoSelR {
        RegLdoSelR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 1: rever ck_data"]
    #[inline(always)]
    pub fn reg_rev_clk(&self) -> RegRevClkR {
        RegRevClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - meta detect time 00: short"]
    #[inline(always)]
    pub fn reg_cmpdec(&self) -> RegCmpdecR {
        RegCmpdecR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 1: enable meta detect"]
    #[inline(always)]
    pub fn reg_meta(&self) -> RegMetaR {
        RegMetaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1 : 1.8V mode(bypassen=0) 0 : 3.3V mode(bypassen=0)"]
    #[inline(always)]
    pub fn en_mode_18(&self) -> EnMode18R {
        EnMode18R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 1:bypass mode, 0: divide mode"]
    #[inline(always)]
    pub fn bypassen(&self) -> BypassenR {
        BypassenR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: enable transmission gate"]
    #[inline(always)]
    pub fn reg_en_tg(&self) -> RegEnTgR {
        RegEnTgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lpad_dmy0(&self) -> LpadDmy0R {
        LpadDmy0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 00:0.4625V 01:0.4500V 10:0.4375V 11:0.4250V"]
    #[inline(always)]
    pub fn reg_vcmin(&self) -> RegVcminR {
        RegVcminR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: comparator low noise for ALL bit"]
    #[inline(always)]
    pub fn sel_lna(&self) -> SelLnaR {
        SelLnaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: comparator low noise for LSB 6bit"]
    #[inline(always)]
    pub fn sel_ln(&self) -> SelLnR {
        SelLnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 11:1.89V 10:1.80V 01:1.71V 00:1.62V"]
    #[inline(always)]
    pub fn vset(&self) -> VsetR {
        VsetR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - pow ldo"]
    #[inline(always)]
    pub fn pow_ldo(&mut self) -> PowLdoW<'_, RegLpad0Spec> {
        PowLdoW::new(self, 0)
    }
    #[doc = "Bit 1 - pow adc"]
    #[inline(always)]
    pub fn pow(&mut self) -> PowW<'_, RegLpad0Spec> {
        PowW::new(self, 1)
    }
    #[doc = "Bit 2 - pow adc refgen"]
    #[inline(always)]
    pub fn pow_ref(&mut self) -> PowRefW<'_, RegLpad0Spec> {
        PowRefW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: clk from external"]
    #[inline(always)]
    pub fn reg_ck(&mut self) -> RegCkW<'_, RegLpad0Spec> {
        RegCkW::new(self, 3)
    }
    #[doc = "Bit 4 - dummy"]
    #[inline(always)]
    pub fn lpad_dmy1(&mut self) -> LpadDmy1W<'_, RegLpad0Spec> {
        LpadDmy1W::new(self, 4)
    }
    #[doc = "Bits 5:7 - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
    #[inline(always)]
    pub fn reg_ldo_sel(&mut self) -> RegLdoSelW<'_, RegLpad0Spec> {
        RegLdoSelW::new(self, 5)
    }
    #[doc = "Bits 8:10 - 000:0.800V 001:0.825V 010:0.850V 011:0.875V 100:0.900V 101:0.950V 110:1.000V 111:1.050V"]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VrefSelW<'_, RegLpad0Spec> {
        VrefSelW::new(self, 8)
    }
    #[doc = "Bit 11 - 1: rever ck_data"]
    #[inline(always)]
    pub fn reg_rev_clk(&mut self) -> RegRevClkW<'_, RegLpad0Spec> {
        RegRevClkW::new(self, 11)
    }
    #[doc = "Bits 12:13 - meta detect time 00: short"]
    #[inline(always)]
    pub fn reg_cmpdec(&mut self) -> RegCmpdecW<'_, RegLpad0Spec> {
        RegCmpdecW::new(self, 12)
    }
    #[doc = "Bit 14 - 1: enable meta detect"]
    #[inline(always)]
    pub fn reg_meta(&mut self) -> RegMetaW<'_, RegLpad0Spec> {
        RegMetaW::new(self, 14)
    }
    #[doc = "Bit 15 - 1 : 1.8V mode(bypassen=0) 0 : 3.3V mode(bypassen=0)"]
    #[inline(always)]
    pub fn en_mode_18(&mut self) -> EnMode18W<'_, RegLpad0Spec> {
        EnMode18W::new(self, 15)
    }
    #[doc = "Bits 16:23 - 1:bypass mode, 0: divide mode"]
    #[inline(always)]
    pub fn bypassen(&mut self) -> BypassenW<'_, RegLpad0Spec> {
        BypassenW::new(self, 16)
    }
    #[doc = "Bit 24 - 1: enable transmission gate"]
    #[inline(always)]
    pub fn reg_en_tg(&mut self) -> RegEnTgW<'_, RegLpad0Spec> {
        RegEnTgW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lpad_dmy0(&mut self) -> LpadDmy0W<'_, RegLpad0Spec> {
        LpadDmy0W::new(self, 25)
    }
    #[doc = "Bits 26:27 - 00:0.4625V 01:0.4500V 10:0.4375V 11:0.4250V"]
    #[inline(always)]
    pub fn reg_vcmin(&mut self) -> RegVcminW<'_, RegLpad0Spec> {
        RegVcminW::new(self, 26)
    }
    #[doc = "Bit 28 - 1: comparator low noise for ALL bit"]
    #[inline(always)]
    pub fn sel_lna(&mut self) -> SelLnaW<'_, RegLpad0Spec> {
        SelLnaW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: comparator low noise for LSB 6bit"]
    #[inline(always)]
    pub fn sel_ln(&mut self) -> SelLnW<'_, RegLpad0Spec> {
        SelLnW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 11:1.89V 10:1.80V 01:1.71V 00:1.62V"]
    #[inline(always)]
    pub fn vset(&mut self) -> VsetW<'_, RegLpad0Spec> {
        VsetW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpad0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpad0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpad0Spec;
impl crate::RegisterSpec for RegLpad0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpad0::R`](R) reader structure"]
impl crate::Readable for RegLpad0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpad0::W`](W) writer structure"]
impl crate::Writable for RegLpad0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPAD0 to value 0xb400_12c0"]
impl crate::Resettable for RegLpad0Spec {
    const RESET_VALUE: u32 = 0xb400_12c0;
}
