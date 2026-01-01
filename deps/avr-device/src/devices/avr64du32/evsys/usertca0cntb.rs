#[doc = "Register `USERTCA0CNTB` reader"]
pub struct R(crate::R<USERTCA0CNTB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERTCA0CNTB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERTCA0CNTB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERTCA0CNTB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERTCA0CNTB` writer"]
pub struct W(crate::W<USERTCA0CNTB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERTCA0CNTB_SPEC>;
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
impl From<crate::W<USERTCA0CNTB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERTCA0CNTB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER` reader - User channel select"]
pub type USER_R = crate::FieldReader<u8, USER_A>;
#[doc = "User channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USER_A {
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
impl From<USER_A> for u8 {
    #[inline(always)]
    fn from(variant: USER_A) -> Self {
        variant as _
    }
}
impl USER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USER_A> {
        match self.bits {
            0 => Some(USER_A::OFF),
            1 => Some(USER_A::CHANNEL0),
            2 => Some(USER_A::CHANNEL1),
            3 => Some(USER_A::CHANNEL2),
            4 => Some(USER_A::CHANNEL3),
            5 => Some(USER_A::CHANNEL4),
            6 => Some(USER_A::CHANNEL5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == USER_A::OFF
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == USER_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USER_A::CHANNEL1
    }
    #[doc = "Checks if the value of the field is `CHANNEL2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == USER_A::CHANNEL2
    }
    #[doc = "Checks if the value of the field is `CHANNEL3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == USER_A::CHANNEL3
    }
    #[doc = "Checks if the value of the field is `CHANNEL4`"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == USER_A::CHANNEL4
    }
    #[doc = "Checks if the value of the field is `CHANNEL5`"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == USER_A::CHANNEL5
    }
}
#[doc = "Field `USER` writer - User channel select"]
pub type USER_W<'a, const O: u8> = crate::FieldWriter<'a, u8, USERTCA0CNTB_SPEC, u8, USER_A, 8, O>;
impl<'a, const O: u8> USER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(USER_A::OFF)
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL0)
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL1)
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL2)
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL3)
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL4)
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut W {
        self.variant(USER_A::CHANNEL5)
    }
}
impl R {
    #[doc = "Bits 0:7 - User channel select"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - User channel select"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<0> {
        USER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCA0 Event B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usertca0cntb](index.html) module"]
pub struct USERTCA0CNTB_SPEC;
impl crate::RegisterSpec for USERTCA0CNTB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usertca0cntb::R](R) reader structure"]
impl crate::Readable for USERTCA0CNTB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usertca0cntb::W](W) writer structure"]
impl crate::Writable for USERTCA0CNTB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERTCA0CNTB to value 0"]
impl crate::Resettable for USERTCA0CNTB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
