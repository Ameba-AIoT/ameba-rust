#[doc = "Register `REG_WL_FUNC_EN` reader"]
pub type R = crate::R<RegWlFuncEnSpec>;
#[doc = "Register `REG_WL_FUNC_EN` writer"]
pub type W = crate::W<RegWlFuncEnSpec>;
#[doc = "Field `BIT_FEN_MAC_PMC` reader - 1: enable MAC PMC; 0: reset"]
pub type BitFenMacPmcR = crate::BitReader;
#[doc = "Field `BIT_FEN_MAC_PMC` writer - 1: enable MAC PMC; 0: reset"]
pub type BitFenMacPmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FEN_WLOFF` reader - 1: enable WLOFF; 0: reset"]
pub type BitFenWloffR = crate::BitReader;
#[doc = "Field `BIT_FEN_WLOFF` writer - 1: enable WLOFF; 0: reset"]
pub type BitFenWloffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FEN_WLON` reader - 1: enable WL partial ON; 0: reset"]
pub type BitFenWlonR = crate::BitReader;
#[doc = "Field `BIT_FEN_WLON` writer - 1: enable WL partial ON; 0: reset"]
pub type BitFenWlonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BT_EN_WL_CK4M` reader - enable 4m clk of wlon if bt is active or switches to active"]
pub type BitBtEnWlCk4mR = crate::BitReader;
#[doc = "Field `BIT_BT_EN_WL_CK4M` writer - enable 4m clk of wlon if bt is active or switches to active"]
pub type BitBtEnWlCk4mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_FEN_BB_GLB_RSTn_V2` reader - When this bit is set to \"0\", whole BB is reset. When this bit is set, BB is enabled."]
pub type BitFenBbGlbRstnV2R = crate::BitReader;
#[doc = "Field `BIT_FEN_BB_GLB_RSTn_V2` writer - When this bit is set to \"0\", whole BB is reset. When this bit is set, BB is enabled."]
pub type BitFenBbGlbRstnV2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FEN_BBRSTB_V2` reader - When this bit is set to \"0\", CCK and OFDM are disabled, and clock are gated. Otherwise, CCK and OFDM are enabled."]
pub type BitFenBbrstbV2R = crate::BitReader;
#[doc = "Field `BIT_FEN_BBRSTB_V2` writer - When this bit is set to \"0\", CCK and OFDM are disabled, and clock are gated. Otherwise, CCK and OFDM are enabled."]
pub type BitFenBbrstbV2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BB_SAVE_EN` reader - 1 to 0:gen req to restore register in power on flow 1:gen req to backup register in power on flow"]
pub type BitBbSaveEnR = crate::BitReader;
#[doc = "Field `BIT_BB_SAVE_EN` writer - 1 to 0:gen req to restore register in power on flow 1:gen req to backup register in power on flow"]
pub type BitBbSaveEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_SEL_SW_SI` reader - indicate SW use indirect path to access WLRFC register"]
pub type BitWlSelSwSiR = crate::BitReader;
#[doc = "Field `BIT_WL_SEL_SW_SI` writer - indicate SW use indirect path to access WLRFC register"]
pub type BitWlSelSwSiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable MAC PMC; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_mac_pmc(&self) -> BitFenMacPmcR {
        BitFenMacPmcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable WLOFF; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_wloff(&self) -> BitFenWloffR {
        BitFenWloffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable WL partial ON; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_wlon(&self) -> BitFenWlonR {
        BitFenWlonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - enable 4m clk of wlon if bt is active or switches to active"]
    #[inline(always)]
    pub fn bit_bt_en_wl_ck4m(&self) -> BitBtEnWlCk4mR {
        BitBtEnWlCk4mR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - When this bit is set to \"0\", whole BB is reset. When this bit is set, BB is enabled."]
    #[inline(always)]
    pub fn bit_fen_bb_glb_rstn_v2(&self) -> BitFenBbGlbRstnV2R {
        BitFenBbGlbRstnV2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is set to \"0\", CCK and OFDM are disabled, and clock are gated. Otherwise, CCK and OFDM are enabled."]
    #[inline(always)]
    pub fn bit_fen_bbrstb_v2(&self) -> BitFenBbrstbV2R {
        BitFenBbrstbV2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1 to 0:gen req to restore register in power on flow 1:gen req to backup register in power on flow"]
    #[inline(always)]
    pub fn bit_bb_save_en(&self) -> BitBbSaveEnR {
        BitBbSaveEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - indicate SW use indirect path to access WLRFC register"]
    #[inline(always)]
    pub fn bit_wl_sel_sw_si(&self) -> BitWlSelSwSiR {
        BitWlSelSwSiR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable MAC PMC; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_mac_pmc(&mut self) -> BitFenMacPmcW<'_, RegWlFuncEnSpec> {
        BitFenMacPmcW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable WLOFF; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_wloff(&mut self) -> BitFenWloffW<'_, RegWlFuncEnSpec> {
        BitFenWloffW::new(self, 1)
    }
    #[doc = "Bit 4 - 1: enable WL partial ON; 0: reset"]
    #[inline(always)]
    pub fn bit_fen_wlon(&mut self) -> BitFenWlonW<'_, RegWlFuncEnSpec> {
        BitFenWlonW::new(self, 4)
    }
    #[doc = "Bit 7 - enable 4m clk of wlon if bt is active or switches to active"]
    #[inline(always)]
    pub fn bit_bt_en_wl_ck4m(&mut self) -> BitBtEnWlCk4mW<'_, RegWlFuncEnSpec> {
        BitBtEnWlCk4mW::new(self, 7)
    }
    #[doc = "Bits 12:15 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegWlFuncEnSpec> {
        RsvdW::new(self, 12)
    }
    #[doc = "Bit 16 - When this bit is set to \"0\", whole BB is reset. When this bit is set, BB is enabled."]
    #[inline(always)]
    pub fn bit_fen_bb_glb_rstn_v2(&mut self) -> BitFenBbGlbRstnV2W<'_, RegWlFuncEnSpec> {
        BitFenBbGlbRstnV2W::new(self, 16)
    }
    #[doc = "Bit 17 - When this bit is set to \"0\", CCK and OFDM are disabled, and clock are gated. Otherwise, CCK and OFDM are enabled."]
    #[inline(always)]
    pub fn bit_fen_bbrstb_v2(&mut self) -> BitFenBbrstbV2W<'_, RegWlFuncEnSpec> {
        BitFenBbrstbV2W::new(self, 17)
    }
    #[doc = "Bit 18 - 1 to 0:gen req to restore register in power on flow 1:gen req to backup register in power on flow"]
    #[inline(always)]
    pub fn bit_bb_save_en(&mut self) -> BitBbSaveEnW<'_, RegWlFuncEnSpec> {
        BitBbSaveEnW::new(self, 18)
    }
    #[doc = "Bit 24 - indicate SW use indirect path to access WLRFC register"]
    #[inline(always)]
    pub fn bit_wl_sel_sw_si(&mut self) -> BitWlSelSwSiW<'_, RegWlFuncEnSpec> {
        BitWlSelSwSiW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_func_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_func_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlFuncEnSpec;
impl crate::RegisterSpec for RegWlFuncEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wl_func_en::R`](R) reader structure"]
impl crate::Readable for RegWlFuncEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_func_en::W`](W) writer structure"]
impl crate::Writable for RegWlFuncEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_FUNC_EN to value 0x80"]
impl crate::Resettable for RegWlFuncEnSpec {
    const RESET_VALUE: u32 = 0x80;
}
