#[doc = "Register `REG_APMC_STA` reader"]
pub type R = crate::R<RegApmcStaSpec>;
#[doc = "Field `cur_instr` reader - regu pmc current instruction code"]
pub type CurInstrR = crate::FieldReader<u16>;
#[doc = "Field `cur_sts_val` reader - current regu status"]
pub type CurStsValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - regu pmc current instruction code"]
    #[inline(always)]
    pub fn cur_instr(&self) -> CurInstrR {
        CurInstrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - current regu status"]
    #[inline(always)]
    pub fn cur_sts_val(&self) -> CurStsValR {
        CurStsValR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_apmc_sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegApmcStaSpec;
impl crate::RegisterSpec for RegApmcStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_apmc_sta::R`](R) reader structure"]
impl crate::Readable for RegApmcStaSpec {}
#[doc = "`reset()` method sets REG_APMC_STA to value 0"]
impl crate::Resettable for RegApmcStaSpec {}
