#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `En_int_split` reader - When, en_int_route = 0, 0: only int.tdu, tok and rok present. 1: IMR0_reg, IMR1_reg and ISR1_reg present. When en_int_route = 1, this field is 1."]
pub type EnIntSplitR = crate::BitReader;
#[doc = "Field `En_int_split` writer - When, en_int_route = 0, 0: only int.tdu, tok and rok present. 1: IMR0_reg, IMR1_reg and ISR1_reg present. When en_int_route = 1, this field is 1."]
pub type EnIntSplitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `En_int_route` reader - 0: disable interrupt routing, IMR0_reg, IMR1_reg. 1: enable interrupt routing, IMR0_reg, IMR1_reg."]
pub type EnIntRouteR = crate::BitReader;
#[doc = "Field `En_int_route` writer - 0: disable interrupt routing, IMR0_reg, IMR1_reg. 1: enable interrupt routing, IMR0_reg, IMR1_reg."]
pub type EnIntRouteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_mdslv_outedge` reader - this bit works only in phy_mode 0: change mdio output data in falling edge 1: change mdio output data in rising edge"]
pub type RMdslvOutedgeR = crate::BitReader;
#[doc = "Field `r_mdslv_outedge` writer - this bit works only in phy_mode 0: change mdio output data in falling edge 1: change mdio output data in rising edge"]
pub type RMdslvOutedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tso_id_sel` reader - 0: each ID of IP header is same as original ID. 1: increment 1 from original ID. Default value is 0."]
pub type RTsoIdSelR = crate::BitReader;
#[doc = "Field `r_tso_id_sel` writer - 0: each ID of IP header is same as original ID. 1: increment 1 from original ID. Default value is 0."]
pub type RTsoIdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rff_size_sel` reader - Set gmac_rxfifo size. 2'b00: 1KB 2'b01: 1664B 2'b10: 2KB"]
pub type RffSizeSelR = crate::FieldReader;
#[doc = "Field `Rff_size_sel` writer - Set gmac_rxfifo size. 2'b00: 1KB 2'b01: 1664B 2'b10: 2KB"]
pub type RffSizeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 24 - When, en_int_route = 0, 0: only int.tdu, tok and rok present. 1: IMR0_reg, IMR1_reg and ISR1_reg present. When en_int_route = 1, this field is 1."]
    #[inline(always)]
    pub fn en_int_split(&self) -> EnIntSplitR {
        EnIntSplitR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: disable interrupt routing, IMR0_reg, IMR1_reg. 1: enable interrupt routing, IMR0_reg, IMR1_reg."]
    #[inline(always)]
    pub fn en_int_route(&self) -> EnIntRouteR {
        EnIntRouteR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - this bit works only in phy_mode 0: change mdio output data in falling edge 1: change mdio output data in rising edge"]
    #[inline(always)]
    pub fn r_mdslv_outedge(&self) -> RMdslvOutedgeR {
        RMdslvOutedgeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: each ID of IP header is same as original ID. 1: increment 1 from original ID. Default value is 0."]
    #[inline(always)]
    pub fn r_tso_id_sel(&self) -> RTsoIdSelR {
        RTsoIdSelR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Set gmac_rxfifo size. 2'b00: 1KB 2'b01: 1664B 2'b10: 2KB"]
    #[inline(always)]
    pub fn rff_size_sel(&self) -> RffSizeSelR {
        RffSizeSelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - When, en_int_route = 0, 0: only int.tdu, tok and rok present. 1: IMR0_reg, IMR1_reg and ISR1_reg present. When en_int_route = 1, this field is 1."]
    #[inline(always)]
    pub fn en_int_split(&mut self) -> EnIntSplitW<'_, ConfigSpec> {
        EnIntSplitW::new(self, 24)
    }
    #[doc = "Bit 25 - 0: disable interrupt routing, IMR0_reg, IMR1_reg. 1: enable interrupt routing, IMR0_reg, IMR1_reg."]
    #[inline(always)]
    pub fn en_int_route(&mut self) -> EnIntRouteW<'_, ConfigSpec> {
        EnIntRouteW::new(self, 25)
    }
    #[doc = "Bit 26 - this bit works only in phy_mode 0: change mdio output data in falling edge 1: change mdio output data in rising edge"]
    #[inline(always)]
    pub fn r_mdslv_outedge(&mut self) -> RMdslvOutedgeW<'_, ConfigSpec> {
        RMdslvOutedgeW::new(self, 26)
    }
    #[doc = "Bit 27 - 0: each ID of IP header is same as original ID. 1: increment 1 from original ID. Default value is 0."]
    #[inline(always)]
    pub fn r_tso_id_sel(&mut self) -> RTsoIdSelW<'_, ConfigSpec> {
        RTsoIdSelW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Set gmac_rxfifo size. 2'b00: 1KB 2'b01: 1664B 2'b10: 2KB"]
    #[inline(always)]
    pub fn rff_size_sel(&mut self) -> RffSizeSelW<'_, ConfigSpec> {
        RffSizeSelW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x2000_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
