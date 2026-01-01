#[doc = "Register `TCBROUTEA` reader"]
pub struct R(crate::R<TCBROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCBROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCBROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCBROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCBROUTEA` writer"]
pub struct W(crate::W<TCBROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCBROUTEA_SPEC>;
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
impl From<crate::W<TCBROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCBROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCB0` reader - Port Multiplexer TCB0"]
pub type TCB0_R = crate::BitReader<TCB0_A>;
#[doc = "Port Multiplexer TCB0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB0_A {
    #[doc = "0: WO on PA2"]
    DEFAULT = 0,
    #[doc = "1: WO on PF4"]
    ALT1 = 1,
}
impl From<TCB0_A> for bool {
    #[inline(always)]
    fn from(variant: TCB0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB0_A {
        match self.bits {
            false => TCB0_A::DEFAULT,
            true => TCB0_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TCB0_A::ALT1
    }
}
#[doc = "Field `TCB0` writer - Port Multiplexer TCB0"]
pub type TCB0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB0_A, O>;
impl<'a, const O: u8> TCB0_W<'a, O> {
    #[doc = "WO on PA2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB0_A::DEFAULT)
    }
    #[doc = "WO on PF4"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(TCB0_A::ALT1)
    }
}
#[doc = "Field `TCB1` reader - Port Multiplexer TCB1"]
pub type TCB1_R = crate::BitReader<TCB1_A>;
#[doc = "Port Multiplexer TCB1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB1_A {
    #[doc = "0: WO on PA3"]
    DEFAULT = 0,
    #[doc = "1: WO on PF5"]
    ALT1 = 1,
}
impl From<TCB1_A> for bool {
    #[inline(always)]
    fn from(variant: TCB1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB1_A {
        match self.bits {
            false => TCB1_A::DEFAULT,
            true => TCB1_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB1_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TCB1_A::ALT1
    }
}
#[doc = "Field `TCB1` writer - Port Multiplexer TCB1"]
pub type TCB1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB1_A, O>;
impl<'a, const O: u8> TCB1_W<'a, O> {
    #[doc = "WO on PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB1_A::DEFAULT)
    }
    #[doc = "WO on PF5"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(TCB1_A::ALT1)
    }
}
#[doc = "Field `TCB2` reader - Port Multiplexer TCB2"]
pub type TCB2_R = crate::BitReader<TCB2_A>;
#[doc = "Port Multiplexer TCB2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB2_A {
    #[doc = "0: WO on PC0"]
    DEFAULT = 0,
    #[doc = "1: WO on PB4"]
    ALT1 = 1,
}
impl From<TCB2_A> for bool {
    #[inline(always)]
    fn from(variant: TCB2_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB2_A {
        match self.bits {
            false => TCB2_A::DEFAULT,
            true => TCB2_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB2_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TCB2_A::ALT1
    }
}
#[doc = "Field `TCB2` writer - Port Multiplexer TCB2"]
pub type TCB2_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB2_A, O>;
impl<'a, const O: u8> TCB2_W<'a, O> {
    #[doc = "WO on PC0"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB2_A::DEFAULT)
    }
    #[doc = "WO on PB4"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(TCB2_A::ALT1)
    }
}
#[doc = "Field `TCB3` reader - Port Multiplexer TCB3"]
pub type TCB3_R = crate::BitReader<TCB3_A>;
#[doc = "Port Multiplexer TCB3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB3_A {
    #[doc = "0: WO on PB5"]
    DEFAULT = 0,
    #[doc = "1: WO on PC1"]
    ALT1 = 1,
}
impl From<TCB3_A> for bool {
    #[inline(always)]
    fn from(variant: TCB3_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB3_A {
        match self.bits {
            false => TCB3_A::DEFAULT,
            true => TCB3_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB3_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TCB3_A::ALT1
    }
}
#[doc = "Field `TCB3` writer - Port Multiplexer TCB3"]
pub type TCB3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB3_A, O>;
impl<'a, const O: u8> TCB3_W<'a, O> {
    #[doc = "WO on PB5"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB3_A::DEFAULT)
    }
    #[doc = "WO on PC1"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(TCB3_A::ALT1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer TCB2"]
    #[inline(always)]
    pub fn tcb2(&self) -> TCB2_R {
        TCB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Multiplexer TCB3"]
    #[inline(always)]
    pub fn tcb3(&self) -> TCB3_R {
        TCB3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<0> {
        TCB0_W::new(self)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<1> {
        TCB1_W::new(self)
    }
    #[doc = "Bit 2 - Port Multiplexer TCB2"]
    #[inline(always)]
    #[must_use]
    pub fn tcb2(&mut self) -> TCB2_W<2> {
        TCB2_W::new(self)
    }
    #[doc = "Bit 3 - Port Multiplexer TCB3"]
    #[inline(always)]
    #[must_use]
    pub fn tcb3(&mut self) -> TCB3_W<3> {
        TCB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer TCB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcbroutea](index.html) module"]
pub struct TCBROUTEA_SPEC;
impl crate::RegisterSpec for TCBROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcbroutea::R](R) reader structure"]
impl crate::Readable for TCBROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcbroutea::W](W) writer structure"]
impl crate::Writable for TCBROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCBROUTEA to value 0"]
impl crate::Resettable for TCBROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
