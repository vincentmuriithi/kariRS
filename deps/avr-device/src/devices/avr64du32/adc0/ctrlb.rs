#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - Clock Pre-scaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "Clock Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: CLK_PER divided by 2"]
    DIV2 = 0,
    #[doc = "1: CLK_PER divided by 4"]
    DIV4 = 1,
    #[doc = "2: CLK_PER divided by 6"]
    DIV6 = 2,
    #[doc = "3: CLK_PER divided by 8"]
    DIV8 = 3,
    #[doc = "4: CLK_PER divided by 10"]
    DIV10 = 4,
    #[doc = "5: CLK_PER divided by 12"]
    DIV12 = 5,
    #[doc = "6: CLK_PER divided by 14"]
    DIV14 = 6,
    #[doc = "7: CLK_PER divided by 16"]
    DIV16 = 7,
    #[doc = "8: CLK_PER divided by 20"]
    DIV20 = 8,
    #[doc = "9: CLK_PER divided by 24"]
    DIV24 = 9,
    #[doc = "10: CLK_PER divided by 28"]
    DIV28 = 10,
    #[doc = "11: CLK_PER divided by 32"]
    DIV32 = 11,
    #[doc = "12: CLK_PER divided by 40"]
    DIV40 = 12,
    #[doc = "13: CLK_PER divided by 48"]
    DIV48 = 13,
    #[doc = "14: CLK_PER divided by 56"]
    DIV56 = 14,
    #[doc = "15: CLK_PER divided by 64"]
    DIV64 = 15,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV2,
            1 => PRESC_A::DIV4,
            2 => PRESC_A::DIV6,
            3 => PRESC_A::DIV8,
            4 => PRESC_A::DIV10,
            5 => PRESC_A::DIV12,
            6 => PRESC_A::DIV14,
            7 => PRESC_A::DIV16,
            8 => PRESC_A::DIV20,
            9 => PRESC_A::DIV24,
            10 => PRESC_A::DIV28,
            11 => PRESC_A::DIV32,
            12 => PRESC_A::DIV40,
            13 => PRESC_A::DIV48,
            14 => PRESC_A::DIV56,
            15 => PRESC_A::DIV64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PRESC_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PRESC_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PRESC_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PRESC_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV40`"]
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PRESC_A::DIV40
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PRESC_A::DIV48
    }
    #[doc = "Checks if the value of the field is `DIV56`"]
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PRESC_A::DIV56
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
}
#[doc = "Field `PRESC` writer - Clock Pre-scaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, PRESC_A, 4, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "CLK_PER divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "CLK_PER divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "CLK_PER divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESC_A::DIV6)
    }
    #[doc = "CLK_PER divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "CLK_PER divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESC_A::DIV10)
    }
    #[doc = "CLK_PER divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESC_A::DIV12)
    }
    #[doc = "CLK_PER divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PRESC_A::DIV14)
    }
    #[doc = "CLK_PER divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "CLK_PER divided by 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PRESC_A::DIV20)
    }
    #[doc = "CLK_PER divided by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PRESC_A::DIV24)
    }
    #[doc = "CLK_PER divided by 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PRESC_A::DIV28)
    }
    #[doc = "CLK_PER divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "CLK_PER divided by 40"]
    #[inline(always)]
    pub fn div40(self) -> &'a mut W {
        self.variant(PRESC_A::DIV40)
    }
    #[doc = "CLK_PER divided by 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(PRESC_A::DIV48)
    }
    #[doc = "CLK_PER divided by 56"]
    #[inline(always)]
    pub fn div56(self) -> &'a mut W {
        self.variant(PRESC_A::DIV56)
    }
    #[doc = "CLK_PER divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Pre-scaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
