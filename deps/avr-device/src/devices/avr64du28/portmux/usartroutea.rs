#[doc = "Register `USARTROUTEA` reader"]
pub struct R(crate::R<USARTROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USARTROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USARTROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USARTROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USARTROUTEA` writer"]
pub struct W(crate::W<USARTROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USARTROUTEA_SPEC>;
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
impl From<crate::W<USARTROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USARTROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART0` reader - USART0 Signals"]
pub type USART0_R = crate::FieldReader<u8, USART0_A>;
#[doc = "USART0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART0_A {
    #[doc = "0: TxD: PA0, RxD: PA1, XCK: PA2, XDIR: PA3"]
    DEFAULT = 0,
    #[doc = "1: TxD: PA4, RxD: PA5, XCK: PA6, XDIR: PA7"]
    ALT1 = 1,
    #[doc = "2: TxD: PA2, RxD: PA3, XCK: -, XDIR: -"]
    ALT2 = 2,
    #[doc = "3: TxD: PD4, RxD: PD5, XCK: PD6, XDIR: PD7"]
    ALT3 = 3,
    #[doc = "5: Not connected to any pins"]
    NONE = 5,
}
impl From<USART0_A> for u8 {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as _
    }
}
impl USART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART0_A> {
        match self.bits {
            0 => Some(USART0_A::DEFAULT),
            1 => Some(USART0_A::ALT1),
            2 => Some(USART0_A::ALT2),
            3 => Some(USART0_A::ALT3),
            5 => Some(USART0_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART0_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == USART0_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == USART0_A::ALT3
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART0_A::NONE
    }
}
#[doc = "Field `USART0` writer - USART0 Signals"]
pub type USART0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, USARTROUTEA_SPEC, u8, USART0_A, 3, O>;
impl<'a, const O: u8> USART0_W<'a, O> {
    #[doc = "TxD: PA0, RxD: PA1, XCK: PA2, XDIR: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(USART0_A::DEFAULT)
    }
    #[doc = "TxD: PA4, RxD: PA5, XCK: PA6, XDIR: PA7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(USART0_A::ALT1)
    }
    #[doc = "TxD: PA2, RxD: PA3, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(USART0_A::ALT2)
    }
    #[doc = "TxD: PD4, RxD: PD5, XCK: PD6, XDIR: PD7"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(USART0_A::ALT3)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(USART0_A::NONE)
    }
}
#[doc = "Field `USART1` reader - USART1 Signals"]
pub type USART1_R = crate::FieldReader<u8, USART1_A>;
#[doc = "USART1 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1_A {
    #[doc = "0: Not connected to any pins"]
    DEFAULT = 0,
    #[doc = "2: TxD: PD6, RxD: PD7, XCK: -, XDIR: -"]
    ALT2 = 2,
}
impl From<USART1_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1_A) -> Self {
        variant as _
    }
}
impl USART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART1_A> {
        match self.bits {
            0 => Some(USART1_A::DEFAULT),
            2 => Some(USART1_A::ALT2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART1_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == USART1_A::ALT2
    }
}
#[doc = "Field `USART1` writer - USART1 Signals"]
pub type USART1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, USARTROUTEA_SPEC, u8, USART1_A, 2, O>;
impl<'a, const O: u8> USART1_W<'a, O> {
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(USART1_A::DEFAULT)
    }
    #[doc = "TxD: PD6, RxD: PD7, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(USART1_A::ALT2)
    }
}
impl R {
    #[doc = "Bits 0:2 - USART0 Signals"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - USART1 Signals"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<0> {
        USART0_W::new(self)
    }
    #[doc = "Bits 3:4 - USART1 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<3> {
        USART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usartroutea](index.html) module"]
pub struct USARTROUTEA_SPEC;
impl crate::RegisterSpec for USARTROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usartroutea::R](R) reader structure"]
impl crate::Readable for USARTROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usartroutea::W](W) writer structure"]
impl crate::Writable for USARTROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USARTROUTEA to value 0"]
impl crate::Resettable for USARTROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
