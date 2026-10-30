#[doc = "Register `SLOT_STAVRE` reader"]
pub type R = crate::R<SlotStavreSpec>;
#[doc = "Register `SLOT_STAVRE` writer"]
pub type W = crate::W<SlotStavreSpec>;
#[doc = "Field `RD_STARVE_CYC` reader - The timeout of read slot would be set when read cmd is no servicing in the interval time= (RD_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
pub type RdStarveCycR = crate::FieldReader;
#[doc = "Field `RD_STARVE_CYC` writer - The timeout of read slot would be set when read cmd is no servicing in the interval time= (RD_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
pub type RdStarveCycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_STARVE_CYC` reader - The timeout of write slot would be set when write cmd is no servicing in the interval time= (WR_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
pub type WrStarveCycR = crate::FieldReader;
#[doc = "Field `WR_STARVE_CYC` writer - The timeout of write slot would be set when write cmd is no servicing in the interval time= (WR_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
pub type WrStarveCycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BASE_STARVE_CYC` reader - This is basic starvation counter cycles for WR_STARVE_CYC and RD_STARVE_CYC. Exists: TAG >= 2.0.5."]
pub type BaseStarveCycR = crate::FieldReader;
#[doc = "Field `BASE_STARVE_CYC` writer - This is basic starvation counter cycles for WR_STARVE_CYC and RD_STARVE_CYC. Exists: TAG >= 2.0.5."]
pub type BaseStarveCycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_TIMEOUT` reader - If the current slot has no command and another slot has command, or both write/read slot have no command and current slot is write slot, to wait IDLE_TIMEOUT cycles and turnaround slot if current slot still has no command Exists: TAG >= 2.0.5."]
pub type IdleTimeoutR = crate::FieldReader<u16>;
#[doc = "Field `IDLE_TIMEOUT` writer - If the current slot has no command and another slot has command, or both write/read slot have no command and current slot is write slot, to wait IDLE_TIMEOUT cycles and turnaround slot if current slot still has no command Exists: TAG >= 2.0.5."]
pub type IdleTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - The timeout of read slot would be set when read cmd is no servicing in the interval time= (RD_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn rd_starve_cyc(&self) -> RdStarveCycR {
        RdStarveCycR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The timeout of write slot would be set when write cmd is no servicing in the interval time= (WR_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn wr_starve_cyc(&self) -> WrStarveCycR {
        WrStarveCycR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - This is basic starvation counter cycles for WR_STARVE_CYC and RD_STARVE_CYC. Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn base_starve_cyc(&self) -> BaseStarveCycR {
        BaseStarveCycR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - If the current slot has no command and another slot has command, or both write/read slot have no command and current slot is write slot, to wait IDLE_TIMEOUT cycles and turnaround slot if current slot still has no command Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn idle_timeout(&self) -> IdleTimeoutR {
        IdleTimeoutR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - The timeout of read slot would be set when read cmd is no servicing in the interval time= (RD_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn rd_starve_cyc(&mut self) -> RdStarveCycW<'_, SlotStavreSpec> {
        RdStarveCycW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The timeout of write slot would be set when write cmd is no servicing in the interval time= (WR_STARVE_CYC+1) x (BASE_STARVE_CYC+1) Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn wr_starve_cyc(&mut self) -> WrStarveCycW<'_, SlotStavreSpec> {
        WrStarveCycW::new(self, 4)
    }
    #[doc = "Bits 8:15 - This is basic starvation counter cycles for WR_STARVE_CYC and RD_STARVE_CYC. Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn base_starve_cyc(&mut self) -> BaseStarveCycW<'_, SlotStavreSpec> {
        BaseStarveCycW::new(self, 8)
    }
    #[doc = "Bits 16:31 - If the current slot has no command and another slot has command, or both write/read slot have no command and current slot is write slot, to wait IDLE_TIMEOUT cycles and turnaround slot if current slot still has no command Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn idle_timeout(&mut self) -> IdleTimeoutW<'_, SlotStavreSpec> {
        IdleTimeoutW::new(self, 16)
    }
}
#[doc = "Slot Starvation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slot_stavre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slot_stavre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotStavreSpec;
impl crate::RegisterSpec for SlotStavreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slot_stavre::R`](R) reader structure"]
impl crate::Readable for SlotStavreSpec {}
#[doc = "`write(|w| ..)` method takes [`slot_stavre::W`](W) writer structure"]
impl crate::Writable for SlotStavreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLOT_STAVRE to value 0x30"]
impl crate::Resettable for SlotStavreSpec {
    const RESET_VALUE: u32 = 0x30;
}
