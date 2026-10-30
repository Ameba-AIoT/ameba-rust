#[doc = "Register `REG_PHYSTS_REL` reader"]
pub type R = crate::R<RegPhystsRelSpec>;
#[doc = "Register `REG_PHYSTS_REL` writer"]
pub type W = crate::W<RegPhystsRelSpec>;
#[doc = "Field `BIT_PHY_SYS_POLL` writer - physts release polling,one pulse"]
pub type BitPhySysPollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_REG_DBGCTRL` reader - Dummy reg"]
pub type BitRegDbgctrlR = crate::FieldReader;
#[doc = "Field `BIT_REG_DBGCTRL` writer - Dummy reg"]
pub type BitRegDbgctrlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_PHY_STS_ADDR` reader - physts write address,8 bytes unit"]
pub type BitPhyStsAddrR = crate::FieldReader;
#[doc = "Field `BIT_PHY_STS_NUM` reader - physts packet number in physts buffer"]
pub type BitPhyStsNumR = crate::FieldReader;
#[doc = "Field `BIT_PHY_STS_STR` reader - physts read address,8 bytes unit"]
pub type BitPhyStsStrR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:7 - Dummy reg"]
    #[inline(always)]
    pub fn bit_reg_dbgctrl(&self) -> BitRegDbgctrlR {
        BitRegDbgctrlR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - physts write address,8 bytes unit"]
    #[inline(always)]
    pub fn bit_phy_sts_addr(&self) -> BitPhyStsAddrR {
        BitPhyStsAddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - physts packet number in physts buffer"]
    #[inline(always)]
    pub fn bit_phy_sts_num(&self) -> BitPhyStsNumR {
        BitPhyStsNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - physts read address,8 bytes unit"]
    #[inline(always)]
    pub fn bit_phy_sts_str(&self) -> BitPhyStsStrR {
        BitPhyStsStrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - physts release polling,one pulse"]
    #[inline(always)]
    pub fn bit_phy_sys_poll(&mut self) -> BitPhySysPollW<'_, RegPhystsRelSpec> {
        BitPhySysPollW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Dummy reg"]
    #[inline(always)]
    pub fn bit_reg_dbgctrl(&mut self) -> BitRegDbgctrlW<'_, RegPhystsRelSpec> {
        BitRegDbgctrlW::new(self, 1)
    }
}
#[doc = "PHYSTS REALSE\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_physts_rel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_physts_rel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPhystsRelSpec;
impl crate::RegisterSpec for RegPhystsRelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_physts_rel::R`](R) reader structure"]
impl crate::Readable for RegPhystsRelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_physts_rel::W`](W) writer structure"]
impl crate::Writable for RegPhystsRelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PHYSTS_REL to value 0"]
impl crate::Resettable for RegPhystsRelSpec {}
