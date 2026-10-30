#[doc = "Register `REG_DSS_CTRL` reader"]
pub type R = crate::R<RegDssCtrlSpec>;
#[doc = "Register `REG_DSS_CTRL` writer"]
pub type W = crate::W<RegDssCtrlSpec>;
#[doc = "Field `dss_data_in` reader - "]
pub type DssDataInR = crate::FieldReader<u32>;
#[doc = "Field `dss_data_in` writer - "]
pub type DssDataInW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `dss_ro_sel` reader - "]
pub type DssRoSelR = crate::FieldReader;
#[doc = "Field `dss_ro_sel` writer - "]
pub type DssRoSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dss_speed_en` reader - "]
pub type DssSpeedEnR = crate::BitReader;
#[doc = "Field `dss_speed_en` writer - "]
pub type DssSpeedEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_wire_sel` reader - "]
pub type DssWireSelR = crate::BitReader;
#[doc = "Field `dss_wire_sel` writer - "]
pub type DssWireSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_rstn` reader - "]
pub type DssRstnR = crate::FieldReader;
#[doc = "Field `dss_rstn` writer - "]
pub type DssRstnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dss_data_in(&self) -> DssDataInR {
        DssDataInR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dss_ro_sel(&self) -> DssRoSelR {
        DssRoSelR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dss_speed_en(&self) -> DssSpeedEnR {
        DssSpeedEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dss_wire_sel(&self) -> DssWireSelR {
        DssWireSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dss_rstn(&self) -> DssRstnR {
        DssRstnR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dss_data_in(&mut self) -> DssDataInW<'_, RegDssCtrlSpec> {
        DssDataInW::new(self, 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dss_ro_sel(&mut self) -> DssRoSelW<'_, RegDssCtrlSpec> {
        DssRoSelW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dss_speed_en(&mut self) -> DssSpeedEnW<'_, RegDssCtrlSpec> {
        DssSpeedEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dss_wire_sel(&mut self) -> DssWireSelW<'_, RegDssCtrlSpec> {
        DssWireSelW::new(self, 25)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dss_rstn(&mut self) -> DssRstnW<'_, RegDssCtrlSpec> {
        DssRstnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dss_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dss_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDssCtrlSpec;
impl crate::RegisterSpec for RegDssCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dss_ctrl::R`](R) reader structure"]
impl crate::Readable for RegDssCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dss_ctrl::W`](W) writer structure"]
impl crate::Writable for RegDssCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DSS_CTRL to value 0"]
impl crate::Resettable for RegDssCtrlSpec {}
