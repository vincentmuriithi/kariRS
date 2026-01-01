#[doc = "Register `USERCCLLUT1B` reader"]
pub struct R(crate::R<USERCCLLUT1B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERCCLLUT1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERCCLLUT1B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERCCLLUT1B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERCCLLUT1B` writer"]
pub struct W(crate::W<USERCCLLUT1B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERCCLLUT1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USERCCLLUT1B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERCCLLUT1B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL` reader - Channel selector"]
pub type CHANNEL_R = crate::FieldReader<u8, CHANNEL_A>;
#[doc = "Channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Connect user to event channel 0"]
    CHANNEL0 = 1,
    #[doc = "2: Connect user to event channel 1"]
    CHANNEL1 = 2,
    #[doc = "3: Connect user to event channel 2"]
    CHANNEL2 = 3,
    #[doc = "4: Connect user to event channel 3"]
    CHANNEL3 = 4,
    #[doc = "5: Connect user to event channel 4"]
    CHANNEL4 = 5,
    #[doc = "6: Connect user to event channel 5"]
    CHANNEL5 = 6,
}
impl From<CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_A) -> Self {
        variant as _
    }
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNEL_A> {
        match self.bits {
            0 => Some(CHANNEL_A::OFF),
            1 => Some(CHANNEL_A::CHANNEL0),
            2 => Some(CHANNEL_A::CHANNEL1),
            3 => Some(CHANNEL_A::CHANNEL2),
            4 => Some(CHANNEL_A::CHANNEL3),
            5 => Some(CHANNEL_A::CHANNEL4),
            6 => Some(CHANNEL_A::CHANNEL5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CHANNEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == CHANNEL_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == CHANNEL_A::CHANNEL1
    }
    #[doc = "Checks if the value of the field is `CHANNEL2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == CHANNEL_A::CHANNEL2
    }
    #[doc = "Checks if the value of the field is `CHANNEL3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == CHANNEL_A::CHANNEL3
    }
    #[doc = "Checks if the value of the field is `CHANNEL4`"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == CHANNEL_A::CHANNEL4
    }
    #[doc = "Checks if the value of the field is `CHANNEL5`"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == CHANNEL_A::CHANNEL5
    }
}
#[doc = "Field `CHANNEL` writer - Channel selector"]
pub type CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, USERCCLLUT1B_SPEC, u8, CHANNEL_A, 8, O>;
impl<'a, const O: u8> CHANNEL_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CHANNEL_A::OFF)
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL0)
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL1)
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL2)
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL3)
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL4)
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut W {
        self.variant(CHANNEL_A::CHANNEL5)
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel selector"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<0> {
        CHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User CCL LUT1 Event B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userccllut1b](index.html) module"]
pub struct USERCCLLUT1B_SPEC;
impl crate::RegisterSpec for USERCCLLUT1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [userccllut1b::R](R) reader structure"]
impl crate::Readable for USERCCLLUT1B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userccllut1b::W](W) writer structure"]
impl crate::Writable for USERCCLLUT1B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERCCLLUT1B to value 0"]
impl crate::Resettable for USERCCLLUT1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
