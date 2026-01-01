#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<SINGLE_CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<SINGLE_CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_CTRLA_SPEC>;
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
impl From<crate::W<SINGLE_CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Module Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Module Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLA_SPEC, bool, O>;
#[doc = "Field `CLKSEL` reader - Clock Selection"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: System Clock"]
    DIV1 = 0,
    #[doc = "1: System Clock / 2"]
    DIV2 = 1,
    #[doc = "2: System Clock / 4"]
    DIV4 = 2,
    #[doc = "3: System Clock / 8"]
    DIV8 = 3,
    #[doc = "4: System Clock / 16"]
    DIV16 = 4,
    #[doc = "5: System Clock / 64"]
    DIV64 = 5,
    #[doc = "6: System Clock / 256"]
    DIV256 = 6,
    #[doc = "7: System Clock / 1024"]
    DIV1024 = 7,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::DIV1,
            1 => CLKSEL_A::DIV2,
            2 => CLKSEL_A::DIV4,
            3 => CLKSEL_A::DIV8,
            4 => CLKSEL_A::DIV16,
            5 => CLKSEL_A::DIV64,
            6 => CLKSEL_A::DIV256,
            7 => CLKSEL_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKSEL_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKSEL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKSEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CLKSEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CLKSEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CLKSEL_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CLKSEL_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CLKSEL_A::DIV1024
    }
}
#[doc = "Field `CLKSEL` writer - Clock Selection"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SINGLE_CTRLA_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "System Clock"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV1)
    }
    #[doc = "System Clock / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV2)
    }
    #[doc = "System Clock / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV4)
    }
    #[doc = "System Clock / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV8)
    }
    #[doc = "System Clock / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV16)
    }
    #[doc = "System Clock / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV64)
    }
    #[doc = "System Clock / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV256)
    }
    #[doc = "System Clock / 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIV1024)
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:3 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<1> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_ctrla](index.html) module"]
pub struct SINGLE_CTRLA_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_ctrla::R](R) reader structure"]
impl crate::Readable for SINGLE_CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_ctrla::W](W) writer structure"]
impl crate::Writable for SINGLE_CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for SINGLE_CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
