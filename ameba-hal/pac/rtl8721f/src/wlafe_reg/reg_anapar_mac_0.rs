#[doc = "Register `REG_ANAPAR_MAC_0` reader"]
pub type R = crate::R<RegAnaparMac0Spec>;
#[doc = "Register `REG_ANAPAR_MAC_0` writer"]
pub type W = crate::W<RegAnaparMac0Spec>;
#[doc = "Field `reg_monckselh` reader - ck_mon selection 111: 160MHz 001: 110MHz 010: 101MHz 100: 320MHz 011: ck_200M_test 010: ck_sync 001: 5MHz (duty50-50) 000: ck_test"]
pub type RegMonckselhR = crate::FieldReader;
#[doc = "Field `reg_monckselh` writer - ck_mon selection 111: 160MHz 001: 110MHz 010: 101MHz 100: 320MHz 011: ck_200M_test 010: ck_sync 001: 5MHz (duty50-50) 000: ck_test"]
pub type RegMonckselhW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ps_en` reader - Phase swallow enable"]
pub type RegPsEnR = crate::BitReader;
#[doc = "Field `reg_ps_en` writer - Phase swallow enable"]
pub type RegPsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ps` reader - Phase swallow setting"]
pub type RegPsR = crate::FieldReader;
#[doc = "Field `reg_ps` writer - Phase swallow setting"]
pub type RegPsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `null0` reader - "]
pub type Null0R = crate::FieldReader;
#[doc = "Field `null0` writer - "]
pub type Null0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_testen` reader - 1: Enable ck_test"]
pub type RegTestenR = crate::BitReader;
#[doc = "Field `reg_testen` writer - 1: Enable ck_test"]
pub type RegTestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_en_mon` reader - 1: Enable 0: Output \"1\""]
pub type RegEnMonR = crate::BitReader;
#[doc = "Field `reg_en_mon` writer - 1: Enable 0: Output \"1\""]
pub type RegEnMonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cp_bia` reader - pll: CP: current: selection: (efuse) 000: 5uA 001: 10uA 010: 15uA 011: 20uA 100: 25uA 101: 30uA 110: 35uA 111: 40uA"]
pub type RegCpBiaR = crate::FieldReader;
#[doc = "Field `reg_cp_bia` writer - pll: CP: current: selection: (efuse) 000: 5uA 001: 10uA 010: 15uA 011: 20uA 100: 25uA 101: 30uA 110: 35uA 111: 40uA"]
pub type RegCpBiaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_vc_th` reader - Tie vc to 3/4 VDD"]
pub type RegVcThR = crate::BitReader;
#[doc = "Field `reg_vc_th` writer - Tie vc to 3/4 VDD"]
pub type RegVcThW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_vc_tl` reader - Tie vc to 1/4 VDD"]
pub type RegVcTlR = crate::BitReader;
#[doc = "Field `reg_vc_tl` writer - Tie vc to 1/4 VDD"]
pub type RegVcTlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dogenb` reader - enable watch dog 0: Enable 1: Disable"]
pub type RegDogenbR = crate::BitReader;
#[doc = "Field `reg_dogenb` writer - enable watch dog 0: Enable 1: Disable"]
pub type RegDogenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ck320_sel` reader - Ck 320MHz selection, 1 from ck_ext"]
pub type RegCk320SelR = crate::BitReader;
#[doc = "Field `reg_ck320_sel` writer - Ck 320MHz selection, 1 from ck_ext"]
pub type RegCk320SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_c3_set` reader - Set LPF c3 00/01/10/11: 0.4pF/0.8pF/1.2pF/1.6pF"]
pub type RegC3SetR = crate::FieldReader;
#[doc = "Field `reg_c3_set` writer - Set LPF c3 00/01/10/11: 0.4pF/0.8pF/1.2pF/1.6pF"]
pub type RegC3SetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cp_set` reader - Set LPF cp 00/01/10/11: 0.8pF/1.2pF/1.6pF/2.0pF"]
pub type RegCpSetR = crate::FieldReader;
#[doc = "Field `reg_cp_set` writer - Set LPF cp 00/01/10/11: 0.8pF/1.2pF/1.6pF/2.0pF"]
pub type RegCpSetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cs_set` reader - Set LPF cs 00/01/10/11: 12pF/20pF/24pF/32pF"]
pub type RegCsSetR = crate::FieldReader;
#[doc = "Field `reg_cs_set` writer - Set LPF cs 00/01/10/11: 12pF/20pF/24pF/32pF"]
pub type RegCsSetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_r3_set` reader - Set LPF r3 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
pub type RegR3SetR = crate::FieldReader;
#[doc = "Field `reg_r3_set` writer - Set LPF r3 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
pub type RegR3SetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_rs_set` reader - Set LPF rs 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
pub type RegRsSetR = crate::FieldReader;
#[doc = "Field `reg_rs_set` writer - Set LPF rs 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
pub type RegRsSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_cp_offset` reader - CP current offset 1: on 0:off"]
pub type RegCpOffsetR = crate::BitReader;
#[doc = "Field `reg_cp_offset` writer - CP current offset 1: on 0:off"]
pub type RegCpOffsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK5M_EN` reader - 1: Enable 0: Output \"1\""]
pub type Ck5mEnR = crate::BitReader;
#[doc = "Field `CK5M_EN` writer - 1: Enable 0: Output \"1\""]
pub type Ck5mEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ck_mon selection 111: 160MHz 001: 110MHz 010: 101MHz 100: 320MHz 011: ck_200M_test 010: ck_sync 001: 5MHz (duty50-50) 000: ck_test"]
    #[inline(always)]
    pub fn reg_monckselh(&self) -> RegMonckselhR {
        RegMonckselhR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Phase swallow enable"]
    #[inline(always)]
    pub fn reg_ps_en(&self) -> RegPsEnR {
        RegPsEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Phase swallow setting"]
    #[inline(always)]
    pub fn reg_ps(&self) -> RegPsR {
        RegPsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn null0(&self) -> Null0R {
        Null0R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 1: Enable ck_test"]
    #[inline(always)]
    pub fn reg_testen(&self) -> RegTestenR {
        RegTestenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn reg_en_mon(&self) -> RegEnMonR {
        RegEnMonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - pll: CP: current: selection: (efuse) 000: 5uA 001: 10uA 010: 15uA 011: 20uA 100: 25uA 101: 30uA 110: 35uA 111: 40uA"]
    #[inline(always)]
    pub fn reg_cp_bia(&self) -> RegCpBiaR {
        RegCpBiaR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Tie vc to 3/4 VDD"]
    #[inline(always)]
    pub fn reg_vc_th(&self) -> RegVcThR {
        RegVcThR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tie vc to 1/4 VDD"]
    #[inline(always)]
    pub fn reg_vc_tl(&self) -> RegVcTlR {
        RegVcTlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - enable watch dog 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn reg_dogenb(&self) -> RegDogenbR {
        RegDogenbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ck 320MHz selection, 1 from ck_ext"]
    #[inline(always)]
    pub fn reg_ck320_sel(&self) -> RegCk320SelR {
        RegCk320SelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Set LPF c3 00/01/10/11: 0.4pF/0.8pF/1.2pF/1.6pF"]
    #[inline(always)]
    pub fn reg_c3_set(&self) -> RegC3SetR {
        RegC3SetR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Set LPF cp 00/01/10/11: 0.8pF/1.2pF/1.6pF/2.0pF"]
    #[inline(always)]
    pub fn reg_cp_set(&self) -> RegCpSetR {
        RegCpSetR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Set LPF cs 00/01/10/11: 12pF/20pF/24pF/32pF"]
    #[inline(always)]
    pub fn reg_cs_set(&self) -> RegCsSetR {
        RegCsSetR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Set LPF r3 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
    #[inline(always)]
    pub fn reg_r3_set(&self) -> RegR3SetR {
        RegR3SetR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Set LPF rs 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
    #[inline(always)]
    pub fn reg_rs_set(&self) -> RegRsSetR {
        RegRsSetR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - CP current offset 1: on 0:off"]
    #[inline(always)]
    pub fn reg_cp_offset(&self) -> RegCpOffsetR {
        RegCpOffsetR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck5m_en(&self) -> Ck5mEnR {
        Ck5mEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ck_mon selection 111: 160MHz 001: 110MHz 010: 101MHz 100: 320MHz 011: ck_200M_test 010: ck_sync 001: 5MHz (duty50-50) 000: ck_test"]
    #[inline(always)]
    pub fn reg_monckselh(&mut self) -> RegMonckselhW<'_, RegAnaparMac0Spec> {
        RegMonckselhW::new(self, 0)
    }
    #[doc = "Bit 3 - Phase swallow enable"]
    #[inline(always)]
    pub fn reg_ps_en(&mut self) -> RegPsEnW<'_, RegAnaparMac0Spec> {
        RegPsEnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Phase swallow setting"]
    #[inline(always)]
    pub fn reg_ps(&mut self) -> RegPsW<'_, RegAnaparMac0Spec> {
        RegPsW::new(self, 4)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn null0(&mut self) -> Null0W<'_, RegAnaparMac0Spec> {
        Null0W::new(self, 7)
    }
    #[doc = "Bit 9 - 1: Enable ck_test"]
    #[inline(always)]
    pub fn reg_testen(&mut self) -> RegTestenW<'_, RegAnaparMac0Spec> {
        RegTestenW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn reg_en_mon(&mut self) -> RegEnMonW<'_, RegAnaparMac0Spec> {
        RegEnMonW::new(self, 10)
    }
    #[doc = "Bits 11:13 - pll: CP: current: selection: (efuse) 000: 5uA 001: 10uA 010: 15uA 011: 20uA 100: 25uA 101: 30uA 110: 35uA 111: 40uA"]
    #[inline(always)]
    pub fn reg_cp_bia(&mut self) -> RegCpBiaW<'_, RegAnaparMac0Spec> {
        RegCpBiaW::new(self, 11)
    }
    #[doc = "Bit 14 - Tie vc to 3/4 VDD"]
    #[inline(always)]
    pub fn reg_vc_th(&mut self) -> RegVcThW<'_, RegAnaparMac0Spec> {
        RegVcThW::new(self, 14)
    }
    #[doc = "Bit 15 - Tie vc to 1/4 VDD"]
    #[inline(always)]
    pub fn reg_vc_tl(&mut self) -> RegVcTlW<'_, RegAnaparMac0Spec> {
        RegVcTlW::new(self, 15)
    }
    #[doc = "Bit 16 - enable watch dog 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn reg_dogenb(&mut self) -> RegDogenbW<'_, RegAnaparMac0Spec> {
        RegDogenbW::new(self, 16)
    }
    #[doc = "Bit 17 - Ck 320MHz selection, 1 from ck_ext"]
    #[inline(always)]
    pub fn reg_ck320_sel(&mut self) -> RegCk320SelW<'_, RegAnaparMac0Spec> {
        RegCk320SelW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Set LPF c3 00/01/10/11: 0.4pF/0.8pF/1.2pF/1.6pF"]
    #[inline(always)]
    pub fn reg_c3_set(&mut self) -> RegC3SetW<'_, RegAnaparMac0Spec> {
        RegC3SetW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Set LPF cp 00/01/10/11: 0.8pF/1.2pF/1.6pF/2.0pF"]
    #[inline(always)]
    pub fn reg_cp_set(&mut self) -> RegCpSetW<'_, RegAnaparMac0Spec> {
        RegCpSetW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Set LPF cs 00/01/10/11: 12pF/20pF/24pF/32pF"]
    #[inline(always)]
    pub fn reg_cs_set(&mut self) -> RegCsSetW<'_, RegAnaparMac0Spec> {
        RegCsSetW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Set LPF r3 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
    #[inline(always)]
    pub fn reg_r3_set(&mut self) -> RegR3SetW<'_, RegAnaparMac0Spec> {
        RegR3SetW::new(self, 24)
    }
    #[doc = "Bits 27:29 - Set LPF rs 000~111: 1k/3k/5k/7k/9k/11k/13k/15k"]
    #[inline(always)]
    pub fn reg_rs_set(&mut self) -> RegRsSetW<'_, RegAnaparMac0Spec> {
        RegRsSetW::new(self, 27)
    }
    #[doc = "Bit 30 - CP current offset 1: on 0:off"]
    #[inline(always)]
    pub fn reg_cp_offset(&mut self) -> RegCpOffsetW<'_, RegAnaparMac0Spec> {
        RegCpOffsetW::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck5m_en(&mut self) -> Ck5mEnW<'_, RegAnaparMac0Spec> {
        Ck5mEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_mac_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_mac_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparMac0Spec;
impl crate::RegisterSpec for RegAnaparMac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_mac_0::R`](R) reader structure"]
impl crate::Readable for RegAnaparMac0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_mac_0::W`](W) writer structure"]
impl crate::Writable for RegAnaparMac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_MAC_0 to value 0x2090_0008"]
impl crate::Resettable for RegAnaparMac0Spec {
    const RESET_VALUE: u32 = 0x2090_0008;
}
