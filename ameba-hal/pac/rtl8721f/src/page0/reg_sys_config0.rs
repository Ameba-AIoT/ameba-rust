#[doc = "Register `REG_SYS_CONFIG0` reader"]
pub type R = crate::R<RegSysConfig0Spec>;
#[doc = "Field `BIT_PLL_CK_RDY` reader - indicate AFE PLL clock is ready"]
pub type BitPllCkRdyR = crate::BitReader;
#[doc = "Field `BIT_VENDOR_ID_V1` reader - Vendor ID \\[7:6\\] : Download FW version ; 00: TSMC , 01: SMIC, 10: UMC \\[5\\] ; 0 : Download FW , init BB and RF registers 1 : Do not download FW and do not init BB and RF registers \\[4\\] ; 0 : BT do not exist in IC 1 : BT exist in IC"]
pub type BitVendorIdV1R = crate::FieldReader;
#[doc = "Field `BIT_IS_SCH_CLK_INSLP_V1` reader - WL TSF clock status: 0: MAC clock 1: 32K clock"]
pub type BitIsSchClkInslpV1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - indicate AFE PLL clock is ready"]
    #[inline(always)]
    pub fn bit_pll_ck_rdy(&self) -> BitPllCkRdyR {
        BitPllCkRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Vendor ID \\[7:6\\] : Download FW version ; 00: TSMC , 01: SMIC, 10: UMC \\[5\\] ; 0 : Download FW , init BB and RF registers 1 : Do not download FW and do not init BB and RF registers \\[4\\] ; 0 : BT do not exist in IC 1 : BT exist in IC"]
    #[inline(always)]
    pub fn bit_vendor_id_v1(&self) -> BitVendorIdV1R {
        BitVendorIdV1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - WL TSF clock status: 0: MAC clock 1: 32K clock"]
    #[inline(always)]
    pub fn bit_is_sch_clk_inslp_v1(&self) -> BitIsSchClkInslpV1R {
        BitIsSchClkInslpV1R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sys_config0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSysConfig0Spec;
impl crate::RegisterSpec for RegSysConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sys_config0::R`](R) reader structure"]
impl crate::Readable for RegSysConfig0Spec {}
#[doc = "`reset()` method sets REG_SYS_CONFIG0 to value 0x80"]
impl crate::Resettable for RegSysConfig0Spec {
    const RESET_VALUE: u32 = 0x80;
}
