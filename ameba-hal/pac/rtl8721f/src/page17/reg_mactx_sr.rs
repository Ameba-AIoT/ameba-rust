#[doc = "Register `REG_MACTX_SR` reader"]
pub type R = crate::R<RegMactxSrSpec>;
#[doc = "Register `REG_MACTX_SR` writer"]
pub type W = crate::W<RegMactxSrSpec>;
#[doc = "Field `r_WMAC_PLCP_SR_FIELD` reader - value of SR field"]
pub type RWmacPlcpSrFieldR = crate::FieldReader<u16>;
#[doc = "Field `r_WMAC_PLCP_SR_FIELD` writer - value of SR field"]
pub type RWmacPlcpSrFieldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_WMAC_SR_VAL15_ALLOW` reader - sr field can be the value 15"]
pub type RWmacSrVal15AllowR = crate::BitReader;
#[doc = "Field `r_WMAC_SR_VAL15_ALLOW` writer - sr field can be the value 15"]
pub type RWmacSrVal15AllowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_SR_FIELD_EN` reader - enable sr field reg cfg"]
pub type RWmacSrFieldEnR = crate::BitReader;
#[doc = "Field `r_WMAC_SR_FIELD_EN` writer - enable sr field reg cfg"]
pub type RWmacSrFieldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_WAIT_PHY_REF` reader - mactx hang handle, reset mactx if the phy_txen is set and wait the begin pulse of phy_txon timeout"]
pub type RWmacWaitPhyRefR = crate::FieldReader;
#[doc = "Field `r_WMAC_WAIT_PHY_REF` writer - mactx hang handle, reset mactx if the phy_txen is set and wait the begin pulse of phy_txon timeout"]
pub type RWmacWaitPhyRefW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - value of SR field"]
    #[inline(always)]
    pub fn r_wmac_plcp_sr_field(&self) -> RWmacPlcpSrFieldR {
        RWmacPlcpSrFieldR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - sr field can be the value 15"]
    #[inline(always)]
    pub fn r_wmac_sr_val15_allow(&self) -> RWmacSrVal15AllowR {
        RWmacSrVal15AllowR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable sr field reg cfg"]
    #[inline(always)]
    pub fn r_wmac_sr_field_en(&self) -> RWmacSrFieldEnR {
        RWmacSrFieldEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - mactx hang handle, reset mactx if the phy_txen is set and wait the begin pulse of phy_txon timeout"]
    #[inline(always)]
    pub fn r_wmac_wait_phy_ref(&self) -> RWmacWaitPhyRefR {
        RWmacWaitPhyRefR::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - value of SR field"]
    #[inline(always)]
    pub fn r_wmac_plcp_sr_field(&mut self) -> RWmacPlcpSrFieldW<'_, RegMactxSrSpec> {
        RWmacPlcpSrFieldW::new(self, 0)
    }
    #[doc = "Bit 16 - sr field can be the value 15"]
    #[inline(always)]
    pub fn r_wmac_sr_val15_allow(&mut self) -> RWmacSrVal15AllowW<'_, RegMactxSrSpec> {
        RWmacSrVal15AllowW::new(self, 16)
    }
    #[doc = "Bit 17 - enable sr field reg cfg"]
    #[inline(always)]
    pub fn r_wmac_sr_field_en(&mut self) -> RWmacSrFieldEnW<'_, RegMactxSrSpec> {
        RWmacSrFieldEnW::new(self, 17)
    }
    #[doc = "Bits 18:23 - mactx hang handle, reset mactx if the phy_txen is set and wait the begin pulse of phy_txon timeout"]
    #[inline(always)]
    pub fn r_wmac_wait_phy_ref(&mut self) -> RWmacWaitPhyRefW<'_, RegMactxSrSpec> {
        RWmacWaitPhyRefW::new(self, 18)
    }
}
#[doc = "SR field configure\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mactx_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mactx_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMactxSrSpec;
impl crate::RegisterSpec for RegMactxSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mactx_sr::R`](R) reader structure"]
impl crate::Readable for RegMactxSrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mactx_sr::W`](W) writer structure"]
impl crate::Writable for RegMactxSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACTX_SR to value 0x0040_0000"]
impl crate::Resettable for RegMactxSrSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
