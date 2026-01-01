#[doc = "Register `MCLKCTRLB` reader"]
pub struct R(crate::R<MCLKCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCTRLB` writer"]
pub struct W(crate::W<MCLKCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCTRLB_SPEC>;
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
impl From<crate::W<MCLKCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN` reader - Prescaler enable"]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - Prescaler enable"]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKCTRLB_SPEC, bool, O>;
#[doc = "Field `PDIV` reader - Prescaler division"]
pub type PDIV_R = crate::FieldReader<u8, PDIV_A>;
#[doc = "Prescaler division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV_A {
    #[doc = "0: Divide by 2"]
    DIV2 = 0,
    #[doc = "1: Divide by 4"]
    DIV4 = 1,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "3: Divide by 16"]
    DIV16 = 3,
    #[doc = "4: Divide by 32"]
    DIV32 = 4,
    #[doc = "5: Divide by 64"]
    DIV64 = 5,
    #[doc = "8: Divide by 6"]
    DIV6 = 8,
    #[doc = "9: Divide by 10"]
    DIV10 = 9,
    #[doc = "10: Divide by 12"]
    DIV12 = 10,
    #[doc = "11: Divide by 24"]
    DIV24 = 11,
    #[doc = "12: Divide by 48"]
    DIV48 = 12,
}
impl From<PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDIV_A) -> Self {
        variant as _
    }
}
impl PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDIV_A> {
        match self.bits {
            0 => Some(PDIV_A::DIV2),
            1 => Some(PDIV_A::DIV4),
            2 => Some(PDIV_A::DIV8),
            3 => Some(PDIV_A::DIV16),
            4 => Some(PDIV_A::DIV32),
            5 => Some(PDIV_A::DIV64),
            8 => Some(PDIV_A::DIV6),
            9 => Some(PDIV_A::DIV10),
            10 => Some(PDIV_A::DIV12),
            11 => Some(PDIV_A::DIV24),
            12 => Some(PDIV_A::DIV48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PDIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PDIV_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PDIV_A::DIV48
    }
}
#[doc = "Field `PDIV` writer - Prescaler division"]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MCLKCTRLB_SPEC, u8, PDIV_A, 4, O>;
impl<'a, const O: u8> PDIV_W<'a, O> {
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PDIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PDIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PDIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PDIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PDIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PDIV_A::DIV64)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PDIV_A::DIV6)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PDIV_A::DIV10)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PDIV_A::DIV12)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PDIV_A::DIV24)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(PDIV_A::DIV48)
    }
}
impl R {
    #[doc = "Bit 0 - Prescaler enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Prescaler division"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits >> 1) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<0> {
        PEN_W::new(self)
    }
    #[doc = "Bits 1:4 - Prescaler division"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<1> {
        PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkctrlb](index.html) module"]
pub struct MCLKCTRLB_SPEC;
impl crate::RegisterSpec for MCLKCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkctrlb::R](R) reader structure"]
impl crate::Readable for MCLKCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkctrlb::W](W) writer structure"]
impl crate::Writable for MCLKCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLB to value 0"]
impl crate::Resettable for MCLKCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
