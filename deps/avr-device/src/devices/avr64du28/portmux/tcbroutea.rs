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
#[doc = "Field `TCB0` reader - TCB0 Output"]
pub type TCB0_R = crate::BitReader<TCB0_A>;
#[doc = "TCB0 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB0_A {
    #[doc = "0: WO: PA2"]
    DEFAULT = 0,
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
    pub fn variant(&self) -> Option<TCB0_A> {
        match self.bits {
            false => Some(TCB0_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
}
#[doc = "Field `TCB0` writer - TCB0 Output"]
pub type TCB0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB0_A, O>;
impl<'a, const O: u8> TCB0_W<'a, O> {
    #[doc = "WO: PA2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB0_A::DEFAULT)
    }
}
#[doc = "Field `TCB1` reader - TCB1 Output"]
pub type TCB1_R = crate::BitReader<TCB1_A>;
#[doc = "TCB1 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB1_A {
    #[doc = "0: WO: PA3"]
    DEFAULT = 0,
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
    pub fn variant(&self) -> Option<TCB1_A> {
        match self.bits {
            false => Some(TCB1_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB1_A::DEFAULT
    }
}
#[doc = "Field `TCB1` writer - TCB1 Output"]
pub type TCB1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCBROUTEA_SPEC, TCB1_A, O>;
impl<'a, const O: u8> TCB1_W<'a, O> {
    #[doc = "WO: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB1_A::DEFAULT)
    }
}
impl R {
    #[doc = "Bit 0 - TCB0 Output"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCB1 Output"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCB0 Output"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<0> {
        TCB0_W::new(self)
    }
    #[doc = "Bit 1 - TCB1 Output"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<1> {
        TCB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCB route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcbroutea](index.html) module"]
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
