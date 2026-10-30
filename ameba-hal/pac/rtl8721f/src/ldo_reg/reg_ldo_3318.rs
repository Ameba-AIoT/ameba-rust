#[doc = "Register `REG_LDO_3318` reader"]
pub type R = crate::R<RegLdo3318Spec>;
#[doc = "Register `REG_LDO_3318` writer"]
pub type W = crate::W<RegLdo3318Spec>;
#[doc = "Field `en_pc_bt_L_3318` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
pub type EnPcBtL3318R = crate::BitReader;
#[doc = "Field `en_pc_bt_L_3318` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
pub type EnPcBtL3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd_regu_L_3318` reader - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode(default) // 1: power-down Mode Note: This field is also controlled by syson pmc . When mldo_tmode is assert , pmc controller is not active"]
pub type PdReguL3318R = crate::BitReader;
#[doc = "Field `pd_regu_L_3318` writer - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode(default) // 1: power-down Mode Note: This field is also controlled by syson pmc . When mldo_tmode is assert , pmc controller is not active"]
pub type PdReguL3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `voadj_L_3318` reader - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 1.655, 1.672, 1.690, 1.708 // 0100, 0101, 0110, 0111: 1.727, 1.745, 1.764, 1.784 // 1000, 1001(default), 1010, 1011: 1.804, 1.825, 1.846, 1.868 // 1100, 1101, 1110, 1111: 1.890, 1.912, 1.935, 1.959 Note: the acture write-able addr move to 0x4C\\[23:20\\] for otp program easy"]
pub type VoadjL3318R = crate::FieldReader;
#[doc = "Field `en_diode_L_3318` reader - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 0: disable clamp // 1: enable clamp (default)"]
pub type EnDiodeL3318R = crate::BitReader;
#[doc = "Field `en_diode_L_3318` writer - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 0: disable clamp // 1: enable clamp (default)"]
pub type EnDiodeL3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_standby_L_3318` reader - //11: iq=1.5uA; load=1mA~10mA (sleep) //10: iq=45uA; load=10mA(Start Up) (normal-0) //00: iq=45uA; load=5mA~160mA (normal-1) Note:this field controlled by syson pmc. When pmc require mode > 0 , and this field indicate pmc request. When pmc require mode is 0, this field has no use When mldo_tmode is assert , pmc controller is not active"]
pub type RegStandbyL3318R = crate::FieldReader;
#[doc = "Field `reg_standby_L_3318` writer - //11: iq=1.5uA; load=1mA~10mA (sleep) //10: iq=45uA; load=10mA(Start Up) (normal-0) //00: iq=45uA; load=5mA~160mA (normal-1) Note:this field controlled by syson pmc. When pmc require mode > 0 , and this field indicate pmc request. When pmc require mode is 0, this field has no use When mldo_tmode is assert , pmc controller is not active"]
pub type RegStandbyL3318W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_itail_half_en_L_3318` reader - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO iq half ctrl // 0(default): iq normal (for normal mode) // 1: iq half (for suspend mode)"]
pub type RegItailHalfEnL3318R = crate::BitReader;
#[doc = "Field `reg_itail_half_en_L_3318` writer - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO iq half ctrl // 0(default): iq normal (for normal mode) // 1: iq half (for suspend mode)"]
pub type RegItailHalfEnL3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_load` reader - 00:0mA 01:200kΩ 10:183Ω 11:183Ω||200kΩ Note:this field controlled by syson pmc. When pmc require dummy on , and this field indicate dummy resident. When pmc require dummy off , this field has no means, analog will see load is 0 When mldo_tmode is assert , pmc controller is not active"]
pub type DmyLoadR = crate::FieldReader;
#[doc = "Field `dmy_load` writer - 00:0mA 01:200kΩ 10:183Ω 11:183Ω||200kΩ Note:this field controlled by syson pmc. When pmc require dummy on , and this field indicate dummy resident. When pmc require dummy off , this field has no means, analog will see load is 0 When mldo_tmode is assert , pmc controller is not active"]
pub type DmyLoadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OP_buffer` reader - 0: PMOS buffer 1: NMOS buffer"]
pub type OpBufferR = crate::BitReader;
#[doc = "Field `OP_buffer` writer - 0: PMOS buffer 1: NMOS buffer"]
pub type OpBufferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Diode` reader - 0: disable force diode 1: enable force diode"]
pub type DiodeR = crate::BitReader;
#[doc = "Field `Diode` writer - 0: disable force diode 1: enable force diode"]
pub type DiodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSV_Diode` reader - 0: disable rsv diode 1: enable rsv diode"]
pub type RsvDiodeR = crate::BitReader;
#[doc = "Field `RSV_Diode` writer - 0: disable rsv diode 1: enable rsv diode"]
pub type RsvDiodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRR_Filter` reader - 0: disable PSRR Filter 1: enable PSRR Filter"]
pub type PsrrFilterR = crate::BitReader;
#[doc = "Field `PSRR_Filter` writer - 0: disable PSRR Filter 1: enable PSRR Filter"]
pub type PsrrFilterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
    #[inline(always)]
    pub fn en_pc_bt_l_3318(&self) -> EnPcBtL3318R {
        EnPcBtL3318R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode(default) // 1: power-down Mode Note: This field is also controlled by syson pmc . When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn pd_regu_l_3318(&self) -> PdReguL3318R {
        PdReguL3318R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 1.655, 1.672, 1.690, 1.708 // 0100, 0101, 0110, 0111: 1.727, 1.745, 1.764, 1.784 // 1000, 1001(default), 1010, 1011: 1.804, 1.825, 1.846, 1.868 // 1100, 1101, 1110, 1111: 1.890, 1.912, 1.935, 1.959 Note: the acture write-able addr move to 0x4C\\[23:20\\] for otp program easy"]
    #[inline(always)]
    pub fn voadj_l_3318(&self) -> VoadjL3318R {
        VoadjL3318R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 0: disable clamp // 1: enable clamp (default)"]
    #[inline(always)]
    pub fn en_diode_l_3318(&self) -> EnDiodeL3318R {
        EnDiodeL3318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - //11: iq=1.5uA; load=1mA~10mA (sleep) //10: iq=45uA; load=10mA(Start Up) (normal-0) //00: iq=45uA; load=5mA~160mA (normal-1) Note:this field controlled by syson pmc. When pmc require mode > 0 , and this field indicate pmc request. When pmc require mode is 0, this field has no use When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn reg_standby_l_3318(&self) -> RegStandbyL3318R {
        RegStandbyL3318R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO iq half ctrl // 0(default): iq normal (for normal mode) // 1: iq half (for suspend mode)"]
    #[inline(always)]
    pub fn reg_itail_half_en_l_3318(&self) -> RegItailHalfEnL3318R {
        RegItailHalfEnL3318R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 00:0mA 01:200kΩ 10:183Ω 11:183Ω||200kΩ Note:this field controlled by syson pmc. When pmc require dummy on , and this field indicate dummy resident. When pmc require dummy off , this field has no means, analog will see load is 0 When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn dmy_load(&self) -> DmyLoadR {
        DmyLoadR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 0: PMOS buffer 1: NMOS buffer"]
    #[inline(always)]
    pub fn op_buffer(&self) -> OpBufferR {
        OpBufferR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0: disable force diode 1: enable force diode"]
    #[inline(always)]
    pub fn diode(&self) -> DiodeR {
        DiodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: disable rsv diode 1: enable rsv diode"]
    #[inline(always)]
    pub fn rsv_diode(&self) -> RsvDiodeR {
        RsvDiodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: disable PSRR Filter 1: enable PSRR Filter"]
    #[inline(always)]
    pub fn psrr_filter(&self) -> PsrrFilterR {
        PsrrFilterR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
    #[inline(always)]
    pub fn en_pc_bt_l_3318(&mut self) -> EnPcBtL3318W<'_, RegLdo3318Spec> {
        EnPcBtL3318W::new(self, 0)
    }
    #[doc = "Bit 1 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode(default) // 1: power-down Mode Note: This field is also controlled by syson pmc . When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn pd_regu_l_3318(&mut self) -> PdReguL3318W<'_, RegLdo3318Spec> {
        PdReguL3318W::new(self, 1)
    }
    #[doc = "Bit 6 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 0: disable clamp // 1: enable clamp (default)"]
    #[inline(always)]
    pub fn en_diode_l_3318(&mut self) -> EnDiodeL3318W<'_, RegLdo3318Spec> {
        EnDiodeL3318W::new(self, 6)
    }
    #[doc = "Bits 7:8 - //11: iq=1.5uA; load=1mA~10mA (sleep) //10: iq=45uA; load=10mA(Start Up) (normal-0) //00: iq=45uA; load=5mA~160mA (normal-1) Note:this field controlled by syson pmc. When pmc require mode > 0 , and this field indicate pmc request. When pmc require mode is 0, this field has no use When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn reg_standby_l_3318(&mut self) -> RegStandbyL3318W<'_, RegLdo3318Spec> {
        RegStandbyL3318W::new(self, 7)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[LDO_UART\\] REG_CTRL // LDO iq half ctrl // 0(default): iq normal (for normal mode) // 1: iq half (for suspend mode)"]
    #[inline(always)]
    pub fn reg_itail_half_en_l_3318(&mut self) -> RegItailHalfEnL3318W<'_, RegLdo3318Spec> {
        RegItailHalfEnL3318W::new(self, 9)
    }
    #[doc = "Bits 10:11 - 00:0mA 01:200kΩ 10:183Ω 11:183Ω||200kΩ Note:this field controlled by syson pmc. When pmc require dummy on , and this field indicate dummy resident. When pmc require dummy off , this field has no means, analog will see load is 0 When mldo_tmode is assert , pmc controller is not active"]
    #[inline(always)]
    pub fn dmy_load(&mut self) -> DmyLoadW<'_, RegLdo3318Spec> {
        DmyLoadW::new(self, 10)
    }
    #[doc = "Bit 12 - 0: PMOS buffer 1: NMOS buffer"]
    #[inline(always)]
    pub fn op_buffer(&mut self) -> OpBufferW<'_, RegLdo3318Spec> {
        OpBufferW::new(self, 12)
    }
    #[doc = "Bit 13 - 0: disable force diode 1: enable force diode"]
    #[inline(always)]
    pub fn diode(&mut self) -> DiodeW<'_, RegLdo3318Spec> {
        DiodeW::new(self, 13)
    }
    #[doc = "Bit 14 - 0: disable rsv diode 1: enable rsv diode"]
    #[inline(always)]
    pub fn rsv_diode(&mut self) -> RsvDiodeW<'_, RegLdo3318Spec> {
        RsvDiodeW::new(self, 14)
    }
    #[doc = "Bit 15 - 0: disable PSRR Filter 1: enable PSRR Filter"]
    #[inline(always)]
    pub fn psrr_filter(&mut self) -> PsrrFilterW<'_, RegLdo3318Spec> {
        PsrrFilterW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ldo_3318::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ldo_3318::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLdo3318Spec;
impl crate::RegisterSpec for RegLdo3318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ldo_3318::R`](R) reader structure"]
impl crate::Readable for RegLdo3318Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ldo_3318::W`](W) writer structure"]
impl crate::Writable for RegLdo3318Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LDO_3318 to value 0x1c22"]
impl crate::Resettable for RegLdo3318Spec {
    const RESET_VALUE: u32 = 0x1c22;
}
