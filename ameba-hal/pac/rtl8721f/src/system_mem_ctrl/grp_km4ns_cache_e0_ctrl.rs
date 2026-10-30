#[doc = "Register `GRP_KM4NS_CACHE_E0_CTRL` reader"]
pub type R = crate::R<GrpKm4nsCacheE0CtrlSpec>;
#[doc = "Register `GRP_KM4NS_CACHE_E0_CTRL` writer"]
pub type W = crate::W<GrpKm4nsCacheE0CtrlSpec>;
#[doc = "Field `grp_km4ns_cache_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpKm4nsCacheE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_km4ns_cache_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpKm4nsCacheE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_km4ns_cache_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpKm4nsCacheE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_km4ns_cache_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpKm4nsCacheE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_km4ns_cache_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpKm4nsCacheE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_km4ns_cache_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpKm4nsCacheE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_km4ns_cache_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpKm4nsCacheE0MselR = crate::FieldReader;
#[doc = "Field `grp_km4ns_cache_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpKm4nsCacheE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `km4ns_cache_e0_rme` reader - RME"]
pub type Km4nsCacheE0RmeR = crate::BitReader;
#[doc = "Field `km4ns_cache_e0_rme` writer - RME"]
pub type Km4nsCacheE0RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4ns_cache_e0_rm` reader - RM"]
pub type Km4nsCacheE0RmR = crate::FieldReader;
#[doc = "Field `km4ns_cache_e0_rm` writer - RM"]
pub type Km4nsCacheE0RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `km4ns_cache_e0_test1` reader - TEST1"]
pub type Km4nsCacheE0Test1R = crate::BitReader;
#[doc = "Field `km4ns_cache_e0_test1` writer - TEST1"]
pub type Km4nsCacheE0Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_pwr_act(&self) -> GrpKm4nsCacheE0PwrActR {
        GrpKm4nsCacheE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_pwr_lps(&self) -> GrpKm4nsCacheE0PwrLpsR {
        GrpKm4nsCacheE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_loop_mode(&self) -> GrpKm4nsCacheE0LoopModeR {
        GrpKm4nsCacheE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_msel(&self) -> GrpKm4nsCacheE0MselR {
        GrpKm4nsCacheE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn km4ns_cache_e0_rme(&self) -> Km4nsCacheE0RmeR {
        Km4nsCacheE0RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn km4ns_cache_e0_rm(&self) -> Km4nsCacheE0RmR {
        Km4nsCacheE0RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn km4ns_cache_e0_test1(&self) -> Km4nsCacheE0Test1R {
        Km4nsCacheE0Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_pwr_act(
        &mut self,
    ) -> GrpKm4nsCacheE0PwrActW<'_, GrpKm4nsCacheE0CtrlSpec> {
        GrpKm4nsCacheE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_pwr_lps(
        &mut self,
    ) -> GrpKm4nsCacheE0PwrLpsW<'_, GrpKm4nsCacheE0CtrlSpec> {
        GrpKm4nsCacheE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_loop_mode(
        &mut self,
    ) -> GrpKm4nsCacheE0LoopModeW<'_, GrpKm4nsCacheE0CtrlSpec> {
        GrpKm4nsCacheE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_km4ns_cache_e0_msel(&mut self) -> GrpKm4nsCacheE0MselW<'_, GrpKm4nsCacheE0CtrlSpec> {
        GrpKm4nsCacheE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn km4ns_cache_e0_rme(&mut self) -> Km4nsCacheE0RmeW<'_, GrpKm4nsCacheE0CtrlSpec> {
        Km4nsCacheE0RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn km4ns_cache_e0_rm(&mut self) -> Km4nsCacheE0RmW<'_, GrpKm4nsCacheE0CtrlSpec> {
        Km4nsCacheE0RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn km4ns_cache_e0_test1(&mut self) -> Km4nsCacheE0Test1W<'_, GrpKm4nsCacheE0CtrlSpec> {
        Km4nsCacheE0Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_km4ns_cache_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_km4ns_cache_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpKm4nsCacheE0CtrlSpec;
impl crate::RegisterSpec for GrpKm4nsCacheE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_km4ns_cache_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpKm4nsCacheE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_km4ns_cache_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpKm4nsCacheE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_KM4NS_CACHE_E0_CTRL to value 0x1310"]
impl crate::Resettable for GrpKm4nsCacheE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
