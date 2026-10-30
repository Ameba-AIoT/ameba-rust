#[doc = "Register `Key_Setup_Trigger` reader"]
pub type R = crate::R<KeySetupTriggerSpec>;
#[doc = "Register `Key_Setup_Trigger` writer"]
pub type W = crate::W<KeySetupTriggerSpec>;
#[doc = "Field `Key_Setup_Trigger_Key_Manager_Idle` reader - Write 1 to trigger key setup process If the selected key is a hardware key, assert hw_key_req and wait for hw_key_ack. The key setup process will not start until hw_key_ack is received. The key managment unit uses the APB interface to pass the key into the engine according to the selected key. The read data represents whether the engine is idle or not. 0: Key setup process ongoing 1: Key setup is completed After triggering the key setup process, read this bit to confirm completion."]
pub type KeySetupTriggerKeyManagerIdleR = crate::BitReader;
#[doc = "Field `Key_Setup_Trigger_Key_Manager_Idle` writer - Write 1 to trigger key setup process If the selected key is a hardware key, assert hw_key_req and wait for hw_key_ack. The key setup process will not start until hw_key_ack is received. The key managment unit uses the APB interface to pass the key into the engine according to the selected key. The read data represents whether the engine is idle or not. 0: Key setup process ongoing 1: Key setup is completed After triggering the key setup process, read this bit to confirm completion."]
pub type KeySetupTriggerKeyManagerIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Allow_Nsec_Use_Sec_key` reader - When setting this field to 1, allow non-secure domain to trigger the secure key. Only secure domain can write this bit. The corresponding bits will be numbered from smallest to largest. For example, the number of the secure domain key is 0, 1, 33. Bits 0, 1, 2 control key0, key1, key33 respectively."]
pub type AllowNsecUseSecKeyR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Write 1 to trigger key setup process If the selected key is a hardware key, assert hw_key_req and wait for hw_key_ack. The key setup process will not start until hw_key_ack is received. The key managment unit uses the APB interface to pass the key into the engine according to the selected key. The read data represents whether the engine is idle or not. 0: Key setup process ongoing 1: Key setup is completed After triggering the key setup process, read this bit to confirm completion."]
    #[inline(always)]
    pub fn key_setup_trigger_key_manager_idle(&self) -> KeySetupTriggerKeyManagerIdleR {
        KeySetupTriggerKeyManagerIdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - When setting this field to 1, allow non-secure domain to trigger the secure key. Only secure domain can write this bit. The corresponding bits will be numbered from smallest to largest. For example, the number of the secure domain key is 0, 1, 33. Bits 0, 1, 2 control key0, key1, key33 respectively."]
    #[inline(always)]
    pub fn allow_nsec_use_sec_key(&self) -> AllowNsecUseSecKeyR {
        AllowNsecUseSecKeyR::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to trigger key setup process If the selected key is a hardware key, assert hw_key_req and wait for hw_key_ack. The key setup process will not start until hw_key_ack is received. The key managment unit uses the APB interface to pass the key into the engine according to the selected key. The read data represents whether the engine is idle or not. 0: Key setup process ongoing 1: Key setup is completed After triggering the key setup process, read this bit to confirm completion."]
    #[inline(always)]
    pub fn key_setup_trigger_key_manager_idle(
        &mut self,
    ) -> KeySetupTriggerKeyManagerIdleW<'_, KeySetupTriggerSpec> {
        KeySetupTriggerKeyManagerIdleW::new(self, 0)
    }
}
#[doc = "Key Setup Process Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`key_setup_trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_setup_trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySetupTriggerSpec;
impl crate::RegisterSpec for KeySetupTriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_setup_trigger::R`](R) reader structure"]
impl crate::Readable for KeySetupTriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`key_setup_trigger::W`](W) writer structure"]
impl crate::Writable for KeySetupTriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key_Setup_Trigger to value 0x01"]
impl crate::Resettable for KeySetupTriggerSpec {
    const RESET_VALUE: u32 = 0x01;
}
