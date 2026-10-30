#[doc = "Register `REG_PRETX_DROP_CTRL` reader"]
pub type R = crate::R<RegPretxDropCtrlSpec>;
#[doc = "Register `REG_PRETX_DROP_CTRL` writer"]
pub type W = crate::W<RegPretxDropCtrlSpec>;
#[doc = "Field `BIT_MGQ_DEL_EN` reader - 1: MGQ packet will be dropped when enable macid delete function; 0: MGQ packet cannot be dropped"]
pub type BitMgqDelEnR = crate::BitReader;
#[doc = "Field `BIT_MGQ_DEL_EN` writer - 1: MGQ packet will be dropped when enable macid delete function; 0: MGQ packet cannot be dropped"]
pub type BitMgqDelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HGQ_DEL_EN` reader - 1: HGQ packet will be dropped when enable macid delete function; 0: HGQ packet cannot be dropped"]
pub type BitHgqDelEnR = crate::BitReader;
#[doc = "Field `BIT_HGQ_DEL_EN` writer - 1: HGQ packet will be dropped when enable macid delete function; 0: HGQ packet cannot be dropped"]
pub type BitHgqDelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRETX_AGGR_TIME_MAX` reader - If BIT_PRETX_AGGR_EN = 1, the bits will count down after PHYTXON signal asserting every transmission in pre-tx mode.(Default: 23F, mac_clk = 20MHz, 3us;Smart,mac_clk = 40MHz, 3.5us)"]
pub type BitPretxAggrTimeMaxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PRETX_AGGR_TIME_MAX` writer - If BIT_PRETX_AGGR_EN = 1, the bits will count down after PHYTXON signal asserting every transmission in pre-tx mode.(Default: 23F, mac_clk = 20MHz, 3us;Smart,mac_clk = 40MHz, 3.5us)"]
pub type BitPretxAggrTimeMaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BIT_PRETX_AGGR_EN` reader - 1: enable PRETX function, default is enable 0: disable PRETX function"]
pub type BitPretxAggrEnR = crate::BitReader;
#[doc = "Field `BIT_PRETX_AGGR_EN` writer - 1: enable PRETX function, default is enable 0: disable PRETX function"]
pub type BitPretxAggrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_COLLI_DETEC_TIME_MAX` reader - 0~15: The field indicated collision detect time for broadcast or Beacon packet.(unit: us)"]
pub type BitColliDetecTimeMaxR = crate::FieldReader;
#[doc = "Field `BIT_COLLI_DETEC_TIME_MAX` writer - 0~15: The field indicated collision detect time for broadcast or Beacon packet.(unit: us)"]
pub type BitColliDetecTimeMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 6 - 1: MGQ packet will be dropped when enable macid delete function; 0: MGQ packet cannot be dropped"]
    #[inline(always)]
    pub fn bit_mgq_del_en(&self) -> BitMgqDelEnR {
        BitMgqDelEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: HGQ packet will be dropped when enable macid delete function; 0: HGQ packet cannot be dropped"]
    #[inline(always)]
    pub fn bit_hgq_del_en(&self) -> BitHgqDelEnR {
        BitHgqDelEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:18 - If BIT_PRETX_AGGR_EN = 1, the bits will count down after PHYTXON signal asserting every transmission in pre-tx mode.(Default: 23F, mac_clk = 20MHz, 3us;Smart,mac_clk = 40MHz, 3.5us)"]
    #[inline(always)]
    pub fn bit_pretx_aggr_time_max(&self) -> BitPretxAggrTimeMaxR {
        BitPretxAggrTimeMaxR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bit 19 - 1: enable PRETX function, default is enable 0: disable PRETX function"]
    #[inline(always)]
    pub fn bit_pretx_aggr_en(&self) -> BitPretxAggrEnR {
        BitPretxAggrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 0~15: The field indicated collision detect time for broadcast or Beacon packet.(unit: us)"]
    #[inline(always)]
    pub fn bit_colli_detec_time_max(&self) -> BitColliDetecTimeMaxR {
        BitColliDetecTimeMaxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - 1: MGQ packet will be dropped when enable macid delete function; 0: MGQ packet cannot be dropped"]
    #[inline(always)]
    pub fn bit_mgq_del_en(&mut self) -> BitMgqDelEnW<'_, RegPretxDropCtrlSpec> {
        BitMgqDelEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: HGQ packet will be dropped when enable macid delete function; 0: HGQ packet cannot be dropped"]
    #[inline(always)]
    pub fn bit_hgq_del_en(&mut self) -> BitHgqDelEnW<'_, RegPretxDropCtrlSpec> {
        BitHgqDelEnW::new(self, 7)
    }
    #[doc = "Bits 8:18 - If BIT_PRETX_AGGR_EN = 1, the bits will count down after PHYTXON signal asserting every transmission in pre-tx mode.(Default: 23F, mac_clk = 20MHz, 3us;Smart,mac_clk = 40MHz, 3.5us)"]
    #[inline(always)]
    pub fn bit_pretx_aggr_time_max(&mut self) -> BitPretxAggrTimeMaxW<'_, RegPretxDropCtrlSpec> {
        BitPretxAggrTimeMaxW::new(self, 8)
    }
    #[doc = "Bit 19 - 1: enable PRETX function, default is enable 0: disable PRETX function"]
    #[inline(always)]
    pub fn bit_pretx_aggr_en(&mut self) -> BitPretxAggrEnW<'_, RegPretxDropCtrlSpec> {
        BitPretxAggrEnW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 0~15: The field indicated collision detect time for broadcast or Beacon packet.(unit: us)"]
    #[inline(always)]
    pub fn bit_colli_detec_time_max(&mut self) -> BitColliDetecTimeMaxW<'_, RegPretxDropCtrlSpec> {
        BitColliDetecTimeMaxW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pretx_drop_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pretx_drop_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPretxDropCtrlSpec;
impl crate::RegisterSpec for RegPretxDropCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pretx_drop_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPretxDropCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pretx_drop_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPretxDropCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PRETX_DROP_CTRL to value 0x00f8_8c00"]
impl crate::Resettable for RegPretxDropCtrlSpec {
    const RESET_VALUE: u32 = 0x00f8_8c00;
}
