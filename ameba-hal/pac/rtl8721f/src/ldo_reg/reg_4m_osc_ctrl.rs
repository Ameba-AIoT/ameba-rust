#[doc = "Register `REG_4M_OSC_CTRL` reader"]
pub type R = crate::R<Reg4mOscCtrlSpec>;
#[doc = "Register `REG_4M_OSC_CTRL` writer"]
pub type W = crate::W<Reg4mOscCtrlSpec>;
#[doc = "Field `IB_Bias_SEL` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type IbBiasSelR = crate::FieldReader;
#[doc = "Field `IB_Bias_SEL` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type IbBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dmy_0` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type Dmy0R = crate::BitReader;
#[doc = "Field `dmy_0` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type Dmy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_4MOSC` reader - // \\[RF\\]\\[4MOSC\\] 1'b1 : enable"]
pub type En4moscR = crate::BitReader;
#[doc = "Field `C_SEL` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type CSelR = crate::BitReader;
#[doc = "Field `C_SEL` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type CSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn ib_bias_sel(&self) -> IbBiasSelR {
        IbBiasSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn dmy_0(&self) -> Dmy0R {
        Dmy0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - // \\[RF\\]\\[4MOSC\\] 1'b1 : enable"]
    #[inline(always)]
    pub fn en_4mosc(&self) -> En4moscR {
        En4moscR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn c_sel(&self) -> CSelR {
        CSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn ib_bias_sel(&mut self) -> IbBiasSelW<'_, Reg4mOscCtrlSpec> {
        IbBiasSelW::new(self, 0)
    }
    #[doc = "Bit 5 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn dmy_0(&mut self) -> Dmy0W<'_, Reg4mOscCtrlSpec> {
        Dmy0W::new(self, 5)
    }
    #[doc = "Bit 7 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn c_sel(&mut self) -> CSelW<'_, Reg4mOscCtrlSpec> {
        CSelW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_4m_osc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_4m_osc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg4mOscCtrlSpec;
impl crate::RegisterSpec for Reg4mOscCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_4m_osc_ctrl::R`](R) reader structure"]
impl crate::Readable for Reg4mOscCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_4m_osc_ctrl::W`](W) writer structure"]
impl crate::Writable for Reg4mOscCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_4M_OSC_CTRL to value 0"]
impl crate::Resettable for Reg4mOscCtrlSpec {}
